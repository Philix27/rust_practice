// Box
#[warn(dead_code)]
struct LinkedListNode {
    value: i32,
    next: Option<Box<LinkedListNode>>,
}
fn main() {}

// trait declaration generalized with lifetime & type parameters
trait Trait<'a, T> {
    // signature uses generic type
    fn func1(arg: T);

    // signature uses lifetime
    fn func2(arg: &'a i32);

    // signature uses generic type & lifetime
    fn func3(arg: &'a T);
    fn func34<'b>(arg: &'b T);
}

struct SomeType;

impl<'a> Trait<'a, i8> for SomeType {
    fn func1(arg: i8) {}
    fn func2(arg: &'a i32) {}
    fn func3(arg: &'a i8) {}

    fn func34<'b>(arg: &'b i8) {
        todo!()
    }
}

impl<'b> Trait<'b, u8> for SomeType {
    fn func1(arg: u8) {}
    fn func2(arg: &'b i32) {}
    fn func3(arg: &'b u8) {}

    fn func34<'c>(arg: &'c u8) {
        todo!()
    }
}
trait TraitD {
    fn func<'a, T>(t: &'a T);
}
