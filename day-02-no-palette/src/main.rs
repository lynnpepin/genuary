#![allow(dead_code, unused_variables, unused_mut, unused_imports)]
use nannou::prelude::*;
use rand::{Rng};
const N: usize = 100;

const MAX_TT: f64 = 10.0;
const MAX_NN: usize = 600;

fn main() {
  nannou::app(model)
    .update(update)
    .simple_window(view)
    .size(512, 512)
    .run();
}

// The state of our program
#[derive(Debug)]
struct Model {
  rng: rand::rngs::ThreadRng,
  tt: f64,
  nn: usize,
  x: f32,
  y: f32,
}


// Model initializer
fn model(app: &App) -> Model {
  let boundary = app.window_rect();
  Model {
    rng: rand::thread_rng(),
    tt: 0.0,
    nn: 0,
    x: 0.0,
    y: 0.0,
  }
}


// Update model in-place 60 times a second
fn update(app: &App, model: &mut Model, update: Update) {
  let boundary = app.window_rect();

  // incremenet time counter and frame counter
  model.tt += update.since_last.secs();
  model.nn += 1;

  // quit if tt > MAX_TT or nn > MAX_NN
  /*
  if model.tt > MAX_TT || model.nn > MAX_NN {
    app.quit();
  }
  */
}

 
// Render
fn view(app: &App, model: &Model, frame: Frame){
  let draw = app.draw();

  draw.ellipse()
    .w(3.0).h(30.0)
    .rgba(10.0, 10.0, 10.0, 1.0)
    .x_y(app.mouse.x, app.mouse.y);

  
  draw.to_frame(app, &frame).unwrap();

  // optional: output frame to "output_{nn}.png"
  //let file_path = "output/".to_string() + &model.nn.to_string() + ".png";
  //app.main_window().capture_frame(file_path);
}

// eof

/*
todos


*/