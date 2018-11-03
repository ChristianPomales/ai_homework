extern crate num;

use num::abs;
use std::io;
use std::io::prelude::*;

static mut TOTAL_EXPANSIONS: u64 = 0;
static mut MAXIMUM_NODES: usize = 0;
static mut PRINT_EXPANDING_NODES_TOGGLE: bool = true;

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
    g: u64,
    h: u64,
    prev_move: Moves
}

fn search(problem: Problem, heuristic: fn(EightPuzzle, EightPuzzle) -> u64) -> Option<EightPuzzle> {
    let root = EightPuzzleNode {
        puzzle: problem.initial_state,
        g: 0,
        h: heuristic(problem.initial_state, problem.goal_state),
        prev_move: Moves::Nothing
    };

    let mut queue: Vec<EightPuzzleNode> = vec![root];

    loop {
        unsafe {
            let queue_length = queue.len();
            if queue_length > MAXIMUM_NODES as usize {
                MAXIMUM_NODES = queue_length;
            }
        }

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
    node: (EightPuzzleNode, usize), heuristic: fn(EightPuzzle, EightPuzzle) -> u64) {
    let mut new_nodes = expand_node(node.0, goal, heuristic);
    new_nodes.reverse();
    
    for element in new_nodes.iter() {
        nodes.insert(node.1, element.clone());
    }
}

fn expand_node(node: EightPuzzleNode, goal: EightPuzzle, heuristic: fn(EightPuzzle, EightPuzzle) -> u64) -> Vec<EightPuzzleNode> {    
    unsafe {
        TOTAL_EXPANSIONS += 1;

        if PRINT_EXPANDING_NODES_TOGGLE {
            println!("Expanding the following state with g(n) = {} and h(n) = {}", node.g, node.h);
            node.puzzle.print_puzzle();
            println!();
        }
    }

    let mut new_nodes: Vec<EightPuzzleNode> = Vec::new();

    match node.prev_move {
        Moves::Up => {
            let up = EightPuzzleNode {
                puzzle: node.puzzle.move_up(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_up(), goal),
                prev_move: Moves::Up
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
        Moves::Down => {
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
        Moves::Left => {
            let up = EightPuzzleNode {
                puzzle: node.puzzle.move_up(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_up(), goal),
                prev_move: Moves::Up
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
        Moves::Right => {
            let up = EightPuzzleNode {
                puzzle: node.puzzle.move_up(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_up(), goal),
                prev_move: Moves::Up
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
        Moves::Nothing => {
            let up = EightPuzzleNode {
                puzzle: node.puzzle.move_up(),
                g: node.g + 1,
                h: heuristic(node.puzzle.move_up(), goal),
                prev_move: Moves::Up
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
    let mut lowest_cost = (0, <u64>::max_value()); // index, cost

    for (index, element) in nodes.iter().enumerate() {
        let cost = element.g + element.h;

        if cost < lowest_cost.1 {
            lowest_cost = (index, cost);
        }
    }
    let return_node = nodes.remove(lowest_cost.0);

    return (return_node, lowest_cost.0);
}

fn uniform_search_heuristic(_puzzle: EightPuzzle, _goal: EightPuzzle) -> u64 {
    return 0
}

fn misplaced_tile_heuristic(puzzle: EightPuzzle, goal: EightPuzzle) -> u64 {
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

fn manhattan_distance_heuristic(puzzle: EightPuzzle, _goal: EightPuzzle) -> u64 {
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

    return total_distance as u64
}

fn build_eightpuzzle_from_input(stdin: & std::io::Stdin) -> EightPuzzle {
    println!("Enter your puzzle, use a zero to represent the blank");

    println!("Enter the first row, use space or tabs between numbers: ");
    let mut first_row = String::new();;
    stdin.lock().read_line(&mut first_row).unwrap();

    println!("Enter the second row, use space or tabs between numbers: ");
    let mut second_row = String::new();;
    stdin.lock().read_line(&mut second_row).unwrap();

    println!("Enter the third row, use space or tabs between numbers: ");
    let mut third_row = String::new();;
    stdin.lock().read_line(&mut third_row).unwrap();

    let first_row_vec: Vec<&str> = first_row.split_whitespace().collect();
    let second_row_vec: Vec<&str> = second_row.split_whitespace().collect();
    let third_row_vec: Vec<&str> = third_row.split_whitespace().collect();

    if (first_row_vec.len() != 3) || (second_row_vec.len() != 3) || (third_row_vec.len() != 3) {
        panic!("Woops! length is {}, please try again :(", first_row_vec.len());
    }

    let first_row_array = [first_row_vec[0].parse::<u8>().unwrap(),
                            first_row_vec[1].parse::<u8>().unwrap(),
                            first_row_vec[2].parse::<u8>().unwrap()] ;

    let second_row_array = [second_row_vec[0].parse::<u8>().unwrap(),
                            second_row_vec[1].parse::<u8>().unwrap(),
                            second_row_vec[2].parse::<u8>().unwrap()] ;

    let third_row_array = [third_row_vec[0].parse::<u8>().unwrap(),
                            third_row_vec[1].parse::<u8>().unwrap(),
                            third_row_vec[2].parse::<u8>().unwrap()] ;

    let mut abcd = EightPuzzle {
        puzzle: [first_row_array, second_row_array, third_row_array],
    };

    for row in 0..3 {
        for col in 0..3 {
            if abcd.puzzle[row][col] == 0 {
                abcd.puzzle[row][col] = 255;
            }
        }
    }

    return abcd
}

fn select_prebuild_eightpuzzle(stdin: & std::io::Stdin) -> EightPuzzle {
    let trivial = EightPuzzle {
        puzzle: [[1, 2, 3], [4, 5, 6], [7, 8, 255]],
    };

    let very_easy = EightPuzzle {
        puzzle: [[1, 2, 3], [4, 5, 6], [7, 255, 8]],
    };

    let easy = EightPuzzle {
        puzzle: [[1, 2, 255], [4, 5, 3], [7, 8, 6]],
    };


    let doable = EightPuzzle {
        puzzle: [[255, 1, 2], [4, 5, 3], [7, 8, 6]],
    };

    let oh_boy = EightPuzzle {
        puzzle: [[8, 7, 1],[6, 255, 2],[5, 4, 3]],
    };

    let impossible = EightPuzzle {
        puzzle: [[1, 2, 3], [4, 5, 6], [8, 7, 255]]
    };

    println!("You wish to use a default puzzle. Please enter a desired difficulty on a scale from 0 to 5.");
    let mut difficulty_input = String::new();;
    stdin.lock().read_line(&mut difficulty_input).unwrap();

    let difficulty_number = difficulty_input.trim().parse::<u8>().unwrap();

    match difficulty_number {
        0 => {
            println!("Difficulty of 'Trivial' selected.");
            return trivial
        },
        1 => {
            println!("Difficulty of 'Very Easy' selected.");
            return very_easy
        },
        2 => {
            println!("Difficulty of 'Easy' selected.");
            return easy
        },
        3 => {
            println!("Difficulty of 'Doable' selected.");
            return doable
        },
        4 => {
            println!("Difficulty of 'Oh Boy' selected.");
            return oh_boy
        },
        5 => {
            println!("Difficulty of 'impossible' selected.");
            return impossible
        },
        _ => {
            panic!("Woops! invalid difficulty! Try again :(");
        },
    }
}

fn main() {
    let stdin = io::stdin();

    println!("Welcome to Christian Pomales's 8-puzzle solver");
    println!("");
    println!("Type \"1\" to use the default puzzle, or \"2\" to enter your own puzzle");

    let mut puzzle_option = String::new();;
    stdin.lock().read_line(&mut puzzle_option).unwrap();

    let goal_state = EightPuzzle {
        puzzle: [[1, 2, 3], [4, 5, 6], [7, 8, 255]],
    };

    let initial_state: EightPuzzle;
    let problem: Problem;
    match puzzle_option.as_str() {
        "1\n" => {
            initial_state = select_prebuild_eightpuzzle(&stdin);

            problem = Problem {
                initial_state: initial_state,
                goal_state: goal_state
            };
            
        },
        "2\n" => {
            initial_state = build_eightpuzzle_from_input(&stdin);

            problem = Problem {
                initial_state: initial_state,
                goal_state: goal_state
            };
        },
        _ => {
            println!("invalid algorithm choice!");
            return();
        }
    }

    println!("Initial State:");
    &problem.initial_state.print_puzzle();
    println!("");

    println!("Enter your choice of algorithm");
    println!("1. \t Uniform Cost Search");
    println!("2. \t A* with the Misplaced Tile heuristic.");
    println!("3. \t A* with the Manhattan distance heuristic.");
    println!("");

    let mut heuristic_input = String::new();;
    stdin.lock().read_line(&mut heuristic_input).unwrap();

    let heuristic: fn(EightPuzzle, EightPuzzle) -> u64;
    match heuristic_input.as_str() {
        "1\n" => {
            heuristic = uniform_search_heuristic
        },
        "2\n" => {
            heuristic = misplaced_tile_heuristic
        },
        "3\n" => {
            heuristic = manhattan_distance_heuristic
        },
        _ => {
            println!("invalid algorithm choice!");
            return();
        }
    }

    println!("Type \"1\" to print each node expansion, or \"2\" to not");
    let mut print_expanding_nodes_toggle = String::new();
    stdin.lock().read_line(&mut print_expanding_nodes_toggle).unwrap();

    unsafe {
        match print_expanding_nodes_toggle.trim().parse::<u8>().unwrap() {
            1 => {
                PRINT_EXPANDING_NODES_TOGGLE = true
            },
            2 => {
                PRINT_EXPANDING_NODES_TOGGLE = false
            },
            _ => {
                println!("invalid choice!");
                return();
            }
        }
    }

    let answer = search(problem, heuristic);

    println!("Goal!!");
    match answer {
        Some(puzzle) => puzzle.print_puzzle(),
        None => println!("no answer :(")
    }
    
    unsafe {
        println!("");
        println!("To solve this problem the search algorithm expansed a total of {} nodes", TOTAL_EXPANSIONS);
        println!("The maximum number of nodes in the queue at any one time was {}", MAXIMUM_NODES);
    }
}