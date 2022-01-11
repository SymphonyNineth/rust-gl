extern crate rand;
use std::{thread, time};
const GRID_SIZE: usize = 120;
fn census(_world: [[u8; GRID_SIZE]; GRID_SIZE]) -> u16 {
    let mut count = 0;
    for i in 0..GRID_SIZE - 1 {
        for j in 0..GRID_SIZE - 1 {
            if _world[i][j] == 1 {
                count += 1;
            }
        }
    }
    count
}
fn generation(_world: [[u8; GRID_SIZE]; GRID_SIZE]) -> [[u8; GRID_SIZE]; GRID_SIZE] {
    let mut newworld = [[0u8; GRID_SIZE]; GRID_SIZE];
    for i in 0..GRID_SIZE - 1 {
        for j in 0..GRID_SIZE - 1 {
            let mut count = 0;
            if i > 0 {
                count = count + _world[i - 1][j];
            }
            if i > 0 && j > 0 {
                count = count + _world[i - 1][j - 1];
            }
            if i > 0 && j < GRID_SIZE - 1 {
                count = count + _world[i - 1][j + 1];
            }
            if i < GRID_SIZE - 1 && j > 0 {
                count = count + _world[i + 1][j - 1]
            }
            if i < GRID_SIZE - 1 {
                count = count + _world[i + 1][j];
            }
            if i < GRID_SIZE - 1 && j < GRID_SIZE - 1 {
                count = count + _world[i + 1][j + 1];
            }
            if j > 0 {
                count = count + _world[i][j - 1];
            }
            if j < GRID_SIZE - 1 {
                count = count + _world[i][j + 1];
            }
            newworld[i][j] = 0;
            if (count < 2) && (_world[i][j] == 1) {
                newworld[i][j] = 0;
            }
            if _world[i][j] == 1 && (count == 2 || count == 3) {
                newworld[i][j] = 1;
            }
            if (_world[i][j] == 0) && (count == 3) {
                newworld[i][j] = 1;
            }
        }
    }
    newworld
}
fn main() {
    let mut world = [[0u8; GRID_SIZE]; GRID_SIZE];
    let mut generations = 0;
    for i in 0..GRID_SIZE - 1 {
        for j in 0..GRID_SIZE - 1 {
            if rand::random() {
                world[i][j] = 1;
            } else {
                world[i][j] = 0;
            }
        }
    }
    let nw = generation(world);
}
