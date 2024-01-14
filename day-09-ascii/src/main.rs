#![allow(dead_code, unused_variables, unused_mut, unused_imports)]
use nannou::prelude::*;
use rand::{Rng};

const N: usize = 80;
const WIDTH: f32 = 720.0;

const MAX_TT: f64 = 10.0;
const MAX_NN: usize = 600;

// Consts specific to this
const ks: f32 = 10.0;  // spring constant
const kd: f32 = 0.5; // spring dampening constant

const kp: f32 = 0.0005; // chance of raindrop
const kr: f32 = 10000.00; // force of a raindrop
const dt: f32 = 1./60.; // deltatime

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
  x:    [f32; N], // position
  dx:   [f32; N], // velocity
  ddx:  [f32; N], // acceleration
}


// Model initializer
fn model(app: &App) -> Model {
  let boundary = app.window_rect();
  Model {
    rng: rand::thread_rng(),
    tt: 0.0,
    nn: 0,
    x:    [0.0; N],
    dx:   [0.0; N],
    ddx:  [0.0; N],
  }
}


// Update model in-place 60 times a second
fn update(app: &App, model: &mut Model, update: Update) {
  let boundary = app.window_rect();

  // incremenet time counter and frame counter
  model.tt += update.since_last.secs();
  model.nn += 1;

  let mut raini: f32;

  for ii in 0..N {
    model.ddx[ii] = -ks * model.x[ii];
    raini = model.rng.gen::<f32>();
    if model.rng.gen::<f32>() < kp {
      model.ddx[ii] -= kr
    };

    model.dx[ii] += model.ddx[ii] * dt;
    model.x[ii]  += model.dx[ii]  * dt * kd;

  }

  // quit if tt > MAX_TT or nn > MAX_NN
  //if model.tt > MAX_TT || model.nn > MAX_NN { app.quit(); }

}


 
// Render
fn view(app: &App, model: &Model, frame: Frame){
  let draw = app.draw();
  //if model.nn <=2 { draw.background().color(WHITE); }
  draw.background().color(WHITE);
  draw.quad()
    .x_y(0.0, 0.0)
    .w_h(WIDTH, WIDTH)
    .rgba(1.0, 1.0, 1.0, 0.01);

  // draw points
  let mut x: f32;
  for ii in 0..N {
    // distance between points = WIDTH / (N + 1)
    x = WIDTH * (-0.5 + ((ii+1) as f32) / ((N as f32) + 1.0));

    // model.x = y displacement
    draw.line()
      .start(pt2(x, 0.0))
      .end(pt2(x, model.x[ii]))
      .weight(1.0)
      .color(BLACK);

    draw.ellipse()
      .x_y(x, model.x[ii])
      .radius(2.0)
      .color(BLACK);
  }

  // debug
  draw.to_frame(app, &frame).unwrap();
  
  // optional: output frame to "output_{nn}.png"
  //let file_path = "output/".to_string() + &model.nn.to_string() + ".png";
  //app.main_window().capture_frame(file_path);

}

// eof