use syn::Path;

pub(crate) fn format_path(path: &Path) -> String {
    let mut segments = Vec::with_capacity(path.segments.len());

    for segment in &path.segments {
        segments.push(segment.ident.to_string());
    }

    segments.join("::")
}
