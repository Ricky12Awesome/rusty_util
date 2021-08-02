pub use split_iterator::*;

mod split_iterator {
  pub struct IndexSplitterIterator<'a, T: Sized> {
    cur_start: usize,
    arr: &'a [T],
    indexes: Vec<usize>,
    inclusive: bool,
  }

  pub trait IndexSplitter<'a, T> {
    fn split_at_indexes_unchecked<I: Into<Vec<usize>>>(&'a self, indexes: I) -> IndexSplitterIterator<'a, T>;
    fn split_at_indexes<I: Into<Vec<usize>>>(&'a self, indexes: I) -> IndexSplitterIterator<'a, T> {
      let mut sorted = indexes.into();
      sorted.sort();

      self.split_at_indexes_unchecked(sorted)
    }

    fn split_at_indexes_inclusive_unchecked<I: Into<Vec<usize>>>(&'a self, indexes: I) -> IndexSplitterIterator<'a, T>;
    fn split_at_indexes_inclusive<I: Into<Vec<usize>>>(&'a self, indexes: I) -> IndexSplitterIterator<'a, T> {
      let mut sorted = indexes.into();
      sorted.sort();

      self.split_at_indexes_inclusive_unchecked(sorted)
    }

  }

  impl<'a, T> IndexSplitterIterator<'a, T> {
    pub fn new<I: Into<Vec<usize>>>(arr: &'a [T], indexes: I, inclusive: bool) -> Self {
      Self {
        cur_start: 0,
        arr,
        indexes: indexes.into(),
        inclusive,
      }
    }
  }

  impl<'a, T> IndexSplitter<'a, T> for [T] {
    fn split_at_indexes_unchecked<I: Into<Vec<usize>>>(&'a self, indexes: I) -> IndexSplitterIterator<'a, T> {
      IndexSplitterIterator::new(self, indexes, false)
    }

    fn split_at_indexes_inclusive_unchecked<I: Into<Vec<usize>>>(&'a self, indexes: I) -> IndexSplitterIterator<'a, T> {
      IndexSplitterIterator::new(self, indexes, true)
    }
  }

  impl<'a, T: Sized> Iterator for IndexSplitterIterator<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
      match *self.indexes {
        [] => match &self.arr[self.cur_start..] {
          [] => None,
          a => {
            self.cur_start = self.arr.len();
            Some(a)
          }
        },
        [first, ..] => {
          let a = &self.arr[self.cur_start..first];
          self.cur_start = first + if self.inclusive { 0 } else { 1 };
          self.indexes.remove(0);
          Some(a)
        }
      }
    }
  }

  #[cfg(test)]
  mod tests {
    use crate::iter::split_iterator::*;

    const RESULT: &[&[i32]] = &[&[1, 2, 3, 4, 5], &[7, 8, 9, 10], &[12, 13, 14, 15], &[17, 18, 19, 20]];

    #[test]
    fn split() {
      let array = (0..20).map(|it| it + 1).collect::<Vec<_>>();
      let indexes = [5, 10, 15];
      let split = array.split_at_indexes(indexes).collect::<Vec<_>>();

      assert_eq!(split, RESULT, "Testing split_at_indexes_unchecked");
    }

    #[test]
    fn split_inclusive() {
      let array = (0..20).map(|it| it + 1).collect::<Vec<_>>();
      let indexes = [5, 10, 15];
      let split = array.split_at_indexes_inclusive(indexes).collect::<Vec<_>>();

      assert_eq!(split, RESULT, "Testing split_at_indexes_inclusive_unchecked");
    }
  }
}