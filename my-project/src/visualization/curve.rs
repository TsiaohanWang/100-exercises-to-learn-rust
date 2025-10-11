use crate::{data_structure::vector::Vector, visualization::Precision};
use std::fs;
use std::io;
use std::path::Path;

pub fn roc_curve(
    (fpr_vec, tpr_vec, precision): (Vector, Vector, Precision),
    path: impl AsRef<Path>,
) {
    if fpr_vec.is_empty() || tpr_vec.is_empty() {
        panic!("Cannot visualize ROC curve: data cannot be empty!")
    }
    if fpr_vec.len() != tpr_vec.len() {
        panic!("Cannot visualize ROC curve: FPR data amount and TPR data amount is not equal!")
    }
    for i in 0..fpr_vec.len() {
        if fpr_vec[i] < 0.0 || fpr_vec[i] > 1.0 {
            panic!("Cannot visualize ROC curve: FPR data is not in [0, 1]!")
        }
        if tpr_vec[i] < 0.0 || tpr_vec[i] > 1.0 {
            panic!("Cannot visualize ROC curve: TPR data is not in [0, 1]!")
        }
    }
    let (interval, example_num): (f64, usize) = match precision {
        Precision::F0001 => (0.0001, 10001),
        Precision::F001 => (0.001, 1001),
        Precision::F01 => (0.01, 101),
        Precision::F1 => (0.1, 11),
        Precision::DEFINE(intrv) => {
            if 1.0 % intrv != 0.0 {
                panic!("Your precision is invalid when defining ROC example interval!")
            }

            (intrv, (1.0 / intrv + 1.0) as usize)
        }
    };
    if example_num != fpr_vec.len() || example_num != tpr_vec.len() {
        panic!("Precision mismatched the data!")
    }

    let tuple_vec: Vec<(f64, f64)> = fpr_vec
        .0
        .iter()
        .zip(tpr_vec.0.iter())
        .map(|(fpr, tpr)| (fpr.clone(), tpr.clone()))
        .collect();

    roc_curve_canvas(&tuple_vec, interval, path);
}

pub fn roc_curve_canvas(points: &Vec<(f64, f64)>, interval: f64, path: impl AsRef<Path>) {
    const POINT: char = '•';
    const V_AXIS: char = '│';
    const H_AXIS: char = '─';
    const ORIGIN: char = '┼';
    const EMPTY: char = ' ';

    let write_char_at = |x: usize, y: usize, character: char| -> io::Result<()> {
        let content = fs::read_to_string(path.as_ref()).unwrap_or_else(|_| String::new());

        // --- 2. 将内容分割成行 ---
        // 使用 .lines() 分割，然后转换为 Vec<String> 以便修改。
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        // --- 3. 修改内存中的表示 ---
        // a. 确保目标行存在。如果 y 超出现有行数，则添加空行。
        while lines.len() <= y {
            lines.push(String::new());
        }

        // b. 将目标行转换为字符向量，以便安全地处理 UTF-8 字符。
        let mut line_chars: Vec<char> = lines[y].chars().collect();

        // c. 确保目标列存在。如果行太短，则用空格填充。
        while line_chars.len() <= x {
            line_chars.push(' ');
        }

        // d. 在指定位置放置或覆盖字符。
        line_chars[x] = character;

        // e. 将修改后的字符向量转换回 String。
        lines[y] = line_chars.into_iter().collect();

        // --- 4. 将所有行重新组合成一个字符串 ---
        // 使用 "\n" 作为换行符连接所有行。
        let output_content = lines.join("\n");

        // --- 5. 将新内容写回文件 ---
        // fs::write 会自动处理打开、截断、写入和关闭文件的整个过程。
        fs::write(path.as_ref(), output_content)?;

        Ok(())
    };

    let iteration = (1.0 / interval) as usize;
    let distance = interval / 2.0;
    let mut grid_plots: Vec<(f64, f64)> = Vec::with_capacity(iteration);

    let mut grid_axis: f64 = 0.0;

    for i in 0..iteration {
        grid_plots.push((grid_axis, grid_axis + interval));

        grid_axis += interval;
    }

    let mut grid_points: Vec<(f64, f64)> = Vec::with_capacity(points.len());

    for (plot_x, plot_y) in points.clone() {
        let mut bucket: ((f64, f64), (f64, f64)) = ((0.0, 1.0), (0.0, 1.0));

        for (lower_x, upper_x) in (&grid_plots).clone() {
            for (lower_y, upper_y) in (&grid_plots).clone() {
                if plot_x > lower_x && plot_x <= upper_x && plot_y > lower_y && plot_y <= upper_y {
                    bucket = ((lower_x, upper_x), (lower_y, upper_y));
                    break;
                }
            }
        }

        let ((lower_x, upper_x), (lower_y, upper_y)) = bucket;

        let mut grid_plot: (f64, f64) = (0.0, 1.0);

        if plot_x >= lower_x + distance {
            grid_plot.0 = upper_x;
        } else {
            grid_plot.0 = lower_x;
        }
        if plot_y >= lower_y + distance {
            grid_plot.1 = upper_y;
        } else {
            grid_plot.1 = lower_y;
        }

        grid_points.push(grid_plot);
    }

    let grid_pos: Vec<(usize, usize)> = grid_points
        .iter()
        .map(|(x, y)| {
            (
                (*x * (iteration as f64)) as usize,
                (*y * (iteration as f64)) as usize,
            )
        })
        .collect();

    for line in 1..(iteration + 2) {
        write_char_at(2, line, V_AXIS);
    }
    for col in 2..(iteration + 3) {
        write_char_at(col, iteration + 1, H_AXIS);
        if col == 2 {
            write_char_at(col, iteration + 1, ORIGIN);
        }
    }
    for (px, py) in grid_pos {
        write_char_at(px + 2, iteration + 1 - py, POINT);
    }
}
