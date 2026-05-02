#![allow(dead_code)]

#[derive(Debug)]
enum DogBreeds {
    PitBull,
    Sheperd,
    RotWeiler
}

impl From<u32> for DogBreeds {
    fn from(value: u32) -> Self {
        match value {
            other if DogBreeds::PitBull as u32 == other => { DogBreeds::PitBull },
            other if DogBreeds::Sheperd as u32 == other => { DogBreeds::Sheperd },
            other if DogBreeds::RotWeiler as u32 == other => { DogBreeds::RotWeiler },
            _ => panic!("Invalid Value"),
        }
    }
}

enum Numbers {
    One = 1,
    Two = 2,
    Three = 3,
}


fn main() {
    let _one = { || 1 }();
    let _zero = { || 0}();

    // println!("{}", _one/_zero); // this program panics, we tricked the compiler into allowing this to compile
    let _values = (1, 2, 3, 4);
    println!("value = {:?}", _values);

    let pitbull = DogBreeds::PitBull;
    println!("{pitbull:?}");

    let sheperd = DogBreeds::from(2);
    println!("{sheperd:?}");

    println!("Number one = {}", Numbers::One as u32)
}