#[allow(dead_code)]
#[derive(Debug)]
pub struct AstonMartin {
    pub model: String,
    version: String,
}

impl AstonMartin {
    pub fn car (model : &str) -> AstonMartin {
        AstonMartin { model: String::from(model), version: (String::from("0.1.0")) }
    }
}