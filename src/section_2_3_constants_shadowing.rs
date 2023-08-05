#[path = "utils.rs"]
mod utils;
use utils::divider;

pub fn constants_shadowing_print() {
    divider();
    octal_hex_binary_print();
    divider();
    numbers_conversion();
    divider();
    shadowing();
    divider();
    shadowing_and_code_segment();
    divider();
    constants();
    divider();
    shadowing_once_more();
    divider();
}

fn octal_hex_binary_print() {
    let (num_a, num_b) = (123, 270.75);

    println!("num_a: {0}, num_b: {1}", num_a, num_b);

    // let overflow_num_value: u8 = 256;

    let num: i16 = 255;

    println!(
        "Value num in octal: {:o}, value num in hex: {:X}, value num in binary {:b}",
        num, num, num,
    );
}

fn numbers_conversion() {
    let num_1: i8 = 10;
    let num_2: f32 = 10.50;

    // convert num_2 to i8 which will be 10
    let sum_1: i8 = num_1 + num_2 as i8;

    // convert num_1 to f32 which will be 10.0
    let sum_2: f32 = num_1 as f32 + num_2;

    println!(
        "Arithmetic operations with numbers - all operands must be of same type , or converted to the same type: \n
        num_1={num_1};  num_2={num_2};
        {expression_1} => {sum_1}; and
        {expression_2} => {sum_2}

        After the operation the variables values will remain of same type, they were converted only within the sum expression
        num_1={num_1};  num_2={num_2};
        ",
        num_1=num_1,
        num_2=num_2,
        sum_1 = sum_1,
        sum_2 = sum_2,
        expression_1="let sum_1: i8 = num_1 + num_2 as i8",
        expression_2="let sum_2: f32 = num_1 as f32 + num_2;"
    );
}

fn shadowing() {
    /*
     * Shadowing
     * - a concept of using, creating or updating a variable with the same name
     * which has been previously declared or used in the program
     *
     */

    #[allow(unused_variables)]
    let num_1: i8 = 8;
    let num_1: i8 = 20;

    println!(
        "1. Shadowing the immutable variable, rust will use the latest value
    num_1 = {0}",
        num_1
    );

    #[allow(unused_variables)]
    #[allow(unused_mut)]
    let mut num_2: i8 = 5;
    let num_2: i8 = 5 * 5;

    println!(
        "2. Shadowing the mutable variable by an immutable one, rust will use the latest value 
    num_2 = {0}
, which will be an immutable variable",
        num_2
    );

    #[allow(unused_variables)]
    let my_var = 22;
    let my_var = '😜';

    println!(
        "3. Shadowing the variable of one type by the same name of var of another type, rust will use the latest value 
    my_var = {0}",
        my_var
      );
}

fn shadowing_and_code_segment() {
    // all variables can be scoped

    let my_var = 22;

    {
        let my_var = 1500;
        println!(
            "Inside the code segment:
    my_var = {} ",
            my_var
        )
    }

    println!(
        "Out of the code segment:
    my_var = {} ",
        my_var
    )
}

fn constants() {
    // const must always have a type definition
    const MY_NUMBER: i8 = 25;

    /*
     * What is the difference between the immutable variable `let` and a constant `const` ?
     * - cannot use keyword `mut` with the `const`
     * - `const` must always have a type definition, the compiler will not infer the type unlike with `let`
     * - in rust: constants naming convention is to use UPPER_CASE
     * - the constants values must be set to a known values or expression's value, but NOT to a result that can be known at a runtime ( Important )
     * eg.
     * const MY_CODE_NUM:i64 = 12345; // OK
     * const SOME_RESULT:i32 = get_some_values(100:i32, 22:i32); // ERROR
     */

    println!("Constant MY_NUMBER = {}", MY_NUMBER);

    println!("
What is the difference between the immutable variable `let` and a constant `const` ?
    - cannot use keyword `mut` with the `const`
    - `const` must always have a type definition, the compiler will not infer the type unlike with `let`
    - in rust: constants naming convention is to use UPPER_CASE
    - the constants values must be set to a known values or expression's value, but NOT to a result that can be known at a runtime ( Important )
    eg.
    const MY_CODE_NUM:i64 = 12345; // OK
    const SOME_RESULT:i32 = get_some_values(100:i32, 22:i32); // ERROR
    ")
}

fn shadowing_once_more() {
    let current: &str = "my value name";
    println!("current: {}", current);
    // shadowing into different type
    let current = get_img_url(current);
    println!("current: {}", current);
}

fn get_img_url(img_name: &str) -> String {
    return "https://my-images-".to_string() + &img_name.replace(" ", "-").to_string();
}
