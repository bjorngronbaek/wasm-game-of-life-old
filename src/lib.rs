extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let mut hello_string: String = "Hello, wasm-game-of-life!".to_owned();
    alert(&format!("{}{}",hello_string,name));
}

pub enum Cell {
    Dead = 0,
    Alive = 1
}

pub struct Universe {
    width: u32,
    height:  u32,
    cells: Vec<Cell>
}

impl Universe {
    pub fn tick(&mut self){
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width{
                let idx = self.get_index(row,col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row,col);

                
            }
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32){
        let mut count = 0;
        for delta_row in [self.height -1,0,1].iter().cloned() {
            for delta_column in [self.column -1,0,1].iter().cloned(){
                if delta_column == 0 && delta_row == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_column = (column + delta_column) % self.width;
                let idx = self.get_index(neighbor_row,neighbor_column);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}