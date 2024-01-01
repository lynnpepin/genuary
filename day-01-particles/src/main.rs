#![allow(dead_code, unused_variables)]
use nannou::prelude::*;
use rand::{Rng};

const N: usize = 100;

fn main() {
  nannou::app(model)
    .update(update)
    .simple_window(view)
    .run();
}

// The state of our program
#[derive(Debug)]
struct Model {
  // Hold x/y dx/dy for each item
  x:  Vec<f32>,
  y:  Vec<f32>,
  dx: Vec<f32>,
  dy: Vec<f32>
}

// Model initializer
fn model(app: &App) -> Model {
  let boundary = app.window_rect();
  let mut rng = rand::thread_rng();

  Model {
    x: (0..N).map(|_| rng.gen_range(boundary.left()..boundary.right())).collect(),
    y: (0..N).map(|_| rng.gen_range(boundary.bottom()..boundary.top())).collect(),
    dx: (0..N).map(|_| rng.gen_range(-2.0..2.0)).collect(),
    dy: (0..N).map(|_| rng.gen_range(-2.0..2.0)).collect(),
  }
}

// Update model in-place 60 times a second
fn update(app: &App, model: &mut Model, update: Update) {

  let boundary = app.window_rect();

  for ii in 0..N {
    model.x[ii] += model.dx[ii];
    model.y[ii] += model.dy[ii];
    model.dy[ii] += -0.1;

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
  }
}

// Render
fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();

    draw.background().color(BLACK);
    for ii in 0..N {
      draw.ellipse().w(6.0).h(6.0).color(WHITE).x_y(model.x[ii], model.y[ii]); 
    }
    draw.to_frame(app, &frame).unwrap();
}