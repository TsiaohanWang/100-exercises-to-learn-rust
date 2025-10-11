use my_project::classification::model::ConfusMatrix1D;
use my_project::data_structure::Vector;
use my_project::logistic_regression::model::LogisReg1D;
use my_project::visualization::Precision;
use plotters::prelude::*; // 确保已引入 plotters

// =============================================================================
// 将我之前提供的 plot_roc_curve 函数完整地粘贴在这里
fn plot_roc_curve(fpr: &Vector, tpr: &Vector, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("ROC Curve", ("sans-serif", 40).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0f64..1.0f64, 0.0f64..1.0f64)?;
    chart.configure_mesh()
        .x_desc("False Positive Rate (FPR)")
        .y_desc("True Positive Rate (TPR)")
        .draw()?;
    let roc_points = fpr.0.iter().zip(tpr.0.iter()).map(|(&x, &y)| (x, y));
    chart.draw_series(
        PointSeries::of_element(
            roc_points,
            5,
            &BLUE,
            &|coord, size, style| {
                EmptyElement::at(coord) + Circle::new((0, 0), size, style.filled())
            },
        )
    )?
    .label("Model Performance")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    chart.draw_series(LineSeries::new(
        vec![(0.0, 0.0), (1.0, 1.0)],
        &RED.mix(0.5),
    ))?
    .label("Random Guess")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED.mix(0.5)));
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    root.present()?;
    println!("绘图完成，图像已保存至 {}", output_path);
    Ok(())
}
// =============================================================================


fn main() {
    // --- 1. 数据准备 (保持不变) ---
    let features_class_0: Vector = vec![5.0, 7.0, 8.0, 10.0, 11.0, 13.0, 14.0, 16.0].into();
    let labels_class_0: Vector = vec![0.0; features_class_0.len()].into();
    let features_class_1: Vector = vec![12.0, 15.0, 17.0, 18.0, 19.0, 21.0, 23.0, 25.0].into();
    let labels_class_1: Vector = vec![1.0; features_class_1.len()].into();
    
    let mut feature_data = features_class_0.clone();
    feature_data.append(&mut features_class_1.clone());
    let mut label_data = labels_class_0.clone();
    label_data.append(&mut labels_class_1.clone());
    
    println!("--- 数据准备完毕 ---");

    // --- 2. 模型初始化与训练 (保持不变) ---
    let mut log_reg = LogisReg1D::new("label".into(), "feature".into(), label_data, feature_data);
    log_reg.train(0.01, 20000, 100);
    println!("");

    // --- 3. 模型评估 (计算并绘制ROC曲线) ---
    println!("--- 开始进行模型评估 ---");
    let mut conf_m = ConfusMatrix1D::new(&log_reg, 0.5);
    
    let (fpr_vec, tpr_vec, _) = conf_m.display_roc(Precision::F001);

    // *** 这是需要修正的关键部分 ***

    // 第 1 步：确保文件名以 .png 结尾
    let file_path = "roc_curve.png"; 

    // 第 2 步：确保您调用的是新的 plot_roc_curve 函数
    if let Err(e) = plot_roc_curve(&fpr_vec, &tpr_vec, file_path) {
        eprintln!("绘制 ROC 曲线时出错: {}", e);
    }
}