/// Return an iterator through the squares of the elements
/// of the vector `v` that are greater than 5.
fn get_impl_iter(v: Vec<u32>) -> impl Iterator<Item = u32> {
    let filter_condition = get_filter_cond(5);
    v.into_iter().map(|i| i * i).filter(filter_condition)
}

/// Create a closure that returns true if the input is a reference
/// to a 32-bit unsigned integer greater than 5.
///
/// The `move` keyword forces the closure to take ownership of `min`.
fn get_filter_cond(min: u32) -> impl Fn(&u32) -> bool {
    move |&i| i > min
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
