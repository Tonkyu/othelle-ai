use std::ops;
use std::fmt;

pub type NumType = f32;

pub struct Matrix {
    rows:  usize,
    cols:  usize,
    data:   Vec<Vec<NumType>>
}

impl Matrix {
    pub fn new(data: Vec<Vec<NumType>>) -> Self {
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data: data
        }
    }

    pub fn zeros(rows: usize, cols: usize) -> Self
    {
        let data = vec![vec![0.0; cols]; rows];
        Matrix {
            rows,
            cols,
            data,
        }
    }

    pub fn t(&self) -> Self
    {
        let mut data = vec![vec![0.0; self.rows]; self.cols];
        for i in 0..self.rows {
            for j in 0..self.cols {
                data[j][i] = self.data[i][j];
            }
        }
        Matrix {
            rows: self.cols,
            cols: self.rows,
            data,
        }
    }
}

impl ops::Add for &Matrix
{
    type Output = Matrix;

    fn add(self, other: &Matrix) -> Matrix {
        assert_eq!(
            self.rows, other.rows,
            "Matrix dimensions do not match for addition (rows)"
        );
        assert_eq!(
            self.cols, other.cols,
            "Matrix dimensions do not match for addition (cols)"
        );

        let mut data = vec![vec![Default::default(); self.cols]; self.rows];

        for i in 0..self.rows {
            for j in 0..self.cols {
                data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }

        Matrix {
            data: data,
            ..*self
        }
    }
}

impl ops::Sub for &Matrix
{
    type Output = Matrix;

    fn sub(self, other: &Matrix) -> Matrix {
        assert_eq!(
            self.rows, other.rows,
            "Matrix dimensions do not match for subtraction (rows)"
        );
        assert_eq!(
            self.cols, other.cols,
            "Matrix dimensions do not match for subtraction (cols)"
        );

        let mut data = vec![vec![Default::default(); self.cols]; self.rows];

        for i in 0..self.rows {
            for j in 0..self.cols {
                data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }

        Matrix {
            data: data,
            ..*self
        }
    }
}

impl ops::Mul<&Matrix> for &Matrix
{
    type Output = Matrix;
    fn mul(self, other: &Matrix) -> Matrix {
        assert_eq!(
            self.cols, other.rows,
            "Matrix dimensions do not match for multiplication"
        );
        let mut data = vec![vec![Default::default(); self.rows]; other.cols];
        for k in 0..self.cols {
            for i in 0..self.rows {
                for j in 0..other.cols {
                    data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }

        Matrix {
            rows: self.rows,
            cols: other.cols,
            data: data,
        }
    }
}

impl ops::Mul<NumType> for &Matrix
{
    type Output = Matrix;
    fn mul(self, other: NumType) -> Matrix {
        let data = self.data.iter()
        .map(|row| row.iter().map(|&element| element * other).collect())
        .collect();

        Matrix {
            data,
            ..*self
        }
    }
}

impl ops::Mul<&Matrix> for NumType
where
    NumType: num_traits::Num,
{
    type Output = Matrix;
    fn mul(self, other: &Matrix) -> Matrix {
        let data = other.data.iter()
        .map(|row| row.iter().map(|&element| element * self).collect())
        .collect();

        Matrix {
            data,
            ..*other
        }
    }
}


impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for element in row {
                write!(f, "{} ", element)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
