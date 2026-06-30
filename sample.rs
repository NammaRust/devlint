use std::env;

fn main() {
    let db = env::var("DATABASE_URL").unwrap();
    let api = env::var("API_KEY").unwrap();
    let secret = env::var("SECRET_KEY").unwrap();
    let missing = env::var("MISSING_VAR").unwrap();

    println!("{db} {api} {secret} {missing}");
}
