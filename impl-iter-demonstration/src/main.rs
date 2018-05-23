fn get_iter(v: Vec<u32>) -> impl Iterator<Item = u32> {
    v.into_iter()
      .map(|i| i * i)
      .filter(|i| *i > 5)
}

fn main() {
  let items = vec![1, 2, 3, 4, 5];

  let x = get_iter(items).into_iter().nth(0);

  match x {
    Some(i) => println!("The first squared value > 5 is: {}", i),
    None => println!("None of the items' square > 5"),
  }
}

