use piston::input::{Motion, Button};
use piston::input::mouse::MouseButton;
use graphics::context::Context;
use graphics::Graphics;
use std::io::Read;
use std::fs::File;
use configuration::CFG;
use error::*;
use math::*;
use physics::{Body, Shape};

#[derive(Debug, Clone)]
struct Effect {
    pos: [f64;2],
    angle: f64,
    timer: f64,
}

pub struct App {
    walls: Vec<Body>,
    ball: Body,
    ball_vel: [f64; 2],
    ball_acc: [f64; 2],
    airjump: bool,
    effects: Vec<Effect>,
    cursor: [f64; 2],
    window_size: [u32; 2],
}

impl App {
    pub fn new() -> AirjumpResult<App> {
        let mut svg_data = Vec::new();
        File::open("map.svg")?.read_to_end(&mut svg_data)?;
        let map = ::map::load_map(&svg_data)?;

        Ok(App {
            ball: Body {
                pos: map.start,
                shape: Shape::Circle(CFG.gameplay.ball_radius),
            },
            ball_vel: [0., 0.],
            ball_acc: [0., 0.],
            walls: map.bodies,
            cursor: [0., 0.],
            window_size: CFG.window.dimensions,
            effects: vec!(),
            airjump: true,
        })
    }
    pub fn render<G: Graphics>(&mut self, context: Context, frame: &mut G) {
        use graphics::*;

        clear(CFG.graphics.background_color, frame);

        let (w, h) = (self.window_size[0] as f64, self.window_size[1] as f64);

        let context = context.reset()
            .scale((h/w).min(1.), (w/h).min(1.))
            .zoom(CFG.camera.zoom)
            .flip_h()
            .trans(-self.ball.pos[0], -self.ball.pos[1]);

        for b in &self.walls {
            match b.shape {
                Shape::Circle(radius) => ellipse(CFG.graphics.wall_color, rectangle::centered_square(b.pos[0], b.pos[1], radius), context.transform, frame),
                Shape::Rectangle(width, height) => rectangle(CFG.graphics.wall_color, rectangle::centered([b.pos[0], b.pos[1], width/2., height/2.]), context.transform, frame),
            }
        }

        ellipse(CFG.graphics.ball_color, rectangle::centered_square(self.ball.pos[0], self.ball.pos[1], CFG.gameplay.ball_radius), context.transform, frame);

        for effect in &self.effects {
            let mut color = CFG.graphics.effect_color;
            color[3] *= (effect.timer/CFG.graphics.effect_timer) as f32;
            let half_width = CFG.gameplay.ball_radius + CFG.graphics.effect_thickness/2.;
            let half_height = CFG.graphics.effect_thickness/2.;
            let rect = rectangle::centered([0., 0., half_height, half_width]);

            let transform = context.transform
                .trans(effect.pos[0], effect.pos[1])
                .rot_rad(effect.angle)
                .trans(-(CFG.gameplay.ball_radius + half_height), 0.);
            rectangle(color, rect, transform, frame);
        }

        let context = context.reset()
            .trans(-1., 1.)
            .scale(2./w, 2./h)
            .flip_v();

        let unit = f64::min(w,h);

        let color = CFG.graphics.cursor_color;
        let center_x = self.cursor[0];
        let center_y = self.cursor[1];
        let half_width = (CFG.graphics.cursor_outer_radius - CFG.graphics.cursor_inner_radius)/2. * unit;
        let half_height = CFG.graphics.cursor_thickness/2. * unit;
        let delta = CFG.graphics.cursor_inner_radius * unit + half_width;

        rectangle(color, rectangle::centered([center_x + delta, center_y, half_width, half_height]), context.transform, frame);
        rectangle(color, rectangle::centered([center_x - delta, center_y, half_width, half_height]), context.transform, frame);
        rectangle(color, rectangle::centered([center_x, center_y + delta, half_height, half_width]), context.transform, frame);
        rectangle(color, rectangle::centered([center_x, center_y - delta, half_height, half_width]), context.transform, frame);
    }
    pub fn update(&mut self, dt: f64) {
        use ::std::f64::consts::PI;

        for effect in &mut self.effects {
            effect.timer -= dt;
        }
        self.effects.retain(|e| e.timer > 0.);

        let mut force = [0., -CFG.gameplay.gravity];

        force[0] -= CFG.gameplay.damping*self.ball_vel[0];
        force[1] -= CFG.gameplay.damping*self.ball_vel[1];

        self.ball_acc = force;

        self.ball_vel[0] += dt*self.ball_acc[0];
        self.ball_vel[1] += dt*self.ball_acc[1];

        self.ball.pos[0] += dt*self.ball_vel[0];
        self.ball.pos[1] += dt*self.ball_vel[1];

        let mut collision = None;
        for w in &self.walls {
            if let Some(c) = self.ball.collide(w) {
                collision = collision.map_or(Some(c.clone()), |mut collision| {collision.push(c); Some(collision)});
            }
        }
        if let Some(collision) = collision {
            self.ball.pos[0] += collision.dx;
            self.ball.pos[1] += collision.dy;

            let col_angle = angle([collision.dx, collision.dy]);
            let vel_angle = angle(self.ball_vel);

            let norm = norm(self.ball_vel);
            let angle = col_angle + col_angle-vel_angle+PI;

            self.ball_vel = from_polar([norm, angle]);
            self.airjump = true;
        }
    }
    pub fn press(&mut self, button: Button) {
        match button {
            Button::Mouse(MouseButton::Left) => {
                if self.airjump {
                    self.airjump = false;
                    if CFG.gameplay.reset {
                        self.ball_vel = [0., 0.];
                    }
                    let x = self.cursor[0] - (self.window_size[0] as f64)/2.;
                    let y = self.cursor[1] - (self.window_size[1] as f64)/2.;
                    let norm = (x.powi(2) + y.powi(2)).sqrt();
                    let impulse = [
                        x/norm*CFG.gameplay.impulse,
                        y/norm*CFG.gameplay.impulse,
                    ];
                    self.ball_vel[0] += impulse[0];
                    self.ball_vel[1] += impulse[1];

                    self.effects.push(Effect {
                        pos: [self.ball.pos[0], self.ball.pos[1]],
                        angle: impulse[1].atan2(impulse[0]),
                        timer: CFG.graphics.effect_timer,
                    });
                }
            },
            Button::Mouse(MouseButton::Right) => {
                if CFG.gameplay.reset {
                    self.ball_vel = [0., 0.];
                }
                let x = self.cursor[0] - (self.window_size[0] as f64)/2.;
                let y = self.cursor[1] - (self.window_size[1] as f64)/2.;
                let norm = (x.powi(2) + y.powi(2)).sqrt();
                let impulse = [
                    x/norm*CFG.gameplay.impulse,
                    y/norm*CFG.gameplay.impulse,
                ];
                self.ball_vel[0] += impulse[0];
                self.ball_vel[1] += impulse[1];

                self.effects.push(Effect {
                    pos: [self.ball.pos[0], self.ball.pos[1]],
                    angle: impulse[1].atan2(impulse[0]),
                    timer: CFG.graphics.effect_timer,
                });
            },
            _ => (),
        }
    }
    pub fn release(&mut self, _button: Button) {}
    pub fn do_move(&mut self, motion: Motion) {
        match motion {
            Motion::MouseRelative(mut dx, mut dy) => {
                dx *= CFG.control.mouse_sensibility;
                dy *= CFG.control.mouse_sensibility;
                self.cursor[0] = f64::max(0., f64::min(self.cursor[0] + dx, self.window_size[0] as f64));
                self.cursor[1] = f64::max(0., f64::min(self.cursor[1] + dy, self.window_size[1] as f64));
            },
            _ => (),
        }
    }
    pub fn resize(&mut self, w: u32, h: u32) {
        self.window_size = [w, h];
    }
}
