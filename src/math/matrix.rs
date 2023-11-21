use std::ops;
use std::fmt;

pub type NumType = f32;

pub struct Vector {
    pub rows:   usize,
    pub data:   Vec<NumType>,
}

impl Vector {
    pub fn new(data: Vec<NumType>) -> Self {
        Vector {
            rows: data.len(),
            data: data,
        }
    }

    pub fn zeros(rows: usize) -> Self {
        let data = vec![NumType::default(); rows];
        Vector {
            rows,
            data,
        }
    }

    pub fn vec2mat(self) -> Matrix {
        Matrix {
            rows: self.rows,
            cols: 1,
            data: self.data
                .chunks(1)
                .map(|chunk| chunk.to_vec())
                .collect(),
        }
    }
}

impl ops::Add for &Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        assert_eq!(
            self.rows, other.rows,
            "Vector dimensions do not match for addition (rows)"
        );

        let mut data = Vector::zeros(self.rows).data;
        for i in 0..self.rows {
            data[i] = self.data[i] + other.data[i];
        }

        Vector {
            data: self.data.iter().zip(&other.data)
                .map(|(a, b)| a + b).collect(),
            ..*self
        }
    }
}

impl ops::Sub for &Vector {
    type Output = Vector;

    fn sub(self, other: &Vector) -> Vector {
        assert_eq!(
            self.rows, other.rows,
            "Vector dimensions do not match for subtraction (rows)"
        );

        let mut data = Vector::zeros(self.rows).data;
        for i in 0..self.rows {
            data[i] = self.data[i] + other.data[i];
        }

        Vector {
            data: self.data.iter().zip(&other.data)
                .map(|(a, b)| a + b).collect(),
            ..*self
        }
    }
}

impl ops::Mul<&Vector> for NumType {
    type Output = Vector;
    fn mul(self, other: &Vector) -> Vector {
        let data = other.data.iter()
        .map(|&element| element * self).collect();

        Vector {
            data,
            ..*other
        }
    }
}

impl ops::Mul<NumType> for &Vector
{
    type Output = Vector;
    fn mul(self, other: NumType) -> Vector {
        let data = self.data.iter()
        .map(|&element| element * other).collect();

        Vector {
            data,
            ..*self
        }
    }
}

pub struct Matrix {
    rows:  usize,
    cols:  usize,
    data:   Vec<Vec<NumType>>,
}

impl Matrix {
    pub fn new(data: Vec<Vec<NumType>>) -> Self {
        assert_eq!(
            data.len(), 0,
            "Can't create new matrix whose length is 0",
        );
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data: data,
        }
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        let data = vec![vec![NumType::default(); cols]; rows];
        Matrix {
            rows,
            cols,
            data,
        }
    }

    pub fn t(&self) -> Self {
        let mut data = vec![vec![NumType::default(); self.rows]; self.cols];
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

    pub fn mat2vec(self) -> Vector {
        assert_eq!(
            self.cols, 1,
            "Can't convert to Vector: cols is not 1",
        );
        Vector::new(self.data.iter().flat_map(|row| row.iter()).cloned().collect())
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
