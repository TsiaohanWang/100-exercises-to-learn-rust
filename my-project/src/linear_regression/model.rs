use super::MLModel;
use super::loss;
use crate::data_structure::Vector;

pub struct LinearReg1D {
    pub label: String,
    pub feature: String,
    pub weight: f64,
    pub bias: f64,

    pub label_data: Vector,
    pub feature_data: Vector,
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
        if label.is_empty() || feature.is_empty() {
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
        if label.is_empty() || feature.is_empty() {
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

    pub fn gradient_descent(&self, learning_rate: f64, epoch: u64) -> (f64, f64, f64) {
        let label = self.label_data.clone();
        let feature = self.feature_data.clone();
        let mut weight = self.weight.clone();
        let mut bias = self.bias.clone();

        let mut predict = self.label_data.clone();
        let mut current_mse: f64 = f64::MAX;
        let mut w_slope: f64 = 0.0;
        let mut b_slope: f64 = 0.0;

        for _ in 0..epoch {
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

        (weight, bias, current_mse)
    }

    pub fn train(&mut self, learning_rate: f64, total_epoch: u64, rec_times: u64) -> Vector {
        println!("╭─");
        println!("│ Linear Regression 1-Dimension Model training results:");

        let epoch: u64 = total_epoch / rec_times;
        let remained_epoch: u64 = total_epoch % rec_times;

        let mut mse_records = Vector::new();

        for _ in 0..rec_times {
            let (new_weight, new_bias, new_mse) = self.gradient_descent(learning_rate, epoch);
            self.weight = new_weight;
            self.bias = new_bias;
            mse_records.push(new_mse);
        }

        if remained_epoch != 0 {
            let (new_weight, new_bias, new_mse) =
                self.gradient_descent(learning_rate, remained_epoch);
            self.weight = new_weight;
            self.bias = new_bias;
            mse_records.push(new_mse);
        }

        println!(
            "│ Learning rate: {}, Total epoch: {}",
            learning_rate, total_epoch
        );
        println!(
            "│ MSE recorded {} times, with every {} epochs",
            rec_times, epoch
        );
        println!("├─");
        println!("│ Current model: y = ({}) * x + ({})", self.weight, self.bias);
        println!("│ Current MSE: {}", mse_records.find_last());
        println!("╰─");

        mse_records
    }
}

impl MLModel for LinearReg1D {
    fn display(&self) {
        println!("Linear Regression 1-Dimension Model")
    }
}
