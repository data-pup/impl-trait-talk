/// This will -not- compile. The error generated by
/// this is used to illustrate the types that adaptor
/// functions like `map` and `filter` return.
fn broken_function() {
    vec![1, 2, 3, 4, 5]
        .into_iter()
        .map(|i| i * 2)
        .filter(|i| *i > 5)
}

fn main() {
    broken_function();
}
