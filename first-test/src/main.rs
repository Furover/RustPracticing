//adding this arrow allows the function to return a value
fn sum(a: i32, b: i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    //here, we don't need to use the return keyword and don't use ;
    a + b
}

fn main() {
    //u is unsigned, which means it can only be positive
    //i is signed, which means it can be negative
    //therefore, i8 can go up to 127 and -127
    //u8 can go up to 255 while using the same amount of memory
    let a: u8 = 128;
    let mut b: f32 = 2.5;
    println!(
        "Variable a: {}, with a size of {} bytes",
        a,
        std::mem::size_of_val(&a)
    );
    println!(
        "Variable b: {}, with a size of {} bytes",
        b,
        std::mem::size_of_val(&b)
    );
    b = 3.5;
    println!(
        "Variable b: {}, with a size of {} bytes",
        b,
        std::mem::size_of_val(&b)
    );
    //constants are always immutable (therefore don't allocate memory) and have a static lifetime (they exist while the program is running)
    const PI:u8 = 3;
    println!(
        "Variable PI: {}, with a size of {} bytes",
        PI,
        std::mem::size_of_val(&PI)
    );
    //static can be mutable (but need to use unsafe) and also have a static lifetime
    static mut NAME: &str = "Furover";
    unsafe {
        println!(
            "Variable NAME: {}, with a size of {} bytes",
            NAME,
            std::mem::size_of_val(&NAME)
        );
    }

    //this is shadowing a variable
    let x:u8 = 5;
    println!("Variable x1: {}", x);
    let x:u8 = 10;
    println!("Variable x2: {}", x);

    println!("Sum result: {}", sum(5, 10));
}
