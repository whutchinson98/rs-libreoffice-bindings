use rs_libreoffice_bindings::Office;

pub fn main() -> Result<(), String> {
    let office_path = std::env::var("LOK_PATH").expect("LOK_PATH not set");

    println!("initializing lok");
    let office = Office::new(&office_path)?;

    println!("loading document");
    let document = office.load_document("./fixtures/example.docx")?;

    document.save_as("./output/example.pdf", "pdf", "writer_pdf_Export")?;
    println!("saved document");

    Ok(())
}
