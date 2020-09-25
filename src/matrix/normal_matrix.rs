use std::ops::{Add, Mul};

/// 動的なサイズを持つ行列
#[derive(Clone, Debug)]
pub struct NormalMatrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl NormalMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0f64; rows * cols],
            rows,
            cols,
        }
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&f64> {
        if r < self.rows
            && c < self.cols {
            Some(&self.data[r * self.cols + c])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut f64> {
        if r < self.rows
            && c < self.cols {
            Some(&mut self.data[r * self.cols + c])
        } else {
            None
        }
    }

    pub fn split_vertical(&self, i: usize) -> Option<(NormalMatrix, NormalMatrix)> {
        if i <= 0 || self.rows <= i { return None; }
        let mut matrix1 = NormalMatrix::new(i, self.cols);
        let mut matrix2 = NormalMatrix::new(self.rows - i, self.cols);

        matrix1.data = self.data.iter().take(self.cols * i).map(Clone::clone).collect();
        matrix2.data = self.data.iter().skip(self.cols * i).take(self.cols * (self.rows - i)).map(Clone::clone).collect();

        Some((matrix1, matrix2))
    }

    pub fn split_horizontal(&self, i: usize) -> Option<(NormalMatrix, NormalMatrix)> {
        if i <= 0 || self.cols <= i { return None; }
        let mut matrix1 = NormalMatrix::new(self.rows, i);
        let mut matrix2 = NormalMatrix::new(self.rows, self.cols - i);

        for r in 0..self.rows {
            for c in 0..i {
                *matrix1.get_mut(r, c).unwrap() = self.get(r, c).unwrap().clone();
            }
            for c in i..self.cols {
                *matrix2.get_mut(r, c - i).unwrap() = self.get(r, c).unwrap().clone();
            }
        }

        Some((matrix1, matrix2))
    }

    pub fn concat_vertical(&self, rhs: &NormalMatrix) -> Option<NormalMatrix> {
        if self.cols != rhs.cols { return None; }
        let mut matrix = NormalMatrix::new(self.rows + rhs.rows, self.cols);

        matrix.data = self.data.iter().chain(rhs.data.iter()).map(Clone::clone).collect();

        Some(matrix)
    }

    pub fn concat_horizontal(&self, rhs: &NormalMatrix) -> Option<NormalMatrix> {
        if self.rows != rhs.rows { return None; }
        let mut matrix = NormalMatrix::new(self.rows, self.cols + rhs.cols);

        for r in 0..self.rows {
            for c in 0..self.cols {
                *matrix.get_mut(r, c).unwrap() = self.get(r, c).unwrap().clone();
            }
            for c in self.cols..self.cols + rhs.cols {
                *matrix.get_mut(r, c).unwrap() = rhs.get(r, c - self.cols).unwrap().clone();
            }
        }

        Some(matrix)
    }
}

impl Add for NormalMatrix
{
    type Output = NormalMatrix;

    fn add(self, rhs: NormalMatrix) -> Self::Output {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        let mut result = NormalMatrix::new(self.rows, self.cols);
        unsafe {
            for i in 0..self.rows * self.cols {
                // result.data[i] = self.data[i] + rhs.data[i];
                *result.data.get_unchecked_mut(i) = self.data.get_unchecked(i) + rhs.data.get_unchecked(i);
            }
        }
        result
    }
}

impl Mul for NormalMatrix
{
    type Output = NormalMatrix;

    fn mul(self, rhs: NormalMatrix) -> Self::Output {
        assert_eq!(self.cols, rhs.rows);

        let mut result = NormalMatrix::new(self.rows, rhs.cols);
        const BLOCK: usize = 8;
        for rr in 0..(self.rows + BLOCK - 1) / BLOCK {
            for cc in 0..(rhs.cols + BLOCK - 1) / BLOCK {
                for ii in 0..(self.cols + BLOCK - 1) / BLOCK {
                    for r in 0..BLOCK {
                        if rr * BLOCK + r >= self.rows { break; }
                        let r = rr * BLOCK + r;
                        for c in 0..BLOCK {
                            if cc * BLOCK + c >= rhs.cols { break; }
                            let c = cc * BLOCK + c;
                            for i in 0..BLOCK {
                                if ii * BLOCK + i >= self.cols { break; }
                                let i = ii * BLOCK + i;
                                result.data[r * rhs.cols + c] += self.data[r * self.cols + i] * rhs.data[i * rhs.cols + c];
                            }
                        }
                    }
                }
            }
        }
        result
    }
}