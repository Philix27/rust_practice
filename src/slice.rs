use std::mem;

mod learning;
pub mod macro_la;

fn main() {
    // Fixed-size array (type signature is superfluous).
    let mut xs: [i32; 6] = [1, 2, 3, 4, 5, 2000000000];
    // All elements can be initialized to the same value.
    let _ys: [i32; 500] = [0; 500];
    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);
    // `len` returns the count of elements in the array.
    xs.reverse();
    println!("Number of elements in array: {}", xs.len());
    println!("Array reversed list: {:?}", xs);

    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    // analyze_slice(&xs);
    analyze_slice(&[xs[0], xs[3]]);
}

fn analyze_slice(slice: &[i32; 2]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}
