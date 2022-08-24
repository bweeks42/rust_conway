extern crate piston_window;
extern crate find_folder;

use opengl_graphics::GlGraphics;
use piston::{Events, EventSettings, RenderEvent, UpdateArgs, GenericEvent, MouseButton};
use piston_window::{RenderArgs, WindowSettings, OpenGL, PistonWindow};

mod conway_core;
use conway_core::{ConwayMatrix};
use graphics::{Context, Graphics};
use piston::input::{Key, Button};


const CONWAY_GRID_SIZE:usize = 50;
const GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const UPDATES_PER_SECOND: u64 = 60;
const FRAMES_PER_SECOND: u64 = 60;
const square_inset:f64 = 2.0; // squares are slightly smaller when drawn to differentiate
const drawable_inset:f64 = 20.0;


struct ViewSettings {
    drawable_area_size: f64,
    cell_size: f64
}

// CONTROLLER
struct ConwayController {
    pub matrix: ConwayMatrix,
    is_running: bool,
    chaos_mode: bool,
    updates_for_tick: u64,
    update_count: u64,
    cursor_pos: [f64; 2]
}

impl ConwayController {
    fn new(matrix: ConwayMatrix) -> Self {
        ConwayController { 
            matrix: matrix,
            is_running: true,
            chaos_mode: false,
            updates_for_tick: UPDATES_PER_SECOND,
            update_count: 0,
            cursor_pos: [0.0; 2]
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.matrix.tick();
        self.matrix.drop_glider();
    }

    pub fn event<E: GenericEvent>(&mut self, event: &E, view_settings: &ViewSettings) {
        let mut should_tick = self.is_running;
        if let Some(pos) = event.mouse_cursor_args() {
            self.cursor_pos = pos;
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Space => self.is_running = !self.is_running,
                Key::Right => {should_tick = true; self.update_count = self.updates_for_tick},
                Key::Up => if self.updates_for_tick >= 5 {self.updates_for_tick -= 5},
                Key::Down => self.updates_for_tick += 5,
                Key::C => self.matrix.clear(),
                Key::G => self.matrix.drop_glider(),
                Key::M => self.chaos_mode = !self.chaos_mode,
                _ => {}
            }
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            let x = self.cursor_pos[0] - drawable_inset;
            let y = self.cursor_pos[1] - drawable_inset;
            let drawable_size = view_settings.drawable_area_size;
            let cell_size = view_settings.cell_size;
            if x >= 0.0 && x <= drawable_size && y >= 0.0 && y <= drawable_size {
                let cell_x = ((x / drawable_size) * CONWAY_GRID_SIZE as f64) as usize;
                let cell_y = ((y / drawable_size) * CONWAY_GRID_SIZE as f64) as usize;
                self.matrix.toggle_cell(cell_x, cell_y);
            }
        }




        if should_tick && self.update_count >= self.updates_for_tick {
            if self.chaos_mode {
                self.matrix.drop_glider()
            }
            self.update_count = 0;
            self.matrix.tick()
        }
        self.update_count += 1
    }

}





// RENDERER
struct ConwayRenderer {
    pub view_settings: ViewSettings
}

impl ConwayRenderer {
    fn new() -> Self {

        Self { view_settings: ViewSettings { drawable_area_size: 0.0, cell_size: 0.0 } }
    }

    fn render<G: Graphics>(&mut self, args: &RenderArgs, c: &Context, graphics: &mut G, controller: &ConwayController) {
        use graphics::rectangle;

        // Draw on board on left side
        let smaller_window_constraint = if args.window_size[0] > args.window_size[1] / 2.0 {args.window_size[1]} else {args.window_size[0]};
        let drawable_area_length = smaller_window_constraint - drawable_inset*2.0;
        let square_size = drawable_area_length / CONWAY_GRID_SIZE as f64;
        let squares_per_row = (drawable_area_length / square_size) as usize;
       
        self.view_settings.cell_size = square_size;
        self.view_settings.drawable_area_size = drawable_area_length;

        for y in 0..squares_per_row {
            for x in 0..squares_per_row {
                rectangle(
                    if controller.matrix.cell_at_index(x, y).is_alive() {RED} else {WHITE},
                    rectangle::square((x as f64 * square_size) + drawable_inset, (y as f64 * square_size) + drawable_inset, square_size - square_inset),
                    c.transform, 
                    graphics
                );
            }
        }

        // Draw buttons/setting on right side

    }


}

// GAME
struct Conway {
    window: PistonWindow,
    events: Events,
    controller: ConwayController,
    renderer: ConwayRenderer
}

impl Conway {
    fn new() -> Self {
        let matrix = ConwayMatrix::new(CONWAY_GRID_SIZE);
        let controller = ConwayController::new(matrix);
        let mut event_settings = EventSettings::new();
        event_settings.max_fps = FRAMES_PER_SECOND;
        event_settings.ups = UPDATES_PER_SECOND;
        let mut events = Events::new(event_settings);
        let opengl = OpenGL::V3_2;
        let window: PistonWindow = WindowSettings::new(
            "Conway's Game of Life",
            [1900, 1000]
        ).exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
        let renderer = ConwayRenderer::new();
        Conway {window, events, controller, renderer}
    }

    fn run(&mut self) {
        let opengl = OpenGL::V3_2;
        let mut gl = GlGraphics::new(opengl);
        while let Some(e) = self.events.next(&mut self.window) {
            self.controller.event(&e, &self.renderer.view_settings);

            if let Some(args) = e.render_args() {
                gl.draw(args.viewport(), |c, g| {
                    use graphics::{clear};
                    clear(GRAY, g);
                    self.renderer.render(&args, &c, g, &self.controller)
                })
            }
        }
    }
}






fn main() {
    let mut conway_game = Conway::new();
    conway_game.run();
}