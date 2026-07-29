use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

use syn::Attribute;
use syn::Fields;
use syn::Ident;
use syn::ImplItem;
use syn::Item;
use syn::ItemEnum;
use syn::ItemImpl;
use syn::ItemMod;
use syn::ItemUse;
use syn::Type;
use syn::UseTree;

use crate::attribute_error::AttributeError;
use crate::canonical_path::CanonicalPath;
use crate::field_identifier::FieldIdentifier;
use crate::flatten_use_tree::flatten_use_tree;
use crate::indexed_associated_type::IndexedAssociatedType;
use crate::indexed_trait_impl::IndexedTraitImpl;
use crate::indexed_variant::IndexedVariant;
use crate::item_kind::ItemKind;
use crate::module_imports::ModuleImports;
use crate::resolve_type::resolve_type;
use crate::scanned_field::ScannedField;
use crate::scanned_item::ScannedItem;
use crate::scanned_method::ScannedMethod;
use crate::struct_shape::StructShape;
use crate::walk_output::WalkOutput;

struct Recordable<'item> {
    attributes: &'item [Attribute],
    fields: Vec<ScannedField>,
    identifier: &'item Ident,
    kind: ItemKind,
    variants: Vec<IndexedVariant>,
}

enum PendingMemberKind {
    Method(ScannedMethod),
    TraitImpl {
        associated_types: Vec<IndexedAssociatedType>,
        trait_path: syn::Path,
    },
}

struct PendingMember {
    kind: PendingMemberKind,
    module_path: Vec<String>,
    self_type: Type,
}

fn has_path_attribute(attributes: &[Attribute]) -> bool {
    attributes
        .iter()
        .any(|attribute| attribute.path().is_ident("path"))
}

fn is_crate_root(module_path: &[String]) -> bool {
    module_path.len() == 1
}

fn member_path(module_path: &[String], identifier: &Ident) -> String {
    let mut segments = module_path.to_vec();
    segments.push(identifier.to_string());

    segments.join("::")
}

fn scan_fields(fields: &Fields) -> Vec<ScannedField> {
    fields
        .iter()
        .enumerate()
        .map(|(position, field)| {
            let identifier = match &field.ident {
                Some(name) => FieldIdentifier::Named(name.to_string()),
                None => FieldIdentifier::Positional(position),
            };

            ScannedField::new(identifier, field.ty.clone(), field.attrs.clone())
        })
        .collect()
}

fn index_variants(item_enum: &ItemEnum) -> Vec<IndexedVariant> {
    item_enum
        .variants
        .iter()
        .map(|variant| {
            IndexedVariant::new(
                variant.ident.to_string(),
                StructShape::from(&variant.fields),
            )
        })
        .collect()
}

fn check_use(item_use: &ItemUse, file_path: &Path) -> Result<(), AttributeError> {
    if use_tree_has_glob(&item_use.tree) {
        return Err(AttributeError::GlobImport {
            file: file_path.display().to_string(),
        });
    }

    Ok(())
}

fn use_tree_has_glob(tree: &UseTree) -> bool {
    match tree {
        UseTree::Glob(_) => true,
        UseTree::Path(use_path) => use_tree_has_glob(&use_path.tree),
        UseTree::Group(use_group) => use_group.items.iter().any(use_tree_has_glob),
        UseTree::Name(_) | UseTree::Rename(_) => false,
    }
}

fn resolve_module_file(directory: &Path, identifier: &Ident) -> Result<PathBuf, AttributeError> {
    let file_module = directory.join(format!("{identifier}.rs"));
    let directory_module = directory.join(identifier.to_string()).join("mod.rs");

    match (file_module.is_file(), directory_module.is_file()) {
        (true, true) => Err(AttributeError::ModuleFileCollision {
            module: identifier.to_string(),
            file_module: file_module.display().to_string(),
            directory_module: directory_module.display().to_string(),
        }),
        (true, false) => Ok(file_module),
        (false, true) => Ok(directory_module),
        (false, false) => Err(AttributeError::ModuleFileNotFound {
            module: identifier.to_string(),
            file_module: file_module.display().to_string(),
            directory_module: directory_module.display().to_string(),
        }),
    }
}

pub(crate) struct ModuleWalker {
    excluded_root_modules: BTreeSet<String>,
    imports: HashMap<CanonicalPath, ModuleImports>,
    items: Vec<ScannedItem>,
    pending_members: Vec<PendingMember>,
    seen_paths: HashSet<CanonicalPath>,
}

