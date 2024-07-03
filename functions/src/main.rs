fn main() {
    println!("Hello, world!");

    // calling the second function
    another_function();
    parameter_function(5);
    print_labeled_measurement(5, 'h');
    statements_and_expressions();
    let num = return_values();
    println!("The value of num is: {num}");
    
    let num2 = return_another(5);
    println!("The value of num2 is: {num2}");
}

// separate function definition. Functions use snake case (all lowercase words separated by _)
fn another_function() {
    println!("Another Function!");
}

// It does not matter where a function is defined, so long as it has a definition
fn parameter_function(x: i32) {
    println!("The value of x is: {x}");
}

// In function signatures, declare the type of each parameter. When defining multiple parameters, separate declarations with commas
fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The meaurement is: {value}{unit_label}");
}

// covering statements and expressions
// statement = instructions to perform some action and do not return a vaue
// expression = evaluate to a resultant value
fn statements_and_expressions() {
    //statement
    //let y = 6;
    //cannot use a let statement with another let statement
    // let x = (let y = 6);

    //expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn return_values() -> i32{
    32
}

fn return_another(x: i32) -> i32{
    //Putting a semicolon ; at the end of this makes it a statement rather than an expression and returns an error
    x + 7
}