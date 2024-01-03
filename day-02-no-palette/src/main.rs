#![allow(dead_code, unused_variables, unused_mut, unused_imports)]
use nannou::prelude::*;
use rand::{Rng};
const N: usize = 100;
const MAX_TT: f32 = 20.0;
const MAX_NN: usize = 1200;

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
  dt: f32,
  tt: f32,
  nn: usize,
  mouse_xs: Vec<f32>,
  mouse_ys: Vec<f32>,

  // rgb moves like a particle in a unit cube
  r: f32,
  g: f32,
  b: f32,
  dr: f32,
  dg: f32,
  db: f32,
  ddr: f32,
  ddg: f32,
  ddb: f32,

  // accumulate downward y offsets for drawing staggered parts
  accy0: Vec<f32>,
  accy1: Vec<f32>,
}


// Model initializer
fn model(app: &App) -> Model {
  let boundary = app.window_rect();
  let mut rng = rand::thread_rng();
  let r = 0.5;
  let g = 0.5;
  let b = 0.5;

  let dr =  (rng.gen::<f32>() - 0.5) / 4.0;
  let dg =  (rng.gen::<f32>() - 0.5) / 4.0;
  let db =  (rng.gen::<f32>() - 0.5) / 4.0;

  let ddr = (rng.gen::<f32>() - 0.5);
  let ddg = (rng.gen::<f32>() - 0.5);
  let ddb = (rng.gen::<f32>() - 0.5);

  Model {
    rng: rng,
    tt: 0.0,
    dt: 0.0,
    nn: 0,
    mouse_xs: vec![0.0, 0.0, 0.0],
    mouse_ys: vec![0.0, 0.0, 0.0],

    r: r,
    g: g,
    b: b,

    dr:  dr,
    dg:  dg,
    db:  db,

    ddr: ddr,
    ddg: ddg,
    ddb: ddb,

    accy0: vec![0.0; 10],
    accy1: vec![0.0; 10],
  }
}


// Update model in-place 60 times a second
fn update(app: &App, model: &mut Model, update: Update) {
  let boundary = app.window_rect();

  // incremenet time counter and frame counter
  model.dt = update.since_last.as_secs_f32();
  model.tt += model.dt;
  model.nn += 1;

  // only update mouse_xys, mouse_ys if distance is far enough
  let dist_sq = (app.mouse.x - model.mouse_xs[0]).powi(2) + (app.mouse.y - model.mouse_ys[0]).powi(2);

  if dist_sq > 1024.0 {
    model.mouse_xs = vec![app.mouse.x, model.mouse_xs[0], model.mouse_xs[1]];
    model.mouse_ys = vec![app.mouse.y, model.mouse_ys[0], model.mouse_ys[1]];
    // accumulate downward y offsets
    for ii in 1..model.accy0.len() {
      model.accy0[ii] = model.accy0[ii-1] - model.rng.gen::<f32>()*32.0*((ii as f32).pow(0.5));
      model.accy1[ii] = model.accy1[ii-1] - model.rng.gen::<f32>()*32.0*((ii as f32).pow(0.5));
    }
  }

  // perturb ddr, ddg, ddb
  model.ddr += model.dt * ((model.rng.gen::<f32>() - 0.5)*2.0).clamp(-1.0, 1.0);
  model.ddg += model.dt * ((model.rng.gen::<f32>() - 0.5)*2.0).clamp(-1.0, 1.0);
  model.ddb += model.dt * ((model.rng.gen::<f32>() - 0.5)*2.0).clamp(-1.0, 1.0);

  // update dr, dg, db
  model.dr = (model.dr + 4.0 * model.dt * model.ddr);//.clamp(-0.5, 0.5);
  model.dg = (model.dg + 4.0 * model.dt * model.ddg);//.clamp(-0.5, 0.5);
  model.db = (model.db + 4.0 * model.dt * model.ddb);//.clamp(-0.5, 0.5);

  // update r, g, b
  model.r += 4.0 * model.dr * model.dt;
  if model.r < 0.0 { model.r = 0.0; model.dr *= -1.0; }
  if model.r > 1.0 { model.r = 1.0; model.dr *= -1.0; }

  model.g += 4.0 * model.dg * model.dt;
  if model.g < 0.0 { model.g = 0.0; model.dg *= -1.0; }
  if model.g > 1.0 { model.g = 1.0; model.dg *= -1.0; }

  model.b += 4.0 * model.db * model.dt;
  if model.b < 0.0 { model.b = 0.0; model.db *= -1.0; }
  if model.b > 1.0 { model.b = 1.0; model.db *= -1.0; }

  // quit after a timer
  if model.tt > MAX_TT || model.nn > MAX_NN {
    app.quit();
  }
}
 
// Render
fn view(app: &App, model: &Model, frame: Frame){
  let draw = app.draw();
  
  
  // draw a line as the mouse moves
  draw.line()
    .start(pt2(model.mouse_xs[0], model.mouse_ys[0]))
    .end(pt2(model.mouse_xs[1], model.mouse_ys[1]))
    .weight(4.0)
    .rgba(model.r, model.g, model.b, 1.0);
  

  // draw staggered lines down
  for ii in 1..10 {
    // draw horizontal part
    draw.line()
      .start(pt2(model.mouse_xs[0], model.mouse_ys[0] + model.accy0[ii]))
      .end(pt2(model.mouse_xs[1], model.mouse_ys[1]   + model.accy1[ii]))
      .weight(2.0)
      .rgba(model.r, model.g, model.b, 1.0);
    // draw vertical parts
    // left
    draw.line()
      .start(pt2(model.mouse_xs[0], model.mouse_ys[0] + model.accy0[ii-1]))
      .end(pt2(model.mouse_xs[0], model.mouse_ys[0] + model.accy0[ii]))
      .weight(2.0)
      .rgba(model.r, model.g, model.b, 1.0);
    // right
    draw.line()
      .start(pt2(model.mouse_xs[1], model.mouse_ys[1] + model.accy1[ii-1]))
      .end(pt2(model.mouse_xs[1], model.mouse_ys[1] + model.accy1[ii]))
      .weight(2.0)
      .rgba(model.r, model.g, model.b, 1.0);
     
  }

  /* debug
  if model.nn % 6 == 0 {
    println!(
      "rgb: ({:.1}, {:.1}, {:.1}), drgb: ({:.2}, {:.2}, {:.2}), ddrgb: ({:.3}, {:.3}, {:.3})",
      model.r, model.g, model.b,
      model.dr, model.dg, model.db,
      model.ddr, model.ddg, model.ddb
    );
  }
  */  
  draw.to_frame(app, &frame).unwrap();

  // output frame to "output_{nn}.png"
  let file_path = "output/".to_string() + &model.nn.to_string() + ".png";
  app.main_window().capture_frame(file_path);
}

// eof
