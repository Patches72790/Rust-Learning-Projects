
pub enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn make_spreadsheet() -> Vec<SpreadsheetCell> {
    Vec::from([
        SpreadsheetCell::Int(42), 
        SpreadsheetCell::Float(3.14159), 
        SpreadsheetCell::Text(String::from("Hello world"))]
    )
}
