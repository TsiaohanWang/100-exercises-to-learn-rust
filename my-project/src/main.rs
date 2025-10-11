use my_project::classification::model::ConfusMatrix1D;
use my_project::data_structure::Vector;
use my_project::logistic_regression::model::LogisReg1D;
use my_project::visualization::{curve, Precision};

fn main() {
    // --- 1. 数据准备 ---
    // 我们将创建一组有重叠区域的数据，以避免“完美分离”问题，
    // 这样可以更好地观察到一条平滑的ROC曲线。

    // 为标签为 0.0 的类别创建特征数据 (数值偏小)
    let features_class_0: Vector = vec![
        5.0, 7.0, 8.0, 10.0, 11.0, 13.0, 14.0, 16.0
    ].into();
    let labels_class_0: Vector = vec![0.0; features_class_0.len()].into();

    // 为标签为 1.0 的类别创建特征数据 (数值偏大)
    // 注意：与 class_0 的数据存在重叠区域 (例如 12.0 < 13.0, 14.0 < 15.0)
    let features_class_1: Vector = vec![
        12.0, 15.0, 17.0, 18.0, 19.0, 21.0, 23.0, 25.0
    ].into();
    let labels_class_1: Vector = vec![1.0; features_class_1.len()].into();

    // 将两组数据合并
    let mut feature_data = features_class_0.clone();
    feature_data.append(&mut features_class_1.clone());
    let mut label_data = labels_class_0.clone();
    label_data.append(&mut labels_class_1.clone());

    println!("--- 数据准备完毕 ---");
    println!("特征数据: {}", feature_data);
    println!("标签数据: {}", label_data);
    println!("");


    // --- 2. 模型初始化与训练 ---
    
    // 使用准备好的数据创建一个新的逻辑回归模型实例
    let mut log_reg = LogisReg1D::new(
        "label".into(), 
        "feature".into(), 
        label_data, 
        feature_data
    );

    // 训练模型
    // - learning_rate: 学习率，控制每次更新的步长
    // - total_epoch: 总训练轮次
    // - rec_times: 训练过程中打印/记录日志的次数
    // 对于这类数据，可能需要较小的学习率和较多的训练次数。
    log_reg.train(0.01, 20000, 100);
    println!("");


    // --- 3. 模型评估 (计算并绘制ROC曲线) ---

    println!("--- 开始进行模型评估 ---");
    // **重要**: 在模型训练完成后，再创建一个新的 ConfusMatrix1D 实例。
    // 这样它才能使用模型学习到的最终权重和偏置。
    // 初始阈值 (0.5) 在这里仅用于初始化，在 display_roc 中会被覆盖。
    let mut conf_m = ConfusMatrix1D::new(&log_reg, 0.5);

    // 计算在不同阈值下的 TPR 和 FPR 向量
    // Precision::F01 表示阈值从 0.0 到 1.0，步长为 0.01
    let (fpr_vec, tpr_vec, _) = conf_m.display_roc(Precision::F001);

    // 将计算出的 ROC 数据绘制并保存到文本文件中
    let file_path = "roc_curve_output.txt";
curve::roc_curve((fpr_vec, tpr_vec, Precision::F001), file_path);
}