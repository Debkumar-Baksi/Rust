fn main() {
    println!("Hello, world!");

    // Integer Type

    let guess: u32 = "42".parse().expect("Not a Number");
    println!("Guess: {guess}");

    // Float Type

    let x=2.0; //f64
    let y:f32 = 3.0; //f32

    println!("x: {x} and y: {y}");

    //Numerical Operation

    let sum = 5 + 10;
    let difference = 95.5-73.3;
    let product = 4*30;
    let quotient = 56.7/32.2;
    let truncated = -5/3;
    let remainder = 43%5;

    println!("Sum: {sum} , Difference: {difference} , Product: {product} , Quotient: {quotient} , Truncated: {truncated} , Remainder: {remainder}");

}
