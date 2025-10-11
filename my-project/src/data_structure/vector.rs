use super::matrix::Matrix;
use super::*;
use std::{
    fmt,
    ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Vector(pub Vec<f64>);

impl MLDataStruct for Vector {}

// 新建空向量 new 关联函数
// 求向量长度 len 方法
// 判定向量是否为空 is_empty 方法
// 取出内部 Vec 数据 unpack 方法
// 压入新分量 push 方法
// 查看最后一个分量 find_last 方法
// 创建一个预先已知容量的向量 with_capacity 关联函数
impl Vector {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn len(&self) -> usize {
        let Vector(vec) = self;
        vec.len()
    }

    pub fn is_empty(&self) -> bool {
        match self.len() {
            0 => true,
            _ => false,
        }
    }

    pub fn unpack(&self) -> Vec<f64> {
        (&self.0).clone()
    }

    pub fn push(&mut self, new_component: f64) -> &Self {
        self.0.push(new_component);
        self
    }

    pub fn find_last(&self) -> f64 {
        self[self.len() - 1]
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Vector(Vec::with_capacity(capacity))
    }
}

// 重载 [] 运算符以实现向量的索引语法
impl Index<usize> for Vector {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

// 为向量实现 Display
impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "[]");
        }

        let max_width = self
            .unpack()
            .iter()
            .map(|val| val.to_string().len())
            .max()
            .unwrap_or(0);

        for (i, val) in self.unpack().iter().enumerate() {
            if i == 0 {
                write!(f, "⎡ {:>width$} ⎤", val, width = max_width)?;
            } else if i == self.len() - 1 {
                writeln!(f)?;
                write!(f, "⎣ {:>width$} ⎦", val, width = max_width)?;
            } else {
                writeln!(f)?;
                write!(f, "⎢ {:>width$} ⎥", val, width = max_width)?;
            }
        }

        Ok(())
    }
}

// Vec<f64>、Vec<f32>、Vec<i32>、Vec<i16>、Vec<u32>、Vec<u16> 可以转为向量
impl From<Vec<f64>> for Vector {
    fn from(value: Vec<f64>) -> Self {
        if value.is_empty() {
            return Vector::new();
        } else {
            return Vector(value);
        }
    }
}
impl From<Vec<f32>> for Vector {
    fn from(value: Vec<f32>) -> Self {
        if value.is_empty() {
            return Vector::new();
        } else {
            return Vector(
                value
                    .iter()
                    .map(|component| component.clone().into())
                    .collect(),
            );
        }
    }
}
impl From<Vec<i32>> for Vector {
    fn from(value: Vec<i32>) -> Self {
        if value.is_empty() {
            return Vector::new();
        } else {
            return Vector(
                value
                    .iter()
                    .map(|component| component.clone().into())
                    .collect(),
            );
        }
    }
}
impl From<Vec<i16>> for Vector {
    fn from(value: Vec<i16>) -> Self {
        if value.is_empty() {
            return Vector::new();
        } else {
            return Vector(
                value
                    .iter()
                    .map(|component| component.clone().into())
                    .collect(),
            );
        }
    }
}
impl From<Vec<u32>> for Vector {
    fn from(value: Vec<u32>) -> Self {
        if value.is_empty() {
            return Vector::new();
        } else {
            return Vector(
                value
                    .iter()
                    .map(|component| component.clone().into())
                    .collect(),
            );
        }
    }
}
impl From<Vec<u16>> for Vector {
    fn from(value: Vec<u16>) -> Self {
        if value.is_empty() {
            return Vector::new();
        } else {
            return Vector(
                value
                    .iter()
                    .map(|component| component.clone().into())
                    .collect(),
            );
        }
    }
}

// 单列或单行矩阵可以转为向量
impl From<Matrix> for Vector {
    fn from(value: Matrix) -> Self {
        if value.row() == 0 {
            return Vector::new();
        }
        if value.col() != 1 && value.row() != 1 {
            let (row, col) = value.len();
            panic!("Cannot convert a {row}-row, {col}-column matrix into a vector!")
        }
        if value.row() == 1 {
            let mut v = Vector::new();
            for i in 0..value.col() {
                v[i] = value[0][i]
            }

            return v;
        }

        let mut v = Vector::new();
        for i in 0..value.row() {
            v[i] = value[i][0]
        }

        return v;
    }
}

// 重载 + 运算符实现向量加法
impl std::ops::Add<&Vector> for &Vector {
    type Output = Vector;
    fn add(self, rhs: &Vector) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Illegal add operation between vectors!")
        }

        let result: Vec<f64> = self
            .unpack()
            .iter()
            .zip(rhs.unpack().iter())
            .map(|(a, b)| a + b)
            .collect();

        return Vector(result);
    }
}

// 重载 * 运算符实现向量点乘
impl std::ops::Mul<&Vector> for &Vector {
    type Output = f64;
    fn mul(self, rhs: &Vector) -> Self::Output {
        if self.len() != rhs.len() {
            panic!(
                "Illegal dot product between a {}-dimension vector and a {}-dimension vector!",
                self.len(),
                rhs.len()
            )
        }

        self.unpack()
            .iter()
            .zip(rhs.unpack().iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

// 重载 * 运算符实现向量与标量点乘
impl std::ops::Mul<f64> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        if self.is_empty() {
            panic!("Illegal scalar-vector multiplication: empty Vector!")
        }

        let result_vec: Vec<f64> = self
            .unpack()
            .iter()
            .map(|component| component * rhs)
            .collect();

        Vector(result_vec)
    }
}
impl std::ops::Mul<f32> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: f32) -> Self::Output {
        if self.is_empty() {
            panic!("Illegal scalar-vector multiplication: empty Vector!")
        }

        let rhs: f64 = rhs.into();

        let result_vec: Vec<f64> = self
            .unpack()
            .iter()
            .map(|component| component * rhs)
            .collect();

        Vector(result_vec)
    }
}
impl std::ops::Mul<i32> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: i32) -> Self::Output {
        if self.is_empty() {
            panic!("Illegal scalar-vector multiplication: empty Vector!")
        }

        let rhs: f64 = rhs.into();

        let result_vec: Vec<f64> = self
            .unpack()
            .iter()
            .map(|component| component * rhs)
            .collect();

        Vector(result_vec)
    }
}
impl std::ops::Mul<i16> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: i16) -> Self::Output {
        if self.is_empty() {
            panic!("Illegal scalar-vector multiplication: empty Vector!")
        }

        let rhs: f64 = rhs.into();

        let result_vec: Vec<f64> = self
            .unpack()
            .iter()
            .map(|component| component * rhs)
            .collect();

        Vector(result_vec)
    }
}
impl std::ops::Mul<u32> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: u32) -> Self::Output {
        if self.is_empty() {
            panic!("Illegal scalar-vector multiplication: empty Vector!")
        }

        let rhs: f64 = rhs.into();

        let result_vec: Vec<f64> = self
            .unpack()
            .iter()
            .map(|component| component * rhs)
            .collect();

        Vector(result_vec)
    }
}
impl std::ops::Mul<u16> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: u16) -> Self::Output {
        if self.is_empty() {
            panic!("Illegal scalar-vector multiplication: empty Vector!")
        }

        let rhs: f64 = rhs.into();

        let result_vec: Vec<f64> = self
            .unpack()
            .iter()
            .map(|component| component * rhs)
            .collect();

        Vector(result_vec)
    }
}
