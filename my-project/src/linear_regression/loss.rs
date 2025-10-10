use crate::data_structure::Vector;

pub fn vector_l1_loss(actual_vec: &Vector, predict_vec: &Vector) -> f64 {
    if actual_vec.is_empty() || predict_vec.is_empty() {
        panic!("Calculating L1 loss / MAE, vectors cannot be empty!");
    }
    if actual_vec.len() != predict_vec.len() {
        panic!("Calculating L1 loss / MAE, vectors must have the same length!");
    }

    actual_vec
        .0
        .iter()
        .zip(predict_vec.0.iter())
        .map(|(a, p)| (a - p).abs())
        .sum()
}

// mean absolute error
pub fn vector_mae(actual_vec: &Vector, predict_vec: &Vector) -> f64 {
    vector_l1_loss(actual_vec, predict_vec) / (actual_vec.len() as f64)
}

pub fn vector_l2_loss(actual_vec: &Vector, predict_vec: &Vector) -> f64 {
    if actual_vec.is_empty() || predict_vec.is_empty() {
        panic!("Calculating L2 loss / MSE / RMSE, vectors cannot be empty!");
    }
    if actual_vec.len() != predict_vec.len() {
        panic!("Calculating L2 loss / MSE / RMSE, vectors must have the same length!");
    }

    actual_vec
        .0
        .iter()
        .zip(predict_vec.0.iter())
        .map(|(a, p)| ((a - p) * (a - p)).abs())
        .sum()
}

// mean squared error
pub fn vector_mse(actual_vec: &Vector, predict_vec: &Vector) -> f64 {
    vector_l2_loss(actual_vec, predict_vec) / (actual_vec.len() as f64)
}

// root mean squared error
pub fn vector_rmse(actual_vec: &Vector, predict_vec: &Vector) -> f64 {
    vector_mse(actual_vec, predict_vec).sqrt()
}
