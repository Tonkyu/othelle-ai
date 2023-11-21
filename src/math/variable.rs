use std::ops;
use num_traits;

pub struct Scalar<T>
where T: num_traits::Num,
{
    data:   T,
}

pub struct Vector<T>
where T: num_traits::Num,
{
    data: Vec<Scalar<T>>
}

pub struct Matrix<T>
where T: num_traits::Num,
{
    rows:   usize,
    cols:   usize,
    data:   Vec<Vector<T>>,
}


impl<T> ops::Add for &Scalar<T>
where T: num_traits::Num,
{
    type Output = Scalar<T>;

    fn add(self, other: &Scalar<T>) -> Scalar<T> {
        Scalar {
            data: self.data + other.data,
        }
    }
}

impl<T> ops::Mul for &Scalar<T>
where T: num_traits::Num,
{
    type Output = Scalar<T>;

    fn mul(self, other: &Scalar<T>) -> Scalar<T> {
        Scalar {
            data: self.data * other.data,
        }
    }
}

trait Function<T>
{
    fn forward(input: T) -> T;
}

impl<F, T> Function<Vector<T>> for F
where T: num_traits::Num + std::cmp::PartialOrd<i32>,
{
    fn forward(input: Vector<T>) -> Vector<T> {
        input.data.iter()
        .map(|x| F::forward(*x)).collect()
    }
}

struct ReLU;

impl<T> Function<Scalar<T>> for ReLU
where T: num_traits::Num + std::cmp::PartialOrd<i32>,
{
    fn forward(input: Scalar<T>) -> Scalar<T> {
        if input.data > 0 {
            input
        } else {
            Scalar { data: num_traits::zero() }
        }
    }
}

