fn main() {
  // 
  let guess: u32 = "42".parse().expect("Not a number!");
  println!("This guess is: {}", guess);

  // Intergers
  let val_signed32: i32 = -55;
  let val_unsigned32: u32 = 55;

  println!("Signed integer: {}", val_signed32);
  println!("Unsigned integer: {}", val_unsigned32);
}
