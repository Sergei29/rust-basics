/*
* ----------------------------------------------
* Array
* - is a collection of elements of the same type
* - createsd using square brackets []
* - the length, amount of elements, must be known at compile time.
*    let my_array: [i32; 3] = [22, 200, 300]
* - if we want to have the elements mutable:
*    let mut my_array: [i32; 3] = [22, 200, 300];
*    my_array[0] = 125;
* - if you want to fill an array with the same elements:
*    [<value>; <length>>];
*    let arr: [i32; 10] = [1; 10]; // [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
* ----------------------------------------------
*/

#[path = "utils.rs"]
mod utils;

use utils::divider;
use utils::type_of;

pub fn compound_type_arrays() {
    divider();
    array_print();
    divider();
    update_element_of_array();
    divider();
    update_each_of_elements();
    divider();
    slice_an_array();
    divider();
}

fn array_print() {
    let my_array: [i32; 3] = [22, 200, 300];
    println!(
        "
  my_array = {:?}
  typeof my_array = {}
  ",
        my_array,
        type_of(my_array)
    );

    println!(
        "
  First element: {},
  Second element: {},
  Third element: {},
  ",
        my_array[0], my_array[1], my_array[2]
    )
}

fn update_element_of_array() {
    let mut my_array: [i32; 3] = [22, 200, 300];
    println!("initial: \n{:?}", my_array);

    my_array[0] = my_array[0] * 10;
    println!("updated: \n{:?}", my_array);
}

fn get_fill_array(value: i32) -> [i32; 5] {
    return [value; 5];
}

fn update_each_of_elements() {
    let mut my_arr_2 = get_fill_array(1);
    println!("initial: \n{:?}", my_arr_2);

    let mut index: usize = 0;
    for num in my_arr_2 {
        my_arr_2[index] = num + index as i32;
        index = index + 1;
    }

    println!("updated: \n{:?}", my_arr_2);
}

fn slice_an_array() {
    let arr_1 = ['a', '+', '😜', '👋', 'H'];
    let subset_arr_1 = &arr_1[0..3]; // this array will point to the first 3 values of the arr_1. ( ref. borrowing, see later in course )

    println!(
        "
    arr_1: {:?}
    subset_arr_1: {:?}
    
    ",
        arr_1, subset_arr_1
    )
}
