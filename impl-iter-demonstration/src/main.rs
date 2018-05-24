fn get_impl_iter(v: Vec<u32>) -> impl Iterator<Item = u32> {
    v.into_iter()
      .map(|i| i * i)
      .filter(|i| *i > 5)
}

fn main() {
  let items = vec![1, 2, 3, 4, 5];
  let mut results = get_impl_iter(items);

  let x_should_be_even = true;

  let x = if x_should_be_even {
    results.filter(|i| i % 2 == 0).next()
  } else {
    results.next()
  };

  match x {
    Some(i) => println!("The first result is: {}", i),
    None => println!("No results found!"),
  }
}
