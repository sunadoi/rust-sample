struct Point<T> {
  x: T,
  y: T,
}

struct PointAnother<T, U> {
  x: T,
  y: U,
}

impl<T, U> PointAnother<T, U> {
  fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
    PointAnother {
      x: self.x,
      y: other.y,
    }
  }
}

pub fn run() {
  let number_list = vec![1, 2, 3, 5, 4];
  let char_list = vec!['a', 'b', 'c', 'd']; // Rustでは''はchar型(4bytes)、""は文字列スライス
  println!("{}", largest(char_list));
  println!("{}", largest(number_list));

  let p1 = Point { x: 1, y: 2 };
  let p2 = PointAnother { x: 1.0, y: "a" };
  let p3 = PointAnother { x: "Rust", y: 100 };
  let p4 = PointAnother {
    x: 1.0,
    y: "Awesome",
  };
  let p5 = p3.mixup(p4);
  println!("{} {}", p5.x, p5.y);
}

// <>でgenericsを定義
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
  let mut largest = list[0];
  for item in list {
    // ただのTだと比較ができないものも含まれるのでtraitで狭める
    if item > largest {
      largest = item;
    }
  }
  largest
}
