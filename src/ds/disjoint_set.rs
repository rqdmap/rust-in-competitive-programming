pub struct DisjointSet {
    par: Vec<usize>
}

impl DisjointSet {
    /// `DisjointSet::new(n)`:
    ///     Create a new disjoint set with n + 1 elements(0-n).
    pub fn new(n: usize) -> Self {
        let mut par = vec![0; n + 1];
        for i in 0..=n {
            par[i] = i;
        }
        return DisjointSet { par }
    }

    fn query(&mut self, n: usize) -> usize {
        if self.par[n] == n { return n; } 
        self.par[n] = self.query(self.par[n]);
        return self.par[n];
    }

    /// `merge(x, y)`: 
    ///     Merge the set that contains x and the set that contains y.
    pub fn merge(&mut self, x: usize, y: usize) {
        if self.query(x) != self.query(y) {
            let rt = self.par[x];
            self.par[rt] = self.par[y];
        }
    }

    /// `same(x, y)`:
    ///    Return true if x and y are in the same set.
    pub fn same(&mut self , x: usize, y: usize) -> bool {
        return self.query(x) == self.query(y);
    }

}