impl ModuleWalker {
    pub(crate) fn walk_crate(
        crate_name: &str,
        source_directory: &Path,
        excluded_root_modules: &BTreeSet<String>,
    ) -> Result<WalkOutput, AttributeError> {
        let mut walker = Self {
            excluded_root_modules: excluded_root_modules.clone(),
            imports: HashMap::new(),
            items: Vec::new(),
            pending_members: Vec::new(),
            seen_paths: HashSet::new(),
        };
        let root_file = source_directory.join("lib.rs");
        let module_path = vec![crate_name.to_string()];

        walker.walk_file(&root_file, &module_path, source_directory)?;

        Ok(walker.into_output())
    }

    fn into_output(self) -> WalkOutput {
        let Self {
            excluded_root_modules: _,
            imports,
            mut items,
            pending_members,
            seen_paths: _,
        } = self;

        let item_paths: HashSet<CanonicalPath> = items
            .iter()
            .map(|item| item.canonical_path().clone())
            .collect();
        let empty_imports = ModuleImports::default();

        for PendingMember {
            kind,
            module_path,
            self_type,
        } in pending_members
        {
            let module = CanonicalPath::new(module_path.clone());
            let module_imports = imports.get(&module).unwrap_or(&empty_imports);
            let Some(self_type_path) =
                resolve_type(&self_type, &module_path, module_imports, &item_paths)
            else {
                continue;
            };
            let Some(item) = items
                .iter_mut()
                .find(|item| item.canonical_path() == &self_type_path)
            else {
                continue;
            };

            match kind {
                PendingMemberKind::Method(method) => {
                    item.add_method(method);
                }
                PendingMemberKind::TraitImpl {
                    associated_types,
                    trait_path,
                } => {
                    item.add_trait_impl(IndexedTraitImpl::new(
                        trait_path,
                        module_path,
                        associated_types,
                    ));
                }
            }
        }

        items.sort_by(|left, right| left.canonical_path().cmp(right.canonical_path()));

        let items = items
            .into_iter()
            .map(|item| {
                item.resolve(|module_path, path| {
                    let module = CanonicalPath::new(module_path.to_vec());
                    let module_imports = imports.get(&module).unwrap_or(&empty_imports);

                    resolve_type(
                        &Type::Path(syn::TypePath {
                            qself: None,
                            path: path.clone(),
                        }),
                        module_path,
                        module_imports,
                        &item_paths,
                    )
                    .unwrap_or_else(|| {
                        CanonicalPath::new(
                            path.segments
                                .iter()
                                .map(|segment| segment.ident.to_string())
                                .collect(),
                        )
                    })
                })
            })
            .collect();

        WalkOutput { imports, items }
    }

    fn record(
        &mut self,
        recordable: Recordable,
        module_path: &[String],
    ) -> Result<(), AttributeError> {
        let Recordable {
            attributes,
            fields,
            identifier,
            kind,
            variants,
        } = recordable;
        let mut segments = module_path.to_vec();
        segments.push(identifier.to_string());

        let canonical_path = CanonicalPath::new(segments);

        if self.seen_paths.contains(&canonical_path) {
            return Err(AttributeError::DuplicateCanonicalPath {
                path: canonical_path.to_string(),
            });
        }

        self.seen_paths.insert(canonical_path.clone());
        self.items.push(ScannedItem::new(
            kind,
            identifier.to_string(),
            canonical_path,
            attributes.to_vec(),
            fields,
            variants,
        ));

        Ok(())
    }

    fn walk_file(
        &mut self,
        file_path: &Path,
        module_path: &[String],
        directory: &Path,
    ) -> Result<(), AttributeError> {
        let source = match std::fs::read_to_string(file_path) {
            Ok(source) => source,
            Err(source) => {
                return Err(AttributeError::FileRead {
                    path: file_path.display().to_string(),
                    source,
                });
            }
        };
        let parsed = match syn::parse_file(&source) {
            Ok(parsed) => parsed,
            Err(source) => {
                return Err(AttributeError::FileParse {
                    path: file_path.display().to_string(),
                    source,
                });
            }
        };

        self.walk_items(&parsed.items, module_path, file_path, directory)
    }

