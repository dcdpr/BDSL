fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=assets/design_tokens.json");
    dtoken::build("assets/design_tokens.json")?;

    Ok(())
}
