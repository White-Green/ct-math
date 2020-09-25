use std::ops::{Add, Index, IndexMut, Mul};

use crate::type_integer::{Integer, MayBeEquals, StaticInteger};

/// 静的または動的なサイズを持つ行列
pub struct PartialMatrix<Data, Rows: Integer, Cols: Integer> {
    data: Vec<Data>,
    rows: Rows,
    cols: Cols,
}

// メンバ/一点取得系
impl<Data, Rows: Integer, Cols: Integer> PartialMatrix<Data, Rows, Cols> {
    pub fn rows(&self) -> &Rows {
        &self.rows
    }
    pub fn cols(&self) -> &Cols {
        &self.cols
    }
    pub fn get(&self, row: usize, col: usize) -> Option<&Data> {
        if row < self.rows.value() && col < self.cols.value() {
            unsafe { Some(self.get_unchecked(row, col)) }
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Data> {
        if row < self.rows.value() && col < self.cols.value() {
            unsafe { Some(self.get_unchecked_mut(row, col)) }
        } else {
            None
        }
    }
    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &Data {
        self.data.get_unchecked(row * self.cols.value() + col)
    }
    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Data {
        self.data.get_unchecked_mut(row * self.cols.value() + col)
    }
}

// 初期化系
impl<Rows: Integer, Cols: Integer> PartialMatrix<(), Rows, Cols> {
    pub unsafe fn allocate<D>(rows: Rows, cols: Cols) -> PartialMatrix<D, Rows, Cols> {
        let mut data = Vec::with_capacity(rows.value() * cols.value());
        data.set_len(rows.value() * cols.value());
        PartialMatrix {
            data,
            rows,
            cols,
        }
    }
    pub fn filled_by<D: Clone>(rows: Rows, cols: Cols, default: D) -> PartialMatrix<D, Rows, Cols> {
        let data = vec![default; rows.value() * cols.value()];
        PartialMatrix {
            data,
            rows,
            cols,
        }
    }
    pub fn filled_by_default<D: Default>(rows: Rows, cols: Cols) -> PartialMatrix<D, Rows, Cols> {
        let mut data = Vec::with_capacity(rows.value() * cols.value());
        for _ in 0..rows.value() * cols.value() {
            data.push(Default::default());
        }
        PartialMatrix {
            data,
            rows,
            cols,
        }
    }
}

impl<Data, Rows: Integer, Cols: Integer> Index<(usize, usize)> for PartialMatrix<Data, Rows, Cols> {
    type Output = Data;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 >= self.rows.value() {
            panic!("rows of {} is {} but passed row index is {}.", std::any::type_name::<Self>(), self.rows.value(), index.0);
        }
        if index.1 >= self.cols.value() {
            panic!("cols of {} is {} but passed column index is {}.", std::any::type_name::<Self>(), self.cols.value(), index.1);
        }
        unsafe { self.get_unchecked(index.0, index.1) }
    }
}

impl<Data, Rows: Integer, Cols: Integer> IndexMut<(usize, usize)> for PartialMatrix<Data, Rows, Cols> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 >= self.rows.value() {
            panic!("rows of {} is {} but passed row index is {}.", std::any::type_name::<Self>(), self.rows.value(), index.0);
        }
        if index.1 >= self.cols.value() {
            panic!("cols of {} is {} but passed column index is {}.", std::any::type_name::<Self>(), self.cols.value(), index.1);
        }
        unsafe { self.get_unchecked_mut(index.0, index.1) }
    }
}

impl<Data1: Clone, Rows1: Integer, Cols1: Integer, Data2: Clone, Rows2: Integer, Cols2: Integer> Add<PartialMatrix<Data2, Rows2, Cols2>> for PartialMatrix<Data1, Rows1, Cols1>
    where Data1: Add<Data2>,
          Rows1: MayBeEquals<Rows2>,
          <Rows1 as MayBeEquals<Rows2>>::EffortStatic: Integer,
          Cols1: MayBeEquals<Cols2>,
          <Cols1 as MayBeEquals<Cols2>>::EffortStatic: Integer {
    type Output = PartialMatrix<<Data1 as Add<Data2>>::Output, <Rows1 as MayBeEquals<Rows2>>::EffortStatic, <Cols1 as MayBeEquals<Cols2>>::EffortStatic>;

    fn add(self, rhs: PartialMatrix<Data2, Rows2, Cols2>) -> Self::Output {
        assert!(<Rows1 as MayBeEquals<Rows2>>::equals(&self.rows, &rhs.rows));
        assert!(<Cols1 as MayBeEquals<Cols2>>::equals(&self.cols, &rhs.cols));
        unsafe {
            let rows = <Rows1 as MayBeEquals<Rows2>>::next_value(&self.rows, &rhs.rows);
            let cols = <Cols1 as MayBeEquals<Cols2>>::next_value(&self.cols, &rhs.cols);
            let mut matrix = PartialMatrix::allocate(rows, cols);
            let rows = <Rows1 as MayBeEquals<Rows2>>::next_value(&self.rows, &rhs.rows);
            let cols = <Cols1 as MayBeEquals<Cols2>>::next_value(&self.cols, &rhs.cols);
            for i in 0..rows.value() * cols.value() {
                *matrix.data.get_unchecked_mut(i) = self.data.get_unchecked(i).clone() + rhs.data.get_unchecked(i).clone();
            }
            matrix
        }
    }
}

impl<Data1: Clone, Rows1: Integer, Cols1: Integer, Data2: Clone, Rows2: Integer, Cols2: Integer> Mul<PartialMatrix<Data2, Rows2, Cols2>> for PartialMatrix<Data1, Rows1, Cols1>
    where Data1: Mul<Data2>,
          <Data1 as Mul<Data2>>::Output: Add<Output=<Data1 as Mul<Data2>>::Output>,
          <Data1 as Mul<Data2>>::Output: Clone,
          Cols1: MayBeEquals<Rows2> {
    type Output = PartialMatrix<<<Data1 as Mul<Data2>>::Output as Add>::Output, Rows1, Cols2>;

    fn mul(self, rhs: PartialMatrix<Data2, Rows2, Cols2>) -> Self::Output {
        assert!(<Cols1 as MayBeEquals<Rows2>>::equals(&self.cols, &rhs.rows));
        unsafe {
            let mut matrix = PartialMatrix::allocate(self.rows.clone(), rhs.cols.clone());
            for r in 0..self.rows.value() {
                for c in 0..rhs.cols.value() {
                    *matrix.get_unchecked_mut(r, c) = self.get_unchecked(r, 0).clone() * rhs.get_unchecked(0, c).clone();
                }
            }
            const BLOCK: usize = 8;
            let row_split = (self.rows.value() + BLOCK - 1) / BLOCK;
            let col_split = (rhs.cols.value() + BLOCK - 1) / BLOCK;
            let dep_split = (self.cols.value() - 1 + BLOCK - 1) / BLOCK;
            for rr in 0..row_split {
                for cc in 0..col_split {
                    for ii in 0..dep_split {
                        for r in rr * self.rows.value() / row_split..(rr + 1) * self.rows.value() / row_split {
                            for c in cc * rhs.cols.value() / col_split..(cc + 1) * rhs.cols.value() / col_split {
                                for i in ii * (self.cols.value() - 1) / dep_split + 1..(ii + 1) * (self.cols.value() - 1) / dep_split + 1 {
                                    *matrix.get_unchecked_mut(r, c) = matrix.get_unchecked(r, c).clone() + self.get_unchecked(r, i).clone() * rhs.get_unchecked(i, c).clone();
                                }
                            }
                        }
                    }
                }
            }
            matrix
        }
    }
}