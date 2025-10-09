use super::vector::Vector;
use super::*;

use std::{
    fmt,
    ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix(pub Vec<Vec<f64>>);

impl MLDataStruct for Matrix {}

// 新建空矩阵 new 关联函数
// 求矩阵行列数 len 方法
// 求矩阵行数 row 方法
// 求矩阵列数 col 方法
// 判定矩阵是否为空 is_empty 方法
// 取出内部 Vec 数据 unpack 方法
// 提取矩阵某列为向量 get_col 方法
// 提取矩阵某行为向量 get_row 方法
impl Matrix {
    pub fn new() -> Self {
        Matrix(Vec::from(Vec::new()))
    }
    pub fn len(&self) -> (usize, usize) {
        let Matrix(vec) = self;
        (vec.len(), vec[0].len())
    }
    pub fn col(&self) -> usize {
        let (_, column) = self.len();
        column
    }
    pub fn row(&self) -> usize {
        let (row, _) = self.len();
        row
    }
    pub fn is_empty(&self) -> bool {
        match self.len() {
            (0, _) => true,
            (_, 0) => true,
            _ => false,
        }
    }
    pub fn unpack(&self) -> Vec<Vec<f64>> {
        (&self.0).clone()
    }

    pub fn get_col(&self, index: usize) -> Vector {
        if index >= self.col() {
            panic!(
                "Index out of bounds: column index {} is larger than width {}",
                index,
                self.col()
            );
        }
        let col_vec: Vec<f64> = self.0.iter().map(|row| row[index]).collect();
        Vector(col_vec)
    }

    pub fn get_row(&self, index: usize) -> Vector {
        if index >= self.row() {
            panic!(
                "Index out of bounds: row index {} is larger than height {}",
                index,
                self.row()
            );
        }
        (self.clone().0[index]).clone().into()
    }
}

// 重载 [] 运算符以实现矩阵的索引语法
impl Index<usize> for Matrix {
    type Output = Vec<f64>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

// 为矩阵实现 Display
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "[]");
        }

        let mut col_widths: Vec<usize> = vec![0; self.col()];

        for row in &self.0 {
            for (col_idx, val) in row.iter().enumerate() {
                let len = val.to_string().len();
                if len > col_widths[col_idx] {
                    col_widths[col_idx] = len;
                }
            }
        }

        for (row_idx, row) in self.0.iter().enumerate() {
            if row_idx == 0 {
                write!(f, "⎡ ")?;
            } else if row_idx == self.row() - 1 {
                writeln!(f)?;
                write!(f, "⎣ ")?;
            } else {
                writeln!(f)?;
                write!(f, "⎢ ")?;
            }

            for (col_idx, val) in row.iter().enumerate() {
                let width = col_widths[col_idx];
                write!(f, "{:>width$}", val, width = width)?;

                if col_idx < self.col() - 1 {
                    write!(f, "  ")?;
                }
            }

            if row_idx == 0 {
                write!(f, " ⎤")?;
            } else if row_idx == self.row() - 1 {
                write!(f, " ⎦")?;
            } else {
                write!(f, " ⎥")?;
            }
        }

        Ok(())
    }
}

// Vec<Vec<f64>>、Vec<Vec<f32>>、Vec<Vec<i32>>、Vec<Vec<i16>>、Vec<Vec<u32>>、Vec<Vec<u16>> 可以转为矩阵
impl From<Vec<Vec<f64>>> for Matrix {
    fn from(value: Vec<Vec<f64>>) -> Self {
        if value[0].is_empty() || value.is_empty() {
            return Matrix::new();
        }

        let max_len: usize = value.iter().map(|row| row.len()).max().unwrap_or(0);

        let processed_value = value
            .into_iter()
            .map(|mut row| {
                row.resize(max_len, 0.0f64);
                row
            })
            .collect();

        return Matrix(processed_value);
    }
}
impl From<Vec<Vec<f32>>> for Matrix {
    fn from(value: Vec<Vec<f32>>) -> Self {
        if value[0].is_empty() || value.is_empty() {
            return Matrix::new();
        }

        let max_len: usize = value.iter().map(|row| row.len()).max().unwrap_or(0);

        let processed_value = value
            .into_iter()
            .map(|row| {
                let mut row: Vec<f64> = row
                    .into_iter()
                    .map(|component| component.clone().into())
                    .collect();
                row.resize(max_len, 0.0);
                row
            })
            .collect();

        return Matrix(processed_value);
    }
}
impl From<Vec<Vec<i32>>> for Matrix {
    fn from(value: Vec<Vec<i32>>) -> Self {
        if value[0].is_empty() || value.is_empty() {
            return Matrix::new();
        }

        let max_len: usize = value.iter().map(|row| row.len()).max().unwrap_or(0);

        let processed_value = value
            .into_iter()
            .map(|row| {
                let mut row: Vec<f64> = row
                    .into_iter()
                    .map(|component| component.clone().into())
                    .collect();
                row.resize(max_len, 0.0);
                row
            })
            .collect();

        return Matrix(processed_value);
    }
}
impl From<Vec<Vec<i16>>> for Matrix {
    fn from(value: Vec<Vec<i16>>) -> Self {
        if value[0].is_empty() || value.is_empty() {
            return Matrix::new();
        }

        let max_len: usize = value.iter().map(|row| row.len()).max().unwrap_or(0);

        let processed_value = value
            .into_iter()
            .map(|row| {
                let mut row: Vec<f64> = row
                    .into_iter()
                    .map(|component| component.clone().into())
                    .collect();
                row.resize(max_len, 0.0);
                row
            })
            .collect();

        return Matrix(processed_value);
    }
}
impl From<Vec<Vec<u32>>> for Matrix {
    fn from(value: Vec<Vec<u32>>) -> Self {
        if value[0].is_empty() || value.is_empty() {
            return Matrix::new();
        }

        let max_len: usize = value.iter().map(|row| row.len()).max().unwrap_or(0);

        let processed_value = value
            .into_iter()
            .map(|row| {
                let mut row: Vec<f64> = row
                    .into_iter()
                    .map(|component| component.clone().into())
                    .collect();
                row.resize(max_len, 0.0);
                row
            })
            .collect();

        return Matrix(processed_value);
    }
}
impl From<Vec<Vec<u16>>> for Matrix {
    fn from(value: Vec<Vec<u16>>) -> Self {
        if value[0].is_empty() || value.is_empty() {
            return Matrix::new();
        }

        let max_len: usize = value.iter().map(|row| row.len()).max().unwrap_or(0);

        let processed_value = value
            .into_iter()
            .map(|row| {
                let mut row: Vec<f64> = row
                    .into_iter()
                    .map(|component| component.clone().into())
                    .collect();
                row.resize(max_len, 0.0);
                row
            })
            .collect();

        return Matrix(processed_value);
    }
}

