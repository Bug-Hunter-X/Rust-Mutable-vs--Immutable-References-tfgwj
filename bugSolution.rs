fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y = 10; // Modifying x through y
    println!("x: {}", x); // Output: x: 10

    // The following code is safe because y is a mutable reference.
    let z = &mut y;
    *z = &mut 100; 
    println!("x: {}", x); // Output: x: 100
} 