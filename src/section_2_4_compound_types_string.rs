/*
 *  Strings
 *  - Fixed length strings ( &str )
 *    Fixed size string (&str) - is a fixed length string and cannot be mutated
 *  - Growable length strings ( String )
 */

#[path = "utils.rs"]
mod utils;
use utils::divider;
use utils::type_of;

pub fn compound_type_string() {
    divider();
    fixed_string();
    divider();
    growable_string();
    divider();
}

fn fixed_string() {
    let some_fixed_size_str = "Hello fixed size!";

    println!("Fixed size string = {}", some_fixed_size_str);
}

fn growable_string() {
    let mut growable_str: String = String::from("Hello growable string!");

    println!("Growable size string = {}", growable_str);

    growable_str.push('👋');
    println!("Growable size string = {}", growable_str);

    growable_str.push_str(" And updated!");
    growable_str.push('😜');
    println!("Growable size string = {}", growable_str);

    let removed_char: Option<char> = growable_str.pop();
    println!("Growable size string, popped = {0}", growable_str);
    println!("Removed char: {}", removed_char.unwrap());

    growable_str.truncate(5);
    growable_str.push('😜');

    println!(
        "
    Basic functions on strings:
    let growable_string = \"{growable_str}\"
    .is_empty(): {is_empty}
    .chars().count(): {lenght_chars}, length in chars
    .len():  {length_bytes}, string length in bytes
    .capacity(): {bytes}, how many bytes does string take in memory
    .contains(\"Hello\"): {contains_hello}
    ",
        growable_str = growable_str,
        is_empty = growable_str.is_empty(),
        lenght_chars = growable_str.chars().count(),
        length_bytes = growable_str.len(),
        bytes = growable_str.capacity(),
        contains_hello = growable_str.contains("Hello"),
    );

    // convert number => string
    let num = 6;
    let num_str = num.to_string();
    // convert char => string
    let char_1 = 'a';
    let char_1_str = char_1.to_string();

    let my_str_1 = "Serge".to_string();
    let my_str_2 = "Serge";

    println!(
        "
Is a string?
  num type = {num_type}
  num_str type = {num_str_type}
  char_1_str type = {char_1_str_type}
  my_str_1 type = {my_str_1_type}
  my_str_2 type = {my_str_2_type}
    ",
        num_str_type = type_of(num_str),
        char_1_str_type = type_of(char_1_str),
        my_str_1_type = type_of(my_str_1),
        my_str_2_type = type_of(my_str_2),
        num_type = type_of(num)
    );

    let empty_string = String::new();
    println!(
        "
  empty_string = {empty_string}
  empty_string.len() = {len}
  empty_string.chars().count() = {length}
  ",
        empty_string = empty_string,
        len = empty_string.len(),
        length = empty_string.chars().count()
    );

    let introduction = concatenate_strings();

    println!(
        "INTRODUCTION:
    {introduction}
    ",
        introduction = introduction
    )
}

fn concatenate_strings() -> String {
    let fname = "Serge".to_string();
    let lname = "Basangovs".to_string();

    let phrase = format!("My first name is {}, and my last name is {}", fname, lname);

    return phrase;
}
