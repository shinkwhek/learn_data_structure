pub mod union_find {
  #[derive(Debug, PartialEq)]
  pub struct UnionFindData {
    pub maxN : usize,
    pub par : Vec<usize>
  }

  impl UnionFindData {
    pub fn new(maxN : usize) -> Self {
      Self {
        maxN : maxN,
        par : (0..maxN).collect()
      }
    }

    pub fn set_path (mut self, i : usize, x : usize) -> Self {
      self.par[i] = x;
      self
    }

    pub fn find_root(&mut self, i : usize) -> usize {
      if self.par[i] == i {
        i
      } else {
        // path compression
        self.par[i] = self.find_root(self.par[i]);
        self.par[i]
      }
    }

    pub fn same(&mut self, a : usize, b : usize) -> bool {
      self.find_root(a) == self.find_root(b)
    }

    pub fn union(mut self, x : usize, y : usize) -> Self {
      let x = self.find_root(x);
      let y = self.find_root(y);
      if x != y {
        self.par[x] = y;
      }
      self
    }
  }
}

#[cfg(test)]
mod union_find_test {
  use super::*;

  #[test]
  fn new_test() {
    let a = union_find::UnionFindData::new(10);
    let b = union_find::UnionFindData {
      maxN : 10,
      par : vec![0,1,2,3,4,5,6,7,8,9],
    };
    assert_eq!(a,b);
  }

  #[test]
  fn find_root_test() {
    let mut a = union_find::UnionFindData::new(3)
              .set_path(1, 0)
              .set_path(2, 1);
    let root = a.find_root(2);

    assert_eq!(0, root);
  }

  #[test]
  fn path_compression_test() {
    let mut a = union_find::UnionFindData::new(3)
                  .set_path(1, 0)
                  .set_path(2, 1);
    let _ = a.find_root(2);
    assert_eq!(a.par[1], a.par[2]);
  }

  #[test]
  fn same_test() {
    let mut a = union_find::UnionFindData::new(6)
                  .set_path(2, 0)
                  .set_path(3, 1)
                  .set_path(5, 1);
    let may_true = a.same(1, 5);
    let may_false = a.same(0, 3);
    assert_eq!(may_true, true);
    assert_eq!(may_false, false);
  }

  #[test]
  fn union_test() {
    let mut a = union_find::UnionFindData::new(6)
                  .set_path(2, 0)
                  .set_path(3, 1)
                  .set_path(5, 1);
    let unioned = a.union(2, 3);
    let b = union_find::UnionFindData::new(6)
                  .set_path(2, 0)
                  .set_path(3, 1)
                  .set_path(5, 1)
                  .set_path(0, 1);
    assert_eq!(unioned, b);
  }

}