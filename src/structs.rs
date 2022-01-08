#[derive(Debug)]
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn create(width: u32, height: u32) -> Self {
    Self { width, height }
  }
  // &をつけないと元々のインスタンスから所有権を奪ってしまう
  fn area(&self) {
    println!("{}", self.width * self.height);
  }
}

pub fn run() {
  let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("somuusername123"),
    active: true,
    sign_in_count: 1,
  };
  let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("somuusername123"),
    active: true,
    sign_in_count: 1,
  };
  user1.email = String::from("anotheremail@example.com");
  println!("{:#?}", user1);

  let user2 = build_uer(String::from("email2"), String::from("user2"));
  println!("{:#?}", user2);

  let rect = Rectangle::create(20, 20);
  rect.area();
  println!("{:#?}", rect);
}

fn build_uer(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
