#![allow(dead_code, unused_variables, unused_mut, unused_imports)]
use nannou::prelude::*;
use rand::prelude::*;
const N: usize = 100;

const MAX_TT: f32 = 10.0;
const MAX_NN: usize = 600;

const WIDTH: f32 = 240.0;
const R1: f32 = WIDTH / 3.0;
const FRAMES: f32 = 360.0;

fn main() {
  nannou::sketch(view)
  .size(WIDTH as u32, WIDTH as u32)
  .run();
}
 

// Render
fn view(app: &App, frame: Frame) {
  let draw = app.draw();
  draw.background().color(WHITE);
  //println!("{}", app.elapsed_frames());
  let frames: f32 = app.elapsed_frames() as f32 % FRAMES;
  let period: f32 = (frames / FRAMES).fract();
  let half_period: f32 = (period * 2.0).fract();

  draw.ellipse()
    .x_y(0.0, 0.0)
    .w_h(R1, R1)
    .color(
      BLACK
      //if period < 0.5 { BLACK } else { WHITE }
    );

    
  let R2: f32 = (frames / FRAMES).clamp(0.0, 1.0) * R1;
  let Y2: f32 = R1 - R2;
  let angle = (frames) / FRAMES * PI * 4.0;


  draw.ellipse()
    .x_y(Y2*angle.sin(), Y2*angle.cos())
    .w_h(R2, R2)
    .color(
      WHITE
      //if period < 0.5 { WHITE } else { BLACK }
    );

  draw.to_frame(app, &frame).unwrap();
  app.main_window().capture_frame(format!{"output/{}.png", app.elapsed_frames()});
}
