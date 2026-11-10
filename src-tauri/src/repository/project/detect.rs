use std::{fs, path::Path};

pub fn detect_language(project_path: &str) -> String {
    let root = Path::new(project_path);
    if root.join("pubspec.yaml").is_file() {
        return "Flutter".to_string();
    }

    let Ok(package_json) = fs::read_to_string(root.join("package.json")) else {
        return "Unknown".to_string();
    };
    let Ok(manifest) = serde_json::from_str::<serde_json::Value>(&package_json) else {
        return "Unknown".to_string();
    };

    let has_dependency = |packages: &[&str]| {
        ["dependencies", "devDependencies", "peerDependencies"]
            .iter()
            .filter_map(|section| {
                manifest
                    .get(*section)
                    .and_then(serde_json::Value::as_object)
            })
            .any(|deps| packages.iter().any(|pkg| deps.contains_key(*pkg)))
    };

    let has_config = |files: &[&str]| files.iter().any(|cfg| root.join(*cfg).is_file());

    if has_config(&["svelte.config.js", "svelte.config.mjs", "svelte.config.ts"])
        || has_dependency(&["svelte", "@sveltejs/kit"])
    {
        return "Svelte".to_string();
    }
    if has_config(&[
        "nuxt.config.js",
        "nuxt.config.mjs",
        "nuxt.config.ts",
        "nuxt.config.mts",
        "nuxt.config.cjs",
    ]) || has_dependency(&["nuxt"])
    {
        return "Nuxt".to_string();
    }
    if has_config(&[
        "next.config.js",
        "next.config.mjs",
        "next.config.ts",
        "next.config.mts",
        "next.config.cjs",
    ]) || has_dependency(&["next"])
    {
        return "Nextjs".to_string();
    }
    if has_config(&["vue.config.js", "vue.config.ts"])
        || has_dependency(&["vue", "@vue/cli-service"])
    {
        return "Vue".to_string();
    }
    if has_dependency(&["react", "react-dom"]) {
        return "React".to_string();
    }
    "Unknown".to_string()
}
