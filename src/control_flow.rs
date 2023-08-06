#[path = "utils.rs"]
mod utils;
use utils::divider;

pub fn control_flow() {
    divider();
    call_all_numbers();
    divider();
    loop_as_loop();
    divider();
    loop_named();
    divider();
    for_loop();
    divider();
}

fn call_all_numbers() {
    let nums: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    println!("calling {:?}", nums);
    for num in nums {
        let called = call_number(num);
        print!(" {}, ", called);
    }
}

fn call_number(num: i32) -> &'static str {
    let name = if num == 5 {
        "five"
    } else if num == 4 {
        "four"
    } else {
        "other"
    };

    return name;
}

fn loop_as_loop() {
    const MAX: i32 = 5;
    let mut count = 0;

    loop {
        print!("count: {}, ", count);
        count = count + 1;
        if count == MAX {
            break;
        }
    }
}

fn loop_named() {
    let nested_arr = [[123, 100, 20], [4, 11, 3], [1, 22, 17]];
    let length_1 = nested_arr.len();

    let length_2: usize = 3;
    let mut index_1: usize = 0;

    'one: loop {
        let current_arr = nested_arr[index_1];
        let mut index_2: usize = 0;

        'two: loop {
            let current_inner_arr_elem = current_arr[index_2];

            let found = current_inner_arr_elem == 1;

            if found {
                println!("Found 1 at position: [{}][{}]", index_1, index_2);
                break 'one;
            }

            index_2 = index_2 + 1;
            if index_2 == length_2 - 1 {
                break;
            }
        }

        index_1 = index_1 + 1;
    }
}

fn for_loop() {
    for num in [1, 2, 3].iter() {
        print!("{} / ", num);
    }

    divider();
    let my_nums = [(1, 2), (3, 4), (5, 6)];

    for (num_1, num_2) in my_nums.iter() {
        print!("{} / {} / ", num_1, num_2);
    }
    divider();

    for num in 0..=10 {
        print!("{}, ", num);
    }
}
