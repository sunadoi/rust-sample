pub fn run() {
  // 配列はコンパイル時に長さが決まっている
  let a1: [u8; 7000000] = [1; 7000000];

  // vectorは長さを変えられる。
  // Stringと同じようなデータ構造。ただしlenとcapacityはバイト数ではなく要素数
  // 最初の8bytesではポインタの位置だけでなく、型の情報も持っているので、2番目の要素は何bytes足せば得られるかがわかっている
  let mut v1 = vec![1, 2, 3, 4];
  let v2 = vec![5, 6, 7, 8];
  let mut v3 = vec![9, 10];
  println!("Stack address of v1, {:p}", &v1);
  println!("Stack address of v2, {:p}", &v2);
  v1.insert(1, 10);
  println!("{:?}", v1);

  // Boxポインタ
  // stackにある内容をheapに移す。移した先のheapの先頭アドレスをstackに保持する。
  let t1: (i64, String) = (10, String::from("hello"));
  let mut b1 = Box::new(t1); // b1はheapに移したデータの先頭アドレス
  (*b1).1 += "world";
  println!("{}, {}", b1.0, b1.1);
}
