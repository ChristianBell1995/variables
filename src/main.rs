fn main() {

    // Variables are imutable by default, in order to change them you must add the 'mut' keyword

    let mut x = 5;

    println!("The value of c is: {}", x);

    x = 6;

    println!("The value of x is: {}", x);

    // Shadowing is the concept of using let after a variable has been declared - you don't get type errors and the original value stays the same

    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);

    // Main data types in rust are integers, floating-point, numbers booleans and characters
    // Integers can either be signed or unsigned and are defined by their bit size (8, 16, 32 etc.)
        // Signed refers to if the number needs to have a sign infront of it, or in otherwords that it could be negative.
        // Unsigned numbers are positive.
        // Rust defaults to i32 (signed 32 bits).
    // Floating-Point types (float with decimal)
        // These are in f32 and f64, default is f64.

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // Character type
      // char type is specified using single quotes as opposed to string literals which use double quotes.

    // Tuple Type
      // (declare the type and then have the value in brackets)
      let tup: (i32, f64, u8) = (500, 6.4, 1);

      // destructuring like JS
      let tup = (500, 6.4, 1);

      let (a, b, c) = tup;

      println!("The value of c is: {}", c);

      // To find the value of the tuple by index
      let d = tup.1;

      println!("The value of d is: {}", d);

    // Arrays
      // Unlike arrays in other language it has a fixed length
      // If you want it to grow and shrink you use a Vector

      // Here the array declares the type and lenth of the array
      let e: [i32; 5] = [1, 2, 3, 4, 5];

      // This:
      // let a = [3; 5];
      // Creates this array
      // let a = [3, 3, 3, 3, 3]

      // To access elements in an array (same as ruby)
      let first = e[0]

      // if you try and access an index that is past the length of the array it will say index is out of bounds.

}
