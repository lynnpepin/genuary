#![allow(dead_code, unused_variables, unused_mut, unused_imports)]
use nannou::prelude::*;
use rand::prelude::*;
use std::ops::*;
const N: usize = 100;

const MAX_TT: f32 = 10.0;
const MAX_NN: usize = 600;

const WIDTH:  f32 = 720.0;

fn main() {
  nannou::sketch(view)
  .size(WIDTH as u32, WIDTH as u32)
  .run();
}

fn randrange(a: f32, b: f32) -> f32 {
  rand::random::<f32>() * (b - a) - a
}

// Render
fn view(app: &App, frame: Frame) {
  let draw = app.draw();
  draw.background().color(WHITE);

  if app.elapsed_frames() == 0 {
    // "globals"
    let SS = 2048;
    let ds = (1.0/256.0);
    
    // cartesian coordinates
    let (mut x1, mut x2, mut y1, mut y2): (f32, f32, f32, f32);
    // polar coordinates
    let (mut r, mut a): (f32, f32);

    for kx in 0..7 {
      for ky in 0.. 7 {
        // initialize values
        // middle point
        //let middle_x = (kx as f32) * 128.0 - 384.0;
        //let middle_y = (ky as f32) * 128.0 - 384.0;
        let middle_x = WIDTH * ((kx as f32) * (1.0 / 8.0) - 3.0/8.0);
        let middle_y = WIDTH * ((ky as f32) * (1.0 / 8.0) - 3.0/8.0);

        // cumulative bias in da, dr
        let mut da_cum = 0.0;
        let mut dr_cum = 0.0;

        // 'strands' in each spiral
        for li in 0..64 {

          // "globals" for each spiral
          x2 = middle_x;
          y2 = middle_x;
          da_cum = 0.0;
          dr_cum = 0.0;
          
          a = (li as f32) / 24.0; // offset a, to create spiral effect
          r = 0.0;  
          // points in line

          for ss in 0..SS {
            // what was new is now old
            x1 = x2;
            y1 = y2;

            // accumulate da, dr bias
            da_cum += randrange(-2.0, 1.0) * ds * (kx as f32).pow(0.5) / 16.0;
            dr_cum += randrange(-2.0, 1.0) * ds * (ky as f32).pow(0.5) / 16.0;

            // intermittent sharp changes in accumulated bias
            if ss % 64 == 0 {
              da_cum += randrange(-2.0, 1.0) * ds * (kx as f32).pow(0.5) * 4.0;
              dr_cum += randrange(-2.0, 1.0) * ds * (ky as f32).pow(0.5) * 4.0;
            }


            // r, a += dr, da
            r += ds * (5.0 + dr_cum);
            a += ds * (3.0 + da_cum);
            //r += ds * (5.0 + rand::random::<f32>() - 0.5 * (kx as f32 / 32.0));
            //a += ds * (3.0 + rand::random::<f32>() - 0.5 * (ky as f32 / 32.0));

            // convert polar to offset cartesian coordinates
            x2 = r * a.cos() + middle_x;
            y2 = r * a.sin() + middle_y;

            // draw
            draw.line()
              .start(pt2(x1, y1))
              .end(pt2(x2, y2))
              .weight(1.0/4.0)
              .rgba(0.0, 0.0, 0.0, 1.0);
          }
        }
      }
    }

    draw.to_frame(app, &frame).unwrap();

    app.main_window().capture_frame("output.png");
  }
}

// eof