pub fn run() {
  // String型は所有権が移動する
  let s1 = String::from("hello");
  let s2 = s1;
  // println!("{} {}", s1, s2) // s1の所有権はs2に移っているのでもうs1へのアクセスはできない
  println!("{}", s2);

  // 整数型はCopy traitを実装しているから所有権のmoveが行われない (別々のアドレスに格納されている)
  let i1 = 1;
  let i2 = i1;
  println!("{} {}", i1, i2);
  println!("Stack address of i1 is {:p}", &i1);
  println!("Stack address of i2 is {:p}", &i2);

  let sl1 = "hello";
  let sl2 = sl1;
  println!("{} {}", sl1, sl2);
  println!("Stack address of i1 is {:p}", &sl1);
  println!("Stack address of i2 is {:p}", &sl2);

  let s3 = String::from("hello");
  let s4 = s3.clone();
  println!("{} {}", s3, s4); // cloneすると所有権がmoveしない (heapに同じデータをdeep copyする)
  println!("Stack address of s3 is {:p}", &s3);
  println!("Stack address of s4 is {:p}", &s4);
  println!("Heap memory address of s3 is {:p}", s3.as_ptr());
  println!("Heap memory address of s4 is {:p}", s4.as_ptr());

  let s5 = String::from("hello");
  println!("Stack address of s5 is {:p}", &s5); // ①
  println!("Heap memory address of s5 is {:p}", s5.as_ptr()); // ②
  take_ownership(s5);
  // println!("{}", s5) // take_ownershipに渡した時点で所有権がmoveしている
  // ①と③は違う値。それぞれ別々のstackに作られる。②と④は同じ値。Heapにある実データのアドレスは引き継がれる。

  let s6 = String::from("hello");
  println!("Stack address of s6 is {:p}", &s6);
  println!("Heap memory address of s6 is {:p}", s6.as_ptr());
  let s7 = take_giveback_ownership(s6);
  println!("Stack address of s7 is {:p}", &s7);
  println!("Heap memory address of s7 is {:p}", s7.as_ptr());
  println!("{}", s7); // 所有権が移っているのでs6にはアクセスできない

  // リファレンスで渡せば所有権は移らない。参照する権利だけを貸す
  let s8 = String::from("hello");
  let len = calculate_length(&s8);
  println!("{} {}", s8, len); // 所有権が移っていないのでs8にアクセス可能

  // mutableなリファレンスを渡せば変更可能
  let mut s9 = String::from("hello");
  change(&mut s9);
  println!("{}", s9);

  let s10 = String::from("hello");
  let r1 = &s10;
  // let r2 = &mut s10; // imutableなコピーとmutableなコピーは共存できない
  let r2 = &s10;
  println!("{} {} {}", s10, r1, r2);

  let mut s11 = String::from("hello");
  let r1 = &mut s11;
  // println!("{}", s11); // mutableなr1が有効な範囲ではたとえもとの所有権の持ち主のs11でもここでは呼び出せない。r1がimutableならok
  println!("{}", r1);
  println!("{}", s11); // ここならmutableなr1は有効ではないためs11が呼び出せる
}

fn take_ownership(s: String) {
  println!("Stack address of s5 is {:p}", &s); // ③
  println!("Heap memory address of s5 is {:p}", s.as_ptr()); // ④
  println!("{}", s)
}

fn take_giveback_ownership(s: String) -> String {
  s
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn change(s: &mut String) {
  s.push_str("_world");
}
