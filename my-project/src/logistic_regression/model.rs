use super::LogisRegModel;
use super::loss;
use crate::data_structure::Vector;

#[derive(Debug, Clone)]
pub struct LogisReg1D {
    pub label: String,
    pub feature: String,
    pub weight: f64,
    pub bias: f64,

    pub label_data: Vector,
    pub feature_data: Vector,
}

impl LogisReg1D {
    pub fn new(label: String, feature: String, label_data: Vector, feature_data: Vector) -> Self {
        if label_data.len() != feature_data.len() {
            panic!("Cannot match the label and feature in logistic regression model!")
        }
        for i in 0..label_data.len() {
            if label_data[i] == 0.0 || label_data[i] == 1.0 {
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

    pub fn sigmoid(input: f64) -> f64 {
        if input >= 0.0 {
            1.0 / (1.0 + (-input).exp())
        } else {
            let exp = input.exp();
            exp / (1.0 + exp)
        }
    }

    pub fn weight_slope(label: &Vector, feature: &Vector, weight: f64, bias: f64) -> f64 {
        if label.is_empty() || feature.is_empty() {
            panic!(
                "When calculating slope, label and feature in logistic regression model cannot be empty!"
            )
        }
        if label.len() != feature.len() {
            panic!("Cannot match the label and feature in logistic regression model!")
        }

        let example_num = feature.len();
        let mut slope: f64 = 0.0;

        for i in 0..example_num {
            slope += LogisReg1D::sigmoid(weight * feature[i] + bias - label[i]) * feature[i];
        }

        slope / (example_num as f64)
    }

    pub fn bias_slope(label: &Vector, feature: &Vector, weight: f64, bias: f64) -> f64 {
        if label.is_empty() || feature.is_empty() {
            panic!(
                "When calculating slope, label and feature in logistic regression model cannot be empty!"
            )
        }
        if label.len() != feature.len() {
            panic!("Cannot match the label and feature in logistic regression model!")
        }

        let example_num = feature.len();
        let mut slope: f64 = 0.0;

        for i in 0..example_num {
            slope += LogisReg1D::sigmoid(weight * feature[i] + bias) - label[i];
        }

        slope / (example_num as f64)
    }

    pub fn gradient_descent(&self, learning_rate: f64, epoch: u64) -> (f64, f64, f64) {
        let label = self.label_data.clone();
        let feature = self.feature_data.clone();
        let mut weight = self.weight.clone();
        let mut bias = self.bias.clone();

        let mut predict = self.label_data.clone();
        let mut current_loss: f64 = f64::MAX;
        let mut w_slope: f64 = 0.0;
        let mut b_slope: f64 = 0.0;

        for _ in 0..epoch {
            for i in 0..feature.len() {
                predict[i] = LogisReg1D::sigmoid(weight * feature[i] + bias);
            }

            current_loss = loss::vector_mean_log_loss(&label, &predict);

            w_slope = LogisReg1D::weight_slope(&label, &feature, weight, bias);
            b_slope = LogisReg1D::bias_slope(&label, &feature, weight, bias);

            weight = weight - (learning_rate * w_slope);
            bias = bias - (learning_rate * b_slope);

            if current_loss == 0.0 {
                println!("Mean log loss has reached 0: gradient descent ends.");
                break;
            }
        }

        (weight, bias, current_loss)
    }

    pub fn train(&mut self, learning_rate: f64, total_epoch: u64, rec_times: u64) -> Vector {
        println!("╭─");
        println!("│ Logistic Regression 1-Dimension Model training results:");

        let epoch: u64 = total_epoch / rec_times;
        let remained_epoch: u64 = total_epoch % rec_times;

        let mut loss_records = Vector::new();

        for _ in 0..rec_times {
            let (new_weight, new_bias, new_mse) = self.gradient_descent(learning_rate, epoch);
            self.weight = new_weight;
            self.bias = new_bias;
            loss_records.push(new_mse);
        }

        if remained_epoch != 0 {
            let (new_weight, new_bias, new_mse) =
                self.gradient_descent(learning_rate, remained_epoch);
            self.weight = new_weight;
            self.bias = new_bias;
            loss_records.push(new_mse);
        }

        println!(
            "│ Learning rate: {}, Total epoch: {}",
            learning_rate, total_epoch
        );
        println!(
            "│ Loss recorded {} times, with every {} epochs",
            rec_times, epoch
        );
        println!("├─");
        println!("│ Current model: y = 1 / [1 + e^(-z)], z = ({}) * x + ({})", self.weight, self.bias);
        println!("│ Current Mean Log Loss: {}", loss_records.find_last());
        println!("╰─");

        loss_records
    }
}

impl LogisRegModel for LogisReg1D {
    type Weight = f64;
    type Bias = f64;

    fn display(&self) {}
}
