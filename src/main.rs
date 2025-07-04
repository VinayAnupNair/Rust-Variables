use std::io::{self, Write};

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
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // one way to unpack tuple
    let (x,y,z) = tup;
    
    // printing out the values of the tuple we unpacked
    println!("the value of x is: {x}");
    println!("the value of y is: {y}");
    println!("the value of z is: {z}");

    // alt way of unpacking the tuple
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    // we can declare an array like so
    let arr:[i32; 5] = [0, 1, 2, 3, 4];
    let first = arr[0];
    println!("{first} {} ", arr[3]);
    // calling function greet
    greet();

    // calling finction number with parameter 6
    number(6);

    // calling function return value to store in a variable
    let w = return_value(7);
    println!("the value of w is: {}",w);
    can_ride();
}

// declaring function greet
fn greet(){

    // printing out hello 
    println!("Hello");

}

// declaring function number with parameter x which is a 32-bit integer
fn number(x: i32){

    // printing out the value x
    println!("The value of x is: {}",x);

}

fn return_value(z: i32)-> i32{
    let y = {
        let x = 3;
        x + 1
    };
    y + z
}

fn can_ride(){
    print!("Enter your height in cm: ");
    io::stdout().flush().unwrap();
    let h = read_int();
    if h < 120{
        println!("Sorry you cannot ride yet");
    }else{
        println!("Ok, go ahead");
    }
}

fn read_int() -> i32{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<i32>().expect("Please enter a valid number")
}
