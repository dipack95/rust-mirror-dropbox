use std::mem;

fn tell_me_about_my_slice(slice: &[i64]) {
    println!("First element is: {}", slice[0] );
    println!("Length of slice: {}", slice.len() );
}

fn main() {
    let first: [i64; 5] = [1, 2, 3, 4, 5];
    let second: [i64; 100] = [1; 100];

    println!("First array");
    tell_me_about_my_slice(&first);
    tell_me_about_my_slice(&second);
    tell_me_about_my_slice(&second[1..5]);

    println!("Stack allocated size for first array is: {}", mem::size_of_val(&first));
    println!("Stack allocated size for second array is: {}", mem::size_of_val(&second));
}