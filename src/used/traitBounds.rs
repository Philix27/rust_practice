use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}
fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

#[derive(Debug)]
struct Sam {
    name: String,
}

impl Display for Sam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // String::from("I am Sammy")
          write!(f, "I am Sammy")
    }
}
fn main() {
    let obj = Sam {
        name: String::from("Hey"),
    };

    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![3, 3, 3];
    compare_prints(&string);

    compare_prints(&obj);
    //compare_prints(&array);
    // TODO ^ Try uncommenting this.
    compare_types(&array, &vec);
}
