mod enums_clx;
mod learning;
mod macro_la;

// use enums_clx::WebEvent;

fn main() {
    use crate::enums_clx::WebEvent;
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    WebEvent::inspect(pressed);
    WebEvent::inspect(pasted);
    WebEvent::inspect(click);
    WebEvent::inspect(load);
    WebEvent::inspect(unload);
    mali();
}
