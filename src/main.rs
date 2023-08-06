#![allow(unused)]

// EXERCISES:
#[path = "exercises/a_variables/a_variables.rs"]
mod a_variables;
#[path = "exercises/b_functions/b_functions.rs"]
mod b_functions;

#[path = "exercises/c_simple_types/c_simple_types.rs"]
mod c_simple_types;
use c_simple_types::c_simple_types;

// COURSE NOTES:
mod functions;
mod modules;
use modules::hello;

fn main() {
    // a_variables::a_variables();
    // functions::functions_print();
    // b_functions::b_functions();
    // hello();
    c_simple_types();
}
