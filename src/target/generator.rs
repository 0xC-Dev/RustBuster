use std::collections::HashSet;

pub fn collect_targets(paths: &Vec<String>, extensions: &Vec<String>, url: &String) -> Vec<(String, String)> {
    paths.iter()
        .flat_map(|path| {
            extensions.iter().map(move |ext| {
                let full_url = if url.ends_with('/') {
                    format!("{}{}{}", url, path, ext)
                } else {
                    format!("{}/{}{}", url, path, ext)
                };
                (full_url, path.clone())
            })
        })
        .collect()
}

pub fn parse_extensions(extensions_input: &str) -> Vec<String> {
    let mut extensions_set: HashSet<String> = extensions_input
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    extensions_set.insert("".to_string()); // ensures it's always included
    extensions_set.into_iter().collect()
}