// 重载 + 运算符实现矩阵加法
impl std::ops::Add<&Matrix> for &Matrix {
    type Output = Matrix;
    fn add(self, rhs: &Matrix) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Illegal add operation between matrices!")
        }

        let result: Vec<Vec<f64>> = self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(row_a, row_b)| row_a.iter().zip(row_b.iter()).map(|(a, b)| a + b).collect())
            .collect();

        return Matrix(result);
    }
}

// 重载 * 运算符实现矩阵与向量的点乘
impl std::ops::Mul<&Vector> for &Matrix {
    type Output = Vector;
    fn mul(self, rhs: &Vector) -> Self::Output {
        if self.col() != rhs.len() {
            panic!(
                "Illegal Multiplication: Matrix columns ({}) must match Vector length ({})!",
                self.col(),
                rhs.len()
            );
        }

        let result_vec: Vec<f64> = self
            .0
            .iter()
            .map(|row| &Vector(row.clone()) * &rhs.clone())
            .collect();

        Vector(result_vec)
    }
}

// 重载 * 运算符实现矩阵与标量的点乘
impl std::ops::Mul<f64> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Self::Output {
        if self.is_empty() {
            panic!("Illegal scalar-matrix multiplication: empty Matrix!",);
        }

        let result_vec = self
            .0
            .iter()
            .map(|row| (&Vector(row.clone()) * rhs).unpack())
            .collect();

        Matrix(result_vec)
    }
}
impl std::ops::Mul<f32> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: f32) -> Self::Output {
        if self.is_empty() {
            panic!("Illegal scalar-matrix multiplication: empty Matrix!",);
        }

        let result_vec = self
            .0
            .iter()
            .map(|row| (&Vector(row.clone()) * rhs).unpack())
            .collect();

        Matrix(result_vec)
    }
}
impl std::ops::Mul<u32> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: u32) -> Self::Output {
        if self.is_empty() {
            panic!("Illegal scalar-matrix multiplication: empty Matrix!",);
        }

        let result_vec = self
            .0
            .iter()
            .map(|row| (&Vector(row.clone()) * rhs).unpack())
            .collect();

        Matrix(result_vec)
    }
}
impl std::ops::Mul<u16> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: u16) -> Self::Output {
        if self.is_empty() {
            panic!("Illegal scalar-matrix multiplication: empty Matrix!",);
        }

        let result_vec = self
            .0
            .iter()
            .map(|row| (&Vector(row.clone()) * rhs).unpack())
            .collect();

        Matrix(result_vec)
    }
}
impl std::ops::Mul<i32> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: i32) -> Self::Output {
        if self.is_empty() {
            panic!("Illegal scalar-matrix multiplication: empty Matrix!",);
        }

        let result_vec = self
            .0
            .iter()
            .map(|row| (&Vector(row.clone()) * rhs).unpack())
            .collect();

        Matrix(result_vec)
    }
}
impl std::ops::Mul<i16> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: i16) -> Self::Output {
        if self.is_empty() {
            panic!("Illegal scalar-matrix multiplication: empty Matrix!",);
        }

        let result_vec = self
            .0
            .iter()
            .map(|row| (&Vector(row.clone()) * rhs).unpack())
            .collect();

        Matrix(result_vec)
    }
}
