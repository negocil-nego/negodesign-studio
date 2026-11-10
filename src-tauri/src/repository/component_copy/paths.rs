use std::path::{Path, PathBuf};

pub fn workspace_root() -> Result<PathBuf, String> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "Não foi possível localizar a raiz do projecto NegoDesign.".to_string())
}

pub fn normalize_logical_path(path: &str) -> String {
    let cleaned = path.replace('\\', "/");
    let mut segments: Vec<&str> = Vec::new();
    for segment in cleaned.split('/') {
        match segment {
            "" | "." => continue,
            ".." => {
                segments.pop();
            }
            other => segments.push(other),
        }
    }
    segments.join("/")
}

pub fn resolve_disk_path(root: &Path, logical_path: &str) -> PathBuf {
    let normalized = normalize_logical_path(logical_path);
    if normalized.starts_with("static/") {
        root.join(&normalized)
    } else {
        root.join("src").join(&normalized)
    }
}

pub fn encode_base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((bytes.len() + 2) / 3 * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;

        out.push(TABLE[((triple >> 18) & 0x3F) as usize] as char);
        out.push(TABLE[((triple >> 12) & 0x3F) as usize] as char);
        out.push(if chunk.len() > 1 {
            TABLE[((triple >> 6) & 0x3F) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            TABLE[(triple & 0x3F) as usize] as char
        } else {
            '='
        });
    }
    out
}

pub fn transform_react_or_vue_hooks(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut rest = content;
    while let Some(pos) = rest.find("/hooks/") {
        result.push_str(&rest[..pos]);
        let after = &rest[pos..];
        if let Some(end_quote) = after.find(|c| c == '"' || c == '\'') {
            let hook_path = &after[..end_quote];
            let cleaned = hook_path.strip_suffix(".svelte").unwrap_or(hook_path);
            result.push_str(cleaned);
            rest = &after[end_quote..];
        } else {
            result.push_str(after);
            rest = "";
        }
    }
    result.push_str(rest);
    result
}

