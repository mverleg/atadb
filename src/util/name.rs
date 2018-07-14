
#[derive(Debug)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: String) -> Option<Self> {
        for chr in name.chars() {
            if ! chr.is_alphabetic() && chr != '_' {
                return Option::None;
            }
        }
        Option::Some(Name { name })
    }

    pub fn valid(name: &str) -> Self {
        match Self::new(name.to_owned()) {
            Some(name) => name,
            None => panic!(),
        }
    }
}