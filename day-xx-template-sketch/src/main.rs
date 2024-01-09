#![allow(dead_code, unused_variables, unused_mut, unused_imports)]
use nannou::prelude::*;
use rand::prelude::*;
const N: usize = 100;

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
  rand::random::<f32>() * (b - a) - a
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
  draw.quad()
    .x_y(app.mouse.x, app.mouse.y)
    .w_h(4.0, 4.0)
    .color(WHITE);
  draw.to_frame(app, &frame).unwrap();

  // animation
  //app.main_window().capture_frame(format!{"output/{}.png", app.elapsed_frames()});

  // single frame
  //if (app.elapsed_frames() == 0) {app.main_window().capture_frame("output.png")}
}