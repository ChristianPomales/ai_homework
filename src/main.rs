extern crate num;

use num::abs;

struct IndexTuple {
    row: usize,
    col: usize
}

#[derive(Clone)]
#[derive(Copy)]
struct EightPuzzle {
    puzzle: [[u8; 3]; 3],
}

impl EightPuzzle {
    fn print_puzzle(&self) {
        for row in self.puzzle.iter() {
            for col in row {
                if *col == 255 {
                    print!("\t []")
                } else {
                    print!("\t {}", col)
                }
            }

            print!("\n")
        }
    }
    
    fn move_up(self) -> EightPuzzle {
        let mut index = IndexTuple {
            row: 0,
            col: 0
        };
        
        for (row, val) in self.puzzle.iter().enumerate() {
            let retult = val.iter().position(|&x| x == 255);

            match retult {
                Some(col) => {
                    index.row = row;
                    index.col = col;
                },
                None => {},
            }
        }

        if index.row == 0 {
            return self.clone();
        }


        let mut new_puzzle = self.clone();

        new_puzzle.puzzle[index.row][index.col] = new_puzzle.puzzle[index.row - 1][index.col];
        new_puzzle.puzzle[index.row - 1][index.col] = 255;

        return new_puzzle
    }

    fn move_down(self) -> EightPuzzle {
        let mut index = IndexTuple {
            row: 0,
            col: 0
        };
        
        for (row, val) in self.puzzle.iter().enumerate() {
            let retult = val.iter().position(|&x| x == 255);

            match retult {
                Some(col) => {
                    index.row = row;
                    index.col = col;
                },
                None => {},
            }
        }

        if index.row == (self.puzzle.len() - 1) {
            return self.clone();
        }

        let mut new_puzzle = self.clone();

        new_puzzle.puzzle[index.row][index.col] = new_puzzle.puzzle[index.row + 1][index.col];
        new_puzzle.puzzle[index.row + 1][index.col] = 255;

        return new_puzzle
    }

    fn move_right(self) -> EightPuzzle {
        let mut index = IndexTuple {
            row: 0,
            col: 0
        };
        
        for (row, val) in self.puzzle.iter().enumerate() {
            let retult = val.iter().position(|&x| x == 255);

            match retult {
                Some(col) => {
                    index.row = row;
                    index.col = col;
                },
                None => {},
            }
        }

        if index.col == (self.puzzle[0].len() - 1) {
            return self.clone();
        }

        let mut new_puzzle = self.clone();

        new_puzzle.puzzle[index.row][index.col] = new_puzzle.puzzle[index.row][index.col + 1];
        new_puzzle.puzzle[index.row][index.col + 1] = 255;

        return new_puzzle
    }

    fn move_left(self) -> EightPuzzle {
        let mut index = IndexTuple {
            row: 0,
            col: 0
        };
        
        for (row, val) in self.puzzle.iter().enumerate() {
            let retult = val.iter().position(|&x| x == 255);

            match retult {
                Some(col) => {
                    index.row = row;
                    index.col = col;
                },
                None => {},
            }
        }

        if index.col == 0 {
            return self.clone();
        }

        let mut new_puzzle = self.clone();

        new_puzzle.puzzle[index.row][index.col] = new_puzzle.puzzle[index.row][index.col - 1];
        new_puzzle.puzzle[index.row][index.col - 1] = 255;

        return new_puzzle
    }
}

struct Problem {
    initial_state: EightPuzzle,
    goal_state: EightPuzzle
}

// going to want to have quque store a tuple of EightPuzzles and costs
fn search(problem: Problem) -> Option<EightPuzzle> {
    let mut queue: Vec<EightPuzzle> = vec![problem.initial_state];

    loop {
        if queue.is_empty() {
            return None;
        }

        let node = queue.pop().unwrap();

        if node.puzzle == problem.goal_state.puzzle {
            return Some(node);
        }

        enqueueing_function(&mut queue, node);

        return Some(problem.goal_state)
    }
}

fn enqueueing_function(nodes: &mut Vec<EightPuzzle>, node: EightPuzzle) {
    let new_nodes = vec![
        node.move_up(), node.move_down(),
        node.move_left(), node.move_right()
    ];

    for element in new_nodes {
        if element.puzzle != node.puzzle {
            nodes.push(element);
        }
    }
}

fn uniform_search_heuristic(puzzle: EightPuzzle, goal: EightPuzzle) -> i64 {
    return 0
}

fn missplaced_tile_heuristic(puzzle: EightPuzzle, goal: EightPuzzle) -> i64 {
    let mut count = 0;

    let row_count = puzzle.puzzle.len();
    let col_count = puzzle.puzzle[0].len();

    for i in 0..row_count {
        for j in 0..col_count {
            if puzzle.puzzle[i][j] != goal.puzzle[i][j] {
                count += 1;
            }
        }
    }

    return count
}

fn manhattan_distance_heuristic(puzzle: EightPuzzle, _goal: EightPuzzle) -> i64 {
    let mut total_distance = 0;

    let row_count = puzzle.puzzle.len();
    let col_count = puzzle.puzzle[0].len();

    for i in 0..row_count {
        for j in 0..col_count {
            let number = puzzle.puzzle[i][j];

            let x_value = j as i64;
            let x_goal = ((number - 1) % 3) as i64;

            let y_value = i as i64;
            let y_goal = ((number as f64 - 1.0) /  3.0).floor() as i64;

            total_distance += abs(x_value - x_goal) + abs(y_value - y_goal);
        }
    }

    let x_value = 1;
    let x_goal = 0; // x position it should be (number - 1) mod  3

    let y_value = 2;
    let y_goal = 0; // floor((number - 1) /  3)

    let distance = abs(x_value - x_goal) + abs(y_value - y_goal);;

    return distance
}

fn main() {
    let initial_state = EightPuzzle {
        puzzle: [[1, 2, 3], [4, 255, 6], [7, 5, 8]],
    };

    let goal_state = EightPuzzle {
        puzzle: [[1, 2, 3], [4, 5, 6], [7, 8, 255]],
    };

    let problem = Problem {
        initial_state: initial_state,
        goal_state: goal_state
    };

    println!("Initial State:");

    &problem.initial_state.print_puzzle();

    let answer = search(problem);

    println!("Answer:");
    match answer {
        Some(puzzle) => puzzle.print_puzzle(),
        None => println!("no answer :(")
    }
}
