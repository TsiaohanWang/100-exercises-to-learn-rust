use crate::logistic_regression::LogisRegModel;

pub struct ConfusMatrix<T: LogisRegModel> {
    model: T,
    true_pos: usize,
    fals_pos: usize,
    fals_neg: usize,
    true_neg: usize,
    threshold: f64,
}

impl<T: LogisRegModel> ConfusMatrix<T> {
    fn accuracy(&self) -> f64 {
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