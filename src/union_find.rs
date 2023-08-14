// Representing a forest of trees, numbered 0 .. count.  the id and
// parent attributes of the Tree struct refer to a tree number.
#[derive(Debug)]
pub struct Forest {
    pub count: usize,
    pub forest: Vec<Tree>,
}

#[derive(Debug)]
pub struct Tree {
    pub id: usize,
    pub parent: usize,
}

impl Forest {
    pub fn walk(&self, tree: usize) -> usize {
        let t = &self.forest[tree];
        assert!(t.id == tree);
        if t.id == t.parent {
            t.id
        } else {
            self.walk(t.parent)
        }
    }
    pub fn union(&mut self, t1: usize, t2: usize) {
        let root1 = self.walk(t1);
        let root2 = self.walk(t2);
        if root1 < root2 {
            self.forest[root2].parent = root1;
        } else {
            self.forest[root1].parent = root2;
        }
    }
    pub fn distinct_sets(&self) -> usize {
        let mut parents: Vec<usize> = self.forest.iter().map(|t| self.walk(t.id)).collect();
        parents.sort();
        parents.dedup();
        parents.len()
    }
}
