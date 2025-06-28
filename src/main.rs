fn main() {

    // initializing the mutable variable x and assigning it the value 5
    let mut x = 5;

    // printing out to verify the value
    println!("The value of x is: {x}");

    // changing the value of the variable to 6
    x = 6;

    // printing out to verify the change
    println!("The value of x is: {x}");

    // adding a constant
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

    // printing out the constant
    println!("The value of the constant is: {THREE_HOURS_IN_SECONDS}");
}
