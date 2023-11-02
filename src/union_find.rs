// Representing a forest of trees, numbered 0 .. count.  the id and
// parent attributes of the Tree struct refer to a tree number.
#[derive(Debug)]
pub struct Forest {
    pub parents: Vec<usize>,
}

impl Forest {
    pub fn new() -> Forest {
        Forest { parents: vec![] }
    }
    pub fn push(&mut self, new: usize) {
        assert!(new == self.parents.len());
        self.parents.push(new);
    }
    pub fn find(&mut self, i: usize) -> usize {
        if self.parents[i] == i {
            i
        } else {
            let found = self.find(self.parents[i]);
            self.parents[i] = found;
            found
        }
    }
    pub fn union(&mut self, t1: usize, t2: usize) {
        let root1 = self.find(t1);
        let root2 = self.find(t2);
        if root2 < root1 {
            self.parents[root1] = root2;
        } else {
            self.parents[root2] = root1;
        }
    }
    pub fn distinct_sets(&mut self) -> usize {
        let mut parents: Vec<usize> = self.parents.clone().iter().map(|p| self.find(*p)).collect();
        parents.sort();
        parents.dedup();
        parents.len()
    }
}
