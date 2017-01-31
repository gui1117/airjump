#[derive(Clone)]
pub struct Body {
    pub pos: [f64; 2],
    pub shape: Shape,
}

impl Body {
    /// if A collide with B then collision must represent
    /// the smallest vector to move A so it doesn't collide anymore
    pub fn collide(&self, other: &Body) -> Option<Collision> {
        shape_collision(self.pos, &self.shape, other.pos, &other.shape)
    }
    #[inline]
    pub fn cells(&self, unit: f64) -> Vec<[i32; 2]> {
        self.shape.cells(unit, self.pos)
    }
}

/// if A collide with B then collision must represent
/// the smallest vector to move A so it doesn't collide anymore
#[derive(Clone)]
pub struct Collision {
    pub dx: f64,
    pub dy: f64,
}
impl Collision {
    pub fn opposite(&self) -> Collision {
        Collision {
            dx: -self.dx,
            dy: -self.dy,
        }
    }
    pub fn push(&mut self, res: Collision) {
        if res.dx.abs() > self.dx.abs() { self.dx = res.dx; }
        if res.dy.abs() > self.dy.abs() { self.dy = res.dy; }
    }
}

#[derive(Clone)]
pub enum Shape {
    /// radius
    Circle(f64),
    /// width and height
    Rectangle(f64, f64),
}
impl Shape {
    fn cells(&self, unit: f64, pos: [f64; 2]) -> Vec<[i32; 2]> {
        use ::std::f64::EPSILON;

        let (w2, h2) = match *self {
            Shape::Circle(r) => (r, r),
            Shape::Rectangle(w, h) => (w / 2., h / 2.),
        };

        let min_x = ((pos[0] - w2 + EPSILON)/unit).floor() as i32;
        let max_x = ((pos[0] + w2 - EPSILON)/unit).floor() as i32;
        let min_y = ((pos[1] - h2 + EPSILON)/unit).floor() as i32;
        let max_y = ((pos[1] + h2 - EPSILON)/unit).floor() as i32;

        let mut cells = Vec::new();
        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                cells.push([x, y]);
            }
        }
        cells
    }
}

fn shape_collision(a_pos: [f64;2], a_shape: &Shape, b_pos: [f64;2], b_shape: &Shape) -> Option<Collision> {
    use self::Shape::*;
    match (a_shape, b_shape) {
        (&Circle(a_radius), &Circle(b_radius)) => circle_circle_collision(a_pos, a_radius, b_pos, b_radius),
        (&Circle(a_radius), &Rectangle(b_w, b_h)) => circle_rectangle_collision(a_pos, a_radius, b_pos, b_w, b_h),
        (&Rectangle(a_w, a_h), &Rectangle(b_w, b_h)) => rectangle_rectangle_collision(a_pos, a_w, a_h, b_pos, b_w, b_h),
        (&Rectangle(a_w, a_h), &Circle(b_radius)) => circle_rectangle_collision(b_pos, b_radius, a_pos, a_w, a_h).map(|col| col.opposite()),
    }
}

fn circle_circle_collision(a_pos: [f64;2], a_rad: f64, b_pos: [f64;2], b_rad: f64) -> Option<Collision> {
    let dx = a_pos[0]-b_pos[0];
    let dy = a_pos[1]-b_pos[1];
    let dn2 = dx.powi(2) + dy.powi(2);
    let rad = a_rad+b_rad;
    if dn2 < rad.powi(2) {
        let angle = dy.atan2(dx);
        let dn = dn2.sqrt();
        let delta = rad - dn;
        Some(Collision {
            dx: delta*angle.cos(),
            dy: delta*angle.sin(),
        })
    } else {
        None
    }
}

fn circle_rectangle_collision(a_pos: [f64;2], a_radius: f64, b_pos: [f64;2], b_width: f64, b_height: f64) -> Option<Collision> {
    let left = a_pos[0] < b_pos[0] - b_width/2.;
    let right = a_pos[0] > b_pos[0] + b_width/2.;
    let down = a_pos[1] < b_pos[1] - b_height/2.;
    let up = a_pos[1] > b_pos[1] + b_height/2.;

    let extern_horizontal = left || right;
    let extern_vertical = up || down;

    if extern_horizontal && extern_vertical {
        let insider = [if left { b_pos[0] - b_width/2. } else { b_pos[0] + b_width/2.},
                       if down { b_pos[1] - b_height/2. } else { b_pos[1] + b_height/2.}];

        if (insider[0]-a_pos[0]).powi(2) + (insider[1]-a_pos[1]).powi(2) >= a_radius.powi(2) {
            return None
        }

        let (a, b, c) = line_equation_from_points(insider, a_pos);
        let outsider = if let Some((x0, y0, x1, y1)) = circle_raycast(a_pos[0], a_pos[1], a_radius, a, b, c) {
            [if left { x0.max(x1) } else { x0.min(x1) },
             if down { y0.max(y1) } else {  y0.min(y1) }]
        } else {
            return None
        };


        Some(Collision {
            dx: insider[0] - outsider[0],
            dy: insider[1] - outsider[1],
        })
    } else {
        rectangle_rectangle_collision(a_pos, a_radius*2., a_radius*2., b_pos, b_width, b_height)
    }
}

