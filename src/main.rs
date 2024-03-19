extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL,Texture};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use crate::piston::ButtonEvent;
use piston::Key;
use piston::ButtonState;
use piston::Button;
use std::default;
use std::path::Path;
use graphics::rectangle::square;
use opengl_graphics::TextureSettings;
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  
    y: f64,
    vertical_velocity: f64,
    canos: Canos,
    timer: f64,
    texture_bird: Texture,
    texture_back: Texture,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        // let square = rectangle::square(0.0, 0.0, 50.0);
        let back = Image::new().rect(rectangle::rectangle_by_corners(0.0 ,0.0,args.window_size[0],args.window_size[1]));
        let image   = Image::new().rect(square(0.0, 0.0, 30.0));
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0]/8.0, self.y);
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            let transform2 = c
                .transform;

            back.draw(&self.texture_back, &DrawState::default() , transform2, gl);
            image.draw(&self.texture_bird, &DrawState::default() , transform, gl);
        });
        self.canos.render(&mut self.gl,&args);
        if self.timer >= 2.0{
            self.timer = 0.0;
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation = self.vertical_velocity/2.0;
        self.vertical_velocity += args.dt;
        self.y += 400.0*self.vertical_velocity*args.dt ;
        self.canos.update(&args);
        self.timer += args.dt*100.0;

    }
    fn button(&mut self, btn: &Button) {
        let last_vertical = self.vertical_velocity.clone();
        self.vertical_velocity = match btn {
            &Button::Keyboard(Key::Up) => -0.05 * f64::abs(last_vertical) + -1.0,
            _ => last_vertical,
        };
    }
}

pub struct Canos{
    x:f64,
    y:f64,
    texture_cano: Texture,
    texture_cano_cima: Texture,
}
impl Canos {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const SPACE: f64 = 100.0;
        const SPACE2: f64 = 80.0;

        let rec1 = rectangle::rectangle_by_corners(self.x ,self.y + SPACE,self.x + SPACE2,args.window_size[1]);
        let rec2 = rectangle::rectangle_by_corners(self.x ,self.y - SPACE,self.x + SPACE2,0.0);
        let cano_baixo   = Image::new().rect(rec1);
        let cano_cima   = Image::new().rect(rec2);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c
                .transform;

            // Draw a box rotating around the middle of the screen.
            cano_cima.draw(&self.texture_cano_cima, &DrawState::default() , transform, gl);
            cano_baixo.draw(&self.texture_cano, &DrawState::default() , transform, gl);
        });
    }
    fn update(&mut self, args: &UpdateArgs) {
        self.x -= args.dt*200.0 ;

    }

}
fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("flappy-rust", [1200, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap() ;
    // Create a new game and run it.

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        y: 0.0,
        vertical_velocity:0.0,
        canos: Canos{x:1100.0,y:400.0,texture_cano:Texture::from_path(Path::new("src/img/cano.png"), &TextureSettings::new()).unwrap(),texture_cano_cima:Texture::from_path(Path::new("src/img/cano_cima.png"), &TextureSettings::new()).unwrap()},
        timer: 0.0,
        texture_back:Texture::from_path(Path::new("src/img/background.png"), &TextureSettings::new()).unwrap(),
        texture_bird:Texture::from_path(Path::new("src/img/bird.png"), &TextureSettings::new()).unwrap(),
    };
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                app.button(&args.button);
            };
        }
    }
}