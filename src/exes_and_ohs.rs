fn xo(string: &'static str) -> bool {
  let mut count_x = 0;
  let mut count_o = 0;
  // 为啥会报错
  for s in string.chars(){
      if s == 'x' || s == 'X'{
          count_x +=1;
      }
      else if s == 'O' || s == 'o'{
          count_o +=1;
      }
  }
  count_x == count_o
}



#[test]
fn returns_expected() {
  assert_eq!(xo("xo"), true);
  assert_eq!(xo("Xo"), true);
  assert_eq!(xo("xxOo"), true);
  assert_eq!(xo("xxxm"), false);
  assert_eq!(xo("Oo"), false);
  assert_eq!(xo("ooom"), false);
}