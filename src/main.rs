extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  sample_type();
  // sample_len();
  // sample_rang();
}

/*
  Type System

 Length 	Signed 	Unsigned
  8-bit 	i8 	u8
  16-bit 	i16 	u16
  32-bit 	i32 	u32
  64-bit 	i64 	u64
  arch 	isize 	usize

  f32
  f64 (default is float 64bit)

  bool

  char
 */
fn sample_type() {
  The Tuple Type
  let tup: (i32, bool) = (500, true);
  let first = tup.0;
  let (x, y) = tup;
  println!("{}-{}-{}", x, y, first);
}

fn sample_len() {
  let spaces = "   ";
  println!("{}", spaces.len())
}

fn sample_rang() {
  let secret_number = rand::thread_rng().gen_range(1, 101);

  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(err) => {
        println!("error: {}", err);
        continue;
      }
    };

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
