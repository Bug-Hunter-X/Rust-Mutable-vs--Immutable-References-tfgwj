fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y; // z is an immutable reference to y

    *y = 10; // Modifying x through y
    println!("x: {}", x); // Output: x: 10

    // This will cause a compile-time error
    // *z = 100;
}