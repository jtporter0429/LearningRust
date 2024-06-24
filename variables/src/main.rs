fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

/* Shadowing allows for variables to have the same name while retaining differing values
   For instance: line 4 creates another variable "x" but uses the previous value to set it
   and in the inner scope on line 7, another variable "x" is created using the second "x"
   as a source for the arithmetic done to calculate this new "x". The third "x" is
   overwritten once the scope is left after line 8.
*/
