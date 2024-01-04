use super::constants;
use super::grid::Grid;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
use raylib::prelude::*;

pub struct Renderer {
    grid: Grid,
    is_paused: bool,
    rl: RaylibHandle,
    thread: RaylibThread,
}

impl Renderer {
    pub fn new() -> Renderer {
        let width = constants::SCREEN_WIDTH as i32;
        let height = constants::SCREEN_HEIGHT as i32;
        let (rl, thread) = raylib::init()
            .size(width, height)
            .title("Game of Life")
            .build();

        Renderer {
            grid: Grid::new(),
            is_paused: false,
            rl,
            thread,
        }
    }

    pub fn update_and_draw_grid(&mut self) -> bool {
        let is_closed = !self.rl.window_should_close();

        self.keyboard_handle();
        if !self.is_paused {
            self.grid.update();
        }

        self.mouse_handle();

        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::WHITE);
        d.draw_text(
            "LMB to add\nRMB to remove\nSpace to pause",
            12,
            12,
            20,
            Color::BLACK,
        );

        for x in 0..constants::GRID_WIDTH {
            for y in 0..constants::GRID_HEIGHT {
                if self.grid.get_cell(x, y) {
                    d.draw_rectangle(
                        (x * constants::CELL_SIZE) as i32,
                        (y * constants::CELL_SIZE) as i32,
                        constants::CELL_SIZE as i32,
                        constants::CELL_SIZE as i32,
                        Color::BLACK,
                    )
                }
            }
        }

        is_closed
    }

    fn set_condition_of_cell(&mut self, mouse_x: i32, mouse_y: i32, condition: bool) {
        let grid_x = mouse_x as usize / constants::CELL_SIZE;
        let grid_y = mouse_y as usize / constants::CELL_SIZE;

        self.grid.set_cell(grid_x, grid_y, condition);
    }

    fn mouse_handle(&mut self) {
        if self.rl.is_mouse_button_down(MOUSE_LEFT_BUTTON) {
            self.set_condition_of_cell(self.rl.get_mouse_x(), self.rl.get_mouse_y(), true)
        }
        if self.rl.is_mouse_button_down(MOUSE_RIGHT_BUTTON) {
            self.set_condition_of_cell(self.rl.get_mouse_x(), self.rl.get_mouse_y(), false)
        }
    }

    fn keyboard_handle(&mut self) {
        if self.rl.is_key_released(KEY_SPACE) {
            self.is_paused = !self.is_paused;
        }
    }
}
