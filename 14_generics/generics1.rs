fn main() {
    // Fix the compiler error by annotating the type of the vector `Vec<T>`.
    // Choose `T` as some integer type that can be created from `u8` and `i8`.
    let mut numbers: Vec<i32> = Vec::new();

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}