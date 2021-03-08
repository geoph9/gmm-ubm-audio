// Performs MAP adaptation on the given UBM model.

use super::mixtures::Mixture;
use ndarray::{Array2, Array1, Axis};
use std::borrow::Borrow;

fn gmm_post(mix: &Mixture, data: Array2<f64>) -> (&Array2<f64>, &Array2<f64>) {
    let n = data.shape()[0];
    // Check a and the return value of gmm_activ
    let a: Array2<f64> = gmm_activ(mix, data);
    let tmp: Array2<f64> = Array2::ones((n, 1));
    let mut post: Array2<f64> = tmp.dot(mix.priors().expect("Priors undefined...").borrow()) * a;
    let mut s: Array1<f64> = post.sum_axis(Axis(1));
    if s.iter().any(|e| e == 0) {
        s.mapv_inplace(|e| {
            if e == 0f64 {
                e + 1
            } else {
                e
            }
        });
        for (i, e) in s.iter().enumerate() {
            if &e == 0 {
                post.row_mut(i).mapv_inplace(|e| 1f64 / mix.ncentres() as f64)
            }
        }
    }
    let ones: Array2<f64> = Array2::ones((1, mix.ncentres()));
    let divider: Array2<f64> = s.dot(&ones);
    (&(post / divider), &a)

}