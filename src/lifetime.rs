pub fn run() {
  let st1 = String::from("x");
  let st2 = String::from("y");
  let res1 = get_longest(&st1, &st2);
  println!("{}", res1);
  let st3 = String::from("x");

  // OK: get_longestの返り値のライフタイムは短いst4のものが適用されるがres2のスコープもst4と同じなので問題なし
  {
    let st4 = String::from("y");
    let res2 = get_longest(&st3, &st4);
    println!("{}", res2);
  }

  // NG: get_longestの返り値のライフタイムはst4と同じ。res2のライフタイムはそれよりも長いため、ダングリングポインタになる。
  // 以下のres2は何も型を指定していないが、コンパイラが型を逆算して推論するのでホバーすると型が表示されている。
  // let res2;
  // {
  //   let st4 = String::from("y");
  //   res2 = get_longest(&st3, &st4);
  // }
  // println!("{}", res2);
}

// <'a>とすることでライフタイムをannotationしている。返り値のライフタイムは引数の短い方のライフタイムが適用される。
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

// 関数の中で実態を作ってその参照を返しても、その関数の実行が終わった段階でメモリが解放されるのでダングリングポインタになり、エラーになる。
// fn dymmy1<'a>() -> &'a str {
//   let s = String::from("demo");
//   &s
// }

// これはOK
// fn dummy3() -> String {
//   let s = String::from("demo");
//   s
// }
