trait Trait {
    fn method(&self) {
        println!("default impl");
    }
}

struct SomeType;
struct OtherType;

// use default impl for Trait::method
impl Trait for SomeType {}

impl Trait for OtherType {
    // use our own impl for Trait::method
    fn method(&self) {
        println!("OtherType impl");
    }
}

fn main() {
    SomeType.method(); // prints "default impl"
    OtherType.method(); // prints "OtherType impl"
}
