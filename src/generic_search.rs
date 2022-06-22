use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    fmt::Debug,
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
    State: Clone + Hash + Eq + Debug,
{
    fn initial(&self) -> State;
    fn is_goal(&self, s: &State) -> bool;
    fn successors(&self, s: &State) -> Vec<State>;

    fn dfs(&self) -> Option<Vec<State>> {
        let mut history: Vec<State> = Vec::new();
        let mut stack: VecDeque<State> = VecDeque::new();
        stack.push_back(self.initial());
        let mut visited: HashSet<State> = HashSet::new();
        visited.insert(self.initial());

        while let Some(current_state) = stack.pop_back() {
            history.push(current_state.clone());

            if self.is_goal(&current_state) {
                return Some(history);
            }

            for successor in self.successors(&current_state).into_iter() {
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

// pub struct Node<T>
// where
//     T: Clone + Hash + Eq,
// {
//     state: T,
//     cost: f64,
//     heuristic: f64,
// }

// impl<T> Node<T>
// where
//     T: Clone + Hash + Eq,
// {
//     fn new(state: T) -> Self {
//         Self {
//             state,
//             cost: 0.0,
//             heuristic: 0.0,
//         }
//     }

//     fn new_with_cost(state: T, cost: f64, heuristic: f64) -> Self {
//         Self {
//             state,
//             cost,
//             heuristic,
//         }
//     }
// }

// impl<T> PartialEq for Node<T>
// where
//     T: Clone + Hash + Eq,
// {
//     fn eq(&self, other: &Self) -> bool {
//         let mine = self.cost + self.heuristic;
//         let theirs = other.cost + other.heuristic;
//         mine.eq(&theirs)
//     }
// }

// impl<T> PartialOrd for Node<T>
// where
//     T: Clone + Hash + Eq,
// {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         let mine = self.cost + self.heuristic;
//         let theirs = other.cost + other.heuristic;
//         mine.partial_cmp(&theirs)
//     }
// }

// pub fn dfs<T>(initial: T, goal_test: fn(&T) -> bool, successors: fn(&T) -> Vec<T>) -> Option<Vec<T>>
// where
//     T: Clone + Hash + Eq,
// {
//     let mut visited: HashSet<T> = HashSet::new();
//     let mut history: Vec<T> = Vec::new();
//     let mut stack: VecDeque<T> = VecDeque::new();
//     stack.push_back(initial);

//     while let Some(current_state) = stack.pop_back() {
//         history.push(current_state.clone());

//         if goal_test(&current_state) {
//             return Some(history);
//         }

//         for succ in successors(&current_state).into_iter() {
//             if visited.insert(succ.clone()) {
//                 stack.push_back(succ);
//             }
//         }
//     }

//     None
// }
