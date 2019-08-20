#![allow(unused)]

use crate::parser::{Parser, UtepParser};
use crate::solver::{SimpleSolver, Solver};

mod stack;
mod bi_map;
mod parser;
mod technique;
mod solver;

pub const DEBUG: bool = true;

fn main() {
    let input_str = r#"
        {"response":true,"size":"9","squares":[{"x":0,"y":3,"value":4},{"x":0,"y":7,"value":2},{"x":0,"y":8,"value":3},{"x":1,"y":1,"value":4},{"x":1,"y":2,"value":2},{"x":1,"y":3,"value":9},{"x":1,"y":5,"value":3},{"x":1,"y":6,"value":5},{"x":2,"y":0,"value":5},{"x":2,"y":1,"value":3},{"x":2,"y":4,"value":1},{"x":2,"y":5,"value":2},{"x":2,"y":8,"value":9},{"x":3,"y":0,"value":3},{"x":3,"y":1,"value":2},{"x":3,"y":4,"value":6},{"x":3,"y":6,"value":9},{"x":3,"y":8,"value":7},{"x":4,"y":2,"value":4},{"x":4,"y":3,"value":2},{"x":4,"y":4,"value":3},{"x":4,"y":5,"value":8},{"x":4,"y":6,"value":6},{"x":5,"y":0,"value":6},{"x":5,"y":2,"value":1},{"x":5,"y":4,"value":5},{"x":5,"y":7,"value":3},{"x":5,"y":8,"value":4},{"x":6,"y":0,"value":8},{"x":6,"y":2,"value":9},{"x":6,"y":3,"value":5},{"x":6,"y":4,"value":4},{"x":6,"y":8,"value":2},{"x":7,"y":0,"value":2},{"x":7,"y":2,"value":3},{"x":7,"y":3,"value":8},{"x":7,"y":5,"value":7},{"x":7,"y":6,"value":4},{"x":8,"y":7,"value":9},{"x":8,"y":8,"value":8}]}
    "#;
    let mut parser = UtepParser;
    let mut solver = SimpleSolver;
    match parser.parse(input_str) {
        Ok(parsed_grid) => {
            println!("{}", &parsed_grid);
            for (index, solved_grid) in solver.solve(&parsed_grid).iter().enumerate() {
                println!("{}\n{}", &index, solved_grid);
            }
        }
        Err(err) => println!("{}", &err)
    }
}