use std::collections::{HashMap, HashSet};
use std::collections::linked_list::LinkedList;
use std::fmt;

use crate::parser::ParsedGrid;
use crate::stack::Stack;
use crate::technique::{NakedSingle, SafeSolvingTechnique};

#[derive(Clone)]
pub struct Cell {
    // TODO cached value? inline?
    pub values: HashSet<u8>
}

impl Cell {
    pub fn default() -> Self {
        Cell { values: (1..=9).collect::<HashSet<u8>>() }
    }

    pub fn of(value: u8) -> Self {
        Cell {
            values: {
                let mut values = HashSet::new();
                values.insert(value);
                values
            }
        }
    }

    pub fn has_value(&self) -> bool {
        self.values.len() == 1
    }

    pub fn get_value(&self) -> Option<u8> {
        if self.has_value() {
            self.values.iter().next().copied()
        } else {
            None
        }
    }

    pub fn set_value(&mut self, value: u8) {
        self.values = HashSet::new();
        self.values.insert(value);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BlockType {
    Row,
    Column,
    Box,
}

pub type CellId = usize;

pub type BlockId = (BlockType, usize);

#[derive(Clone)]
pub struct SolvableGrid {
    pub cells: Vec<Cell>,
    pub values: Vec<u8>,
}

impl SolvableGrid {
    pub fn cell(&self, cell_id: CellId) -> &Cell {
        self.cells.get(cell_id).unwrap()
    }

    pub fn cell_mut(&mut self, cell_id: CellId) -> &mut Cell {
        self.cells.get_mut(cell_id).unwrap()
    }

    pub fn block(&self, block_id: BlockId) -> Vec<CellId> {
        match block_id.0 {
            BlockType::Row => (0..9).map(|index| block_id.1 * 9 + index).collect(),
            BlockType::Column => (0..9).map(|index| block_id.1 + index * 9).collect(),
            BlockType::Box => {
                let cell_index = (block_id.1 / 3) * 27 + (block_id.1 % 3) * 3;
                [
                    cell_index + 0,
                    cell_index + 1,
                    cell_index + 2,
                    cell_index + 9,
                    cell_index + 10,
                    cell_index + 11,
                    cell_index + 18,
                    cell_index + 19,
                    cell_index + 20,
                ].to_vec()
            }
        }
    }

    pub fn block_ids(&self, cell_id: CellId) -> Vec<BlockId> {
        vec![
            (BlockType::Row, cell_id / 9),
            (BlockType::Column, cell_id % 9),
            (BlockType::Box, (cell_id / 27) * 3 + (cell_id % 9) / 3),
        ]
    }


    pub fn is_solved(&self) -> bool {
        self.cells.iter().all(|cell| cell.has_value())
    }
}

impl From<ParsedGrid> for SolvableGrid {
    fn from(parsed_grid: ParsedGrid) -> Self {
        SolvableGrid {
            cells: {
                parsed_grid.cells
                    .into_iter()
                    .map(|cell| {
                        if let Some(value) = cell {
                            Cell::of(value)
                        } else {
                            Cell::default()
                        }
                    })
                    .collect()
            },
            values: (1..=9).collect(),
        }
    }
}

impl fmt::Display for SolvableGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (index, cell) in self.cells.iter().enumerate() {
            write!(f, "{}{}",
                   if let Some(value) = cell.get_value() {
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

pub struct SolvedGrid {
    pub cells: Vec<u8>
}

impl fmt::Display for SolvedGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (index, cell) in self.cells.iter().enumerate() {
            write!(f, "{}{}",
                   cell.to_string(),
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

impl From<SolvableGrid> for SolvedGrid {
    fn from(grid: SolvableGrid) -> Self {
        SolvedGrid {
            cells: grid.cells.iter().map(|cell| cell.get_value().unwrap()).collect()
        }
    }
}

pub trait Solver {
    fn solve(&mut self, grid: &ParsedGrid) -> Vec<SolvedGrid>;
}

pub struct SimpleSolver;

impl SimpleSolver {
    fn initial_block_ids(&self, grid: &SolvableGrid) -> Stack<BlockId> {
        let mut block_ids = Stack::new();
        for block_index in 0..9 {
            for &block_type in [BlockType::Row, BlockType::Column, BlockType::Box].iter() {
                if grid
                    .block((block_type, block_index))
                    .iter()
                    .any(|&cell_id| grid.cell(cell_id).has_value()) {
                    block_ids.push((block_type, block_index));
                }
            }
        }

        block_ids
    }

    fn solve_inner(&self, grid: &mut SolvableGrid, mut block_ids: Stack<BlockId>) -> LinkedList<SolvedGrid> {
        while let Some(block_id) = block_ids.pop() {
            if let Some(mut new_block_ids) = NakedSingle.apply(grid, block_id) {
                new_block_ids.insert_into(&mut block_ids);
            } else {
                return LinkedList::new();
            }
        }

        println!("We got this far:\n{}", &grid);
        LinkedList::new()
        /*
        if grid.is_solved() {
            let mut solutions = LinkedList::new();
            solutions.push_front(grid.clone().into());
            solutions
        } else {
            // TODO temporary
            let mut solutions = LinkedList::new();
            solutions.push_front(grid.clone().into());
            solutions
        }
        */
    }
}

impl Solver for SimpleSolver {
    fn solve(&mut self, grid: &ParsedGrid) -> Vec<SolvedGrid> {
        let mut grid: SolvableGrid = grid.clone().into();
        let block_ids = self.initial_block_ids(&grid);
        self.solve_inner(&mut grid, block_ids).into_iter().collect()
    }
}