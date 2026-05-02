

#[allow(non_snake_case)]
fn print_String(value: String) {
    println!("{value}")
}

fn print_str(value: &str) {
    println!("{value}");
}

fn main() {
    // there is only one type of string in rust and maybe ways to represent it, many ways to handle it's allocation
    // str - stack allocated sequence of UTF-8 String which can be  borrowed but not moved or mutated
    // String - heap allocated UTF-8 String which can be borrowed, moved and mutated
    // 
    // 'static lifetime is special lifetime specifier that is valid for the entire life of a process
    // 
    // 
    let s = "stack string";
    
    print_str(s);
    print_String(String::from("heap string"));
}