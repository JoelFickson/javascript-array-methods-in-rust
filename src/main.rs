pub mod arrays;
use arrays::Array;
use arrays::{IsEmpty, Length, Pop, Push};

fn main() {
    let array: Array<i32> = Array {
        items: vec![1, 2, 3, 4, 5],
    };

    array.push(6);

    println!("array: {:?}", array);

    let size = array.length();

    print!("array size: {}", size);

    print!("array is empty: {}", array.is_empty());

    let array2 = array.pop();

    println!("array2: {:?}", array2);
}
