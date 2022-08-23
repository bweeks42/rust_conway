extern crate piston_window;
extern crate find_folder;

use opengl_graphics::GlGraphics;
use piston::{Events, EventSettings, RenderEvent, UpdateEvent, UpdateArgs};
use piston_window::{RenderArgs, WindowSettings, OpenGL, PistonWindow};
use std::cmp;

mod conway_core;
use conway_core::{ConwayMatrix};


struct Conway {
    matrix: ConwayMatrix,
    window: PistonWindow,
    graphics: GlGraphics,
}
const CONWAY_GRID_SIZE:usize = 200;

impl Conway {
    fn new() -> Self {
        let opengl = OpenGL::V3_2;
        let window: PistonWindow = WindowSettings::new(
            "Conway's Game of Life",
            [1900, 1000]
        ).exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
        let matrix = ConwayMatrix::new(CONWAY_GRID_SIZE);
        Self { matrix: ConwayMatrix::new(CONWAY_GRID_SIZE), window: window, graphics: GlGraphics::new(opengl),  }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        const GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        
        let square_inset = 2.0; // squares are slightly smaller to differentiate
        let drawable_inset = 20.0;
        let smaller_window_constraint = if args.window_size[0] > args.window_size[1] {args.window_size[1]} else {args.window_size[0]};
        let drawable_area_length = smaller_window_constraint - drawable_inset*2.0;
        let square_size = drawable_area_length / self.matrix.size() as f64;
        let squares_per_row = (drawable_area_length / square_size) as usize;



        // for y in 0..squares_per_row {
        //     let mut row: Vec<DrawableCell> = vec![];
        //     for x in 0..squares_per_row {
        //         //row.push(DrawableCellrectangle::square((x as f64 * square_size) + drawable_inset, (y as f64 * square_size) + drawable_inset, square_size - square_inset));
        //         row.push(DrawableCell { 
        //             cell: self.matrix.cell_at_index(x as usize, y as usize), 
                    
        //     }
        //     squares.push(row);
        // }
        //let (x_center, y_center) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        self.graphics.draw(args.viewport(), |c, gl| {
            clear(GRAY, gl);
            //let transform = c.transform.trans(x_center, y_center).rot_rad(self.rotation).trans(square_size / -2.0, square_size / -2.0);
            for y in 0..squares_per_row {
                for x in 0..squares_per_row {
                    rectangle(
                        if self.matrix.cell_at_index(x, y).is_alive() {RED} else {WHITE}, 
                        rectangle::square((x as f64 * square_size) + drawable_inset, (y as f64 * square_size) + drawable_inset, square_size - square_inset),
                        c.transform, 
                        gl
                    );
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.matrix.tick();
        self.matrix.drop_glider();
    }

    fn run(&mut self) {
        let mut event_settings = EventSettings::new();
        event_settings.max_fps = 60;
        event_settings.ups = 30;
        let mut events = Events::new(event_settings);
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(args) = e.update_args() {
                self.update(&args);
            }
        }
    }
}

fn main() {
    let mut conway_game = Conway::new();
    conway_game.run();
}