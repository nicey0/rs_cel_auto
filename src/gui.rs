use super::auto::Auto;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use graphics::*;

const TRANS: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const SIZE: [f64; 2] = [600.0, 600.0];

pub trait Colored {
    fn get_color(&self) -> [f32; 4];
}

impl<CellType: Copy + PartialEq + Colored> Auto<CellType> {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        gl.draw(args.viewport(), |c, g| {
            clear(TRANS, g);
            let xl = self.get_grid()[0].len();
            let yl = self.get_grid().len();
            for y in 0..yl {
                for x in 0..xl {
                    // SIZE[0] / self.n as f64 * self.x as f64
                    // SIZE[1] / self.n as f64 * self.y as f64);
                    let dim = [
                        SIZE[0] / xl as f64 * x as f64,
                        SIZE[1] / yl as f64 * y as f64,
                        SIZE[0] / xl as f64,
                        SIZE[1] / yl as f64];
                    rectangle(self.get_grid()[y][x].get_color(), dim, c.transform, g);
                }
            }
        })
    }

    pub fn run(&mut self) {
        let opengl = OpenGL::V3_2;
        let mut window: Window = WindowSettings::new("3x3 Squares", SIZE)
            .graphics_api(opengl)
            .exit_on_esc(true)
            .resizable(false)
            .decorated(true)
            .build().unwrap();
        let mut gl = GlGraphics::new(opengl);
        let mut events = Events::new(EventSettings::new().ups(3));
        while let Some(e) = events.next(&mut window) {
            if let Some(args) = e.render_args() { // render event
                self.render(&mut gl, &args);
            }
            if let Some(_) = e.update_args() { // update event
                self.step_panic();
            }
        }
    }
}

