fn main() {
    println!("Hello, world!");
    let x=10;
    let y=5;
    // println!("{},fn(param1,param2)");
    println!("{}",add(x,y));
    println!("{}",sub(x,y));
    println!("{}",mul(x,y));
    println!("{}",div(x,y));
}
   // (param1:type, param2: type) -> return type
fn add(x:i32,y:i32) -> i32 {
    return x+y;
}
fn sub(a:i32,b:i32)->i32{
    return a-b;
}
fn mul(a:i32,b:i32)->i32{
    return a*b;
}
fn div(x:i32,y:i32)->i32{
    return x/y;
}