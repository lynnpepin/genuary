#![allow(dead_code, unused_variables)]
use nannou::prelude::*;
use rand::{Rng};

const MAX_TT: f64 = 3.0;
const MAX_NN: usize = 300;

const N: usize = 5000;

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
  // Hold x/y dx/dy for each item
  x:  Vec<f32>,
  y:  Vec<f32>,
  dx: Vec<f32>,
  dy: Vec<f32>,
  rng: rand::rngs::ThreadRng,
  tt: f64,
  nn: usize,
}

// Model initializer
fn model(app: &App) -> Model {
  let boundary = app.window_rect();
  let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

  Model {
    x: (0..N).map(|_| rng.gen_range(boundary.left()..boundary.right())).collect(),
    y: (0..N).map(|_| rng.gen_range(boundary.bottom()..boundary.top())).collect(),
    dx: (0..N).map(|_| rng.gen_range(-1.0..1.0)).collect(),
    dy: (0..N).map(|_| rng.gen_range(-1.0..1.0)).collect(),
    rng: rng,
    tt: 0.0,
    nn: 0
  }
}

// Update model in-place 60 times a second
fn update(app: &App, model: &mut Model, update: Update) {
  // incremenet time counter and frame counter
  model.tt += update.since_last.secs();
  model.nn += 1;


  let boundary = app.window_rect();

  for ii in 0..N {
    model.x[ii] += model.dx[ii];
    model.y[ii] += model.dy[ii];
    // random change in direction each unit time

    model.dx[ii] += model.rng.gen_range(-0.125..0.125);
    model.dy[ii] += model.rng.gen_range(-0.125..0.125);

    // downward biased for y
    model.dy[ii] += -0.25;

    
    // bounce off walls
    if model.x[ii] < boundary.left() {
      model.dx[ii] *= -1.0;
      model.x[ii] = boundary.left()
    } else if model.x[ii] > boundary.right() {
      model.dx[ii] *= -1.0;
      model.x[ii] = boundary.right()
    } 

    if model.y[ii] < boundary.bottom() {
      model.dy[ii] *= -1.0;
      model.y[ii] = boundary.bottom()
    } else if model.y[ii] > boundary.top() {
      model.dy[ii] *= -1.0;
      model.y[ii] = boundary.top()
    } 

    /* more performant way to write above:
    model.x[ii] = model.x[ii].max(boundary.left()).min(boundary.right());
    */
  }

  // quit if tt > MAX_TT or nn > MAX_NN
  if model.tt > MAX_TT || model.nn > MAX_NN {
    app.quit();
  }
}

// Render
fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();

    draw.background().color(BLACK);
    for ii in 0..N {

      draw.ellipse().w(3.0).h(3.0).rgba(2.0, 2.0, 2.0, 0.125).x_y(model.x[ii], model.y[ii]); 
    }
    draw.to_frame(app, &frame).unwrap();
    // for transparent white in .color above, use .rgba(1.0, 1.0, 1.0, 0.1)

    // same frame to "output_{nn}.png"
    let file_path = "output/".to_string() + &model.nn.to_string() + ".png";
    app.main_window().capture_frame(file_path);
  
}