
pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: String) -> Option<Self> {
        for chr in name.chars() {
            if ! chr.is_alphabetic() {
                return Option::None;
            }
        }
        Option::Some(Name { name })
    }
}