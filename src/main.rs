mod items {
    include!(concat!(env!("OUT_DIR"), "/broken.rs"));
}

fn main() {
    println!("Hello, world!");
}
