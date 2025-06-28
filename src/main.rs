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

    // demonstrating shadowing and scopes
    // we declare and innitialize an immutable variable y
    let y = 5;

    // notice that we can use let to change the calue of y
    let y = y + 1;
    {
        // we can change it again in this scope
        let y = y * 2;

        // printing out the value of y
        println!("The value of y is: {y}");
    }

    // printing out the value of y
    println!("The value of y is: {y}");

    // declaring a new variable spaces
    let tabs = "          ";

    // using shadowing to change the type of tabs implicitly
    let tabs = tabs.len();
    
    // we can now see the value of stabs
    println!("The value of tabs is: {tabs}");

    // note that if spaces were muted it would cause a compile time error since we cannot
    // change the type of a mutable variable

    // the scalar data types in rust are Integer, FLoating-Point, Boolean and Character
    // declaration and initialization of g as an integer
    let g: u8 = "255".parse().expect("Not a number");
    println!("The value of g is: {g}");

    // declaring floating: point variables
    let a = 3.0;
    let b: f64 = 4.0;

    let sum = a+b;
    let product = a*b;

    println!("sum is {sum}, product is {product}");
    // note in rust you cannot multiply an integer and a floating-point number
    println!("Truncated {}",-5.0/3.0);

    // now let's look at booleans
    // implicit assignment
    let _t = true;
    // exlicit assignment
    let _f: bool = false;

    // character type
    let _c = 'a';
    let _c: char = 'A';
    let happy: char = '😊';
    println!("{}",happy);

    // Compound types
    // -- tuples
    // -- arrays
    
    // we start with tuples_
}
