fn main() {

    // Learning Variables in rust

    // integer, unsigned integer, float

    let mut  x: i8 = -2; // by default variable are immutable in rust, so we have to add mut in every variable to make changes in stored values later
    let y: u32 = 1000;
    let z: f32 = -100.002;
    x = x + 5;

    print!("x : {} \n", x + 5);
    print!("y : {} \n", y );
    print!("z : {} \n", z );

    // Boolean

    let is_male: bool = false;
    let is_above_18 : bool = true;

    if is_male {
        println!("You are a male");
    } else {
        println!("You are above 18");
    }

    if is_male && is_above_18 {
        println!("You are a legal male")
    }

    //String

    let greeting = String:: from("Hello world"); //simple
    println!("{}", greeting);

    let char1 = greeting.chars().nth(0);

    // it might be character / option 

    // pattern matching
    // match char1 {
    //     Some(c) => println!("{}", c),
    //     None => println!("No character at index 100")
    // }

    // println!("first character in greeting {} ", char1);
    println!("first character in greeting {} ", char1.unwrap())
}
