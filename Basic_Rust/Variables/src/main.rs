fn main()
{
    let mut x=5;
    println!("Value of x: {x}");
    x=6;
    println!("Value of x: {x}");


    // constant

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing

    let y=5;

    let y=y+1;
    {
        let y=y*2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let space = "    ";
    let space = space.len();

    println!("Space : {space}"); // output: 4

    let mut spaces="    ";
    let spaces = spaces.len(); // Shows error due to type conversion

    println!("Spaces : {spaces}");

}