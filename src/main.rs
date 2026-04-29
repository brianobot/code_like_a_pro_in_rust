//! # Main Binary for Code like a pro in Rust Lesson by Brian
//! 
//! This is another quest in my overarching journey to master Rust and it's tooling
//! See you later.

#![warn(missing_docs)]

/// # Talk
/// 
/// Says something really important i guess
pub fn talk() -> String {
    String::from("Some Discourse")
}

fn main() {
    println!("Hello, world!");
    let speech = talk();
    println!("{speech}")
}
