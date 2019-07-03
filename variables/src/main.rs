/*
fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    let dm: char = dog_man();

    if dm == 'd' && 0 == 0 {
        println!("Dog man: {}", dm);
    }
}
*/

fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

// fn dog_man() -> char {
//     'd'
// }