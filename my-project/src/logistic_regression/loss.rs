use crate::data_structure::Vector;

pub fn vector_log_loss(actual_vec: &Vector, predict_vec: &Vector) -> f64 {
    if actual_vec.is_empty() || predict_vec.is_empty() {
        panic!("Calculating Logistic loss, vectors cannot be empty!");
    }
    if actual_vec.len() != predict_vec.len() {
        panic!("Calculating Logistic loss, vectors must have the same length!");
    }
    for i in actual_vec.0 {
        if actual_vec[i] == 0 || actual_vec[i] == 1 {
            ()
        } else {
            panic!(
                "In logistic regression, the label for the example must either be 0 or 1!"
            )
        }

        if predict_vec[i] < 0 || predict_vec[i] > 1 {
            panic!(
                "In logistic regression, the prediction for the example must be between 0 and 1!"
            )
        } else {
            ()
        }
    }

    let n = actual_vec.len() as f64;

    let total_loss = actual_vec
        .0
        .iter()
        .zip(predict_vec.0.iter())
        .map(|(a, p)| a * p.ln() + (1.0 - a) * (1.0 - p).ln())
        .sum();

    total_loss / (-n)
}
