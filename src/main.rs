trait Greet {
    fn greet(&self, name: &str) -> String;
    fn greet_loudly(&self, name: &str) -> String {
        self.greet(name) + "!"
    }
}

struct Hello;
struct Hola;

// impl Clone for Hello {
//     fn clone_from(&mut self, source: &Self) {
//         *self = source.clone()
//     }

//     fn clone(&self) -> Self {
//         Self {  }
//     }
// }

// impl Copy for Hello {}

impl Greet for Hello {
    fn greet(&self, name: &str) -> String {
        format!("Hello {}", name)
    }
    // use default impl for greet_loudly
}

impl Greet for Hola {
    fn greet(&self, name: &str) -> String {
        format!("Hola {}", name)
    }
    // override default impl
    fn greet_loudly(&self, name: &str) -> String {
        let mut greeting = self.greet(name);
        greeting.insert_str(0, "¡");
        greeting + "!"
    }
}

fn main() {
    println!("{}", Hello.greet("John")); // prints "Hello John"
    println!("{}", Hello.greet_loudly("John")); // prints "Hello John!"
    println!("{}", Hola.greet("John")); // prints "Hola John"
    println!("{}", Hola.greet_loudly("John")); // prints "¡Hola John!"
}
