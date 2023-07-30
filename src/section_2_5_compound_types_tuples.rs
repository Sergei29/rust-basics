/*
* ----------------------------------------------
* Tuple:
* - is a collection of values, and the collection of a fixed size
* - the items within the tuple can be of different types
*   ('a', 25, 10.12, "hello")
* ----------------------------------------------
*/

#[path = "utils.rs"]
mod utils;

use utils::divider;
use utils::type_of;

pub fn compound_type_tuple() {
    divider();
    tuple_print();
    divider();
    tuple_descructure();
    divider();
}

fn tuple_print() {
    let my_info = ("Software Engineer", 70_000);

    println!(
        "
      My role = {role}
      My salary = £ {salary}
      type_of(my_info) = {my_info_type}
  ",
        role = my_info.0,
        salary = my_info.1,
        my_info_type = type_of(my_info),
    );

    println!("the tuple = {:?} ", my_info)
}

fn tuple_descructure() {
    let my_info = ("Software engineer", 70_000);
    let (role, wage) = my_info;

    println!(
        "
    the tuple my_info = {:?}
    my role = {1}
    my salary = {2}
    ",
        my_info, role, wage
    );

    let nested = (
        ("Serge", 48),
        ("40 Belsize Park Gardens", "NW3 4NA", "London"),
    );

    let (personal_details, address) = nested;

    let addess_lines = format!(
        "
    Street line 1: {}
    Postcode: {}
    City: {}
    ",
        nested.1 .0, nested.1 .1, nested.1 .2
    );

    println!(
        "
    My personal details are {:?}
    And my address is:
    {1}
    ",
        personal_details, addess_lines
    );

    let (street, postcode, city) = address;
    let my_full_details = format!(
        "
    My name is {fname},
    I am {age} years old,
    my address is:
    {street},
    {city}, {postcode}
    ",
        fname = personal_details.0,
        age = personal_details.1,
        street = street,
        city = city,
        postcode = postcode,
    );

    println!(
        "INTRODUCTION:
    {}
    ",
        my_full_details
    )
}
