#![allow(dead_code, unused_variables, unused_mut, unused_imports)]
use nannou::prelude::*;
use rand::{Rng};
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
  draw.background().color(BLACK);
  draw.quad()
    .x_y(app.mouse.x, app.mouse.y)
    .w_h(4.0, 4.0)
    .color(WHITE);
  draw.to_frame(app, &frame).unwrap();
}