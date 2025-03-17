use crate::window_manager::Window;
use core::f32;

#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Line3D {
    pub from: Point3D,
    pub to: Point3D,
}

pub struct Object3D {
    pub lines: &'static [Line3D],
}

pub struct Camera {
    pub position: Point3D,
    pub rotation: (f32, f32, f32),
    pub scale: f32,
    pub distance: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Point3D { x: 0.0, y: 0.0, z: -5.0 },
            rotation: (0.0, 0.0, 0.0),
            scale: 1.0,
            distance: 5.0,
        }
    }
}

pub fn create_cube() -> Object3D {
    static LINES: [Line3D; 12] = [
        Line3D { from: Point3D { x: -1.0, y: -1.0, z: 1.0 }, to: Point3D { x: 1.0, y: -1.0, z: 1.0 } },
        Line3D { from: Point3D { x: 1.0, y: -1.0, z: 1.0 }, to: Point3D { x: 1.0, y: 1.0, z: 1.0 } },
        Line3D { from: Point3D { x: 1.0, y: 1.0, z: 1.0 }, to: Point3D { x: -1.0, y: 1.0, z: 1.0 } },
        Line3D { from: Point3D { x: -1.0, y: 1.0, z: 1.0 }, to: Point3D { x: -1.0, y: -1.0, z: 1.0 } },
        
        Line3D { from: Point3D { x: -1.0, y: -1.0, z: -1.0 }, to: Point3D { x: 1.0, y: -1.0, z: -1.0 } },
        Line3D { from: Point3D { x: 1.0, y: -1.0, z: -1.0 }, to: Point3D { x: 1.0, y: 1.0, z: -1.0 } },
        Line3D { from: Point3D { x: 1.0, y: 1.0, z: -1.0 }, to: Point3D { x: -1.0, y: 1.0, z: -1.0 } },
        Line3D { from: Point3D { x: -1.0, y: 1.0, z: -1.0 }, to: Point3D { x: -1.0, y: -1.0, z: -1.0 } },
        
        Line3D { from: Point3D { x: -1.0, y: -1.0, z: 1.0 }, to: Point3D { x: -1.0, y: -1.0, z: -1.0 } },
        Line3D { from: Point3D { x: 1.0, y: -1.0, z: 1.0 }, to: Point3D { x: 1.0, y: -1.0, z: -1.0 } },
        Line3D { from: Point3D { x: 1.0, y: 1.0, z: 1.0 }, to: Point3D { x: 1.0, y: 1.0, z: -1.0 } },
        Line3D { from: Point3D { x: -1.0, y: 1.0, z: 1.0 }, to: Point3D { x: -1.0, y: 1.0, z: -1.0 } },
    ];
    
    Object3D { lines: &LINES }
}

fn get_line_char(dx: f32, dy: f32) -> u8 {
    let slope = if dx.abs() < 0.001 { 1000.0 } else { dy / dx };
    
    if slope.abs() < 0.5 {
        b'-'
    } else if slope.abs() > 2.0 {
        b'|'
    } else if slope > 0.0 {
        b'\\'
    } else {
        b'/'
    }
}

fn sin(x: f32) -> f32 {
    let x = x % (2.0 * 3.14159);
    let mut result = 0.0;
    let mut term = x;
    let mut i = 1;
    
    for _ in 0..5 {
        result += term;
        i += 2;
        term = -term * x * x / ((i-1) * i) as f32;
    }
    
    result
}

fn cos(x: f32) -> f32 {
    sin(x + 3.14159 / 2.0)
}

pub struct Renderer3D {
    pub camera: Camera,
    pub rotation: (f32, f32, f32),
}

impl Renderer3D {
    pub fn new() -> Self {
        Renderer3D {
            camera: Camera::new(),
            rotation: (0.0, 0.0, 0.0),
        }
    }
    
    fn rotate_point(&self, point: Point3D) -> Point3D {
        let (rot_x, rot_y, rot_z) = self.rotation;
        
        let cos_x = cos(rot_x);
        let sin_x = sin(rot_x);
        let y1 = point.y * cos_x - point.z * sin_x;
        let z1 = point.y * sin_x + point.z * cos_x;
        
        let cos_y = cos(rot_y);
        let sin_y = sin(rot_y);
        let x2 = point.x * cos_y + z1 * sin_y;
        let z2 = -point.x * sin_y + z1 * cos_y;
        
        let cos_z = cos(rot_z);
        let sin_z = sin(rot_z);
        let x3 = x2 * cos_z - y1 * sin_z;
        let y3 = x2 * sin_z + y1 * cos_z;
        
        Point3D { x: x3, y: y3, z: z2 }
    }
    
    fn project_point(&self, point: Point3D) -> (usize, usize) {
        let rotated_point = self.rotate_point(point);
        let z_offset = rotated_point.z + self.camera.distance;
        
        let scale = self.camera.scale / z_offset.max(0.1);
        let screen_x = ((rotated_point.x * scale) + 1.0) * 0.5 * 46.0 + 2.0;
        let screen_y = ((rotated_point.y * scale) + 1.0) * 0.5 * 16.0 + 2.0;
        
        (screen_x as usize, screen_y as usize)
    }
    
    fn draw_line(&self, window: &Window, from: (usize, usize), to: (usize, usize)) {
        let (x0, y0) = from;
        let (x1, y1) = to;
        
        let dx = (x1 as f32) - (x0 as f32);
        let dy = (y1 as f32) - (y0 as f32);
        let char_to_use = get_line_char(dx, dy);
        
        let steps = dx.abs().max(dy.abs()) as usize + 1;
        for i in 0..steps {
            let t = (i as f32) / (steps as f32);
            let x = x0 as f32 + dx * t;
            let y = y0 as f32 + dy * t;
            
            let char_array = [char_to_use];
            let c_str = core::str::from_utf8(&char_array).unwrap_or("");
            window.print_at(x as usize, y as usize, c_str);
        }
    }
    
    pub fn render_object(&self, window: &Window, object: &Object3D) {
        for line in object.lines {
            let from_2d = self.project_point(line.from);
            let to_2d = self.project_point(line.to);
            
            if line.from.z + self.camera.distance > 0.0 && 
               line.to.z + self.camera.distance > 0.0 {
                self.draw_line(window, from_2d, to_2d);
            }
        }
    }
    
    pub fn rotate_camera(&mut self, pitch: f32, yaw: f32) {
        let (current_pitch, current_yaw, current_roll) = self.camera.rotation;
        self.camera.rotation = (current_pitch + pitch, current_yaw + yaw, current_roll);
    }
    
    pub fn rotate(&mut self, dx: f32, dy: f32, dz: f32) {
        let (rx, ry, rz) = self.rotation;
        self.rotation = (rx + dx, ry + dy, rz + dz);
    }
}