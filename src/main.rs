extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  // sample_string_slice();
  // sample_slice_type();
  // sample_ownership_multi_ref();
  // sample_ownership_ref();

  // sample_ownership();
  // sample_control_flow();

  // sample_function_call();
  // sample_type();
  // sample_len();
  // sample_rang();
}

fn sample_string_slice() {
  let s = String::from("Hello World");
  let commonS = "Hello Common World";

  // let hello = &s[..5];
  // let world = &s[6..];
  // println!("{}, {}", hello, world)
  let s2 = first_word(&s);
  // let c2 = first_word(&commonS[..]);
  let c2 = first_word(commonS);
  println!("{}, c2 -> {}", s2, c2);

  fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
        return &s[0..i];
      }
    }

    &s[..]
  }
}

fn sample_slice_type() {
  let s = String::from("Hello");
  let len = first_word(&s);
  println!("{}, {}", s, len);
  fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
      println!("{}, {}", item, b' ');
      if item == b' ' {
        return i;
      }
    }
    s.len()
  }
}

fn sample_ownership_multi_ref() {
  let mut s = String::from("Hello");

  {
    let r1 = &mut s;
  }
  let r2 = &mut s;
}

/*
 * 利用 & 指標
 * 使得變數不會 move 進另一個方法中
 */
fn sample_ownership_ref() {
  // immutable
  let s = String::from("Hello");
  let len = calculate_length(&s);
  println!("{}, {}", s, len); // s 保持
  fn calculate_length(s: &String) -> usize {
    s.len()
  }
  // mutable - &mut
  let mut s1 = String::from("Hello");
  change_string(&mut s1);
  println!("{}", s1);
  fn change_string(s: &mut String) {
    s.push_str(", World")
  }
}

/* 會強制 copy 等於 clone
 * All the integer types, such as u32.
 * The Boolean type, bool, with values true and false.
 * All the floating point types, such as f64.
 * The character type, char.
 * Tuples, but only if they contain types that are also Copy. For example,
 * (i32, i32) is Copy, but (i32, String) is not.
 *
 * 當一個（除上述類型）變數被傳入一個方法中，就會移動至方法內
 * 原本的則無法再使用。
 * 強制 copy 類型不在此限制。
 */
fn sample_ownership() {
  let s = String::from("Hello");
  // let number = 5;
  // println!("{}", s);
  // let mut s2 = s;
  // s2.push_str(", World");
  // println!("{}", s2);

  // takes_ownership(s);
  // makes_copy(number);
  // println!("{}", number);
  // fn takes_ownership(some_string: String) {
  //   println!("{}", some_string);
  // }
  // fn makes_copy(some_interger: i32) {
  //   println!("{}", some_interger);
  // }

  let (s2, len) = calculate_length(s);
  println!("{}, {}", s2, len);
  fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
  }
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
