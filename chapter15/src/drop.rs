struct Data {
    local: String
}

impl Drop for Data {
    fn drop(&mut self) {
        println!("{}", self.local);
    }
}

pub fn test() {
    let _s1 = Data{local: "people".to_string()};
    let _s1 = Data{local: "See you soon !!".to_string()};
    println!("Now both the values will be droped.")
}