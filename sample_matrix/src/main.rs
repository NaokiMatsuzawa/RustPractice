struct Matrix<T>{
    width : usize,
    elements: Vec<T>,
}

impl<T: Default + Copy> Matrix<T>{
    fn new(width: usize, height: usize) -> Matrix<T>{
        Matrix { 
            width,
            elements : vec![T::default(); width * height] 
        }
    }
}

impl<T> std::ops::Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, row: usize) -> &[T]{
        let start = row * self.width;
        &self.elements[start..start+self.width]
    }
}

impl<T> std::ops::IndexMut<usize> for Matrix<T>{
    fn index_mut(&mut self, row: usize) -> &mut [T]{
        let start = row * self.width;
        &mut self.elements[start..start+self.width]
    }
}

fn main() {
    let mut matrix = Matrix::<u32>::new(2,3);
    matrix[0][0] = 1;

    let mut matrix2 = {{1,2,3},
                        {4,5,6},
                        {7,8,9}};
}
