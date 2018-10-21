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
                    print!("\t *")
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

#[derive(Clone)]
enum Moves {
    Up,
    Down,
    Left,
    Right,
    Nothing
}

#[derive(Clone)]
struct EightPuzzleNode {
    puzzle: EightPuzzle,
    g: i64,
    h: i64,
    prev_move: Moves
}

// going to want to have queue store a tuple of EightPuzzles and costs
fn search(problem: Problem, heuristic: fn(EightPuzzle, EightPuzzle) -> i64) -> Option<EightPuzzle> {
    let root = EightPuzzleNode {
        puzzle: problem.initial_state,
        g: 0,
        h: heuristic(problem.initial_state, problem.goal_state),
        prev_move: Moves::Nothing
    };

    let mut queue: Vec<EightPuzzleNode> = vec![root];

    loop {
        if queue.is_empty() {
            return None;
        }

        let node = dequeueing_function(&mut queue);

        if node.0.puzzle.puzzle == problem.goal_state.puzzle {
            return Some(node.0.puzzle);
        }

        enqueueing_function(&mut queue, problem.goal_state, node, heuristic);
    }
}


fn enqueueing_function(nodes: &mut Vec<EightPuzzleNode>, goal: EightPuzzle,
    node: (EightPuzzleNode, usize), heuristic: fn(EightPuzzle, EightPuzzle) -> i64) {
    let mut new_nodes = expand_node(node.0, goal, heuristic);
    new_nodes.reverse();
    
    for element in new_nodes.iter() {
        nodes.insert(node.1, element.clone());
    }
}

fn expand_node(node: EightPuzzleNode, goal: EightPuzzle, heuristic: fn(EightPuzzle, EightPuzzle) -> i64) -> Vec<EightPuzzleNode> {    
    println!("Expanding the following state with g(n) = {} and h(n) = {}", node.g, node.h);
    node.puzzle.print_puzzle();
    println!();

    let mut new_nodes: Vec<EightPuzzleNode> = Vec::new();

    match node.prev_move {
        Moves::Up => {
            let down = EightPuzzleNode {
                puzzle: node.puzzle.move_down(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_down(), goal),
                prev_move: Moves::Down
            };
            let left = EightPuzzleNode {
                puzzle: node.puzzle.move_left(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_left(), goal),
                prev_move: Moves::Left
            };
            let right = EightPuzzleNode {
                puzzle: node.puzzle.move_right(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_right(), goal),
                prev_move: Moves::Right
            };

            let created_nodes = vec![down, left, right];

            for element in created_nodes {
                if element.puzzle.puzzle != node.puzzle.puzzle {
                    new_nodes.push(element);
                }
            }
        },
        Moves::Down => {
            let up = EightPuzzleNode {
                puzzle: node.puzzle.move_up(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_up(), goal),
                prev_move: Moves::Down
            };
            let left = EightPuzzleNode {
                puzzle: node.puzzle.move_left(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_left(), goal),
                prev_move: Moves::Left
            };
            let right = EightPuzzleNode {
                puzzle: node.puzzle.move_right(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_right(), goal),
                prev_move: Moves::Right
            };

            let created_nodes = vec![up, left, right];

            for element in created_nodes {
                if element.puzzle.puzzle != node.puzzle.puzzle {
                    new_nodes.push(element);
                }
            }
        },
        Moves::Left => {
            let up = EightPuzzleNode {
                puzzle: node.puzzle.move_up(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_up(), goal),
                prev_move: Moves::Down
            };
            let down = EightPuzzleNode {
                puzzle: node.puzzle.move_down(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_down(), goal),
                prev_move: Moves::Down
            };
            let right = EightPuzzleNode {
                puzzle: node.puzzle.move_right(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_right(), goal),
                prev_move: Moves::Right
            };

            let created_nodes = vec![up, down, right];

            for element in created_nodes {
                if element.puzzle.puzzle != node.puzzle.puzzle {
                    new_nodes.push(element);
                }
            }
        },
        Moves::Right => {
            let up = EightPuzzleNode {
                puzzle: node.puzzle.move_up(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_up(), goal),
                prev_move: Moves::Down
            };
            let down = EightPuzzleNode {
                puzzle: node.puzzle.move_down(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_down(), goal),
                prev_move: Moves::Down
            };
            let left = EightPuzzleNode {
                puzzle: node.puzzle.move_left(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_left(), goal),
                prev_move: Moves::Left
            };

            let created_nodes = vec![up, down, left];

            for element in created_nodes {
                if element.puzzle.puzzle != node.puzzle.puzzle {
                    new_nodes.push(element);
                }
            }
        },
        Moves::Nothing => {
            let up = EightPuzzleNode {
                puzzle: node.puzzle.move_up(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_up(), goal),
                prev_move: Moves::Down
            };
            let down = EightPuzzleNode {
                puzzle: node.puzzle.move_down(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_down(), goal),
                prev_move: Moves::Down
            };
            let left = EightPuzzleNode {
                puzzle: node.puzzle.move_left(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_left(), goal),
                prev_move: Moves::Left
            };
            let right = EightPuzzleNode {
                puzzle: node.puzzle.move_right(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_right(), goal),
                prev_move: Moves::Right
            };

            let created_nodes = vec![up, down, left, right];

            for element in created_nodes {
                if element.puzzle.puzzle != node.puzzle.puzzle {
                    new_nodes.push(element);
                }
            }
        },
    }

    return new_nodes
}

fn dequeueing_function(nodes: &mut Vec<EightPuzzleNode>) -> (EightPuzzleNode, usize) {
    let mut lowest_cost = (0, <i64>::max_value()); // index, cost

    for (index, element) in nodes.iter().enumerate() {
        let cost = element.g + element.h;

        if cost < lowest_cost.1 {
            lowest_cost = (index, cost);
        }
    }
    let return_node = nodes.remove(lowest_cost.0);

    return (return_node, lowest_cost.0);
}

fn uniform_search_heuristic(_puzzle: EightPuzzle, _goal: EightPuzzle) -> i64 {
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

            if number == 255 {
                continue;
            }

            let x_value = j as i64;
            let x_goal = ((number - 1) % 3) as i64;

            let y_value = i as i64;
            let y_goal = ((number as f64 - 1.0) /  3.0).floor() as i64;

            total_distance += abs(x_value - x_goal) + abs(y_value - y_goal);
        }
    }

    // let x_value = 1;
    // let x_goal = 0; // x position it should be (number - 1) mod  3

    // let y_value = 2;
    // let y_goal = 0; // floor((number - 1) /  3)

    // let distance = abs(x_value - x_goal) + abs(y_value - y_goal);;

    return total_distance
}

fn main() {
    let initial_state = EightPuzzle {
        puzzle: [[1, 2, 3], [4, 8, 255], [7, 6, 5]],
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

    println!("///////////////////////////////////");

    let answer = search(problem, manhattan_distance_heuristic);

    println!("///////////////////////////////////");

    println!("Answer:");
    match answer {
        Some(puzzle) => puzzle.print_puzzle(),
        None => println!("no answer :(")
    }
}