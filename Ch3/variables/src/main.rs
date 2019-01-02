let mut global_variable = 4;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    x = 7;
    println!("The value of x is: {}", x);

    let mut y: u8 = b'a';
    y = b'b';
    println!("Value of y is {}", y);

    let z: f64 = 2.345554;
    println!("z is {}", z);

    let z: bool = true;
    println!("Value of z is {}", z);

    let a: char = '😇';
    println!("Value of a is {}", a);

    let tup: (i32, f64, char) = (5, 0.1, 'a');
    let (f, g, h) = tup;
    println!("Value of g is {}", g);
    println!("Value of h is {}", tup.2);

    let my_array = [1, 2, 3, 4, 5];
    println!("Break this! {}", my_array[2]);
    println!("Length is {}", my_array.len());

    another_function();
}

fn another_function(){
    println!("Another function");
}
