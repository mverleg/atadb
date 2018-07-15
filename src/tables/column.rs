use tables::data_type::DataType;
use util::name::Name;

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    name: Name,
    typ: DataType,
    nullable: bool,
}

impl Column {
    pub fn new(name: Name, typ: DataType) -> Self {
        Column {
            name,
            typ,
            nullable: false,
        }
    }
}
