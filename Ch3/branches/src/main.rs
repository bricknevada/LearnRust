use std::env;

fn main() {
    for x in env::args(){
        println!("{:?}", x );
    }
}
