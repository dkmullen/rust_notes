fn main() {
  // Integers
  let num = 98_222 + 1;
  // same as ... i32 is the default. 
  let num: i32 = 98_222 + 1;
    println!("{}", num);

  // Floating point
  let x = 2.0; // f64, which is default
  let y: f32 = 3.0; // f32

  // Boolean
  let t = true;
  let f: bool = false; // with explicit type annotation

  // Characters
  let c = 'z';
  let z = 'ℤ';
  let heart_eyed_cat = '😻';
  // Rust’s char type is four bytes in size and represents 
  // a Unicode Scalar Value, which means it can represent a lot more than just ASCII.

  //Tuples - Must stay the same length as when declared; can contain multiple types
  let tup: (i32, f64, u8) = (500, 6.4, 1); // sets a different type for each value

  let tup = (500, 6.4, 1);
  let (x, y, z) = tup; // Access by destructuring; turns tuple into three different variables
  println!("The value of y is: {}", y);

  // Or access values directly
  let x: (i32, f64, u8) = (500, 6.4, 1);

  let five_hundred = x.0;
  let six_point_four = x.1;
  let one = x.2;

  // Arrays in Rust can't change length AND can contain only one type
    let a = [1, 2, 3, 4, 5]; // like JS, elemets are at a[0], a[1] etc

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // (explicitly setting type and length)
    let a = [3; 5]; // = let a = [3, 3, 3, 3, 3];
  }


// Rust is a statically typed language, 
// which means that it must know the types of all variables at compile time. 

// Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, 
// where n is the number of bits that variant uses. So an i8 can store numbers 
// from -(27) to 27 - 1, which equals -128 to 127. Unsigned variants can store 
// numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.