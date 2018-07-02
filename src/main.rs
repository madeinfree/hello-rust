extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::ErrorKind;

fn main() {
  sample_validating_lifetime();
  // sample_impl_trait_type();
  // sample_generic_types();

  // sample_error_result();
  // sample_error_panic();
  // sample_hash_map();
  // sample_vector();
  // sample_if_let();
  // sample_match();

  // sample_enum();
  // sample_method_syntax();
  // sample_use_struct();
  // sample_defining_struct();
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

fn sample_validating_lifetime() {
  // error sample
  // let r;
  // {
  //   let x = 5;
  //   r = &x;
  // }
  // println!("r: {}", r);

  // Lifetime Annotation Syntax (生命週期註釋語法)
  /*
   * &i32 - a reference
   * &'a i32 - a reference with an explicit lifetime
   * &'a mut i32 - a mutable reference with and explicit lifetime
   */
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
      x
    } else {
      y
    }
  }

  struct ImportantExcerpt<'a> {
    part: &'a str,
  }
  let novel = String::from("Call. me...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");

  let i = ImportantExcerpt {
    part: first_sentence,
  };
  println!("{:?}", i.part);

  // Static Lifetime
  let s: &'static str = "I have a static lifetime";
  println!("{}", s);

  // G.T.L Together
  use std::fmt::Display;
  fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
  where
    T: Display,
  {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
      x
    } else {
      y
    }
  }
  let l = longest_with_an_announcement("OK", "NOT", 1);
  println!("{}", l);
}

// ***
fn sample_impl_trait_type() {
  pub trait Summary {
    fn summarize(&self) -> String;
  }
  pub struct NewsArticle {
    pub headline: String,
  }
  pub trait Displayer {
    fn summarizeD(&self) -> String;
  }
  impl Summary for NewsArticle {
    fn summarize(&self) -> String {
      format!("{}", self.headline)
    }
  }
  impl Displayer for NewsArticle {
    fn summarizeD(&self) -> String {
      format!("{}", self.headline)
    }
  }
  let summary = NewsArticle {
    headline: String::from("HELLO"),
  };

  // println!("{}", summary.summarize());

  pub fn notify<T: Summary + Displayer>(item: T) {
    println!("{}, {}", item.summarize(), item.summarizeD());
  }
  notify(summary);

  use std::fmt::Display;

  struct Pair<T> {
    x: T,
    y: T,
  }

  impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
      Self { x, y }
    }
  }

  impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
      if self.x >= self.y {
        println!("The largest member is x = {}", self.x);
      } else {
        println!("The largest member is y = {}", self.y);
      }
    }
  }
  let pair = Pair::new(3, 5);
  pair.cmp_display();
}

fn sample_generic_types() {
  fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
      if item > largest {
        largest = item;
      }
    }

    largest
  }

  let number_list = vec![2, 4, 6];
  let result = largest(&number_list);
  println!("{}", result);

  struct Point<T, U> {
    x: T,
    y: U,
  }

  let integer = Point { x: 5, y: 10.0 };
  println!("{}", integer.y);
}

fn sample_error_result() {
  /**
   *
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
   */
  use std::fs::File;
  use std::io::Read;
  let f = File::open("./src/hello.txt");
  let f = match f {
    Ok(file) => file,
    Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
      Ok(fc) => fc,
      Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
    },
    Err(error) => panic!("There was a problem opening the file: {:?}", error),
  };

  // let f2 = File::open("world.txt").expect("Failed to open world.txt");

  fn read_file() -> Result<String, io::Error> {
    let f = File::open("./src/hello.txt");

    let mut f = match f {
      Ok(file) => file,
      Err(e) => return Err(e),
    };

    let mut s = String::new();

    // shortcut
    f.read_to_string(&mut s)?;
    Ok(s)
    // match f.read_to_string(&mut s) {
    //   Ok(_) => Ok(s),
    //   Err(e) => Err(e),
    // }
  }
  println!("{:?}", read_file());

  // shortcut use function()?
  fn read_file2() -> Result<String, io::Error> {
    let mut s2 = String::new();

    File::open("./src/hello.txt")?.read_to_string(&mut s2);
    Ok(s2)
  }

  println!("{:?}", read_file2());

  // Guess - 09-03 last sample
  pub struct Guess {
    value: u32,
  }

  impl Guess {
    pub fn new(value: u32) -> Guess {
      if value < 1 || value > 100 {
        panic!("Guess value must be between 1 and 100, got {}.", value);
      }
      // 定義 Guess 物件
      Guess { value }
    }
    pub fn value(&self) -> u32 {
      self.value
    }
  }

  println!("{}", Guess::new(120).value);
}

