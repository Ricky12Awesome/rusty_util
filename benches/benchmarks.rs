use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rusty_util::IndexSplitter;

mod ref_iter {
  pub struct IndexSplitter<'a, T: Sized> {
    cur_start: usize,
    arr: &'a [T],
    indexes: &'a [usize],
  }

  impl<'a, T> IndexSplitter<'a, T> {
    pub fn new(arr: &'a [T], indexes: &'a [usize]) -> Self {
      Self {
        cur_start: 0,
        arr,
        indexes,
      }
    }
  }

  impl<'a, T: Sized> Iterator for IndexSplitter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
      match &self.indexes {
        [] => match &self.arr[self.cur_start..] {
          [] => None,
          a => {
            self.cur_start = self.arr.len();
            Some(a)
          }
        },
        [first, ..] => {
          let a = &self.arr[self.cur_start..*first];
          self.cur_start = first + 1;
          self.indexes = &self.indexes[1..];
          Some(a)
        }
      }
    }
  }
}

pub fn criterion_benchmark(c: &mut Criterion) {
  let elements = 1_000_000;
  let input = (0..elements, (0..elements).step_by(100));
  let input = (input.0.collect::<Vec<_>>(), input.1.collect::<Vec<_>>());

  c.bench_with_input(
    BenchmarkId::new("Split At Indexes [Clone]", format!("{}", elements)), &input,
    |b, (arr, indexes)| {
      b.iter(move || {
        arr.split_at_indexes_unchecked(indexes).for_each(drop)
      })
    },
  );

  c.bench_with_input(
    BenchmarkId::new("Split At Indexes [Ref]", format!("{}", elements)), &input,
    |b, (arr, indexes)| {
      b.iter(move || {
        ref_iter::IndexSplitter::new(arr, indexes).for_each(drop);
      })
    },
  );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);