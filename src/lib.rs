mod iter;
mod util;

pub use iter::*;
pub use util::*;

#[cfg(test)]
mod tests {
  use crate::IndexSplitter;

  #[test]
  fn it_works() {
    let array = (0..20).map(|it| it + 1).collect::<Vec<_>>();
    println!("Array: {:?}", array);
    let indexes = [5, 10, 15];
    println!("Indexes: {:?}", indexes);
    let split = array.split_at_indexes_unchecked(indexes).collect::<Vec<_>>();
    println!("Split: {:?}", split);
    let split_inclusive = array.split_at_indexes_inclusive_unchecked(indexes).collect::<Vec<_>>();
    println!("Split Inclusive: {:?}", split_inclusive);

  }
}