fn sample_error_panic() {
  let v = vec![1, 2, 3];
  v[99];
  panic!("crash and burn");
  println!("{}", "Run here?"); // Not run
}

fn sample_hash_map() {
  use std::collections::HashMap;
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);

  // let teams = vec![String::from("Blue")];
  // let init_scores = vec![10];

  // let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();

  // 檢查是否存在, 否則帶入 50
  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);

  println!("{:?}", scores);

  let text = "hello world wonderful world";
  let mut map = HashMap::new();

  // dereference count using the asterisk (*)
  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);
}

fn sample_vector() {
  // let v: Vec<i32> = Vec::new();
  // let v = vec![1, 2, 3];
  // for i in &v {
  //   println!("{}", i);
  // }

  // let mut update_v = Vec::new();
  // update_v.push(5);

  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(1),
    SpreadsheetCell::Float(1.11),
    SpreadsheetCell::Text(String::from("Hello World")),
  ];
}

// if let 相等於 match 用法
fn sample_if_let() {
  let some_u8_value = Some(0u8);
  // match some_u8_value {
  //   Some(3) => println!("{}", "three"),
  //   _ => println!("{}", "other"),
  // }
  if let Some(3) = some_u8_value {
    println!("{}", "three");
  }
}
/*
 * match 像 if 一樣
 * 不同的是, if 只能返回 boolean
 * 而 match 可以返回一個值
 */
fn sample_match() {
  #[derive(Debug)]
  enum UsState {
    Alabama,
    Alaska,
  }
  enum Coin {
    Penny,
    Whien(UsState),
  }

  fn value_in_cents(coin: Coin) -> u32 {
    match coin {
      Coin::Penny => {
        println!("{}", "Lucky Penny!");
        1
      }
      Coin::Whien(state) => {
        println!("State from {:?}!", state);
        25
      }
    }
  }

  let c = value_in_cents(Coin::Whien(UsState::Alabama));
  println!("{}", c);

  fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
      None => None,
      Some(i) => Some(i + 1),
    }
  }
  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  // _ Placeholder
  let some_u8_value = 0u8;
  match some_u8_value {
    1 => println!("{}", "one"),
    7 => println!("{}", "seven"),
    _ => println!("{}", "None"),
  }
}

fn sample_enum() {
  enum IpAddrKind {
    V4,
  }

  struct IpAddr {
    kind: IpAddrKind,
    address: String,
  }

  let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
  };
}

fn sample_method_syntax() {
  #[derive(Debug)] //Debug
  struct Rectangle {
    width: u32,
    height: u32,
  }

  impl Rectangle {
    fn area(&self) -> u32 {
      self.width * self.height
    }
  }

  let rect = Rectangle {
    width: 30,
    height: 50,
  };

  println!("{}", rect.area());
}

fn sample_use_struct() {
  // adding structs
  #[derive(Debug)] //Debug
  struct Rectangle {
    width: u32,
    height: u32,
  }

  // Refactoring with Tuples
  // let rect = (30, 50);
  let rect = Rectangle {
    width: 30,
    height: 50,
  };

  // Debug
  println!("{:?}", rect); // Rectangle { width: 30, height: 50 }
  println!("{}", area(&rect));
  fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
  }
}

fn sample_defining_struct() {
  struct User {
    username: String,
    active: bool,
  }

  struct Color(i32, i32, i32);

  fn build_user(username: String) -> User {
    User {
      username: username,
      active: true,
    }
  }

  let user1 = build_user(String::from("Whien"));
  let user2 = User { ..user1 };
  println!("{}", user2.username);

  let black = Color(0, 0, 0);
  println!("{}", black.0);
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
