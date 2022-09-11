//! A spinning text cube
//! 
//! 
//! 4    +------+  6
//!     /|     /| 
//! 5  +------+ |  7
//!    | |    | | 
//! 0  | +----|-+  2
//!    |/     |/   
//! 1  +------+    3


#[derive(Debug, Clone, Copy)]
struct Matrix([[f32; 4]; 4]);

#[derive(Debug, Clone, Copy)]
struct Vector([f32; 4]);

const VERTICES : [Vector; 8] = [
    Vector([-1.0, -1.0, -1.0, 1.0]),
    Vector([-1.0, -1.0,  1.0, 1.0]),
    Vector([ 1.0, -1.0, -1.0, 1.0]),
    Vector([ 1.0, -1.0,  1.0, 1.0]),
    Vector([-1.0,  1.0, -1.0, 1.0]),
    Vector([-1.0,  1.0,  1.0, 1.0]),
    Vector([ 1.0,  1.0, -1.0, 1.0]),
    Vector([ 1.0,  1.0,  1.0, 1.0]),
];

const FACES : [[u8; 4]; 6] = [
    [1, 5, 7, 3],
    [3, 7, 6, 2],
    [0, 4, 5, 1],
    [2, 6, 4, 0],
    [0, 1, 3, 2],
    [5, 4, 6, 7],
];

fn matrix_times_vector(m: &Matrix, v: &Vector) -> Vector {
    let [mx, my, mz, mw] = &m.0;
    let [x, y, z, w] = v.0;
    // The product is the weighted sum of the columns.
    Vector([
        x * mx[0] + y * my[0] + z * mz[0] + w * mw[0],
        x * mx[1] + y * my[1] + z * mz[1] + w * mw[1],
        x * mx[2] + y * my[2] + z * mz[2] + w * mw[2],
        x * mx[3] + y * my[3] + z * mz[3] + w * mw[3],
    ])
}

const SCREEN_WIDTH : usize = 80;
const SCREEN_HEIGHT : usize = 40;
const OFFSET_X : f32 = SCREEN_WIDTH as f32 * 0.5;
const OFFSET_Y : f32 = SCREEN_HEIGHT as f32 * 0.5;
const SCALE_X : f32 = SCREEN_WIDTH as f32 * 0.5;
const SCALE_Y : f32 = SCREEN_HEIGHT as f32 * 0.5;



fn main() {
    for frame_number in 0.. {
        let mut frame = [[b' ';SCREEN_WIDTH]; SCREEN_HEIGHT];

        let t = frame_number as f32 * 0.01;
        let (c, s) = (t.cos(), t.sin());

        let cube_to_world = Matrix([
            // Each row is a column of a matrix.
            [  c, 0.0,   s, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [ -s, 0.0,   c, 0.0],
            [0.0, 0.0,-2.5, 1.0],
        ]);


        let mut screen_pos = [[0.0, 0.0]; 8];
        for (v, s) in VERTICES.iter().zip(screen_pos.iter_mut()) {
            let world_pos = matrix_times_vector(&cube_to_world, v);
            let recip_z = 1.0 /  world_pos.0[2];
            let screen_x = world_pos.0[0] * recip_z * SCALE_X + OFFSET_X;
            let screen_y = world_pos.0[1] * recip_z * SCALE_Y + OFFSET_Y;
            *s = [screen_x, screen_y];
            // frame[screen_y as usize][screen_x as usize] = b'.';
        }

        for face in FACES {
            if !cull(screen_pos[face[0] as usize], screen_pos[face[1] as usize], screen_pos[face[2] as usize]) {
                let mut end = face[3];
                for start in face {
                    draw_line(&mut frame, screen_pos[start as usize], screen_pos[end as usize]);
                    end = start;
                }
            }
        }

        for l in 0..SCREEN_HEIGHT {
            let row = std::str::from_utf8(&frame[l]).unwrap();
            println!("{}", row);
        }

        print!("\x1b[{}A;", SCREEN_HEIGHT);

        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}

fn cull(p0: [f32; 2], p1: [f32; 2], p2: [f32; 2]) -> bool {
    let dx = [p1[0] - p0[0], p2[0] - p1[0]];
    let dy = [p1[1] - p0[1], p2[1] - p1[1]];
    dx[0] * dy[1] > dx[1] * dy[0]
}


fn draw_line(frame: &mut [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT], start: [f32; 2], end: [f32; 2]) {
    let [x0, y0] = start;
    let [x1, y1] = end;
    let [dx, dy] = [x1 - x0, y1 - y0];
    if dy.abs() > dx.abs() {
        let ymin = y0.min(y1);
        let ymax = y0.max(y1);
        let iymin = ymin.ceil() as usize;
        let iymax = ymax.ceil() as usize;
        let dxdy = dx / dy;
        for iy in iymin..iymax {
            let ix = ((iy as f32 - y0) * dxdy + x0) as usize;
            frame[iy][ix] = b'|';
        }
    } else {
        let xmin = x0.min(x1);
        let xmax = x0.max(x1);
        let ixmin = xmin.ceil() as usize;
        let ixmax = xmax.ceil() as usize;
        let dydx = dy / dx;
        for ix in ixmin..ixmax {
            let iy = ((ix as f32 - x0) * dydx + y0) as usize;
            frame[iy][ix] = b'-';
        }
    }
}