fn rectangle_rectangle_collision(a_pos: [f64;2], a_width: f64, a_height: f64, b_pos: [f64;2], b_width: f64, b_height: f64) -> Option<Collision> {
    let a_min_x = a_pos[0] - a_width/2.;
    let a_max_x = a_pos[0] + a_width/2.;
    let a_min_y = a_pos[1] - a_height/2.;
    let a_max_y = a_pos[1] + a_height/2.;
    let b_min_x = b_pos[0] - b_width/2.;
    let b_max_x = b_pos[0] + b_width/2.;
    let b_min_y = b_pos[1] - b_height/2.;
    let b_max_y = b_pos[1] + b_height/2.;

    if (a_min_x >= b_max_x) || (b_min_x >= a_max_x) || (a_min_y >= b_max_y) || (b_min_y >= a_max_y) {
        None
    } else {
        let delta_ox = b_max_x - a_min_x;
        let delta_oxp = b_min_x - a_max_x;
        let delta_oy = b_max_y - a_min_y;
        let delta_oyp =  b_min_y - a_max_y;

        let delta_x = if delta_ox.abs() < delta_oxp.abs() {
            delta_ox
        } else {
            delta_oxp
        };

        let delta_y = if delta_oy.abs() < delta_oyp.abs() {
            delta_oy
        } else {
            delta_oyp
        };

        if delta_x.abs() < delta_y.abs() {
            Some(Collision {
                dx: delta_x,
                dy: 0.,
            })
        } else {
            Some(Collision {
                dx: 0.,
                dy: delta_y,
            })
        }
    }
}

/// the coordinate of the intersections (if some) of a circle of center (x,y) and radius,
/// and the line of equation ax+by+c=0
fn circle_raycast(x: f64, y: f64, radius: f64, a: f64, b: f64, c: f64) -> Option<(f64, f64, f64, f64)> {
    use ::std::f64::EPSILON;
    // println!("x:{}, y:{}, radius:{}, a:{}, b:{}, c:{}",x,y,radius,a,b,c);
    if a == 0. && b == 0. {
        panic!("invalid line equation")
    } else if (a / radius).abs() < EPSILON {
        let y_ray = -c / b;
        if (y_ray - y).abs() < radius {
            let dx = (radius.powi(2) - (y_ray - y).powi(2)).sqrt();
            Some((x - dx, y_ray, x + dx, y_ray))
        } else {
            None
        }
    } else if (b / radius).abs() < EPSILON {
        let x_ray = -c / a;
        if (x_ray - x).abs() < radius {
            let dy = (radius.powi(2) - (x_ray - x).powi(2)).sqrt();
            Some((x_ray, y - dy, x_ray, y + dy))
        } else {
            None
        }
    } else {
        // the equation of intersection abscisse: d*x^2 + e*x + f = 0
        let d = 1. + (a / b).powi(2);
        let e = 2. * (-x + a / b * (c / b + y));
        let f = x.powi(2) + (c / b + y).powi(2) - radius.powi(2);

        let delta = e.powi(2) - 4. * d * f;

        if delta > 0. {
            let (x1, x2) = {
                let x1 = (-e - delta.sqrt()) / (2. * d);
                let x2 = (-e + delta.sqrt()) / (2. * d);
                if x1 > x2 { (x2, x1) } else { (x1, x2) }
            };
            let y1 = (-c - a * x1) / b;
            let y2 = (-c - a * x2) / b;

            Some((x1, y1, x2, y2))
        } else {
            None
        }
    }
}

/// The line of equation ax + by + c = 0 that pass through the two points
fn line_equation_from_points(p: [f64; 2], q: [f64; 2]) -> (f64, f64, f64) {
    let (a, b) = if (p[0] - q[0]).abs() > (p[1] - q[1]).abs() {
        (- (p[1] - q[1]) / (p[0] - q[0]), 1.)
    } else {
        (1., - (p[0] - q[0]) / (p[1] - q[1]))
    };
    let c = - a*p[0] - b*p[1];

    (a, b, c)
}

