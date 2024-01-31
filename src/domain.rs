// domain.rs
pub struct Coin {
    pub name: String,
}

impl Coin {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_lowercase(),
        }
    }
}