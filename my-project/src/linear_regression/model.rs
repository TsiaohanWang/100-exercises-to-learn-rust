use super::MLModel;
use super::loss;
use crate::data_structure::{Matrix, Vector};

pub struct LinearReg1D {
    label: String,
    feature: String,
    weight: f64,
    bias: f64,

    label_data: Vector,
    feature_data: Vector,
}

impl LinearReg1D {
    pub fn new(label: String, feature: String, label_data: Vector, feature_data: Vector) -> Self {
        if label_data.len() != feature_data.len() {
            panic!("Cannot match the label and feature in linear regression model!")
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

    pub fn weight_slope(label: &Vector, feature: &Vector, weight: f64, bias: f64) -> f64 {
        if label.len() == 0 || feature.len() == 0 {
            panic!(
                "When calculating slope, label and feature in linear regression model cannot be empty!"
            )
        }
        if label.len() != feature.len() {
            panic!("Cannot match the label and feature in linear regression model!")
        }

        let example_num = feature.len();
        let mut slope: f64 = 0.0;

        for i in 0..example_num {
            slope += (weight * feature[i] + bias - label[i]) * 2.0 * feature[i];
        }

        slope / (example_num as f64)
    }

    pub fn bias_slope(label: &Vector, feature: &Vector, weight: f64, bias: f64) -> f64 {
        if label.len() == 0 || feature.len() == 0 {
            panic!(
                "When calculating slope, label and feature in linear regression model cannot be empty!"
            )
        }
        if label.len() != feature.len() {
            panic!("Cannot match the label and feature in linear regression model!")
        }

        let example_num = feature.len();
        let mut slope: f64 = 0.0;

        for i in 0..example_num {
            slope += (weight * feature[i] + bias - label[i]) * 2.0;
        }

        slope / (example_num as f64)
    }

    pub fn gradient_descent(&self, learning_rate: f64, iteration: u64) -> (f64, f64) {
        let label = self.label_data.clone();
        let feature = self.feature_data.clone();
        let mut weight = self.weight.clone();
        let mut bias = self.bias.clone();

        let mut predict = self.label_data.clone();
        let mut current_mse: f64 = f64::MAX;
        let mut w_slope: f64 = 0.0;
        let mut b_slope: f64 = 0.0;

        for _ in 0..iteration {
            for i in 0..feature.len() {
                predict[i] = weight * feature[i] + bias;
            }
            
            current_mse = loss::vector_mse(&label, &predict);

            w_slope = LinearReg1D::weight_slope(&label, &feature, weight, bias);
            b_slope = LinearReg1D::bias_slope(&label, &feature, weight, bias);

            weight = weight - (learning_rate * w_slope);
            bias = bias - (learning_rate * b_slope);

            if current_mse == 0.0 {
                println!("MSE has reached 0: gradient descent ends.");
                break;
            }
        }
        println!("w: {}, b: {}, MSE: {}", weight, bias, current_mse);

        (weight, bias)
    }
}

impl MLModel for LinearReg1D {
    fn display(&self) {}
}
