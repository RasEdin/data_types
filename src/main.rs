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

  // Boolean
  let t = true;

  let f: bool = false; // Explicit type annotation
  println!("t is: {} and f is: {}", t, f);
  // tuples
  let tup = (500, 36, 25);

  let (x, y, _z) = tup;

  println!("The value of y is {} and the value of x is {}", y, x);
  
  let tup2 = tup.2;
  println!("The third value in the tuple should be 25, it is: {}", tup2);

  // Arrays
  let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
  let a: [i32; 5] = [1, 2, 3, 4, 5]; // Explicit type defined, length of five

  let _first = a[0]; // Underscore at a variable name makes the variable not neccesary to be called.
  let _second = a[1];

  println!("month 2 is: {}", months[1]);
  
  // functions
  let function_parameter: i32 = 64;
  let second_parameter: i32 = 46;
  a_second_function(function_parameter, second_parameter);
  bindingfunction();
  println!("The function five() returns: {}",five());
  println!("Add one to value: {}, {}", 6, plus_one(6));

  // Control structures
}

// function 1 parameter signed 32-bit integer 
fn a_second_function(x: i32, y: i32)
{
  println!("Called the second function");
  println!("The value passed to the function is: {}", x);
  println!("The 2nd value passed to the function is: {}", y);
}

fn bindingfunction()
{
  let _x = 5;

  let y = {
      let x = 3;
      x + 1
  };

  println!("The value of y is: {}", y);
}
// Function with return type
fn five() -> i32 { 
  return 5
}

fn plus_one(x: i32) -> i32 {
  return x + 1
}