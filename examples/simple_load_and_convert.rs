use rs_libreoffice_bindings::Office;

pub fn main() -> Result<(), String> {
    let office_path = std::env::var("LOK_PATH").expect("LOK_PATH not set");

    println!("initializing lok");
    let office = Office::new(&office_path)?;

    println!("loading document");
    // Load document
    let document = office.load_document("./fixtures/example.docx")?;

    // Save as PDF
    document.save_as("./example.pdf", "pdf", "writer_pdf_Export")?;

    Ok(())
}
