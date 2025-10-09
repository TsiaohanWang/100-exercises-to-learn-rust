use my_project::ml_data_structure::*;

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
    
    println!("{}", &(&m + &m) * &s );
}