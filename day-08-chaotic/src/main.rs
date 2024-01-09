#![allow(dead_code, unused_variables, unused_mut, unused_imports)]
use nannou::prelude::*;
use rand::prelude::*;

const N: usize = 10000;
//const S: f32   = 100;
//const ds: f32  = 0.01;
const MAX_TT: f32 = 10.0;
const MAX_NN: usize = 600;
const WIDTH:  f32 = 720.0;

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
      (randrange(-WIDTH, WIDTH), randrange(-WIDTH, WIDTH))
    }).collect::<Vec<(f32, f32)>>();

    let mut r: f32;
    let mut a: f32;

    // For each point...
    for ii in 0..N {
      let (mut x, mut y) = (points[ii].0, points[ii].1);
      let ds = 0.001;

      for t in 0..5000 {
        // Iterate a bit
        
        (r, a) = xy_to_polar(x, y);
        r  += -r.sin()*ds;
        a  += -ds/2.0;
        (x, y) = polar_to_xy(r, a);
        x  += y.sin()*ds;
        y  -= y*ds;


        // Draw
        draw
          .ellipse()
          .w(1.0).h(1.0)
          .rgba(0.0, 0.0, 0.0, 0.125)
          .x_y(x, y);
        }
    }
    
    draw.to_frame(app, &frame).unwrap();
    app.main_window().capture_frame("output.png")
  }
}