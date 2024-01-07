#![allow(dead_code, unused_variables, unused_mut, unused_imports)]
use nannou::prelude::*;
use rand::prelude::*;
use std::ops::*;
const N: usize = 100;

const MAX_TT: f32 = 10.0;
const MAX_NN: usize = 600;

fn main() {
  nannou::sketch(view)
  .size(512, 512)
  .run();
}
 
fn bounceval(v: f32, max: f32) -> f32 {
  (v % max) 
  * (((v as i32 / max as i32) % 2) as f32).mul(-2.0).add(1.0)
  + (((v as i32 / max as i32) % 2) as f32).mul(max)
}

// Render
fn view(app: &App, frame: Frame) {
  let draw = app.draw();
  //draw.background().color(BLACK);
  draw.text("i don't wanna do this one today")
    .x_y(
      bounceval(2.6 * app.elapsed_frames() as f32, 512.0) - 256.0,
      bounceval(1.6 * app.elapsed_frames() as f32, 512.0) - 256.0,
    )
    .font_size(48)
    .hsv(
      (app.elapsed_frames() as f32 / 120.0) % 1.0,
      1.0,
      1.0
    );
  draw.to_frame(app, &frame).unwrap();

  
  app.main_window().capture_frame(
    format!{"output/{}.png", app.elapsed_frames()}
  );
  
}