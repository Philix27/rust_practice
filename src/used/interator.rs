fn returnIter() -> impl Iterator {
    let lister = vec![1, 3, 5];
    lister.into_iter()
}

struct Sam;

impl Iterator for Sam {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn mali() {
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        // *name  = match name {
        //     &mut "Ferris" => println!("There is a rustacean among us!"),
        //     _ => println!("Hello {}", name),
        // };
        let c_name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        };
        let b = *name;
    }
    // println!("names: {:?}", names);

    let reference = 3;
    match reference {
        vl => println!("Got a value via destructuring: {:?}", vl),
    }
    println!("{}", reference);
}
