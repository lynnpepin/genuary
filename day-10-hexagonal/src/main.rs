#![allow(dead_code, unused_variables, unused_mut, unused_imports)]
use nannou::prelude::*;
use rand::prelude::*;

const N: usize = 4000;
const T: usize = 2000;
//const S: f32   = 100;
const ds: f32  = 0.001;
const MAX_TT: f32 = 10.0;
const MAX_NN: usize = 600;
const WIDTH:  f32 = 720.0;

const TAU: f32 = std::f32::consts::PI * 2.0;

// Functions I like
fn xy_to_polar(x: f32, y: f32) -> (f32, f32) {
  ((x.pow(2.0) + y.pow(2.0)).sqrt(), y.atan2(x))
}

fn polar_to_xy(r: f32, theta: f32) -> (f32, f32) {
  (r * theta.cos(), r * theta.sin())
}

fn randrange(a: f32, b: f32) -> f32 {
  rand::random::<f32>() * (b - a) + a
}

fn main() {
  nannou::sketch(view)
  .size(WIDTH as u32, WIDTH as u32)
  .run();
}
 
// Render
fn view(app: &App, frame: Frame) {
  let draw = app.draw();
  draw.background().color(WHITE);
  if (app.elapsed_frames() == 0) {
    // starting points
    let mut points = (0..N).map(|_| {
      (randrange(-WIDTH*0.75, WIDTH*0.75), randrange(-WIDTH*0.75, WIDTH*0.75))
    }).collect::<Vec<(f32, f32)>>();

    let mut r: f32;
    let mut a: f32;
    let mut x: f32;
    let mut y: f32;

    let mut dx: f32;
    let mut dy: f32;

    // For each N point
    for ii in 0..N {
      (x, y) = (points[ii].0, points[ii].1);

      // For each timestep t
      for t in 0..T {

        // Advance around the center of the hexagon
        (r, a) = xy_to_polar(x, y);


        match (6.0 * (((a+TAU) % TAU) / TAU)).floor() {
          0.0 => { dx =  -1.0; dy =  3.0.sqrt() }
          1.0 => { dx =  -2.0; dy =  0.0 }
          2.0 => { dx =  -1.0; dy = -3.0.sqrt() }
          3.0 => { dx =   1.0; dy = -3.0.sqrt() }
          4.0 => { dx =   2.0; dy =  0.0 }
          5.0 => { dx =   1.0; dy =  3.0.sqrt() }
          6.0 => { dx =  -1.0; dy =  3.0.sqrt() }
          _   => { dx =  -1.0; dy =  3.0.sqrt() }
        }

        if rand::random::<f32>() < 0.01 {
          dx = dx + dy; // new dx = dx + dy
          dy = dx - dy; // new dy = dx + dy - dy = dx
          dx = dx - dy; // new dx = dx + dy - dx = dy

          if rand::random::<f32>() < 0.5 {
            dx = dx * -1.0;
          } else {
            dy = dy * -1.0;
          }
        }

        x += (dx + randrange(-0.2, 0.2)) * (r + 6.0) * ds * randrange(0.0, 1.0);
        y += (dy + randrange(-0.2, 0.2)) * (r + 6.0) * ds * randrange(0.0, 1.0);



        // Draw
        draw
          .ellipse()
          .w(1.0).h(1.0)
          .rgba(0.0, 0.0, 0.0, 0.0625)
          .x_y(x, y);
        }
    }
    draw.to_frame(app, &frame).unwrap();

    app.main_window().capture_frame("output.png");
  }
}