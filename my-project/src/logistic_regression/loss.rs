use crate::data_structure::Vector;

pub fn vector_log_loss(actual_vec: &Vector, predict_vec: &Vector) -> f64 {
    if actual_vec.is_empty() || predict_vec.is_empty() {
        panic!("Calculating Logistic loss, vectors cannot be empty!");
    }
    if actual_vec.len() != predict_vec.len() {
        panic!("Calculating Logistic loss, vectors must have the same length!");
    }
    for i in 0..actual_vec.len() {
        if actual_vec[i] == 0.0 || actual_vec[i] == 1.0 {
            ()
        } else {
            panic!("In logistic regression, the label for the example must either be 0 or 1!")
        }

        if predict_vec[i] < 0.0 || predict_vec[i] > 1.0 {
            panic!(
                "In logistic regression, the prediction for the example must be between 0 and 1!"
            )
        } else {
            ()
        }
    }

    let n = actual_vec.len() as f64;

    let total_loss: f64 = actual_vec
        .unpack()
        .iter()
        .zip(predict_vec.unpack().iter())
        .map(|(a, p)| a * p.ln() + (1.0 - a) * (1.0 - p).ln())
        .sum();

    total_loss / (-n)
}

pub fn vector_mean_log_loss(actual_vec: &Vector, predict_vec: &Vector) -> f64 {
    vector_log_loss(actual_vec, predict_vec) / (actual_vec.len() as f64)
}
