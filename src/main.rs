#![allow(unused)]

// EXERCISES:
#[path = "exercises/a_variables/a_variables.rs"]
mod a_variables;
#[path = "exercises/b_functions/b_functions.rs"]
mod b_functions;

// COURSE NOTES:
mod functions;

fn main() {
    // a_variables::a_variables();
    // functions::functions_print();
    b_functions::b_functions();
}
