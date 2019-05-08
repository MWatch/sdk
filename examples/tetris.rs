

//! The start of a tetris application

#![no_std]
#![no_main]

use mwatch_sdk::prelude::*;
use embedded_graphics::prelude::*;
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::primitives::Rect;
use heapless::String;
use heapless::consts::*;
use core::fmt::Write;

// use cortex_m::interrupt::Mutex;

pub const TILE_SIZE: usize = 4;
pub const DISPLAY_SIZE: usize = 128;
pub const BOARD_SIZE: usize = DISPLAY_SIZE / TILE_SIZE;

pub const BOARD_LENGTH: usize = BOARD_SIZE * BOARD_SIZE;

static mut STATE: State = State::Running;

static mut BOARD: [bool; BOARD_LENGTH] = [false; BOARD_LENGTH];

enum State {
    Menu,
    Running,
    GameOver
}

#[no_mangle]
pub fn main() -> i32 {
    0
}

#[no_mangle]
pub fn update(_system: &mut UserSpace, display: &mut Display) -> i32 {
    let mut string: String<U128> = String::new();
    let board = unsafe { &BOARD };
    match unsafe { &STATE } {
        State::Menu => {
            write!(string, "Play Tetris!").unwrap();
        },
        State::Running => {
            // let rect = Rect::new(Coord::new(0,0), Coord::new(TILE_SIZE as i32, TILE_SIZE as i32));
            for (y, row) in board.chunks(BOARD_SIZE).enumerate() {
                for (x, tile) in row.iter().enumerate() {
                    display.draw(Rect::new(Coord::new(0,0), Coord::new(TILE_SIZE as i32, TILE_SIZE as i32))
                        .with_fill(Some(0xFFFFu16.into()))
                        .translate(Coord::new((x * TILE_SIZE) as i32, (y * TILE_SIZE) as i32))
                        // .clone()
                        .into_iter()).unwrap();
                    // system.logger.log_fmt(format_args!("Tile({},{}): {}", x, y, tile));
                }
            }
            // check for sucess rows
            // reduce the y of the current piece
            // check for collisions
            // render
        },
        State::GameOver => {
            // print game over
        }
    }
    666
}

// fn render_board(board: &[bool; BOARD_SIZE]) {
//     for tile in board {

//     }
// }

#[no_mangle]
pub fn input(system: &mut UserSpace, input: InputEvent) -> i32 {

    match input {
        InputEvent::Left => {} // move left
        InputEvent::Right => {} // move right
        InputEvent::Middle => {} // rotate the current shape
        InputEvent::Dual => {} // snap to the bottom
        _ => system.logger.log_fmt(format_args!("Input not handled {:?}", input)),
    }

    666
}