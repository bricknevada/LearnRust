fn main() {
    let s = String::from("Hey y'all");
    let x = first_word(&s);
    println!("{}",s);
    println!("First word ends at position {}", x);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            println!("Working on {} at '{}'", i, item as char);
            return i;
        }
    }
    s.len()
}
