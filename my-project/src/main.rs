use my_project::classification::model::ConfusMatrix1D;
use my_project::data_structure::*;
use my_project::linear_regression::model::LinearReg1D;
use my_project::logistic_regression::model::LogisReg1D;
use my_project::visualization::curve;
use my_project::visualization::Precision;

fn main() {
    let v: Vector = vec![1, 2, 4].into();
    let s: Vector = vec![3, 5, 7].into();
    println!("{}", &v + &s);

    let m: Matrix = vec![vec![1, 2, 4], vec![3, 5, 7], vec![4, 8, 10]].into();

    println!("");

    let mut model = LinearReg1D::new("label".into(), "feature".into(), v.clone(), v);
    model.gradient_descent(0.001, 50000);

    let label: Vector = vec![1, 0, 1, 0, 0, 1, 1].into();
    let feature: Vector = vec![1, 0, 1, 0, 0, 1, 1].into();

    let mut log_reg = LogisReg1D::new("label".into(), "feature".into(), label, feature);

    let mut conf_m = ConfusMatrix1D::new(&log_reg, 0.6);

    model.train(0.0001, 100000, 1000);

    log_reg.train(0.001, 100000, 100);

    let file_path = "my_text_canvas.txt";

    curve::roc_curve(conf_m.display_roc(Precision::F01), file_path);

    let l:Vec<(f64,f64)> = vec![(0.0,1.0),(0.25,0.5),(0.5,0.5),(1.0,0.25)].into();

    curve::roc_curve_canvas(&l, 0.25, file_path);
}
