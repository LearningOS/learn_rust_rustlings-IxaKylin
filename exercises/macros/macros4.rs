// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

//

macro_rules! my_macro {
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }; //加分号
    () => {
        println!("Check out my macro!");
    }; //加分号
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
