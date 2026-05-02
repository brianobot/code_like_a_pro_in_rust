

fn as_slice<T>(vec: &Vec<T>) -> &[T] {
    vec
}

fn main() {
    let data = vec![0u8; 32];
    let slice = as_slice(&data);

    println!("{slice:?}");
    println!("{data:?}");
    
}

