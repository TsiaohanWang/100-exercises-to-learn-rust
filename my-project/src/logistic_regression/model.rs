pub struct LogisReg1D {
    base: LinearReg1D
}

impl LogisReg1D {
    fn sigmoid(input: f64) -> f64 {
        if input >= 0.0 {
            1.0 / (1.0 + (-input).exp())
        } else {
            let exp = input.exp();
            exp / (1.0 + exp)
        }
    }
}