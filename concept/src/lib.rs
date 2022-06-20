//! # the_rust_programming_language Crate
//! It contains some examples of the book the rust programming language\
//! one package should contains at least one crate\
//! crate = {binary crate, library crate} \
//! src/main.rs is the binary crate root \
//! src/lib.rs is the library crate root \
//! one package can contains many binary crate \
//! each file under scr/bin is a singleton binary crate
#![allow(dead_code)]
mod collection_ex;
mod enum_ex;
mod io_ex;
mod struct_ex;

///
/// This is an documentation example
///
/// # Example
/// ```
/// println!("This is an example");
///
/// ```
/// # Panics
/// here's panic
/// # Errors
/// here's error
/// # Safety
/// here's safety
pub fn add_one(x: i32) -> i32 {
    x + 1
}