    fn walk_impl(&mut self, item_impl: &ItemImpl, module_path: &[String]) {
        if let Some((_, trait_path, _)) = &item_impl.trait_ {
            let mut associated_types = Vec::new();

            for impl_item in &item_impl.items {
                if let ImplItem::Type(associated_type) = impl_item {
                    associated_types.push(IndexedAssociatedType::new(
                        associated_type.ident.to_string(),
                        associated_type.ty.clone(),
                    ));
                }
            }

            self.pending_members.push(PendingMember {
                kind: PendingMemberKind::TraitImpl {
                    associated_types,
                    trait_path: trait_path.clone(),
                },
                module_path: module_path.to_vec(),
                self_type: (*item_impl.self_ty).clone(),
            });

            return;
        }

        for impl_item in &item_impl.items {
            if let ImplItem::Fn(method) = impl_item {
                self.pending_members.push(PendingMember {
                    kind: PendingMemberKind::Method(ScannedMethod::new(
                        method.sig.ident.to_string(),
                        method.attrs.clone(),
                        method.sig.clone(),
                        module_path.to_vec(),
                    )),
                    module_path: module_path.to_vec(),
                    self_type: (*item_impl.self_ty).clone(),
                });
            }
        }
    }

    fn walk_item(
        &mut self,
        item: &Item,
        module_path: &[String],
        file_path: &Path,
        directory: &Path,
    ) -> Result<(), AttributeError> {
        let recordable = match item {
            Item::Struct(item_struct) => Some(Recordable {
                attributes: &item_struct.attrs,
                fields: scan_fields(&item_struct.fields),
                identifier: &item_struct.ident,
                kind: ItemKind::Struct(StructShape::from(&item_struct.fields)),
                variants: Vec::new(),
            }),
            Item::Enum(item_enum) => Some(Recordable {
                attributes: &item_enum.attrs,
                fields: Vec::new(),
                identifier: &item_enum.ident,
                kind: ItemKind::Enum,
                variants: index_variants(item_enum),
            }),
            Item::Fn(item_fn) => Some(Recordable {
                attributes: &item_fn.attrs,
                fields: Vec::new(),
                identifier: &item_fn.sig.ident,
                kind: ItemKind::Function,
                variants: Vec::new(),
            }),
            Item::Trait(item_trait) => Some(Recordable {
                attributes: &item_trait.attrs,
                fields: Vec::new(),
                identifier: &item_trait.ident,
                kind: ItemKind::Trait,
                variants: Vec::new(),
            }),
            Item::Mod(item_mod) => {
                if is_crate_root(module_path)
                    && self
                        .excluded_root_modules
                        .contains(&item_mod.ident.to_string())
                {
                    return Ok(());
                }

                self.walk_module(item_mod, module_path, file_path, directory)?;

                Some(Recordable {
                    attributes: &item_mod.attrs,
                    fields: Vec::new(),
                    identifier: &item_mod.ident,
                    kind: ItemKind::Module,
                    variants: Vec::new(),
                })
            }
            Item::Use(item_use) => {
                check_use(item_use, file_path)?;

                for (name, path) in flatten_use_tree(&item_use.tree, module_path) {
                    self.imports
                        .entry(CanonicalPath::new(module_path.to_vec()))
                        .or_default()
                        .insert(name, path);
                }

                None
            }
            Item::Impl(item_impl) => {
                self.walk_impl(item_impl, module_path);

                None
            }
            _ => None,
        };

        match recordable {
            Some(recordable) => self.record(recordable, module_path),
            None => Ok(()),
        }
    }

    fn walk_items(
        &mut self,
        items: &[Item],
        module_path: &[String],
        file_path: &Path,
        directory: &Path,
    ) -> Result<(), AttributeError> {
        for item in items {
            self.walk_item(item, module_path, file_path, directory)?;
        }

        Ok(())
    }

    fn walk_module(
        &mut self,
        item_mod: &ItemMod,
        module_path: &[String],
        file_path: &Path,
        directory: &Path,
    ) -> Result<(), AttributeError> {
        if has_path_attribute(&item_mod.attrs) {
            return Err(AttributeError::ModulePathAttribute {
                module: member_path(module_path, &item_mod.ident),
                file: file_path.display().to_string(),
            });
        }

        let mut child_module_path = module_path.to_vec();
        child_module_path.push(item_mod.ident.to_string());

        let child_directory = directory.join(item_mod.ident.to_string());

        if let Some((_brace, items)) = &item_mod.content {
            self.walk_items(items, &child_module_path, file_path, &child_directory)
        } else {
            let resolved_file = resolve_module_file(directory, &item_mod.ident)?;

            self.walk_file(&resolved_file, &child_module_path, &child_directory)
        }
    }
}
