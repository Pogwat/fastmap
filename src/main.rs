use fastmap::FastMap;
fn main() {
    let mut test: FastMap<&str,&str> =  FastMap::new();
    test.insert("hi", "bye");
    println!("Hello, world! {}", test.get(&"hi").unwrap()); //By key indexing
    println!("Hello, world! {}", test[0]); //Index by order of insertion
}
