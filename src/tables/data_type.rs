
pub enum DataType {
    Bool,
    Int(u32),
    Float(u32),
    Decimal(u32, u32),
    Datetime,
    String(u32),
    Binary(u32),
}
