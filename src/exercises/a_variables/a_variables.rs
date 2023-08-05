#[path = "../../utils.rs"]
mod utils;
use utils::divider;

const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

pub fn a_variables() {
    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;
    let bang = '💥';

    divider();
    println!("Firing {} of my {} missiles", ready, missiles);
    println!("Bang! Bang! {bang} {bang}", bang = bang);
    divider();

    missiles = missiles - ready;
    println!("Remaining {} missiles", missiles);
    divider();
}
