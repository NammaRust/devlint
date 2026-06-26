use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;
use std::io;


pub fn extract_env_variables(source: &str) -> Vec<String> {
    let mut vars = HashSet::new();

    
    let rust_re = Regex::new(r#"env::var\(\s*"([A-Za-z_][A-Za-z0-9_]*)"\s*\)"#).unwrap();

    
    let python_re = Regex::new(r#"os\.getenv\(\s*"([A-Za-z_][A-Za-z0-9_]*)"\s*\)"#).unwrap();

    
    let js_re = Regex::new(r#"process\.env\.([A-Za-z_][A-Za-z0-9_]*)"#).unwrap();

    for cap in rust_re.captures_iter(source) {
        vars.insert(cap[1].to_string());
    }

    for cap in python_re.captures_iter(source) {
        vars.insert(cap[1].to_string());
    }

    for cap in js_re.captures_iter(source) {
        vars.insert(cap[1].to_string());
    }

    let mut result: Vec<String> = vars.into_iter().collect();
    result.sort();
    result
}

pub fn parse_env_file(path: &str) -> io::Result<HashMap<String, String>> {
    let content = fs::read_to_string(path)?;
    let mut vars = HashMap::new();

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            vars.insert(
                key.trim().to_string(),
                value.trim().to_string(),
            );
        }
    }

    Ok(vars)
}