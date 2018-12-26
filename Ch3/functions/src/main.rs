fn main() {
    println!("Hello, world!");
    another_function(4, 100);

    let y = {
        let x = 3;
        x + 1
    };

    let zzz: char = {
        let temp_value: char = 'a';
        temp_value
    };
    println!("zzz is {}", zzz);

    println!("The value of y is {}", y);

    println!("call to five() returned {}", five());
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}.", x);
    println!("The value of y is {}.", y);
}

fn five() -> i32 {
    let mut _return_value = 0;
    _return_value = 54;
    // return 5454;
    _return_value
}
