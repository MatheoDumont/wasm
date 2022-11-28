mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// #[wasm_bindgen]
// pub fn greetings(name: &str, nb: u32) {
//     alert(&format!("Hello, {}! and the number is {}", name, nb));
// }

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Window {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Window {
    fn _comp_idx(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn at(&self, row: u32, col: u32) -> &Cell {
        let idx = self._comp_idx(row, col);
        &self.cells[idx]
    }
    
}


