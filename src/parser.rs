/// Parser trait and an implementation for sudoku puzzles returned by
/// http://www.cs.utep.edu/cheon/ws/sudoku/?size=[size]&level=[level].

use std::fmt;
use std::iter;

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct ParsedGrid {
    pub cells: Vec<Option<u8>>
}

impl fmt::Display for ParsedGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (index, cell) in self.cells.iter().enumerate() {
            write!(f, "{}{}",
                   if let Some(value) = cell {
                       value.to_string()
                   } else {
                       "_".to_owned()
                   },
                   if index % 27 == 26 {
                       "\n\n"
                   } else if index % 9 == 8 {
                       "\n"
                   } else if index % 3 == 2 {
                       " "
                   } else {
                       ""
                   })?;
        }

        Ok(())
    }
}

pub trait Parser {
    fn parse(&mut self, s: &str) -> Result<ParsedGrid, String>;
}

#[derive(Serialize, Deserialize)]
struct ValidCellInput {
    x: usize,
    y: usize,
    value: u8,
}

#[derive(Serialize, Deserialize)]
struct ValidGridInput {
    response: bool,
    size: String,
    squares: Vec<ValidCellInput>,
}

#[derive(Serialize, Deserialize)]
struct InvalidGridInput {
    response: bool,
    reason: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum GridInput {
    Valid(ValidGridInput),
    Invalid(InvalidGridInput),
}

pub struct UtepParser;

impl Parser for UtepParser {
    fn parse(&mut self, s: &str) -> Result<ParsedGrid, String> {
        match serde_json::from_str::<GridInput>(s)
            .map_err(|e| "Invalid json response: ".to_owned() + &e.to_string())? {
            GridInput::Valid(valid_grid_input) =>
                Ok(ParsedGrid {
                    cells: {
                        let mut cells = iter::repeat(Default::default()).take(81).collect::<Vec<Option<u8>>>();
                        for valid_cell_input in valid_grid_input.squares {
                            let cell_index = valid_cell_input.y * 9 + valid_cell_input.x;
                            cells.remove(cell_index);
                            cells.insert(cell_index, Some(valid_cell_input.value));
                        }

                        cells
                    }
                }),
            GridInput::Invalid(invalid_grid_input) => Err(invalid_grid_input.reason)
        }
    }
}