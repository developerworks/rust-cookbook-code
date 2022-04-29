extern crate ndarray;
use ndarray::Array;

fn main() {
  let a = Array::from_vec(vec![1., 2., 3., 4., 5.]);
  let b = Array::from_vec(vec![5., 4., 3., 2., 1.]);
  let mut c = Array::from_vec(vec![1., 2., 3., 4., 5.]);
  let mut d = Array::from_vec(vec![5., 4., 3., 2., 1.]);

  let z = a + b;
  let w =  &c + &d;

  let epsilon = 1e-8;
  for elem in z.iter() {
    let diff: f32 = *elem - 6.;
    assert!(diff.abs() < epsilon);
  }

  println!("c = {}", c);
  c[0] = 10.;
  d[1] = 10.;

  for elem in w.iter() {
    let diff: f32 = *elem - 6.;
    assert!(diff.abs() < epsilon);
  }

}
