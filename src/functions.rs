#[path = "utils.rs"]
mod utils;
use utils::divider;

pub fn functions_print() {
    divider();
    du_stuff("hellow", "world!");
    divider();
    let sum = get_sum(1, 2);
    println!("sum of 1 and 2 equals {sum}", sum = sum);
    divider();
    let (num_1, num_2) = (2.3215, 200045.98712);
    let multiply_result = get_multiply(num_1, num_2);
    println!(
        "the multiplication of {} and {} equals {}",
        num_1, num_2, multiply_result
    );
    divider();
}

fn du_stuff(a: &str, b: &str) {
    println!("Message: {} {}", a, b)
}

fn get_sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn get_multiply(a: f64, b: f64) -> f64 {
    // if we omit the ; at the end - it is an implicit return,
    // or so called TAIL expression
    a * b
}
