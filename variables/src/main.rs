fn main() {
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);

  // A constant can be set globally
  const MAX_POINTS: u32 = 100_000;
  println!("Max points: {}", MAX_POINTS);

  // Shadowing. Differs from mut vars because 1) we use 'let' each time,
  // 2) we can change it a few times with let, but then it is immutable
  // when we don't use let, and 3) We can change type with shadowing.
  let x = 5;

  // This 'x' 'shadows' the first one
  let x = x + 1;

  let x = x * 2;

  println!("The value of x is: {}", x);

  // Changing type with shadowing
  let spaces = "   ";
  let spaces = spaces.len();
  println!("Spaces: {}", spaces);

  
}
