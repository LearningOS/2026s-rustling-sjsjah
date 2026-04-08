// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

/*We may be tempted to use our own approximations for certain mathematical constants,
but clippy recognizes those imprecise mathematical constants as a source of
potential error.
See the suggestions of the clippy warning in compile output and use the
appropriate replacement constant from std::f32::consts... */
#[deny(clippy::approx_constant)]
#[rustfmt::skip]


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}
use std::f32::consts::PI;
fn main() {
    let pi = std::f32::consts::PI;
    my_macro!(pi);
    my_macro!(7777);
}
