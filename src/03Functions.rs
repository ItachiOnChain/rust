fn main (){

    // Functions in rust

    let x = 10;
    let y = 20;
    let z = add(x, y);
    println!("z : {}", z);
}

fn add(x: i32, y: i32) -> i32 {
    return x + y
}