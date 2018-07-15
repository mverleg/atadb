#[derive(Debug, Serialize, Deserialize)]
pub enum DataType {
    Bool,
    Int(Option<u32>),
    Float(Option<u32>),
    Decimal(Option<u32>, Option<u32>),
    Datetime,
    String(Option<u32>),
    Binary(Option<u32>),
}
