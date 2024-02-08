fn main() {
    use std::mem;
    let color = String::from("green");
    // A closure to print `color` which immediately borrows (`&`) `color` an
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time.
    //
    // `println!` only requires arguments by immutable reference so it doesn
    // impose anything more restrictive.
    let print = |section: &str| println!("{} - `color`: {}", section, color);
    // Call the closure using the borrow.
    print("First");
    // `color` can be borrowed immutably again, because the closure only hol
    // an immutable reference to `color`.
    let _reborrow = &color;
    print("Second");
    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;
    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `cou
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    // Call the closure using a mutable borrow.
    inc();
    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    // let _reborrow = &count;
    // ^ TODO: try uncommenting this line.
    inc();
    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;
    // A non-copy type.
    let movable = Box::new(3);
    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::size_of::<i32>();
        mem::drop(movable);
        // movable
    };
    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume();
    // ^ TODO: Try uncommenting this line
}
