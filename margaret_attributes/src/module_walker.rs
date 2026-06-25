use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

use syn::Attribute;
use syn::Ident;
use syn::ImplItem;
use syn::Item;
use syn::ItemImpl;
use syn::ItemMod;
use syn::ItemUse;
use syn::Type;
use syn::UseTree;

use crate::attribute_error::AttributeError;
use crate::attribute_holder::AttributeHolder;
use crate::canonical_path::CanonicalPath;
use crate::indexed_item::IndexedItem;
use crate::indexed_method::IndexedMethod;
use crate::item_kind::ItemKind;
use crate::struct_shape::StructShape;

struct Recordable<'item> {
    attributes: &'item [Attribute],
    identifier: &'item Ident,
    kind: ItemKind,
}

pub(crate) struct ModuleWalker {
    holders: Vec<AttributeHolder>,
    seen_paths: HashSet<CanonicalPath>,
}

impl ModuleWalker {
    pub(crate) fn walk_crate(
        crate_name: &str,
        source_directory: &Path,
    ) -> Result<Vec<AttributeHolder>, AttributeError> {
        let mut walker = Self {
            holders: Vec::new(),
            seen_paths: HashSet::new(),
        };
        let root_file = source_directory.join("lib.rs");
        let module_path = vec![crate_name.to_string()];

        walker.walk_file(&root_file, &module_path, source_directory)?;
        walker.holders.sort_by_key(AttributeHolder::target_path);

        Ok(walker.holders)
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
                identifier: &item_struct.ident,
                kind: ItemKind::Struct(StructShape::from(&item_struct.fields)),
            }),
            Item::Enum(item_enum) => Some(Recordable {
                attributes: &item_enum.attrs,
                identifier: &item_enum.ident,
                kind: ItemKind::Enum,
            }),
            Item::Fn(item_fn) => Some(Recordable {
                attributes: &item_fn.attrs,
                identifier: &item_fn.sig.ident,
                kind: ItemKind::Function,
            }),
            Item::Trait(item_trait) => Some(Recordable {
                attributes: &item_trait.attrs,
                identifier: &item_trait.ident,
                kind: ItemKind::Trait,
            }),
            Item::Mod(item_mod) => {
                self.walk_module(item_mod, module_path, file_path, directory)?;

                Some(Recordable {
                    attributes: &item_mod.attrs,
                    identifier: &item_mod.ident,
                    kind: ItemKind::Module,
                })
            }
            Item::Use(item_use) => {
                check_use(item_use, file_path)?;

                None
            }
            Item::Impl(item_impl) => {
                self.walk_impl(item_impl, module_path);

                None
            }
            _ => None,
        };

        match recordable {
            Some(recordable) => self.record(&recordable, module_path),
            None => Ok(()),
        }
    }

    fn record(
        &mut self,
        recordable: &Recordable,
        module_path: &[String],
    ) -> Result<(), AttributeError> {
        let mut segments = module_path.to_vec();
        segments.push(recordable.identifier.to_string());

        let canonical_path = CanonicalPath::new(segments);

        if self.seen_paths.contains(&canonical_path) {
            return Err(AttributeError::DuplicateCanonicalPath {
                path: canonical_path.to_string(),
            });
        }

        self.seen_paths.insert(canonical_path.clone());
        self.holders.push(AttributeHolder::Item(IndexedItem::new(
            recordable.kind,
            recordable.identifier.to_string(),
            canonical_path,
            recordable.attributes.to_vec(),
        )));

        Ok(())
    }

    fn walk_impl(&mut self, item_impl: &ItemImpl, module_path: &[String]) {
        if item_impl.trait_.is_some() {
            return;
        }

        let Some(self_identifier) = self_type_identifier(&item_impl.self_ty) else {
            return;
        };

        let mut segments = module_path.to_vec();
        segments.push(self_identifier.to_string());

        let self_type_path = CanonicalPath::new(segments);

        for impl_item in &item_impl.items {
            if let ImplItem::Fn(method) = impl_item {
                self.holders
                    .push(AttributeHolder::Method(IndexedMethod::new(
                        self_type_path.clone(),
                        method.sig.ident.to_string(),
                        method.attrs.clone(),
                        method.sig.clone(),
                    )));
            }
        }
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

        match &item_mod.content {
            Some((_brace, items)) => {
                self.walk_items(items, &child_module_path, file_path, &child_directory)
            }
            None => {
                let resolved_file = resolve_module_file(directory, &item_mod.ident)?;

                self.walk_file(&resolved_file, &child_module_path, &child_directory)
            }
        }
    }
}

fn self_type_identifier(self_type: &Type) -> Option<&Ident> {
    match self_type {
        Type::Path(type_path) => type_path.path.segments.last().map(|segment| &segment.ident),
        _ => None,
    }
}

fn has_path_attribute(attributes: &[Attribute]) -> bool {
    attributes
        .iter()
        .any(|attribute| attribute.path().is_ident("path"))
}

fn member_path(module_path: &[String], identifier: &Ident) -> String {
    let mut segments = module_path.to_vec();
    segments.push(identifier.to_string());

    segments.join("::")
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
