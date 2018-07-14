use util::name::Name;
use tables::data_type::DataType;
use tables::table::Table;

pub struct Column {
    name: Name,
    typ: DataType,
    nullable: bool,
}

impl Column {
    pub fn new(name: Name, typ: DataType) -> Self {
        Column { name, typ, nullable: false }
    }
}
