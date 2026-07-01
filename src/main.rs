mod env_validator;

use std::fs;

fn main() {
    println!("Issue 4/5/6 Solution\n");

    let rs = fs::read_to_string("sample.rs").expect("sample.rs not found");
    let py = fs::read_to_string("sample.py").expect("sample.py not found");
    let js = fs::read_to_string("sample.js").expect("sample.js not found");

    let mut source = String::new();
    source.push_str(&rs);
    source.push('\n');
    source.push_str(&py);
    source.push('\n');
    source.push_str(&js);

    let extracted = env_validator::extract_env_variables(&source);

    println!("Extracted Variables:");
    for var in &extracted {
        println!("- {}", var);
    }

    let env = env_validator::parse_env_file(".env")
        .expect(".env file not found");

    println!();
    env_validator::validate_env(&extracted, &env);
}