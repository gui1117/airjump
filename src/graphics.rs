extern crate vecmath;

use glium::{self, Blend, Surface, VertexBuffer, index, vertex, Program, DrawParameters, Depth, DepthTest};
use glium::backend::{Facade, Context};
use glium::backend::glutin_backend::GlutinFacade;
use glium::program::ProgramChooserCreationError;

use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::f32::consts::PI;

pub type Transformation = vecmath::Matrix2x3<f32>;

const CIRCLE_PRECISION: usize = 64;

pub trait Transformed {
    fn translate(self, x: f32, y: f32) -> Self;
    fn rotate(self, angle: f32) -> Self;
    fn scale(self, sx: f32, sy: f32) -> Self;
    fn identity() -> Self;
    fn into_3d(self, z: f32) -> [[f32; 4]; 4];
}

impl Transformed for Transformation {
    #[inline(always)]
    fn translate(self, x: f32, y: f32) -> Self {
        let trans = {
            [[1., 0., x], [0., 1., y]]
        };
        vecmath::row_mat2x3_mul(self, trans)
    }

    #[inline(always)]
    fn rotate(self, angle: f32) -> Self {
        let rot = {
            let c = angle.cos();
            let s = angle.sin();
            [[c, -s, 0.], [s, c, 0.]]
        };
        vecmath::row_mat2x3_mul(self, rot)
    }

    #[inline(always)]
    fn scale(self, sx: f32, sy: f32) -> Self {
        let scale = {
            [[sx, 0., 0.], [0., sy, 0.]]
        };
        vecmath::row_mat2x3_mul(self, scale)
    }

    #[inline(always)]
    fn identity() -> Self {
        [[1., 0., 0.], [0., 1., 0.]]
    }

    #[inline(always)]
    fn into_3d(self, z: f32) -> [[f32; 4]; 4] {
        [[self[0][0], self[1][0], 0., 0.],
         [self[0][1], self[1][1], 0., 0.],
         [        0.,         0., 1., 0.],
         [self[0][2], self[1][2],  z, 1.]]
    }
}

#[derive(Clone,Copy)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

pub struct Graphics {
    context: Rc<Context>,

    quad_vertex_buffer: VertexBuffer<Vertex>,
    quad_indices: index::NoIndices,
    circle_vertex_buffer: VertexBuffer<Vertex>,
    circle_indices: index::NoIndices,
    program: Program,

    draw_parameters: DrawParameters<'static>,
}

#[derive(Debug)]
pub enum GraphicsError {
    ProgramChooserCreation(ProgramChooserCreationError),
    VertexBufferCreation(vertex::BufferCreationError),
    IndexBufferCreation(index::BufferCreationError),
}

impl Error for GraphicsError {
    fn description(&self) -> &str {
        use self::GraphicsError::*;
        match *self {
            ProgramChooserCreation(ref err) => err.description(),
            VertexBufferCreation(ref err) => err.description(),
            IndexBufferCreation(ref err) => err.description(),
        }
    }
    fn cause(&self) -> Option<&Error> {
        use self::GraphicsError::*;
        match *self {
            ProgramChooserCreation(ref e) => e.cause(),
            VertexBufferCreation(ref e) => e.cause(),
            IndexBufferCreation(ref e) => e.cause(),
        }
    }
}
impl fmt::Display for GraphicsError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::GraphicsError::*;
        match *self {
            ProgramChooserCreation(ref e) => write!(fmt, "Glium program chooser creation error: {}", e),
            VertexBufferCreation(ref e) => write!(fmt, "Glium vertex buffer creation error: {}", e),
            IndexBufferCreation(ref e) => write!(fmt, "Glium index buffer creation error: {}", e),
        }
    }
}
impl From<ProgramChooserCreationError> for GraphicsError {
    fn from(err: ProgramChooserCreationError) -> GraphicsError {
        GraphicsError::ProgramChooserCreation(err)
    }
}
impl From<index::BufferCreationError> for GraphicsError {
    fn from(err: index::BufferCreationError) -> GraphicsError {
        GraphicsError::IndexBufferCreation(err)
    }
}
impl From<vertex::BufferCreationError> for GraphicsError {
    fn from(err: vertex::BufferCreationError) -> GraphicsError {
        GraphicsError::VertexBufferCreation(err)
    }
}

