use crate::data_structure::{Matrix, Vector};
use super::MLModel;

pub struct LinearReg1D {
    label: String,
    feature: String,
    weight: f64,
    bias: f64,

    data: Matrix,
}

impl LinearReg1D {
    pub fn new(label: String, feature: String, data: Matrix) -> Self {
        Self {
            label,
            feature,
            weight: 0.0,
            bias: 0.0,
            data,
        }
    }

    
}

impl MLModel for LinearReg1D {
    fn display(&self) {
        println!("1-Dimension Linear Regression")
    }
}