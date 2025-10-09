use my_project::data_structure::*;
use my_project::linear_regression;
use my_project::linear_regression::MLModel;
use my_project::linear_regression::model::LinearReg1D;

fn main() {
    let v: Vector = vec![1, 2, 4].into();
    let s: Vector = vec![3, 5, 7].into();
    println!("{}", &v + &s);

    let m: Matrix = vec![
        vec![1, 2, 4],
        vec![3, 5, 7],
        vec![4, 8, 10]
    ].into();

    println!("{} is matrix m", &m);
    println!("");
    println!("{}", &(&m + &m) * &s );
    println!("");
    println!("{}", &v * 5.0f64);

    println!("");

    let model = LinearReg1D::new("label".into(), "feature".into(), v.clone(), v);
    model.gradient_descent(0.0001, 50000);
}