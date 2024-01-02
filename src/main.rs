struct Point {
    x:i64,
    y:i64,
    z:i64,
}
struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}
fn main() {
    let mut greetings = "Hi";
    println!("{}", greetings);
    greetings = "Hello";
    let subject = "Worldy";
    println!("{}, {}!", greetings, subject);
    let x: f64 = 3.1;
    let y = 2.2;
    println!("x times y is {}", x*y);

    let answer = function(x, y);
    println!("x func y is {}", answer);
    let z: f32 = 10.0 / 3.0;
    println!("{}",z);
    let mut point:(i64, i64, i64) = (1,2,3);
    let x = point.0;
    let y = point.1;
    let z = point.2;
    println!("{},{},{}", x,y,z);
    point.1 = 3;
    let k = new_point(x, y, z);
    println!("{},{},{} ",k.x, k.y, k.z);
    println!("tuples can not change its size after compile");
    println!("let println_return_val: () = println!(Hi!)");
    println!("Unit is void");
    println!("Rust arrays are like c arrays");
    let mut year: [i32; 3] = [1995, 2000, 2005];
    let first_year = year[0];
    println!("{}",first_year);
    for y in year.iter() {
        println!("Next year: {}", y + 1);
    }
    println!("Cant iterate over tuples and structs");

}
fn new_point(x:i64, y:i64, z:i64) -> Point {
    Point {x,y,z}
}
fn function(x:f64, y:f64) -> u8 {
    let x = if x > y {
        x as u8 +y as u8
    } else if x < y {
        x as u8 -y as u8
    }else{
        x as u8 * y as u8
    };
    return x;
}