impl Graphics {
    pub fn new(facade: &GlutinFacade) -> Result<Graphics, GraphicsError> {
        let quad_vertex = vec![Vertex { position: [-1., -1.] },
                               Vertex { position: [1., -1.] },
                               Vertex { position: [-1., 1.] },
                               Vertex { position: [1., 1.] }];
        let quad_vertex_buffer = VertexBuffer::new(facade, &quad_vertex)?;

        let quad_indices = index::NoIndices(index::PrimitiveType::TriangleStrip);

        let mut circle_vertex = vec![Vertex { position: [0., 0.] }];
        {
            let delta_angle = PI * 2. / CIRCLE_PRECISION as f32;
            let mut angle = 0f32;
            circle_vertex.push(Vertex { position: [angle.cos(), angle.sin()] });
            for _ in 0..CIRCLE_PRECISION {
                angle += delta_angle;
                circle_vertex.push(Vertex { position: [angle.cos(), angle.sin()] });
            }
        }

        let circle_vertex_buffer = VertexBuffer::new(facade, &circle_vertex)?;

        let circle_indices = index::NoIndices(index::PrimitiveType::TriangleFan);

        let vertex_shader_src = r#"
            #version 100
            attribute vec2 position;
            uniform mat4 trans;
            uniform mat4 camera;
            void main() {
                mat4 matrix = camera * trans;
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 100
            precision mediump float;
            uniform vec4 color;
            void main() {
                gl_FragColor = color;
            }
        "#;
        let program = program!(facade,
            100 => {
                vertex: vertex_shader_src,
                fragment: fragment_shader_src,
            },
        )?;

        let draw_parameters = DrawParameters {
            blend: Blend::alpha_blending(),
            depth: Depth {
                test: DepthTest::IfMoreOrEqual,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        Ok(Graphics {
            context: facade.get_context().clone(),

            quad_vertex_buffer: quad_vertex_buffer,
            quad_indices: quad_indices,
            circle_vertex_buffer: circle_vertex_buffer,
            circle_indices: circle_indices,
            program: program,

            draw_parameters: draw_parameters,
        })
    }
}

pub struct Frame<'a> {
    pub frame: &'a mut glium::Frame,
    graphics: &'a mut Graphics,
    camera_matrix: [[f32; 4]; 4],
    billboard_camera_matrix: [[f32; 4]; 4],
}

#[derive(Clone)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, zoom: f32) -> Self {
        Camera {
            x: x,
            y: y,
            zoom: zoom,
        }
    }
}

impl<'a> Frame<'a> {
    pub fn new(graphics: &'a mut Graphics, frame: &'a mut glium::Frame, camera: &'a Camera) -> Frame<'a> {
        let (width, height) = graphics.context.get_framebuffer_dimensions();
        let ratio = width as f32 / height as f32;

        let camera_matrix = {
            let kx = camera.zoom;
            let ky = camera.zoom * ratio;
            let dx = -camera.x;
            let dy = -camera.y;
            [[     kx,      0., 0., 0.],
             [     0.,      ky, 0., 0.],
             [     0.,      0., 1., 0.],
             [kx * dx, ky * dy, 0., 1.]]
        };
        let billboard_camera_matrix = {
            [[1.,    0., 0., 0.],
             [0., ratio, 0., 0.],
             [0.,    0., 1., 0.],
             [0.,    0., 0., 1.]]
        };

        Frame {
            billboard_camera_matrix: billboard_camera_matrix,
            camera_matrix: camera_matrix,
            frame: frame,
            graphics: graphics,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.graphics.context.get_framebuffer_dimensions()
    }

    pub fn clear(&mut self) {
        self.frame.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 0f32);
    }

    #[inline]
    fn camera(&self, layer: Layer) -> [[f32; 4]; 4] {
        if layer.billboard() {
            self.billboard_camera_matrix
        } else {
            self.camera_matrix
        }
    }

    pub fn draw_square(&mut self, x: f32, y: f32, radius: f32, layer: Layer, color: [f32; 4]) {
        self.draw_rectangle(x, y, radius * 2., radius * 2., layer, color);
    }

    pub fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, layer: Layer, color: [f32; 4]) {
        let trans = {
            [[width / 2., 0., 0., 0.],
             [0., height / 2., 0., 0.],
             [0., 0., 1., 0.],
             [x, y, layer.into(), 1.]]
        };

        let uniform = uniform!{
            trans: trans,
            camera: self.camera(layer),
            color: color,
        };

        self.frame
            .draw(&self.graphics.quad_vertex_buffer,
                  &self.graphics.quad_indices,
                  &self.graphics.program,
                  &uniform,
                  &self.graphics.draw_parameters)
            .unwrap();
    }

    pub fn draw_circle(&mut self, x: f32, y: f32, radius: f32, layer: Layer, color: [f32; 4]) {
        let trans = {
            [[radius, 0., 0., 0.],
             [0., radius, 0., 0.],
             [0., 0., 1., 0.],
             [x, y, layer.into(), 1.]]
        };

        let uniform = uniform!{
            trans: trans,
            camera: self.camera(layer),
            color: color,
        };

        self.frame
            .draw(&self.graphics.circle_vertex_buffer,
                  &self.graphics.circle_indices,
                  &self.graphics.program,
                  &uniform,
                  &self.graphics.draw_parameters)
            .unwrap();
    }

    pub fn draw_quad(&mut self, trans: Transformation, layer: Layer, color: [f32; 4]) {

        let uniform = uniform!{
            trans: trans.into_3d(layer.into()),
            camera: self.camera(layer),
            color: color,
        };

        self.frame
            .draw(&self.graphics.quad_vertex_buffer,
                  &self.graphics.quad_indices,
                  &self.graphics.program,
                  &uniform,
                  &self.graphics.draw_parameters)
            .unwrap();
    }
}

#[derive(Clone,Copy,PartialEq)]
pub enum Layer {
    #[allow(dead_code)]
    World,
    #[allow(dead_code)]
    Billboard,
}

impl Into<f32> for Layer {
    fn into(self) -> f32 {
        match self {
            Layer::World => 0.1,
            Layer::Billboard => 0.11,
        }
    }
}

impl Layer {
    fn billboard(self) -> bool {
        use self::Layer::*;
        match self {
            Billboard => true,
            _ => false,
        }
    }
}
