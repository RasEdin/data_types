fn main() {
  let guess: u32 = "42".parse().expect("Not a number!");
  println!("This guess is: {}", guess);
}
