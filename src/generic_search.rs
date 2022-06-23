use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    hash::Hash,
};

pub fn linear_contains<T>(v: &[T], key: &T) -> bool
where
    T: Ord,
{
    v.iter().any(|e| e.cmp(key) == Ordering::Equal)
}

pub fn binary_contains<T>(v: &[T], key: &T) -> bool
where
    T: Ord + std::clone::Clone,
{
    let mut s = v.to_vec();
    s.sort_unstable();
    s.binary_search(key).is_ok()
}

pub trait Searchable<State>
where
    State: Clone + Hash + Eq,
{
    fn initial(&self) -> State;
    fn is_goal(&self, s: &State) -> bool;
    fn successors(&self, s: &State) -> Vec<State>;

    fn dfs(&self) -> Option<Vec<State>> {
        let mut history: Vec<State> = Vec::new();
        let mut stack: VecDeque<State> = VecDeque::new();
        stack.push_back(self.initial());
        let mut visited: HashSet<State> = HashSet::new();

        while let Some(current_state) = stack.pop_back() {
            history.push(current_state.clone());

            if self.is_goal(&current_state) {
                return Some(history);
            }

            for successor in self.successors(&current_state).into_iter().rev() {
                if visited.insert(successor.clone()) {
                    stack.push_back(successor);
                }
            }
        }

        None
    }

    fn bfs(&self) -> Option<Vec<State>> {
        let mut history: Vec<State> = Vec::new();
        let mut queue: VecDeque<State> = VecDeque::new();
        queue.push_back(self.initial());
        let mut visited: HashSet<State> = HashSet::new();
        visited.insert(self.initial());

        while let Some(current_state) = queue.pop_front() {
            history.push(current_state.clone());

            if self.is_goal(&current_state) {
                return Some(history);
            }

            for successor in self.successors(&current_state).into_iter() {
                if visited.insert(successor.clone()) {
                    queue.push_back(successor);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Tree {
        parent_of: Vec<usize>,
        child_of: Vec<Vec<usize>>,
        goal: usize,
    }

    impl Searchable<usize> for Tree {
        fn initial(&self) -> usize {
            0
        }

        fn is_goal(&self, s: &usize) -> bool {
            *s == self.goal
        }

        fn successors(&self, s: &usize) -> Vec<usize> {
            self.child_of[*s].clone()
        }
    }
    #[test]
    fn tree1_dfs() {
        //       0
        //   1       2
        // 3   4      5
        //6 7   8   9  10
        let t = Tree {
            parent_of: vec![0, 0, 0, 1, 1, 2, 3, 3, 4, 5, 5],
            child_of: vec![
                vec![1, 2],
                vec![3, 4],
                vec![5],
                vec![6, 7],
                vec![8],
                vec![9, 10],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            goal: 10,
        };

        let h = t.dfs();
        assert_eq!(h, Some(vec![0, 1, 3, 6, 7, 4, 8, 2, 5, 9, 10]));
    }

    #[test]
    fn tree1_bfs() {
        //       0
        //   1       2
        // 3   4      5
        //6 7   8   9  10
        let t = Tree {
            parent_of: vec![0, 0, 0, 1, 1, 2, 3, 3, 4, 5, 5],
            child_of: vec![
                vec![1, 2],
                vec![3, 4],
                vec![5],
                vec![6, 7],
                vec![8],
                vec![9, 10],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            goal: 10,
        };
        let h = t.bfs();
        assert_eq!(h, Some(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    }

    #[test]
    fn tree2_dfs() {
        let t = Tree {
            parent_of: vec![0, 0, 0, 1, 1, 2, 2],
            child_of: vec![
                vec![1, 2],
                vec![3, 4],
                vec![5, 6],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            goal: 6,
        };
        let h = t.dfs();
        assert_eq!(h, Some(vec![0, 1, 3, 4, 2, 5, 6]))
    }
}
