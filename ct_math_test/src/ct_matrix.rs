use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Add, Mul};

use ct_math::{F, Integer, Math, TAdd, TIsNeg, TIsZero, TMinus, TWhereFalse};

trait MatrixSize {
    const ROWS: usize;
    const COLS: usize;
}

pub struct CTMatrix<Rows: Integer, Cols: Integer>
{
    data: Vec<f64>,
    phantom_data: PhantomData<(Rows, Cols)>,
}

impl<Rows: Integer, Cols: Integer> Debug for CTMatrix<Rows, Cols> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<Rows: Integer, Cols: Integer> Clone for CTMatrix<Rows, Cols> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            phantom_data: PhantomData,
        }
    }
}

impl<Rows: Integer, Cols: Integer> MatrixSize for CTMatrix<Rows, Cols> {
    const ROWS: usize = Rows::VALUE;
    const COLS: usize = Cols::VALUE;
}

impl<Rows: Integer, Cols: Integer> CTMatrix<Rows, Cols> {
    pub fn new() -> Self {
        Self {
            data: vec![0f64; Rows::VALUE * Cols::VALUE],
            phantom_data: PhantomData,
        }
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&f64> {
        if r < Rows::VALUE
            && c < Cols::VALUE {
            Some(&self.data[r * Cols::VALUE + c])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut f64> {
        if r < Rows::VALUE
            && c < Cols::VALUE {
            Some(&mut self.data[r * Cols::VALUE + c])
        } else {
            None
        }
    }

    pub fn split_vertical<I: Integer>(&self) -> (CTMatrix<I, Cols>, CTMatrix<tadd!(Rows,tminus!(I)), Cols>)
        where Math: TMinus<I> + TAdd<Rows, tminus!(I), F>,
              Math: TIsZero<I> + TIsNeg<I> + TIsZero<tadd!(Rows,tminus!(I))> + TIsNeg<tadd!(Rows,tminus!(I))>,
              Math: TWhereFalse<tiszero!(I)> + TWhereFalse<tisneg!(I)> + TWhereFalse<tiszero!(tadd!(Rows,tminus!(I)))> + TWhereFalse<tisneg!(tadd!(Rows,tminus!(I)))>,
              tadd!(Rows,tminus!(I)): Integer
    {
        let mut matrix1 = CTMatrix::new();
        let mut matrix2 = CTMatrix::new();

        matrix1.data = self.data.iter().take(Cols::VALUE * I::VALUE).map(Clone::clone).collect();
        matrix2.data = self.data.iter().skip(Cols::VALUE * I::VALUE).take(Cols::VALUE * <tadd!(Rows,tminus!(I)) as Integer>::VALUE).map(Clone::clone).collect();

        (matrix1, matrix2)
    }

    pub fn split_horizontal<I: Integer>(&self) -> (CTMatrix<Rows, I>, CTMatrix<Rows, tadd!(Cols,tminus!(I))>)
        where Math: TMinus<I> + TAdd<Cols, tminus!(I), F>,
              Math: TIsZero<I> + TIsNeg<I> + TIsZero<tadd!(Cols,tminus!(I))> + TIsNeg<tadd!(Cols,tminus!(I))>,
              Math: TWhereFalse<tiszero!(I)> + TWhereFalse<tisneg!(I)> + TWhereFalse<tiszero!(tadd!(Cols,tminus!(I)))> + TWhereFalse<tisneg!(tadd!(Cols,tminus!(I)))>,
              tadd!(Cols,tminus!(I)): Integer
    {
        let mut matrix1 = CTMatrix::new();
        let mut matrix2 = CTMatrix::new();

        for r in 0..Rows::VALUE {
            for c in 0..I::VALUE {
                *matrix1.get_mut(r, c).unwrap() = self.get(r, c).unwrap().clone();
            }
            for c in I::VALUE..Cols::VALUE {
                *matrix2.get_mut(r, c - I::VALUE).unwrap() = self.get(r, c).unwrap().clone();
            }
        }

        (matrix1, matrix2)
    }

    pub fn concat_vertical<RhsRows: Integer>(&self, rhs: &CTMatrix<RhsRows, Cols>) -> CTMatrix<tadd!(Rows,RhsRows), Cols>
        where Math: TAdd<Rows, RhsRows, F>,
              tadd!(Rows,RhsRows): Integer {
        let mut matrix = CTMatrix::new();

        matrix.data = self.data.iter().chain(rhs.data.iter()).map(Clone::clone).collect();

        matrix
    }

    pub fn concat_horizontal<RhsCols: Integer>(&self, rhs: &CTMatrix<Rows, RhsCols>) -> CTMatrix<Rows, tadd!(Cols,RhsCols)>
        where Math: TAdd<Cols, RhsCols, F>,
              tadd!(Cols,RhsCols): Integer,
    {
        let mut matrix = CTMatrix::new();

        for r in 0..Rows::VALUE {
            for c in 0..Cols::VALUE {
                *matrix.get_mut(r, c).unwrap() = self.get(r, c).unwrap().clone();
            }
            for c in Cols::VALUE..<tadd!(Cols,RhsCols) as Integer>::VALUE {
                *matrix.get_mut(r, c).unwrap() = rhs.get(r, c - Cols::VALUE).unwrap().clone();
            }
        }

        matrix
    }
}

impl<Rows: Integer, Cols: Integer> Add for CTMatrix<Rows, Cols>
{
    type Output = CTMatrix<Rows, Cols>;

    fn add(self, rhs: CTMatrix<Rows, Cols>) -> Self::Output {
        let mut result = CTMatrix::new();
        for i in 0..Rows::VALUE * Cols::VALUE {
            result.data[i] = self.data[i] + rhs.data[i];
        }
        result
    }
}

impl<Rows: Integer, Cols: Integer, T: Integer> Mul<CTMatrix<T, Cols>> for CTMatrix<Rows, T>
{
    type Output = CTMatrix<Rows, Cols>;

    fn mul(self, rhs: CTMatrix<T, Cols>) -> Self::Output {
        let mut result = CTMatrix::new();
        const BLOCK: usize = 8;
        for rr in 0..(Rows::VALUE + BLOCK - 1) / BLOCK {
            for cc in 0..(Cols::VALUE + BLOCK - 1) / BLOCK {
                for ii in 0..(T::VALUE + BLOCK - 1) / BLOCK {
                    for r in 0..BLOCK {
                        if rr * BLOCK + r >= Rows::VALUE { break; }
                        let r = rr * BLOCK + r;
                        for c in 0..BLOCK {
                            if cc * BLOCK + c >= Cols::VALUE { break; }
                            let c = cc * BLOCK + c;
                            for i in 0..BLOCK {
                                if ii * BLOCK + i >= T::VALUE { break; }
                                let i = ii * BLOCK + i;
                                result.data[r * Cols::VALUE + c] += self.data[r * T::VALUE + i] * rhs.data[i * Cols::VALUE + c];
                            }
                        }
                    }
                }
            }
        }
        result
    }
}