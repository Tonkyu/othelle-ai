use std::f32::consts::E;

use super::{matrix::NumType, matrix::Vector};

trait Function {
    fn forward(input: NumType) -> NumType;
    fn forward_vec(input: &Vector) -> Vector {
        Vector {
            rows: input.rows,
            data: input.data.iter()
                    .map(|x| Self::forward(*x))
                    .collect(),
        }
    }
}

struct Sin;
impl Function for Sin {
    fn forward(input: NumType) -> NumType {
        f32::sin(input)
    }
}

struct Cos;
impl Function for Cos {
    fn forward(input: NumType) -> NumType {
        f32::cos(input)
    }
}

struct ReLU;
impl Function for ReLU {
    fn forward(input: NumType) -> NumType {
        input.max(NumType::default())
    }
}

struct Tanh;
impl Function for Tanh {
    fn forward(input: NumType) -> NumType {
        input.tanh()
    }
}

struct Sigmoid;
impl Function for Sigmoid {
    fn forward(input: NumType) -> NumType {
        1.0 / (1.0 + E.powf(input))
    }
}

pub fn mse(target: &Vector, x: &Vector) -> NumType {
    assert_eq!(
        target.rows, x.rows,
        "Vector length does not match in mse()"
    );
    x.data.iter().zip(&target.data).map(|(v1, v2)| (v1 - v2).powi(2)).sum()
}