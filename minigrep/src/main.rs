use std::ops::Deref;

struct MyBox<T>(T); // 创建自定义类型，它包含一个内部值

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> { // 定义 new 方法
        MyBox(x)
    }
}

// impl<T> Deref for MyBox<T> { // 实现 Deref
//     type Target = T; // 转换目标是 T 类型
// 
//     fn deref(&self) -> &Self::Target {
//         &self.0 // 返回元组结构体的唯一元素的引用
//     }
// }

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]); // 这里 &m 在底层被转换为什么？
}