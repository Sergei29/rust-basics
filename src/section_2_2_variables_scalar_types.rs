/* ----------------------
 *  - Variables
 *  - Scalar Data Types
 * ----------------------
 *  Naming rules:
 *  - variable name can include letters, digits and _underscore characters only
 *  - variable name should begin with letter or _underscore
 *  - the variable names are case sensitive
 *
 *  a Scalar type - represents a single value. Rust has 4 primary scalar types:
 *  - integer number
 *    a) signed: can store both (-)negative and (+)positive numbers,
 *       i8, i16, i32, i64
 *    b) unsigned: can store positive numbers only
 *       u8, u16, u32, u64
 *  - floating-point number, can store numbers with decimal points
 *    a) they are all signed: can store both (-)negative and (+)positive numbers
 *       f32, f64
 *  - arithmetical operations between two different types of numbers are
 *  not allowed, they maust be converted to same type.
 *
 *  - boolean: true or false
 *
 *  - character: always included in single quotes (')
 *    a) a single letter
 *    b) a digit
 *    c) an emoji
 *    d) a unicode scalar value
 *
 */

pub fn variables_scalar() {
    print_mut_variables();
    print_i_int_num_range();
    print_u_int_num_range();
    print_float_num_range();
    print_boolean_values();
    print_complex_type();
    print_character_types()
}

fn print_mut_variables() {
    let x: f32 = 15.0;
    let result: f32 = x * 10.0;
    let mut x_mutable: f32 = 32.0;

    println!(
        "The value of x is {0}, and x_mutable is {1}, result is {2}.",
        x, x_mutable, result
    );

    x_mutable = 55.0;

    println!(
        "The value of x is {0}, and x_mutable is {1}, result is {2}.",
        x, x_mutable, result
    );
}

fn print_i_int_num_range() {
    println!(
        "
The maximum value of i8 = {0}, and minimum value of i8 = {1},
The maximum value of i16 = {2}, and minimum value of i16 = {3},
The maximum value of i32 = {4}, and minimum value of i32 = {5},
The maximum value of i64 = {6}, and minimum value of i64 = {7},
    ",
        std::i8::MAX,
        std::i8::MIN,
        std::i16::MAX,
        std::i16::MIN,
        std::i32::MAX,
        std::i32::MIN,
        std::i64::MAX,
        std::i64::MIN
    );
}

fn print_u_int_num_range() {
    println!(
        "
The maximum value of u8 = {0}, and minimum value of u8 = {1},
The maximum value of u16 = {2}, and minimum value of u16 = {3},
The maximum value of u32 = {4}, and minimum value of u32 = {5},
The maximum value of u64 = {6}, and minimum value of u64 = {7},
  ",
        std::u8::MAX,
        std::u8::MIN,
        std::u16::MAX,
        std::u16::MIN,
        std::u32::MAX,
        std::u32::MIN,
        std::u64::MAX,
        std::u64::MIN
    );
}

fn print_float_num_range() {
    println!(
        "
The maximum value of f32 = {0}, and minimum value of f32 = {1},\n\n
The maximum value of f64 = {2}, and minimum value of f64 = {3},
  ",
        std::f32::MAX,
        std::f32::MIN,
        std::f64::MAX,
        std::f64::MIN,
    );
}

fn print_boolean_values() {
    let num: i8 = 7;
    let is_gt_5: bool = num > 5;
    let is_lt_10: bool = num < 10;
    let is_not_eq: bool = num != 12;

    println!(
        "The number {number} is greater than 5: {is_greater}, and is less than 10: {is_less}, also the number {number} - is not equal 12: {is_not_equal}",
        number = num,
        is_greater = is_gt_5,
        is_less = is_lt_10,
        is_not_equal = is_not_eq
    )
}

fn print_complex_type() {
    let is_true = true;
    let num: i8 = 8;
    let num_float: f32 = 10.5;
    // the {:?} is a placeholder for a non-scalar, but a complex data type, like a tuple for example;
    println!(
        "The list of values is a tuple: {:?}",
        (is_true, num, num_float)
    )
}

fn print_character_types() {
    let my_char_letter: char = 'A';
    let my_char_digit: char = '1';
    let my_char_sign: char = '+';
    let my_char_unicode: char = '\u{00A9}';
    let my_emoji: char = '😀';

    println!(
        "Character type can be single letter, digit, sign, emoji or signle char unicode: {:?}",
        (
            my_char_letter,
            my_char_digit,
            my_emoji,
            my_char_sign,
            my_char_unicode
        )
    )
}
