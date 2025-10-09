pub mod ml_data_structure {
    use std::{
        fmt,
        ops::{Index, IndexMut},
    };

    #[derive(Debug, PartialEq, Clone)]
    pub struct Vector(Vec<f64>);

    #[derive(Debug, PartialEq, Clone)]
    pub struct Matrix(Vec<Vec<f64>>);

    pub trait MLDataStruct {}

    impl MLDataStruct for Vector {}
    impl MLDataStruct for Matrix {}

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

    // 为向量实现 Display
    impl fmt::Display for Vector {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_empty() {
                return write!(f, "[]");
            }

            let max_width = self
                .0
                .iter()
                .map(|val| val.to_string().len())
                .max()
                .unwrap_or(0);

            for (i, val) in self.0.iter().enumerate() {
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

    // 为矩阵实现 Display
    impl std::fmt::Display for Matrix {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

    // 新建空向量 new 关联函数
    // 求向量长度 len 方法
    // 判定向量是否为空 is_empty 方法
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
    }

    // 新建空矩阵 new 关联函数
    // 求矩阵行列数 len 方法
    // 求矩阵行数 row 方法
    // 求矩阵列数 col 方法
    // 判定矩阵是否为空 is_empty 方法
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

    // 重载 + 运算符实现向量加法
    impl std::ops::Add<&Vector> for &Vector {
        type Output = Vector;
        fn add(self, rhs: &Vector) -> Self::Output {
            if self.len() != rhs.len() {
                panic!("Illegal add operation between vectors!")
            }

            let result: Vec<f64> = self
                .0
                .iter()
                .zip(rhs.0.iter())
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

            self.0.iter().zip(rhs.0.iter()).map(|(a, b)| a * b).sum()
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

    enum MLDataStructErr {
        IllegalAddOperation,
        IllegalDotProduct,
        IllegalCrossProduct,
    }
}
