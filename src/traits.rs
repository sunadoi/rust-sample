trait Fruits {
  fn price(&self) -> u32;
}

struct Apple;
impl Fruits for Apple {
  fn price(&self) -> u32 {
    10
  }
}

struct Banana;
impl Fruits for Banana {
  fn price(&self) -> u32 {
    5
  }
}

trait Summary {
  fn summarize(&self) -> String; // ここに具体的な処理を書くと、それがデフォルトの挙動になる。オーバーライド可能
}

trait Message {
  fn message(&self) -> String {
    String::from("Message")
  }
}

struct NewsArticle {
  headline: String,
  location: String,
  author: String,
  content: String,
}
impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}
impl Message for NewsArticle {}

struct Tweet {
  username: String,
  content: String,
  reply: bool,
  retweet: bool,
}
impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

pub fn run() {
  let apple = Apple {};
  let banana = Banana {};
  get_price(apple);
  get_price(banana);

  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };
  println!("1 new tweet: {}", tweet.summarize());

  let article = NewsArticle {
    headline: String::from("headline"),
    location: String::from("location"),
    author: String::from("author"),
    content: String::from("content"),
  };
  println!("{}", article.summarize());
  notify(&article);
  notify_another(&article);
}

fn get_price<T: Fruits>(fruits: T) {
  println!("price is: {}", fruits.price())
}

fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

fn notify_another(item: &(impl Summary + Message)) {
  println!("Breaking news! {}", item.summarize());
  println!("Message! {}", item.message());
}
