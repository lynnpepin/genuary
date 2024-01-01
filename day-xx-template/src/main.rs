#![allow(dead_code, unused_variables, unused_mut, unused_imports)]
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

}


// Model initializer
fn model(app: &App) -> Model {
  let boundary = app.window_rect();
  let mut rng = rand::thread_rng();
  Model {

  }
}


// Update model in-place 60 times a second
fn update(app: &App, model: &mut Model, update: Update) {
  let boundary = app.window_rect();
  let mut rng = rand::thread_rng();
}


 
// Render
fn view(app: &App, model: &Model, frame: Frame){
  let draw = app.draw();
  draw.background().color(BLACK);
  draw.to_frame(app, &frame).unwrap();
}

// eof