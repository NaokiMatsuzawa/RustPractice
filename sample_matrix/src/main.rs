use std::sync::Arc;

use sample_matrix::Matrix;
use rand::prelude::*;

fn make_rand_matrix<const H: usize, const W: usize>() -> Matrix<u32, H, W>{
    let mut matrix = Matrix::<u32, H, W>::new();
    let mut rng = thread_rng();
    let distr = rand::distributions::Uniform::new_inclusive(0, 100);
    for y in 0..H{
        for x in 0..W{
            matrix[y][x] = rng.sample(distr);
        }
    }
    matrix
}

fn main(){
    let matrix_lhs = make_rand_matrix::<400, 400>();
    let matrix_rhs = make_rand_matrix::<400, 400>();
    let _matrix_ans = matrix_lhs.mul_mult_thread(Arc::new(matrix_rhs));
}