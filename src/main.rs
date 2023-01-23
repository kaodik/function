fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5,'h');
    let x = five();
    println!("The value of x is: {x}");
}
fn another_function(x:i32){
    println!("Another funcition.");
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurment is: {value}{unit_label}");
}

//main function is the entry point of many programs

// Trying to do the following will cause an error:
// let x = (let y = 6);

//Things that are expressions are calling a function,
//Calling a macro, a new scope block with curly brackets.

//Expressions that don't include ending semicolon is a Statment
//Statements don't return a value.

// you don't name return values but must use ->
fn five() -> i32{
    5
}