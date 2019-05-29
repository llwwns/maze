pub struct UnionFind {
  parent: Vec<usize>,
  rank: Vec<usize>,
}

impl UnionFind {
  pub fn new(n: usize) -> Self {
    UnionFind {
      parent: (0..n).into_iter().collect(),
      rank: std::iter::repeat(0).take(n).collect(),
    }
  }

  fn find_root(&mut self, x: usize) -> usize {
    if self.parent[x] == x {
      return x;
    }
    self.parent[x] = self.find_root(self.parent[x]);
    self.parent[x]
  }

  pub fn unite(&mut self, mut x: usize, mut y: usize) {
    x = self.find_root(x);
    y = self.find_root(y);
    if self.rank[x] < self.rank[y] {
      self.parent[x] = y;
    } else {
      self.parent[y] = x;
      if self.rank[x] == self.rank[y] {
        self.rank[x] += 1;
      }
    }
  }

  pub fn is_same(&mut self, x: usize, y: usize) -> bool {
    self.find_root(x) == self.find_root(y)
  }
}

#[test]
fn test() {
  let mut x = UnionFind::new(10);
  x.unite(1, 5);
  x.unite(2, 5);
  assert_eq!(x.is_same(1, 2), true);
  assert_eq!(x.is_same(1, 3), false);
}
