use sqlite_parser::{
    header::{validate_header_string, parse_page_size},
    error::Error,
};

fn main() -> Result<(), Error> {
    let contents = std::fs::read("data.sqlite").expect("Failed to read data.sqlite.");
    validate_header_string(&contents)?;
    let page_size = parse_page_size(&contents)?;
    println!("{:?}", page_size);
    Ok(())
}