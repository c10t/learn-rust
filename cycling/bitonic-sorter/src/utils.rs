use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
  let mut rng = Pcg64Mcg::from_seed([0; 16]);
  rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
  x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
  x.windows(2).all(|pair| pair[0] >= pair[1])
}

mod tests {
  use crate::utils::{new_u32_vec, is_sorted_ascending, is_sorted_descending};
  use crate::SortOrder::{Ascending, Descending};
  use crate::third::sort;

  #[test]
  fn sort_u32_large() {
    {
      let mut x = new_u32_vec(65536);
      assert_eq!(sort(&mut x, &Ascending), Ok(()));
      assert!(is_sorted_ascending(&x))
    }
    {
      let mut x = new_u32_vec(65536);
      assert_eq!(sort(&mut x, &Descending), Ok(()));
      assert!(is_sorted_descending(&x))
    }
  }
}
