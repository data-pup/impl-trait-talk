fn broken_function() {
    vec![1, 2, 3, 4, 5]
      .into_iter()
      .map(|i| i * 2)
      .filter(|i| *i > 5)
}

fn main() {
  broken_function();
}
