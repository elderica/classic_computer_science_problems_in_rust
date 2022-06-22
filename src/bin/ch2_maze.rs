use classic_computer_science_problems_in_rust::generic_search::Searchable;
use rand::prelude::*;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Blocked,
    Start,
    Goal,
    Path,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, " "),
            Self::Blocked => write!(f, "X"),
            Self::Start => write!(f, "S"),
            Self::Goal => write!(f, "G"),
            Self::Path => write!(f, "*"),
        }
    }
}

struct Maze {
    rows: isize,
    columns: isize,
    start: MazeLocation,
    goal: MazeLocation,
    grid: Vec<Vec<Cell>>,
}

impl Maze {
    fn new() -> Self {
        Self::generate(
            10,
            10,
            MazeLocation::new(0, 0),
            MazeLocation::new(9, 9),
            0.2,
        )
    }

    fn generate(
        rows: usize,
        columns: usize,
        start: MazeLocation,
        goal: MazeLocation,
        sparseness: f64,
    ) -> Self {
        let mut grid = vec![vec![Cell::Empty; columns]; rows];
        randomly_fill(&mut grid, sparseness);

        grid[start.row as usize][start.column as usize] = Cell::Start;
        grid[goal.row as usize][goal.column as usize] = Cell::Goal;

        Self {
            rows: rows as isize,
            columns: columns as isize,
            start,
            goal,
            grid,
        }
    }

    fn mark(&mut self, path: &Vec<MazeLocation>) {
        for ml in path {
            self.grid[ml.row as usize][ml.column as usize] = Cell::Path;
        }
        self.grid[self.start.row as usize][self.start.column as usize] = Cell::Start;
        self.grid[self.goal.row as usize][self.goal.column as usize] = Cell::Goal;
    }

    fn clear(&mut self, path: &Vec<MazeLocation>) {
        for ml in path {
            self.grid[ml.row as usize][ml.column as usize] = Cell::Empty;
        }
        self.grid[self.start.row as usize][self.start.column as usize] = Cell::Start;
        self.grid[self.goal.row as usize][self.goal.column as usize] = Cell::Goal;
    }

    // fn euclidean_distance(&self, ml: &MazeLocation) -> f64 {
    //     let xdist = ml.column - self.goal.column;
    //     let ydist = ml.row - self.goal.row;
    //     ((xdist * xdist + ydist * ydist) as f64).sqrt()
    // }

    // fn manhattan_distance(&self, ml: &MazeLocation) -> f64 {
    //     let xdist = ml.column - self.goal.column;
    //     let ydist = ml.row - self.goal.row;
    //     (xdist + ydist) as f64
    // }
}

fn randomly_fill(grid: &mut [Vec<Cell>], sparseness: f64) {
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            if random::<f64>() < sparseness {
                *cell = Cell::Blocked;
            }
        }
    }
}

impl Searchable<MazeLocation> for Maze {
    fn initial(&self) -> MazeLocation {
        self.start.clone()
    }

    fn is_goal(&self, ml: &MazeLocation) -> bool {
        self.goal == *ml
    }

    fn successors(&self, ml: &MazeLocation) -> Vec<MazeLocation> {
        let mut locations = Vec::new();
        let grid = &self.grid;
        let rows = self.rows;
        let columns = self.columns;

        if ml.row + 1 < rows && grid[(ml.row + 1) as usize][ml.column as usize] != Cell::Blocked {
            locations.push(MazeLocation::new(ml.row + 1, ml.column));
        }
        if ml.row > 0 && grid[(ml.row - 1) as usize][ml.column as usize] != Cell::Blocked {
            locations.push(MazeLocation::new(ml.row - 1, ml.column));
        }
        if ml.column + 1 < columns
            && grid[ml.row as usize][(ml.column + 1) as usize] != Cell::Blocked
        {
            locations.push(MazeLocation::new(ml.row, ml.column + 1));
        }
        if ml.column > 0 && grid[ml.row as usize][(ml.column - 1) as usize] != Cell::Blocked {
            locations.push(MazeLocation::new(ml.row, ml.column - 1));
        }

        locations
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid = &self.grid;
        for row in grid {
            let s: String = row.iter().map(Cell::to_string).collect();
            writeln!(f, "{}", s)?;
        }

        Ok(())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct MazeLocation {
    row: isize,
    column: isize,
}

impl MazeLocation {
    fn new(row: isize, column: isize) -> Self {
        Self { row, column }
    }
}

fn main() {
    println!("====================================================");
    let mut m1 = Maze::new();
    println!("{}", m1);
    println!("----------------------------------------------------");
    if let Some(solution1) = m1.dfs() {
        m1.mark(&solution1);
        println!("{}", m1);
        m1.clear(&solution1);
    } else {
        println!("No solution found using depth-first search!")
    }
    println!("====================================================");
    let mut m2 = Maze::new();
    println!("{}", m2);
    println!("----------------------------------------------------");
    if let Some(solution2) = m2.bfs() {
        m2.mark(&solution2);
        println!("{}", m2);
        m2.clear(&solution2);
    } else {
        println!("No solution found using breadth-first search!")
    }
    println!("====================================================");
}
