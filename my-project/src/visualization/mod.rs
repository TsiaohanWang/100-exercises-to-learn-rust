pub mod curve;

pub enum Precision {
    F0001,
    F001,
    F01,
    F1,
    DEFINE(f64)
}