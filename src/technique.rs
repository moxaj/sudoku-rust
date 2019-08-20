use std::collections::HashSet;

use crate::bi_map::BiMap;
use crate::solver::{BlockId, CellId, SolvableGrid};
use crate::stack::Stack;
use crate::DEBUG;

impl SolvableGrid {
    pub fn block_bi_map<'a>(&'a self, block: &'a Vec<CellId>) -> BiMap<'a, usize, u8> {
        BiMap::new(
            block,
            &self.values,
            block
                .iter()
                .map(|&cell_id| (cell_id, self.cell(cell_id)))
                .filter(|(_, cell)| !cell.has_value())
                .map(|(cell_id, cell)| (cell_id, cell.values.clone().into_iter().collect()))
                .collect())
    }
}

pub trait SafeSolvingTechnique {
    fn apply(&self, grid: &mut SolvableGrid, block_id: BlockId) -> Option<Stack<BlockId>>;
}

pub struct NakedHidden {

}

pub struct NakedSingle;

impl SafeSolvingTechnique for NakedSingle {
    fn apply(&self, grid: &mut SolvableGrid, block_id: BlockId) -> Option<Stack<BlockId>> {
        let mut block_ids = Stack::new();
        grid.block_bi_map(&grid.block(block_id)).singles().and_then(|singles| {
            if DEBUG {
                println!("Singles: {:?}", &singles);
            }

            for (cell_id, value) in singles {
                grid.cell_mut(cell_id).set_value(value);
                for block_id in grid.block_ids(cell_id) {
                    for neighbor_cell_id in grid.block(block_id) {
                        if grid.cell_mut(neighbor_cell_id).values.remove(&value) {
                            for neighbor_block_id in grid.block_ids(neighbor_cell_id) {
                                block_ids.push(neighbor_block_id);
                            }
                        }
                    }
                }
            }

            Some(block_ids)
        })
    }
}
