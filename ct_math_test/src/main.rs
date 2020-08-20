#[macro_use]
extern crate ct_math;
extern crate rand;

use std::time::Instant;

use rand::{Rng, thread_rng};

use ct_math::{F, Int32, Integer, T};

use crate::ct_matrix::CTMatrix;
use crate::normal_matrix::NormalMatrix;

mod normal_matrix;
mod ct_matrix;

fn main() {
    eprintln!("Start!");
    println!("type,size,time");
    for i in 0..16 {
        eprintln!("done {}/16", i);

        test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T>>();
        test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F>>();
        test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F>>();
        test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F>>();
        test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F>>();
        test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F>>();
        test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F>>();
        test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F>>();
        test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F>>();
        test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F>>();
        test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F>>();
        test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F>>();

        test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T>>();
        test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F>>();
        test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F>>();
        test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F>>();
        test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F>>();
        test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F>>();
        test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F>>();
        test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F>>();
        test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F>>();

        test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T>>();
        test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F>>();
        test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F>>();
        test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F>>();
        test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F>>();
        test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F>>();
        test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F>>();
        test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F>>();
        test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F>>();
        test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F>>();
        test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F>>();
        test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F>>();

        test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T>>();
        test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F>>();
        test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F>>();
        test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F>>();
        test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F>>();
        test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F>>();
        test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F>>();
        test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F>>();
        test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F>>();
    }
}

fn test_normal_matrix_add<Size: Integer>() {
    let mut rng = thread_rng();
    let mut matrix = NormalMatrix::new(Size::VALUE, Size::VALUE);
    for i in 0..Size::VALUE {
        for j in 0..Size::VALUE {
            *matrix.get_mut(i, j).unwrap() = rng.gen();
        }
    }

    let mut time = 0;
    for _ in 0..1024 {
        let mut matrix2 = NormalMatrix::new(Size::VALUE, Size::VALUE);
        for i in 0..Size::VALUE {
            for j in 0..Size::VALUE {
                *matrix2.get_mut(i, j).unwrap() = rng.gen();
            }
        }
        let start = Instant::now();
        matrix = matrix + matrix2;
        time += start.elapsed().as_nanos();
    }
    println!("0,{},{}", Size::VALUE, time);
    // eprintln!("{:?}", matrix);
}

fn test_normal_matrix_mul<Size: Integer>() {
    let mut rng = thread_rng();
    let mut matrix = NormalMatrix::new(Size::VALUE, Size::VALUE);
    for i in 0..Size::VALUE {
        for j in 0..Size::VALUE {
            *matrix.get_mut(i, j).unwrap() = rng.gen();
        }
    }

    let mut time = 0;
    for _ in 0..1024 {
        let mut matrix2 = NormalMatrix::new(Size::VALUE, Size::VALUE);
        for i in 0..Size::VALUE {
            for j in 0..Size::VALUE {
                *matrix2.get_mut(i, j).unwrap() = rng.gen();
            }
        }
        let start = Instant::now();
        matrix = matrix * matrix2;
        time += start.elapsed().as_nanos();
    }
    println!("1,{},{}", Size::VALUE, time);
    // eprintln!("{:?}", matrix);
}

fn test_ct_matrix_add<Size: Integer>() {
    let mut rng = thread_rng();
    let mut matrix = CTMatrix::<Size, Size>::new();
    for i in 0..Size::VALUE {
        for j in 0..Size::VALUE {
            *matrix.get_mut(i, j).unwrap() = rng.gen();
        }
    }

    let mut time = 0;
    for _ in 0..1024 {
        let mut matrix2 = CTMatrix::<Size, Size>::new();
        for i in 0..Size::VALUE {
            for j in 0..Size::VALUE {
                *matrix2.get_mut(i, j).unwrap() = rng.gen();
            }
        }
        let start = Instant::now();
        matrix = matrix + matrix2;
        time += start.elapsed().as_nanos();
    }
    println!("2,{},{}", Size::VALUE, time);
    // eprintln!("{:?}", matrix);
}

fn test_ct_matrix_mul<Size: Integer>() {
    let mut rng = thread_rng();
    let mut matrix = CTMatrix::<Size, Size>::new();
    for i in 0..Size::VALUE {
        for j in 0..Size::VALUE {
            *matrix.get_mut(i, j).unwrap() = rng.gen();
        }
    }

    let mut time = 0;
    for _ in 0..1024 {
        let mut matrix2 = CTMatrix::<Size, Size>::new();
        for i in 0..Size::VALUE {
            for j in 0..Size::VALUE {
                *matrix2.get_mut(i, j).unwrap() = rng.gen();
            }
        }
        let start = Instant::now();
        matrix = matrix * matrix2;
        time += start.elapsed().as_nanos();
    }
    println!("3,{},{}", Size::VALUE, time);
    // eprintln!("{:?}", matrix);
}