//BINDING AND MUTABILITY

// assigned using "let" keyboard
// print => print!() or println!()

/*
fn main(){
    let x: i32; //here x is uninitialized but used below => ERROR
    let y: i32; //uninitialized but also unused => only give a warning 

    assert_eq!(x,5);
    println!("successfully executed!")
}


fn main(){
    let x: i32=5;
    let _y: i32;

    assert_eq!(x,5);
    println!("successfully executed!")
}
*/

//use "mut" to mark a variable as mutable bcz by default its immutable in rust
//annotating type is not compulsory
/*
fn main(){
    let mut x: i32=1;
    x+=2;

    assert_eq!(x,3);
    println!("success!");
}
*/

//SCOPE

// scope of variable is defined by block of code in which it is decleared

/*
fn main(){
    let x: i32=10;

    {
        let y: i32=5;
        println!("the value of x is {} and y is {}",x,y);
    }
    //println!("the value of x is {} and y is {}",x,y);
    // it will throw error bcz y is not defined in this scope but x is defined in both scope
}
*/

// function is named block of code that is reusable

/*
fn main(){
    define_x();
}

fn define_x(){
    let x: &str ="hello";
    println!("{}, world", x);
}
*/

// shadowing allows a variable to be decleared again in same scope with same name

/*
fn main(){
    let x: i32= 10;

    {
        let x=12;
        assert_eq!(x,12);
    }

    assert_eq!(x,10);

    let x= 42;
    println!("{}",x); //prints 42
}
*/

//NOTE : mutable means we can re-asign value, in shadowing we can assign value of disfferent data types

//UN-USED variables

/*
#[allow(unused_variables)] //another-way warning 
fn main(){
    let x =1; //warning 
    //let _x =1; //one-way of fixing
}
*/

//we can use pattern with let to destructure a tuple to seperate variables

/*
fn main(){
    let (mut x,y)=(1,2);
    x+=2;

    assert_eq!(x,3);
    assert_eq!(y,2);

    println!("success")
}
*/

// Destructuring
fn main(){
    let (x,y);
    (x,..)=(2,3);
    [..,y]=[1,5];

    assert_eq!([x,y],[2,5]);
    println!("success")
}