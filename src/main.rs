mod game;
use std::process;

use game::grid_renderer::Renderer;

fn main() {
    let mut renderer = Renderer::new();
    loop {
        if !renderer.update_and_draw_grid() {
            process::exit(0);
        }
    }
}
