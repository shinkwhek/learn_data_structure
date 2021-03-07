pub mod SegmentTree {

  use std::cmp;
  use std::cmp::Ordering;


  #[derive(Debug, Eq)]
  struct Node {
    elm : usize,
    interval : (usize, usize)
  }

  impl Ord for Node {
    fn cmp(&self, other : &Self) -> Ordering {
      self.elm.cmp(&other.elm)
    }
  }
  impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
  }
  impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.elm == other.elm
    }
  }

  impl Node {
    pub fn zero() -> Self {
      Self {
        elm : usize::MAX,
        interval : (0,0)
      }
    }
    
    pub fn new(elm: usize, interval: (usize, usize)) -> Self {
      Self {
        elm: elm,
        interval: interval
      }
    }
  }

  #[derive(Debug, PartialEq)]
  pub struct SegTree {
    max_n : usize,
    dat : Vec<Node>,
  }

  impl SegTree {
    pub fn new(max_n : usize) -> Self {

      fn go(v : &mut Vec<Node>, k : usize, (l,r) : (usize, usize)) {
        v[k] = Node { interval : (l,r), ..v[k] };
        
        // is leaf
        if l == r {
          return;

        } else {

          // is node
          //  left child
          go(v, 2*k + 1, (l, (l+r)/2) );

          //  right child
          go(v, 2*k + 2, ((l+r)/2 + 1, r) );
        }
      }

      let mut initial_data = (0..4*max_n).map(|_| Node::zero()).collect::<Vec<Node>>();
      go(&mut initial_data, 0, (1, max_n) );

      Self {
        max_n : max_n,
        dat : initial_data
      }
    }
    
    pub fn update(mut self, i : usize, x : usize) -> Self {
      let mut current = self.max_n + i - 1;
      // update leaf
      self.dat[current].elm = x;

      // update as climbing
      while current > 0 {
        current = (current - 1) /2;
        self.dat[current].elm = cmp::min(self.dat[current * 2 + 1].elm, self.dat[current * 2 + 2].elm);
      }

      self
    }

    pub fn query(&self, (a,b) : (usize,usize)) -> usize {
      fn go(seg : &SegTree, (a,b) : (usize,usize), k : usize) -> usize {
        let (l,r) = seg.dat[k].interval;
        if r <= a || b <= l {
          usize::MAX
      
        } else if a <= l && r <= b {
          seg.dat[k].elm
      
        } else {
          let vl = go(seg, (a,b), k*2 +1);
          let vr = go(seg, (a,b), k*2 +2);
          cmp::min(vl, vr)
        }
      }

      go(&self, (a,b), 0)
    }

  }



  #[cfg(test)]
  mod segment_test {
    use super::*;
    use crate::SegmentTree::*;

    #[test]
    fn new_test1() {
      let a = SegTree::new(1);
      let b = vec![ Node::new(usize::MAX, (1,1)),
                    Node::zero(),
                    Node::zero(),
                    Node::zero() ];
      
      assert_eq!(a.dat, b);
    }

    #[test]
    fn new_test2() {
      let a = SegTree::new(4);
      let b = vec![ Node::new(usize::MAX, (1,4)),
                    Node::new(usize::MAX, (1,2)),
                    Node::new(usize::MAX, (3,4)),
                    Node::new(usize::MAX, (1,1)),
                    Node::new(usize::MAX, (2,2)),
                    Node::new(usize::MAX, (3,3)),
                    Node::new(usize::MAX, (4,4)),
                    
                    Node::zero(),
                    Node::zero(),
                    Node::zero(),
                    Node::zero(),
                    Node::zero(),
                    Node::zero(),
                    Node::zero(),
                    Node::zero(),
                    Node::zero() ];
      assert_eq!(a.dat, b);
    }

    #[test]
    fn update_test1() {
      let a = SegTree::new(4)
                        .update(0, 3)
                        .update(1, 1)
                        .update(2, 2)
                        .update(3, 4);
      let b = vec![ Node::new(1, (1,4)),
                    Node::new(1, (1,2)),
                    Node::new(2, (3,4)),
                    Node::new(3, (1,1)),
                    Node::new(1, (2,2)),
                    Node::new(2, (3,3)),
                    Node::new(4, (4,4)),
                
                    Node::zero(),
                    Node::zero(),
                    Node::zero(),
                    Node::zero(),
                    Node::zero(),
                    Node::zero(),
                    Node::zero(),
                    Node::zero(),
                    Node::zero() ];
    
      assert_eq!(a.dat, b);  
    }

    #[test]
    fn query_test1() {
      let a = SegTree::new(4)
                        .update(0, 3)
                        .update(1, 1)
                        .update(2, 2)
                        .update(3, 4);
      let a = a.query( (1,3) );
      assert_eq!(1, a);
    }


  }

}