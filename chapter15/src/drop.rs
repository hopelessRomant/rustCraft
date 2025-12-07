struct Data {
    local: String
}

impl Drop for Data {
    fn drop(&mut self) {
        println!("{}", self.local);
    }
}

pub fn drop_test() {
    let s1 = Data{local: "people".to_string()};
    drop(s1); // drop method can not be manually used but drop() function is available in the prelude.
    let _s1 = Data{local: "See you soon !!".to_string()};
    println!("Now both the values will be droped.")
}
