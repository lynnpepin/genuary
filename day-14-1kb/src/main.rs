use nannou::prelude::*;
use rand::prelude::*;
fn H(x: f32, y: f32) -> (f32, f32) {((x.pow(2.0) + y.pow(2.0)).sqrt(), y.atan2(x))}
fn G(r: f32, theta: f32) -> (f32, f32) {(r * theta.cos(), r * theta.sin())}
fn R(a:f32,b:f32) -> f32 {rand::random::<f32>()*(b-a)+a}
fn main(){nannou::sketch(v).size(720,720).run();}
fn v(A: &App, F: Frame) {
  let B = A.draw();
  B.background().color(WHITE);
  if (A.elapsed_frames()==0){
    let mut P = (0..999).map(|_|{(R(-1.,1.)*720., R(-1., 1.)*720.)}).collect::<Vec<(f32, f32)>>();
    let (mut r, mut a, mut x, mut y): (f32, f32, f32, f32);
    let D=0.001;
    for i in 0..999 {
      (x, y)=(P[i].0, P[i].1);
      for t in 0..9999 {
        (r, a) = H(x, y);
        r  += -r.cos()*D*R(0.,2.);
        a  += D*(R(-1.,1.) -0.1);
        (x, y) = G(r, a);
        x  -= y*D*R(0.,1.);
        y  += y.cos()*D*R(0.,4.);
        B.ellipse().w(2.).h(2.).rgba(0., 0., 0., 0.125).x_y(x, y);}}
    B.to_frame(A, &F).unwrap();A.main_window().capture_frame("o.png")
  }
}