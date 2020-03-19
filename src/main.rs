fn main() {
  // Learning a bit of Rust
  let guess: u32 = "42".parse().expect("Not a number!");
  println!("This guess is: {}", guess);

  // Intergers
  let val_signed32: i32 = -55;
  let val_unsigned32: u32 = 55;

  println!("Signed integer: {}", val_signed32);
  println!("Unsigned integer: {}", val_unsigned32);

  // Floats
  let x = 2.0;
  let y: f32 = 3.0;

  println!("x : f64 is {}, y : f32 is {}", x, y);

  // Numeric operators
  let sum = 3f64 / 1.5;
  println!("Sum of 3 / 1.5: {}", sum);
}
