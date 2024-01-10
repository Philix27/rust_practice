// use macro_rules! <name of macro>{<Body>}
#[macro_export]
macro_rules! add_values {
    // macth like arm for macro
    ($a:expr,$b:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            $a + $b
        }
    };
}

// pub mod add;