use crate::visualization::Precision;
use crate::{data_structure::Vector, logistic_regression::model::LogisReg1D};

#[derive(Debug, Clone)]
pub struct ConfusMatrix1D {
    pub model: LogisReg1D,
    pub true_pos: usize,
    pub fals_pos: usize,
    pub fals_neg: usize,
    pub true_neg: usize,
    pub threshold: f64,
}

impl ConfusMatrix1D {
    pub fn new(model: &LogisReg1D, threshold: f64) -> Self {
        let weight = model.weight;
        let bias = model.bias;
        let label = model.label_data.clone();
        let feature = model.feature_data.clone();

        let mut true_pos = 0;
        let mut fals_pos = 0;
        let mut fals_neg = 0;
        let mut true_neg = 0;

        let predict_vec: Vec<f64> = feature
            .unpack()
            .iter()
            .map(|f| LogisReg1D::sigmoid(weight * f + bias))
            .collect();

        let predictions = predict_vec
            .iter()
            .map(|prob| if *prob >= threshold { 1.0 } else { 0.0 });

        for (prediction, actual) in predictions.zip(label.unpack().iter()) {
            match (prediction, *actual) {
                (1.0, 1.0) => true_pos += 1,
                (1.0, 0.0) => fals_pos += 1,
                (0.0, 1.0) => fals_neg += 1,
                (0.0, 0.0) => true_neg += 1,
                _ => (),
            }
        }

        Self {
            model: model.clone(),
            true_pos,
            fals_pos,
            fals_neg,
            true_neg,
            threshold,
        }
    }

    pub fn refresh(&mut self) -> &mut Self {
        let weight = self.model.weight;
        let bias = self.model.bias;
        let label = self.model.label_data.clone();
        let feature = self.model.feature_data.clone();

        self.true_pos = 0;
        self.fals_pos = 0;
        self.fals_neg = 0;
        self.true_neg = 0;

        let predict_vec: Vec<f64> = feature
            .unpack()
            .iter()
            .map(|f| LogisReg1D::sigmoid(weight * f + bias))
            .collect();

        let predictions = predict_vec
            .iter()
            .map(|prob| if *prob >= self.threshold { 1.0 } else { 0.0 });

        for (prediction, actual) in predictions.zip(label.unpack().iter()) {
            match (prediction, *actual) {
                (1.0, 1.0) => self.true_pos += 1,
                (1.0, 0.0) => self.fals_pos += 1,
                (0.0, 1.0) => self.fals_neg += 1,
                (0.0, 0.0) => self.true_neg += 1,
                _ => (),
            }
        }

        self
    }

    pub fn display_roc(&mut self, precision: Precision) -> (Vector, Vector, Precision) {
        let (interval, iteration): (f64, usize) = match precision {
            Precision::F0001 => (0.0001, 10001),
            Precision::F001 => (0.001, 1001),
            Precision::F01 => (0.01, 101),
            Precision::F1 => (0.1, 11),
            Precision::DEFINE(intrv) => {
                if 1.0 % intrv != 0.0 {
                    panic!("Your precision is invalid when defining ROC example interval!")
                }

                (intrv, (1.0 / intrv + 1.0) as usize)
            }
        };

        let restored_threshold = self.threshold;

        self.threshold = 0.0;

        let mut fpr_data = Vector::with_capacity(iteration);

        let mut tpr_data = Vector::with_capacity(iteration);

        for _ in 0..iteration {
            self.refresh();
            fpr_data.push(self.fpr());
            tpr_data.push(self.tpr());
            self.threshold += interval;
        }

        self.threshold = restored_threshold;
        self.refresh();

        (fpr_data, tpr_data, precision)
    }

    pub fn accuracy(&self) -> f64 {
        let t_p = self.true_pos as f64;
        let t_n = self.true_neg as f64;
        let f_p = self.fals_pos as f64;
        let f_n = self.fals_neg as f64;

        (t_p + t_n) / (t_p + t_n + f_p + f_n)
    }

    // true positive rate
    fn tpr(&self) -> f64 {
        let t_p = self.true_pos as f64;
        let f_n = self.fals_neg as f64;

        t_p / (t_p + f_n)
    }

    // false positive rate
    fn fpr(&self) -> f64 {
        let t_n = self.true_neg as f64;
        let f_p = self.fals_pos as f64;

        f_p / (f_p + t_n)
    }

    fn precision(&self) -> f64 {
        let t_p = self.true_pos as f64;
        let f_p = self.fals_pos as f64;

        t_p / (t_p + f_p)
    }

    fn f1_score(&self) -> f64 {
        let t_p = self.true_pos as f64;
        let f_p = self.fals_pos as f64;
        let f_n = self.fals_neg as f64;

        2.0 * t_p / (2.0 * t_p + f_p + f_n)
    }
}
