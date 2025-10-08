mod MLDataStructure {
    use std::{
        ops::{Index, IndexMut},
        vec,
    };

    #[derive(Debug, PartialEq, Clone)]
    struct Vector(Vec<f64>);

    #[derive(Debug, PartialEq, Clone)]
    struct Matrix(Vec<Vec<f64>>);

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
    // 新建空向量 new 关联函数
    // 求向量长度 len 方法
    impl Vector {
        fn new() -> Self {
            Vector(Vec::new())
        }

        fn len(&self) -> usize {
            let Vector(vec) = self;
            vec.len()
        }
    }
    // 新建空矩阵 new 关联函数
    // 求矩阵行列数 len 方法
    // 求矩阵行数 row 方法
    // 求矩阵列数 col 方法
    // 提取矩阵某列为向量 get_col 方法
    impl Matrix {
        fn new() -> Self {
            Matrix(Vec::from(Vec::new()))
        }
        fn len(&self) -> (usize, usize) {
            let Matrix(vec) = self;
            (vec.len(), vec[0].len())
        }
        fn col(&self) -> usize {
            let (_, column) = self.len();
            column
        }
        fn row(&self) -> usize {
            let (row, _) = self.len();
            row
        }

        fn get_col(&self, index: usize) -> Vector {
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
    }
    // 重载 + 运算符实现向量加法
    impl std::ops::Add for Vector {
        type Output = Vector;
        fn add(self, rhs: Self) -> Self::Output {
            if self.len() != rhs.len() {
                panic!("Illegal add operation between vectors!")
            }

            let mut v = Vector::new();

            for i in 0..self.len() {
                v[i] = self[i] + rhs[i]
            }

            return v;
        }
    }
    // 重载 * 运算符实现向量点乘
    impl std::ops::Mul for Vector {
        type Output = f64;

    }
    impl std::ops::Mul<&Vector> for Vector {
        type Output = f64;

    }
    // 重载 + 运算符实现矩阵加法
    impl std::ops::Add for Matrix {
        type Output = Matrix;
        fn add(self, rhs: Self) -> Self::Output {
            if self.len() != rhs.len() {
                panic!("Illegal add operation between matrices!")
            }

            let mut v = Matrix::new();

            for i in 0..self.row() {
                for j in 0..self.col() {
                    v[i][j] = self[i][j] + rhs[i][j]
                }
            }

            return v;
        }
    }
    // 重载 * 运算符实现矩阵与向量的点乘
    impl std::ops::Mul<&Vector> for Matrix {
        type Output = Vector;
        fn mul(self, rhs: &Vector) -> Self::Output {
            if self.col() != rhs.len() {
                panic!(
                    "Illegal Multiplication: Matrix columns ({}) must match Vector length ({}).",
                    self.col(),
                    rhs.len()
                );
            }

            // 遍历矩阵的每一行，与向量进行点积
            let result_vec: Vec<f64> = self
                .0
                .iter()
                .map(|row| {
                    // 将矩阵的行（&Vec<f64>）临时包装成一个 Vector 来使用 dot 方法
                    Vector(row.clone()) * rhs
                })
                .collect();

            Vector(result_vec)
        }
    }

    enum MLDataStructErr {
        IllegalAddOperation,
        IllegalSubOperation,
        IllegalDotProduct,
        IllegalCrossProduct,
    }
}
