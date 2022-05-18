use std::collections::VecDeque;

struct Hanoi {
    num_discs: i32,
    tower_a: VecDeque<i32>,
    tower_b: VecDeque<i32>,
    tower_c: VecDeque<i32>,
}

impl Hanoi {
    pub fn new(discs: i32) -> Hanoi {
        let ds = (1..=discs).collect::<Vec<i32>>();
        Hanoi {
            num_discs: discs,
            tower_a: VecDeque::from(ds),
            tower_b: VecDeque::new(),
            tower_c: VecDeque::new(),
        }
    }

    fn move_discs(
        begin: &mut VecDeque<i32>,
        end: &mut VecDeque<i32>,
        temp: &mut VecDeque<i32>,
        n: i32,
    ) {
        if n == 1 {
            end.push_back(begin.pop_back().expect("disc of begin is available"));
        } else {
            Self::move_discs(begin, temp, end, n - 1);
            Self::move_discs(begin, end, temp, 1);
            Self::move_discs(temp, end, begin, n - 1);
        }
    }

    fn solve(&mut self) {
        Self::move_discs(
            &mut self.tower_a,
            &mut self.tower_c,
            &mut self.tower_b,
            self.num_discs,
        );
    }
}

fn main() {
    let mut hanoi = Hanoi::new(10);
    hanoi.solve();
    println!("{:?}", hanoi.tower_a);
    println!("{:?}", hanoi.tower_b);
    println!("{:?}", hanoi.tower_c);
}
