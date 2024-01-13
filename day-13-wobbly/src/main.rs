#![allow(dead_code, unused_variables, unused_mut, unused_imports)]
use nannou::prelude::*;
use rand::{Rng};

const N: usize = 80;
const WIDTH: f32 = 720.0;

const MAX_TT: f64 = 10.0;
const MAX_NN: usize = 600;

// Useful functions I like
fn xy_to_polar(x: f32, y: f32) -> (f32, f32) {
  ((x.pow(2.0) + y.pow(2.0)).sqrt(), y.atan2(x))
}

fn polar_to_xy(r: f32, theta: f32) -> (f32, f32) {
  (r * theta.cos(), r * theta.sin())
}

fn main() {
  nannou::app(model)
    .update(update)
    .simple_window(view)
    .size(WIDTH as u32, WIDTH as u32)
    .run();
}

// The state of our program
#[derive(Debug)]
struct Model {
  rng: rand::rngs::ThreadRng,
  tt: f64,
  nn: usize,
  Y:  [f32; N], // position
  V:  [f32; N], // velocity
  A:  [f32; N], // acceleration
  K:  [i64; N], // kinetic energy
  P:  [i64; N], // potential energy
}


// Model initializer
fn model(app: &App) -> Model {
  let boundary = app.window_rect();
  Model {
    rng: rand::thread_rng(),
    tt: 0.0,
    nn: 0,
    Y:  [0.0; N],
    V:  [0.0; N],
    A:  [0.0; N],
    K:  [0; N],
    P:  [0; N],
  }
}


// Update model in-place 60 times a second
fn update(app: &App, model: &mut Model, update: Update) {
  let boundary = app.window_rect();

  // incremenet time counter and frame counter
  model.tt += update.since_last.secs();
  model.nn += 1;

  for ii in 0..N {
    model.Y[ii] = (
        ((ii as f32 + model.tt as f32 * 1.5 )*1.5).sin() * (WIDTH / 3.142)
      + ((ii as f32 + model.tt as f32       )/2.0).sin() * (WIDTH / 2.718)
      + ((ii as f32 + model.tt as f32 * 0.5 )/3.2).sin() * (WIDTH / 1.414)
    ) / 4.0;
  }

  // quit if tt > MAX_TT or nn > MAX_NN
  if model.tt > MAX_TT || model.nn > MAX_NN { app.quit(); }

}


 
// Render
fn view(app: &App, model: &Model, frame: Frame){
  let draw = app.draw();
  if model.nn <=2 { draw.background().color(WHITE); }
  //draw.background().rgba(1.0, 1.0, 1.0, 0.01);
  draw.quad()
    .x_y(0.0, 0.0)
    .w_h(WIDTH, WIDTH)
    .rgba(1.0, 1.0, 1.0, 0.01);

  // draw points
  let mut x: f32;
  for ii in 0..N {
    // distance between points = WIDTH / (N + 1)
    x = WIDTH * (-0.5 + ((ii+1) as f32) / ((N as f32) + 1.0));

    draw.line()
      .start(pt2(x, 0.0))
      .end(pt2(x, model.Y[ii]))
      .weight(1.0)
      .color(BLACK);

    draw.ellipse()
      .x_y(x + ((model.tt*(8./7.)).sin() as f32)*(WIDTH as f32 / N as f32), model.Y[ii])
      .radius(2.0)
      .color(BLACK);
  }

  draw.to_frame(app, &frame).unwrap();

  
  // optional: output frame to "output_{nn}.png"
  let file_path = "output/".to_string() + &model.nn.to_string() + ".png";
  app.main_window().capture_frame(file_path);

}

// eof