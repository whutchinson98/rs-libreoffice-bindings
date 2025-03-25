use rs_libreoffice_bindings::Office;

pub fn main() -> Result<(), String> {
    let office_path = std::env::var("LOK_PATH").expect("LOK_PATH not set");

    println!("initializing lok");
    let office = Office::new(&office_path)?;

    println!("loading document");
    let document = office.load_document("./fixtures/example.xlsx")?;

    document.save_as("./output/example.html", "html", "writer_html")?;
    println!("saved document");

    Ok(())
}
