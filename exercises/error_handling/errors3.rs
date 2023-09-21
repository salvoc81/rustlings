// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a hint.

use std::num::ParseIntError;

// By default the main function and any other functions returns nothing detoned with '()'
// The error is due the fact that if we use '?' in the call to total_costs
// than the main needs to return a Result type.
// The solution is the change the return type of the main to a Result type same as the one returned by the total_cost
fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    // This is required because:
    // 1) We change the main return type to Result<(), ParseIntError>
    // 2) The error case is being propagated using the '?' operator 
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
