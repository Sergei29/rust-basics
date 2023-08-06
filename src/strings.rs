#[path = "utils.rs"]
mod utils;
use utils::divider;
use utils::type_of;

pub fn strings_notes() {
    divider();
    conversion_strings();
    divider();
    get_the_nth_letter()
}

fn conversion_strings() {
    let name: &str = "hi 👋";
    let name_converted: String = name.to_string();
    let name_converted_2 = String::from("hi 👋");

    println!(
        "
    name: {},
    name_converted: {}
    name_converted_2: {}
    ",
        name, name_converted, name_converted_2
    );

    println!(
        "
    type of name: {},
    type of name_converted: {},
    type of name_converted_2: {},
    ",
        type_of(name),
        type_of(name_converted),
        type_of(name_converted_2)
    )
}

fn get_the_nth_letter() {
    // we cannot get exact letter just by indexing like : my_name[2],
    // because different characters/letters may have different byte length,
    // so no guarantee that a string "onetwothree😃" has 12 letters or characters;

    let name = "john💥silver😜";
    let emoji_bang = name.chars().nth(4);
    println!("emoji_bang: {}", emoji_bang.unwrap())
}
