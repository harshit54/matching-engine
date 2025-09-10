fn main() {
    let mut vec1: Vec<String> = Vec::new();
    vec1.push(String::from("hello"));
    vec1.push(String::from("world"));

    let mut vec2: Vec<String> = Vec::new();
    for val in vec1.into_iter() {
        vec2.push(val);
    }

}
