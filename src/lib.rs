/*
one package should contains at least one crate
crate = {binary crate, library crate}
src/main.rs is the binary crate root
src/lib.rs is the library crate root

one package can contains many binary crate
each file under scr/bin is a singleton binary crate
*/
#![allow(dead_code)]
mod collection_ex;
mod enum_ex;
mod io_ex;
mod struct_ex;
mod fizzbuzz;
