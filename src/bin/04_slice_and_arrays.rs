

fn main() {
    let array = [0u8; 64];
    let slice = &array;

    let (first_half, second_half) = slice.split_at(32);
    println!("first_half = {first_half:?}\nsecond_half = {second_half:?}");

    let wordlist = "one,two,three,four,five";
    for word in wordlist.split(",") {
        println!("word = {word}");
    }
}

