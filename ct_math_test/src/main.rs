extern crate rand;


use std::thread;
use std::time::Instant;

use rand::{Rng, thread_rng};

use ct_math::matrix::ct_matrix::CTMatrix;
use ct_math::matrix::normal_matrix::NormalMatrix;
use ct_math::matrix::partial_dynamic_matrix::PartialMatrix;
use ct_math::type_boolean::{F, T};
use ct_math::type_integer::{Int32, Integer, StaticInteger};

fn main() {
    for i in 1..=1000 {
        for j in 1..=i {
            let mut vec = Vec::with_capacity(i);
            let length = i;
            let n = j;
            for block_num in 0..n {
                for index in block_num * length / n..(block_num + 1) * length / n {
                    vec.push(index);
                }
            }
            if vec.iter().enumerate().all(|(e, value)| e == *value) {
                // println!("{}/{} ok", i, j);
            } else {
                println!("{}/{} ng", i, j);
            }
        }
    }
    return;
    println!("type,size,time");
    type P = Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F>;
    unsafe { PartialMatrix::allocate::<f64>(<P as Default>::default(), <P as Default>::default()); }
    for i in 0..16 / P::VALUE {
        eprintln!("done {}/{}", i, 16 / P::VALUE);

        wrap::<P, _>(test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T>>);
        wrap::<P, _>(test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F>>);
        wrap::<P, _>(test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F>>);
        wrap::<P, _>(test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F>>);
        wrap::<P, _>(test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F>>);
        wrap::<P, _>(test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F>>);
        wrap::<P, _>(test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F>>);
        wrap::<P, _>(test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F>>);
        wrap::<P, _>(test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F>>);
        wrap::<P, _>(test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F>>);
        wrap::<P, _>(test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F>>);
        wrap::<P, _>(test_normal_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F>>);

        wrap::<P, _>(test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T>>);
        wrap::<P, _>(test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F>>);
        wrap::<P, _>(test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F>>);
        wrap::<P, _>(test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F>>);
        wrap::<P, _>(test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F>>);
        wrap::<P, _>(test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F>>);
        wrap::<P, _>(test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F>>);
        wrap::<P, _>(test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F>>);
        wrap::<P, _>(test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F>>);
        wrap::<P, _>(test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F>>);
        wrap::<P, _>(test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F>>);
        wrap::<P, _>(test_normal_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F>>);

        wrap::<P, _>(test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T>>);
        wrap::<P, _>(test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F>>);
        wrap::<P, _>(test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F>>);
        wrap::<P, _>(test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F>>);
        wrap::<P, _>(test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F>>);
        wrap::<P, _>(test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F>>);
        wrap::<P, _>(test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F>>);
        wrap::<P, _>(test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F>>);
        wrap::<P, _>(test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F>>);
        wrap::<P, _>(test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F>>);
        wrap::<P, _>(test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F>>);
        wrap::<P, _>(test_ct_matrix_add::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F>>);

        wrap::<P, _>(test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T>>);
        wrap::<P, _>(test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F>>);
        wrap::<P, _>(test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F>>);
        wrap::<P, _>(test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F>>);
        wrap::<P, _>(test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F>>);
        wrap::<P, _>(test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F>>);
        wrap::<P, _>(test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F>>);
        wrap::<P, _>(test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F>>);
        wrap::<P, _>(test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F>>);
        wrap::<P, _>(test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F>>);
        wrap::<P, _>(test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F>>);
        wrap::<P, _>(test_ct_matrix_mul::<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F>>);
    }
}

fn wrap<P: StaticInteger, F: Fn() + Clone + 'static + Send + Sync>(f: F) {
    let mut vec = Vec::with_capacity(P::VALUE);
    for _ in 0..P::VALUE {
        vec.push(thread::spawn(f.clone()));
    }
    vec.into_iter().for_each(|j| j.join().unwrap());
}

fn test_normal_matrix_add<Size: StaticInteger>() {
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

fn test_normal_matrix_mul<Size: StaticInteger>() {
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

fn test_ct_matrix_add<Size: StaticInteger>() {
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

fn test_ct_matrix_mul<Size: StaticInteger>() {
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