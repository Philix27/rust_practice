mod Cylinder {
    pub struct Cylinder {
        pub height: u16,
        pub width: u16,
        pub volume: u16,
    }

    impl Cylinder {
        pub fn new(h: u16, w: u16, v: u16) -> Self {
            Cylinder {
                height: h,
                width: w,
                volume: v,
            }
        }

        pub fn area(&self) -> u16 {
            let area = self.height * self.width;
            println!("Your perimeter is: {}", area);
            area
        }

        pub fn perimeter(&mut self) -> u16 {
            let peri = self.height + self.width + self.volume;
            println!("Your perimeter is: {}", peri);
            peri
        }
    }
}
fn main() {
    use crate::Cylinder::Cylinder;

    let mut an_obj = Cylinder::new(12, 45, 38);

    an_obj.perimeter();
    an_obj.area();

    let outer_var = 42;

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    println!(
        "cannot reuse closure_inferred with another type: {}",
        closure_inferred(23)
    );
    println!("closure_annotated: {}", closure_annotated(5));

    let one = || 1;
    println!("closure returning one: {}", one());
}
