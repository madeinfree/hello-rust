extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  // sample_control_flow();
  // sample_function_call();
  // sample_type();
  // sample_len();
  // sample_rang();
}

fn sample_control_flow() {
  /*
   * reverse 顛倒 -> (1..5).rev()
   * 1..5 不包含 5 -> 1,2,3,4
   */
  for number in (1..5) {
    println!("{}", number);
  }

  // *** iter
  // let a = [10, 20, 30];
  // for elem in a.iter() {
  //   println!("{}", elem);
  // }

  // let mut number = 3;
  // while number != 0 {
  //   println!("{}", number);
  //   number = number - 1;
  // }
  // println!("{}", "STOP!");

  // 禁止亂玩
  // loop {
  //   println!("{}", "?")
  // }

  // let condition = false;
  // let number = if condition { 5 } else { 6 }; //  = condition ? 5 : 6
  // println!("{}", number);
  // let mut number = String::new();
  // println!("{}", "Please input your number !");
  // io::stdin()
  //   .read_line(&mut number)
  //   .expect("Failed to read line");
  // let number: i32 = number.trim().parse().expect("ERROR");
  // if number < 5 {
  //   println!("{} < 5", number);
  // } else if number < 8 {
  //   println!("{} < 8", number);
  // } else if number <= 10 {
  //   println!("{} <= 10", number);
  // } else {
  //   println!("{} > 10", number);
  // }
}

fn sample_function_call() {
  // -> 返回一個類型(i32)
  fn plus_five(x: i32) -> i32 {
    x + 5
  }
  // let x = 5;
  // let y = { x + 1 // return (x + 1) };
  println!("{}", plus_five(5));
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
  // The Tuple Type
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
