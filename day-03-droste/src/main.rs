#![allow(dead_code, unused_variables, unused_mut, unused_imports)]
use nannou::prelude::*;
use rand::prelude::*;
use rand::{Rng};
use std::ops::*;
const N: usize = 100;

const MAX_TT: f32 = 10.0;
const MAX_NN: usize = 600;

fn main() {
  nannou::sketch(view)
  .size(512, 512)
  .run();
}
 
// Render
fn view(app: &App, frame: Frame) {
  let draw = app.draw();

  let (mut ax, mut ay) = (-256.0,  -256.0);
  let (mut bx, mut by) = ( 256.0,  256.0);
  let (mut mid_x, mut mid_y);
  let (    mx,     my) = (app.mouse.x, app.mouse.y);

  if app.elapsed_frames() % 24 == 0 {
    for ii in 0..7 {
      (mid_x, mid_y) = ((ax+bx)/2.0, (ay+by)/2.0);
      if mx < mid_x { bx = mid_x; } else { ax = mid_x; }
      if my < mid_y { by = mid_y; } else { ay = mid_y; }

      // draw mesh
      draw.quad()
        .x_y(mid_x, mid_y)
        .w_h(bx-ax, by-ay)
        .hsva(
          1.0,
          0.0,
          rand::random::<f32>().mul(2.0).round().div(2.0),
          0.5
        );
    }
  }
  draw.to_frame(app, &frame).unwrap();

  app.main_window().capture_frame(format!{"output/{}.png", app.elapsed_frames()});

}