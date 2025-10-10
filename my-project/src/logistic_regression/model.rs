use super::LogisRegModel;
use super::loss;
use crate::data_structure::{Matrix, Vector};

pub struct LogisReg1D {
    label: String,
    feature: String,
    weight: f64,
    bias: f64,

    label_data: Vector,
    feature_data: Vector,
}

impl LogisReg1D {
    fn new(label: String, feature: String, label_data: Vector, feature_data: Vector) -> Self {
        if label_data.len() != feature_data.len() {
            panic!("Cannot match the label and feature in logistic regression model!")
        }
        for i in label_data.0 {
            if label_data[i] == 0 || label_data[i] == 1 {
                ()
            } else {
                panic!("In logistic regression, the label for the example must either be 0 or 1!")
            }
        }

        Self {
            label,
            feature,
            weight: 0.0,
            bias: 0.0,
            label_data,
            feature_data,
        }
    }

    fn sigmoid(input: f64) -> f64 {
        if input >= 0.0 {
            1.0 / (1.0 + (-input).exp())
        } else {
            let exp = input.exp();
            exp / (1.0 + exp)
        }
    }

    fn predict(&self) {}
}

impl LogisRegModel for LogisReg1D {
    fn display(&self) {}
}
