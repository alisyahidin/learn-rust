use std::collections::HashMap;

pub fn get_vector() {
  let mut vector: Vec<u8> = Vec::new();
  vector.push(3);
  vector.push(7);
  vector.push(255);

  for (pos, i) in vector.iter().enumerate() {
    println!("loop vector: {} {}", pos, i);
  }

  let mut scores = HashMap::new();

  let test = "a";
  scores.insert(test, 3);
  scores.insert("b", 10);

  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);

  println!("{:?}", scores);
}
