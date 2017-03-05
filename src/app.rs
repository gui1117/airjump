use configuration::CFG;
use map::MAP;
use math::*;
use physics::{Body, Shape};
use spatial_hashing::SpatialHashing;
use graphics::{self, Layer, Transformed};

#[derive(Debug, Clone)]
struct Effect {
    pos: [f64;2],
    angle: f64,
    timer: f64,
}

pub struct App {
    walls: SpatialHashing,
    ball: Body,
    ball_vel: [f64; 2],
    ball_acc: [f64; 2],
    air_jump: bool,
    effects: Vec<Effect>,
    jump_angle: f64,
    pub must_quit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            ball: Body {
                pos: MAP.start,
                shape: Shape::Circle(CFG.gameplay.ball_radius),
            },
            ball_vel: [0., 0.],
            ball_acc: [0., 0.],
            walls: SpatialHashing::new(CFG.physics.unit, &MAP.bodies),
            jump_angle: 0.,
            effects: vec!(),
            air_jump: true,
            must_quit: false,
        }
    }
    pub fn camera(&self) -> graphics::Camera {
        graphics::Camera {
            x: self.ball.pos[0] as f32,
            y: self.ball.pos[1] as f32,
            zoom: CFG.camera.zoom as f32,
        }
    }
    pub fn draw(&mut self, frame: &mut graphics::Frame) {
        let (w, h) = {
            let (w, h) = frame.size();
            (w as f64, h as f64)
        };

        let field_of_view = Body {
            pos: self.ball.pos,
            shape: Shape::Rectangle(2./(h/w).min(1.)/CFG.camera.zoom, 2./(w/h).min(1.)/CFG.camera.zoom),
        };

        for b in &self.walls.get_on_body(&field_of_view) {
            match b.shape {
                Shape::Circle(radius) => frame.draw_circle(b.pos[0] as f32, b.pos[1] as f32, radius as f32, Layer::World, CFG.graphics.wall_color),
                Shape::Rectangle(width, height) => frame.draw_rectangle(b.pos[0] as f32, b.pos[1] as f32, width as f32, height as f32, Layer::World, CFG.graphics.wall_color),
            }
        }

        frame.draw_circle(self.ball.pos[0] as f32, self.ball.pos[1] as f32, CFG.gameplay.ball_radius as f32, Layer::World, CFG.graphics.ball_color);

        for effect in &self.effects {
            let mut color = CFG.graphics.effect_color;
            color[3] *= (effect.timer/CFG.graphics.effect_timer) as f32;
            let half_width = CFG.gameplay.ball_radius  as f32;
            let half_height = CFG.graphics.effect_thickness/2.;

            let transform = graphics::Transformation::identity()
                .translate(effect.pos[0] as f32, effect.pos[1] as f32)
                .rotate(effect.angle as f32)
                .translate(-(CFG.gameplay.ball_radius as f32 + half_height), 0.)
                .scale(half_height, half_width);
            frame.draw_quad(transform, Layer::World, color);
        }
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
        for w in &self.walls.get_on_body(&self.ball) {
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
            self.air_jump = true;
        }
    }
    pub fn do_jump(&mut self) {
        if self.air_jump {
            self.air_jump = false;
            if CFG.gameplay.reset {
                self.ball_vel = [0., 0.];
            }
            self.ball_vel[0] += self.jump_angle.cos()*CFG.gameplay.impulse;
            self.ball_vel[1] += self.jump_angle.sin()*CFG.gameplay.impulse;

            self.effects.push(Effect {
                pos: [self.ball.pos[0], self.ball.pos[1]],
                angle: self.jump_angle,
                timer: CFG.graphics.effect_timer,
            });
        }
    }
    pub fn set_jump_angle(&mut self, angle: f64) {
        self.jump_angle = angle;
    }
}
