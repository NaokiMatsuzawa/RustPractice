#[derive(Debug, Clone, PartialEq)]
///数値T型のH行W列の行列を表す構造体
pub struct Matrix<T, const H : usize, const W : usize>{
    elements: Vec<T>,
}

impl<T : Default + Copy, const H : usize, const W : usize> Matrix<T, H, W>{
    pub fn new() -> Matrix<T, H, W>{
        Matrix{
            elements : vec![T::default(); H*W]
        }
    }
}

impl<T, const H : usize, const W : usize> std::ops::Index<usize> for Matrix<T, H, W> {
    type Output = [T];
    fn index(&self, row: usize) -> &[T]{
        let start = row * W;
        &self.elements[start..start+W]
    }
}

impl<T, const H : usize, const W : usize> std::ops::IndexMut<usize> for Matrix<T, H, W>{
    fn index_mut(&mut self, row: usize) -> &mut [T]{
        let start = row * W;
        &mut self.elements[start..start+W]
    }
}

impl<T : Default + Copy + std::ops::Add<Output = T>, const H : usize, const W : usize> std::ops::Add for Matrix<T, H, W>{
    type Output = Self;
    fn add(self, rhs: Self) -> Self{
        let mut return_matrix = Matrix::<T, H, W>::new();
        for i in 0..H{
            for j in 0..W{
                return_matrix[i][j] = self[i][j] + rhs[i][j];
            }
        }
        return_matrix
    }
}

use std::sync::Arc;

impl<T : Default + Copy + std::ops::AddAssign + std::ops::Mul<Output = T> + std::marker::Send + std::marker::Sync, const H :usize, const W : usize> Matrix<T, H, W>{
    pub fn mul<const RW : usize>(&self, rhs: Matrix<T,W,RW>) -> Matrix<T, H, RW>{
        let mut ans = Matrix::<T, H, RW>::new();
        
        for y in 0..H{
            for x in 0..RW{
                for i in 0..W{
                    ans[y][x] += self[y][i] * rhs[i][x];
                }
            }
        }

        ans
    }

    pub fn mul_mult_thread<const RW : usize>(&self, rhs: Arc<Matrix<T,W,RW>>) -> Matrix<T, H, RW>{
        let mut ans = Matrix::<T, H, RW>::new();

        let threads = 8;
        let rows_per_band = H * RW / threads;

        let mut tmp = vec![0];
        for i in 1..H{
            tmp.push(i);
        }

        let bands : Vec<&mut [T]>= ans.elements.chunks_mut(rows_per_band).collect();

//        let index_num = H * RW;
//        for index in 0..index_num{
//            let y = index / RW;
//            let x = index % RW;
//            for i in 0..W{
//               ans[y][x] += self[y][i] * rhs[i][x];
//            }
//        }
//  を並列化
        crossbeam::scope(|spawner|{
            for (i, band) in bands.into_iter().enumerate(){
                let rhs_for_child = rhs.clone();

                spawner.spawn(move |_|{
                    let index = i * rows_per_band;
                    for ans_grid in band{
                        let y = index / RW;
                        let x = index % RW;
                        for i in 0..W{
                            *ans_grid += self[y][i] *  rhs_for_child[i][x];
                        }
                    }
        
                });
            }
        }).unwrap();
        
        ans
    }
}

#[test]
fn add_test(){
    let mut matrix_0 = Matrix::<u32, 2, 3>::new();
    matrix_0[0][0] = 1;
    matrix_0[0][1] = 1;
    matrix_0[0][2] = 1;
    matrix_0[1][0] = 1;
    matrix_0[1][1] = 1;
    matrix_0[1][2] = 1;

    let mut matrix_1 = Matrix::<u32, 2, 3>::new();
    matrix_1[0][0] = 0;
    matrix_1[0][1] = 1;
    matrix_1[0][2] = 2;
    matrix_1[1][0] = 3;
    matrix_1[1][1] = 4;
    matrix_1[1][2] = 5;

    let mut matrix_2 = Matrix::<u32, 2, 3>::new();
    matrix_2[0][0] = 1;
    matrix_2[0][1] = 2;
    matrix_2[0][2] = 3;
    matrix_2[1][0] = 4;
    matrix_2[1][1] = 5;
    matrix_2[1][2] = 6;

    assert_eq!(matrix_0 + matrix_1, matrix_2);
}

#[test]
fn mul_test(){
    let mut matrix_lhs = Matrix::<u32, 4, 2>::new();
    matrix_lhs[0][0] = 0;
    matrix_lhs[0][1] = 1;
    matrix_lhs[1][0] = 2;
    matrix_lhs[1][1] = 3;
    matrix_lhs[2][0] = 4;
    matrix_lhs[2][1] = 5;
    matrix_lhs[3][0] = 6;
    matrix_lhs[3][1] = 7;
    let mut matrix_rhs = Matrix::<u32, 2, 3>::new();
    matrix_rhs[0][0] = 1;
    matrix_rhs[0][1] = 2;
    matrix_rhs[0][2] = 3;
    matrix_rhs[1][0] = 4;
    matrix_rhs[1][1] = 5;
    matrix_rhs[1][2] = 6;
    let mut matrix_ans = Matrix::<u32, 4, 3>::new();
    matrix_ans[0][0]= 4;
    matrix_ans[0][1]= 5;
    matrix_ans[0][2]= 6;
    matrix_ans[1][0]= 14;
    matrix_ans[1][1]= 19;
    matrix_ans[1][2]= 24;
    matrix_ans[2][0]= 24;
    matrix_ans[2][1]= 33;
    matrix_ans[2][2]= 42;
    matrix_ans[3][0]= 34;
    matrix_ans[3][1]= 47;
    matrix_ans[3][2]= 60;
    assert_eq!(matrix_lhs.mul(matrix_rhs), matrix_ans);
    
}

#[test]
fn mul_test_in_mult_thread(){
    let mut matrix_lhs = Matrix::<u32, 4, 2>::new();
    matrix_lhs[0][0] = 0;
    matrix_lhs[0][1] = 1;
    matrix_lhs[1][0] = 2;
    matrix_lhs[1][1] = 3;
    matrix_lhs[2][0] = 4;
    matrix_lhs[2][1] = 5;
    matrix_lhs[3][0] = 6;
    matrix_lhs[3][1] = 7;
    let mut matrix_rhs = Matrix::<u32, 2, 3>::new();
    matrix_rhs[0][0] = 1;
    matrix_rhs[0][1] = 2;
    matrix_rhs[0][2] = 3;
    matrix_rhs[1][0] = 4;
    matrix_rhs[1][1] = 5;
    matrix_rhs[1][2] = 6;
    let mut matrix_ans = Matrix::<u32, 4, 3>::new();
    matrix_ans[0][0]= 4;
    matrix_ans[0][1]= 5;
    matrix_ans[0][2]= 6;
    matrix_ans[1][0]= 14;
    matrix_ans[1][1]= 19;
    matrix_ans[1][2]= 24;
    matrix_ans[2][0]= 24;
    matrix_ans[2][1]= 33;
    matrix_ans[2][2]= 42;
    matrix_ans[3][0]= 34;
    matrix_ans[3][1]= 47;
    matrix_ans[3][2]= 60;
    assert_eq!(matrix_lhs.mul_mult_thread(Arc::new(matrix_rhs)), matrix_ans);
    
}