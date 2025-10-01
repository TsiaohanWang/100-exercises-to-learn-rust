## 数据类型

### Rust 的变量及其遮蔽机制

变量默认是不可变的，但可以通过 `let` 关键字重新绑定一个变量名来实现变量的遮蔽（shadowing）。遮蔽允许你在同一作用域内使用相同的变量名，而不会引起冲突。

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x); // 输出 5

    let x = x + 1; // 这里的 x 遮蔽了外层的 x
    println!("The value of x is: {}", x); // 输出 6

    {
        let x = x * 2; // 这里的 x 遮蔽了外层的 x
        println!("The value of x in the inner scope is: {}", x); // 输出 12
    }

    println!("The value of x is: {}", x); // 输出 6，外层的 x 没有被改变
}
```

### Rust 的整型字面值写法

可以直接在数字后指定类型后缀来定义整型字面值：例如 `42u8`、`42i32`、`42isize` 等。

允许使用下划线来分隔数字以提高可读性，例如 `1_000_000`。

### 整型溢出

Rust 在编译时会检查整型溢出，并在发现溢出时抛出错误。在运行时，如果启用了 `debug` 模式，Rust 会在发生溢出时 panic；如果在 `release` 模式下，Rust 会进行环绕（wrapping）操作。

### 浮点型

所有的浮点型都是有符号的。

### 数学运算

整数除法会向零舍入到最接近的整数。

```rust
fn main() {
    let truncated = -5 / 3; // 结果为 -1
}
```

### 字符类型

用单引号声明 char 字面值，而与之相反的是，使用双引号声明字符串字面值。

### 复合类型：元组与数组

元组是一个将多个不同类型的值组合进一个复合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小。

我们使用包含在圆括号中的逗号分隔的值列表来创建一个元组。

使用 let 解构元组，它将一个元组拆成了若干部分，这样我们就可以将元组的各个部分绑定到不同的变量上。

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup; // 解构元组

    println!("The value of y is: {y}");
}
```

也可以使用点号（.）后跟值的索引来直接访问所需的元组元素。

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

不带任何值的元组有个特殊的名称，叫做单元（unit）元组。这种值以及对应的类型都写作 `()`。如果表达式不返回任何其他值，则会隐式返回单元值。

Rust 中的数组长度是固定的。我们将数组的值写成在方括号内，用逗号分隔的列表，例如 `[1, 2, 3]`。

当你确定元素个数不会改变时，数组会很有用。但是数组并不如 vector 类型灵活。vector 类型是标准库提供的一个 允许 增长和缩小长度的类似数组的集合类型。当不确定是应该使用数组还是 vector 的时候，那么很可能应该使用 vector。

创建每个元素都相同的数组可以通过在方括号中指定初始值加分号再加元素个数的方式来实现，例如 `[3; 5]` 表示创建一个包含 5 个值为 3 的元素的数组。

当尝试用索引访问一个元素时，Rust 会检查指定的索引是否小于数组的长度。如果索引超出了数组长度，Rust 会 panic。

### `size_of` 函数

对于存放在栈上的类型，可以使用 `std::mem::size_of` 函数来获取其占用的字节数。注意类型是以泛型形式传递给函数的。

```rust
assert_eq!(std::mem::size_of::<u8>(), 1);
// u8 占用 1 个字节，即 8 位
```

但是像 String 这样的类型在栈上存储的只是指向堆上数据的指针、长度和容量，因此 `size_of` 返回的是这些数据在栈上占用的字节数，而不是堆上数据的字节数。在 Rust 中容量、长度和指针都表示为 `usize` 类型。

机器上的每个内存位置都有一个地址，通常表示为一个无符号整数。根据地址空间的最大大小（即您的机器可以寻址多少内存），这个整数可以有不同的大小。大多数现代机器使用 32 位或 64 位的地址空间。Rust 通过提供 `usize` 类型来抽象掉这些特定于架构的细节：它是一个无符号整数，其大小与您机器上寻址内存所需的字节数相同。在 32 位机器上，`usize` 等同于 `u32`。在 64 位机器上，它与 `u64` 相匹配。

```rust
assert_eq!(size_of::<String>(), 24);
// 在 64 位机器上，String 在栈上占用 24 个字节：3 个 usize，每个 usize 占用 8 个字节
```

对于大多数引用（不论是否可变），它们都是以指针形式存在的，因此引用的大小也是 `usize`。

```rust
assert_eq!(size_of::<&u16>(), 8);
assert_eq!(std::mem::size_of::<&String>(), 8);
assert_eq!(std::mem::size_of::<&mut String>(), 8);
// 在 64 位机器上的情形
```

## 函数

Rust 代码中的函数和变量名使用 snake case 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词。

Rust 不关心函数定义所在的位置，只要函数被调用时出现在调用之处可见的作用域内就行（也就是说函数定义可以在调用该函数之后再进行）。

当函数拥有参数（形参）时，可以为这些参数提供具体的值（实参）。在函数签名中，必须声明每个参数的类型。当定义多个参数时，使用逗号分隔。

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

## 语句与表达式

函数定义也是语句，调用函数并不是语句。语句不返回值，因此不能把 let 语句赋值给另一个变量。


函数调用是一个表达式。宏调用是一个表达式。用大括号创建的一个新的块作用域也是一个表达式。表达式会计算出一个值。表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1 // 注意这里没有分号
    };

    println!("The value of y is: {y}");
}
```

函数可以向调用它的代码返回值。我们并不对返回值命名，但要在箭头（->）后声明它的类型。在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。使用 return 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式。

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five(); // 等同于 let x = 5;

    println!("The value of x is: {x}");
}
```

## 控制流

### if 表达式

所有的 if 表达式都以 if 关键字开头，其后跟一个条件。代码中的条件必须是 bool 值。

可以将 else if 表达式与 if 和 else 组合来实现多重条件。

因为 if 是一个表达式，我们可以在 let 语句的右侧使用它。

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}"); // The value of number is: 5
}
```

if 的每个分支的可能的返回值都必须是相同类型，如果它们的类型不匹配，则会出现一个错误。

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" }; // 错误：if 和 else 分支的类型不匹配

    println!("The value of number is: {number}");
}
```

### 循环

Rust 有三种循环：loop、while 和 for。

loop 关键字告诉 Rust 一遍又一遍地执行一段代码直到你明确要求停止。使用 break 关键字来告诉程序何时停止循环。continue 关键字告诉程序跳过这个循环迭代中的任何剩余代码，并转到下一个迭代。

可以在用于停止循环的 break 表达式后添加你希望返回的值，这个值就会作为循环的返回值返回。

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // counter * 2 作为循环的返回值
        }
    };

    println!("The result is {result}"); // The result is 20
}
```

如果存在嵌套循环，break 和 continue 默认应用于此时最内层的循环。可以选择在一个循环上指定一个循环标签（loop label），然后将标签与 break 或 continue 一起使用，这样可以应用在指定循环上。

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop { // 外层循环 counting_up
        println!("count = {count}");
        let mut remaining = 10;

        loop { // 内层循环
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // 退出内层循环
            }
            if count == 2 {
                break 'counting_up; // 退出外层循环
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

while 条件循环：当条件为 true，执行循环。当条件不再为 true，循环结束。

除此以外，还可以使用 while 结构遍历集合中的元素，比如数组。

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

相比于 while 循环，更推荐使用 for 循环来遍历集合中的元素，因为 for 循环更简洁且不易出错。

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

for 循环的优势之一是可以使用 Range，它是标准库提供的类型，用来生成从一个数字开始到另一个数字之前结束的所有数字的序列。可以使用 rev来反转这个序列。

```rust
fn main() {
    for number in 1..4 {
        println!("{number}!"); // 输出 1! 2! 3!
    }

    for number in (1..4).rev() {
        println!("{number}!"); // 输出 3! 2! 1!
    }
}
```

## 所有权

所有权（ownership）是 Rust 用于如何管理内存的一组规则。

栈和堆都是代码在运行时可供使用的内存，但是它们的结构不同。栈遵循后进先出的原则，栈中的所有数据都必须占用已知且固定的大小。而堆是缺乏组织的，向堆放入数据时，你要请求一定大小的空间。内存分配器（memory allocator）在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置地址的指针，该过程称作在堆上分配内存（allocating on the heap），有时简称为 “分配”（allocating）。（将数据推入栈中并不被认为是分配）。入栈比在堆上分配内存要快，访问堆上的数据比访问栈上的数据慢。所有权的主要目的就是管理堆数据。

所有权的规则如下：

1. Rust 中的每一个值都有一个所有者（owner）。
2. 值在任一时刻有且只有一个所有者。
3. 当所有者离开作用域，这个值将被丢弃。

作用域是一个项（item）在程序中有效的范围。变量从声明的点开始直到当前作用域结束时都是有效的。当变量离开作用域后，Rust 自动隐式调用 drop 函数并清理变量的堆内存。

字符串字面值是不可变的，并不适合使用文本的每一种场景。Rust 有另一种字符串类型：String。这个类型管理被分配到堆上的数据，能够存储在编译时未知大小的文本。可以使用 from 函数基于字符串字面值来创建 String。

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() 在字符串后追加字面值

println!("{s}"); // 输出 "hello, world!"
```

多个变量可以采取不同的方式与同一数据进行交互。例如

```rust
let x = 5;
let y = x;
```

这里因为整数是有已知固定大小的简单值，x 的值拷贝到 y 中，这两个 5 都被压入了栈中。但下面的情况不同：

```rust
let s1 = String::from("hello");
let s2 = s1;
// println!("{}, world!", s1); // 这里会报错，因为 s1 的所有权被移动到 s2
```

第一条语句实际上发生了如下的事情：指向存放字符串内容内存的指针、长度和容量这一组数据存储在栈上，堆上存放内容 hello。

```
         s1
+----------+-------+       +-------+-------+
|   name   | value |       | index | value |
+----------+-------+       +-------+-------+
|   ptr    |   ----------> |   0   |   h   |
+----------+-------+       +-------+-------+
|   len    |   5   |       |   1   |   e   |
+----------+-------+       +-------+-------+
| capacity |   5   |       |   2   |   l   |
                           +-------+-------+
                           |   3   |   l   |
                           +-------+-------+
                           |   4   |   o   |
                           +-------+-------+
```

将 s1 赋值给 s2 时，Rust 从栈上拷贝了它的指针、长度和容量，但并没有复制指针指向的堆上数据。由于当 s2 和 s1 离开作用域时，它们都会尝试释放相同的内存。两次释放（相同）内存会导致内存污染，可能导致潜在的安全漏洞。为了确保内存安全，在 let s2 = s1; 之后，Rust 认为 s1 不再有效，因此无法使用 s1。这个行为被称为移动（move）。

由于任何值在任一时刻有且只有一个所有者，这里相当于 s1 的所有权被移动到了 s2。

Rust 永远也不会自动创建数据的 “深拷贝”（拷贝指针、长度和容量，而且拷贝数据本身）。如果确实需要深拷贝，可以使用 String 类型的 clone 方法。

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // 不光拷贝栈上的数据，还拷贝了堆上的数据

println!("s1 = {s1}, s2 = {s2}");
```

如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然有效。类似整型这样的存储在栈上的类型都实现了 Copy trait。

向函数传递值可能会移动或者复制，就像赋值语句一样。

```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
    // println!("{}", s); 这里会报错，因为 s 的所有权被移动了

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动到函数里，但由于 i32 是 Copy 的类型
    println!("{}", x);              // 所以在后面可继续使用 x

} // 这里，应该是 x 先移出了作用域，然后是 s。但因为 s 的值已被移走，因此没有特殊之处

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{some_string}");
} // 这里，some_string 移出作用域并调用 `drop` 方法。

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{some_integer}");
} // 这里，some_integer 移出作用域。没有特殊之处
```

返回值也可以转移所有权。

```rust
fn main() {
    let s1 = gives_ownership();        // gives_ownership 将它的返回值传递给 s1

    let s2 = String::from("hello");    // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被传入 takes_and_gives_back，它的返回值又传递给 s3
} // 此处，s3 移出作用域并被丢弃。s2 被 move，s1 移出作用域并被丢弃

fn gives_ownership() -> String { // gives_ownership 将会把返回值传入调用它的函数
    let some_string = String::from("yours"); // some_string 进入作用域
    some_string // 返回 some_string 并将其移至调用函数
}

// 该函数将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}
```

### 引用与借用

Rust 提供了一个不用获取所有权就可以使用值的功能，叫做引用（references）。引用像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。但与指针不同，引用在其生命周期内保证指向某个特定类型的有效值。用符号 & 来表示引用。将创建一个引用的行为称为借用（borrowing）。

当函数使用引用而不是实际值作为参数，无需返回值来交还所有权，因为就不曾拥有所有权。

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // 传入一个指向值 s1 的引用，但是并不拥有它

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize { // s 是 String 的引用
    s.len()
} // s 在这里离开作用域，但因为它并不拥有引用的值的所有权，所以什么也不会发生
```

正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。但是可以通过可变引用（mutable reference）让我们修改引用指向的值。

```rust
fn main() {
    let mut s = String::from("hello"); // s 必须是可变的

    change(&mut s); // 传入 s 的可变引用
}

fn change(some_string: &mut String) { // some_string 是 String 的可变引用
    some_string.push_str(", world"); // 修改引用指向的值
}
```

可变引用有很大的限制：如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用。也不能在拥有不可变引用的同时拥有可变引用。然而，多个不可变引用是可以的，因为不可变引用只能读取数据，不会影响其他引用者读取到的数据。这一限制以一种非常小心谨慎的方式允许可变性，防止同一时间对同一数据存在多个可变引用。好处是 Rust 可以在编译时就避免数据竞争（data race）。

```rust
let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
let r3 = &mut s; // 编译错误，因为不能在拥有不可变引用的同时拥有可变引用
```

一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。编译器可以在作用域结束之前判断不再使用的引用。

```rust
let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
println!("{r1} and {r2}"); // 这是最后一次使用 r1 和 r2
// 此位置之后 r1 和 r2 不再使用，所以它们的作用域到此结束

let r3 = &mut s; // 虽然不能在拥有不可变引用的同时拥有可变引用，但是这里不会发生编译错误
println!("{r3}");
```

在 Rust 中编译器确保引用永远也不会变成悬垂引用：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。

```rust
fn dangle() -> &String { // dangle 函数返回一个字符串的引用
    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃，其内存被释放。编译错误，因为它试图返回一个指向已被释放内存的引用
```

综上，在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。引用必须总是有效的。

### 切片（Slice）

切片（slice）允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一种引用，所以它不拥有所有权。

字符串 slice（string slice）是 String 中一部分值的引用，它看起来像这样：

```rust
let s = String::from("hello world");

let hello = &s[0..5]; // 从索引 0 开始时，也可以不写两个点号之前的值，即 &s[..5]
let world = &s[6..11]; // 到最后一个字节结束时，也可以不写两个点号之后的值，即 &s[6..]
// 整个字符串 slice 可以省略两头的值，写作 &s[..]
```

此时的切片结构如下：

```
         s
+----------+-------+      +-------+-------+
|   name   | value |      | index | value |
+----------+-------+      +-------+-------+
|   ptr    |   ----------->   0   |   h   |
+----------+-------+      +-------+-------+
|   len    |   11  |      |   1   |   e   |
+----------+-------+      +-------+-------+
| capacity |   11  |      |   2   |   l   |
+----------+-------+      +-------+-------+
                          |   3   |   l   |
                          +-------+-------+
       world              |   4   |   o   |
+----------+-------+      +-------+-------+
|   name   | value |      |   5   |       |
+----------+-------+      +-------+-------+
|   ptr    |   ----------->   6   |   w   |
+----------+-------+      +-------+-------+
|   len    |   5   |      |   7   |   o   |
+----------+-------+      +-------+-------+
                          |   8   |   r   |
                          +-------+-------+
                          |   9   |   l   |
                          +-------+-------+
                          |  10   |   d   |
                          +-------+-------+
```

字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内，如果尝试从一个多字节字符的中间位置创建字符串 slice，则程序将会因错误而退出。

字符串字面值实际是一个指向二进制程序特定位置的 slice，类型是 `&str`，因此字符串字面值是不可变的，因为 `&str` 是一个不可变引用。字符串 slice 并不拥有数据，它只是指向数据：它是一个对堆上分配的 `String` 数据的引用。当字符串 slice 被丢弃时，堆上分配的数据不会被释放，因为它仍然被 `s` 所拥有。这就是为什么 `world` 没有 `capacity`（容量）字段：它不拥有数据，所以它不需要知道为数据分配了多少空间；它只关心它所引用的数据。

字符串 slice 作为参数可以接收 String 或 &str，因为它们都可以被视为字符串 slice。

除了字符串切片（&str），还可以创建数组切片。

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..4]; // 创建一个指向数组 a 中元素 2、3、4 的切片
// 这个 slice 的类型是 &[i32]，它会存储第一个集合元素的引用和一个集合总长度
assert_eq!(slice, &[2, 3]); // 断言 slice 的值是否等于 &[2, 3]
```

当你需要一个对文本数据的引用时，使用 `&str` 而不是 `&String`。`&str` 更灵活，并且在 Rust 代码中通常被认为是更地道的用法：如果一个函数返回 `&String`，你是在承诺某处存在一个堆上分配的 UTF-8 文本，它与你返回的引用完全匹配；相反，如果一个函数返回 `&str`，你的自由度就大得多：你只是说某处有一堆文本数据，其中的一个子集符合你的需要，因此你返回对它的引用。

## 结构体

结构体（struct 或 structure），是一种自定义数据类型，允许你包装和命名多个相关的值，从而形成一个有意义的组合。

和元组一样，结构体的每一部分可以是不同类型。但不同于元组，结构体需要命名各部分数据以便能清楚的表明其值的意义。因此，结构体不需要依赖顺序来指定或访问实例中的值。

定义结构体时，需要使用 struct 关键字并为整个结构体命名。在后面的花括号中，定义每一部分数据的名字和类型，我们称为字段（field）。

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
} // 如果在函数体内定义结构体，结尾处需要加分号

struct User_updated {
    active: mut bool, // 报错，不能在定义结构体时将某个字段标记为 mut
    username: String,
    email: String,
    sign_in_count: u64,
}
```

一旦定义了结构体，为了使用它，需要创建这个结构体的实例。创建实例时以结构体的名字开头，接着在花括号中使用 key: value 的形式提供字段。不能在定义结构体时将任一字段标记为 mut。

默认情况下，实例是不可变的。若整个实例是可变的，则各个字段均为可变。

```rust
let user1 = User { // 创建一个 User 结构体的不可变实例 user1
    active: true,
    username: String::from("example_user"),
    email: String::from("user1@example.com"),
    sign_in_count: 1, // 各个字段的顺序无关紧要
};

user1.email = String::from("user1_updated@example.com"); // 报错，user1 是不可变实例，无法修改字段

let mut user2 = User { // 创建一个 User 结构体的可变实例 user2
    active: true,
    username: String::from("another_user"),
    email: String::from("user2@example.com"),
    sign_in_count: 2,
};

user2.email = String::from("user2_updated@example.com"); // user2 是可变实例，可以修改字段
```

函数可以返回结构体的实例，这可以通过在最后一个表达式中构造一个结构体的新实例来隐式返回它。如果函数的参数名与字段名都完全相同，可以使用字段初始化简写语法（field init shorthand）

```rust
fn build_user(email: String, username: String) -> User { //函数参数名与 User 字段名相同
    User {
        active: true,
        username, // 等价于 username: username,
        email, // 等价于 email: email,
        sign_in_count: 1,
    }
}
```

可以借助其他已有实例来创建新的实例，这种语法叫做结构体更新语法（struct update syntax）。使用 `已有实例.字段名` 语法来使用其他实例的字段值；使用 `..已有实例名` 语法来复制另一实例的其余字段值，注意此时必须放在最后。

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("username1"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active, // 从 user1 复制 active 字段
        username: String::from("username2"),
        email: String::from("user2@example.com"),
        sign_in_count: user1.sign_in_count, // 从 user1 复制 sign_in_count 字段
    };

    let user3 = User {
        email: String::from("user3@example.com"),
        ..user1 // 表示 user3 的其余字段与 user1 相同
    };
}
```

由于以上两种方式会将字段值从旧实例移动到新实例（相当于 `=` 赋值），因此如果旧实例的某个字段值没有实现 Copy trait，则旧实例的该字段将不再有效。

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("username1"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        username: String::from("username2"), // 这是新的 username 字段，不会影响 user1.username
        ..user1 // 这里会发生所有权的移动
    };

    println!("{}", user1.username); // 不会报错，因为 user2 创建了新的 username 字段

    println!("{}", user2.email); // user2 的 email 字段值为 "someone@example.com"

    println!("{}", user1.email); // 报错，user1 的 email 字段值已被移动到 user2

    println!("{}", user1.sign_in_count); // 不会报错，因为 sign_in_count 实现了 Copy trait，所以值被复制而不是被移动
}
```

你可以继续访问那些所有权未被移动的字段，但你不能再访问那些所有权已经被移动的字段。并且你不能再将旧实例作为一个整体来使用（例如 `let user4 = user1;`）。

可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要指定生命周期（lifetimes）。生命周期确保结构体引用的数据有效性跟结构体本身保持一致。如果你尝试在结构体中存储一个引用而不指定生命周期将是无效的。

```rust
struct User {
    active: bool,
    username: &str, // 报错，结构体中存储了一个引用，但没有指定生命周期
    email: &str, // 报错，结构体中存储了一个引用，但没有指定生命周期
    sign_in_count: u64,
}
```

### 元组结构体（tuple structs）

当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，元组结构体是很有用的。元组结构体有着结构体名称，但没有具体的字段名，只有字段的类型。

定义元组结构体时，以 struct 关键字和结构体名开头，后跟元组中的类型。

元组结构体实例类似于元组，你可以将它们解构为单独的部分，也可以使用 `.索引` 来访问单独的值。与元组不同的是，解构元组结构体时必须写明结构体的类型。

```rust
fn main() {
    struct Point(i32, i32, i32);
    struct Color(i32, i32, i32); // 定义两个元组结构体 Point 和 Color

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0); // 创建 Point 和 Color 的实例
    // 即使它们的字段类型和数量相同，它们仍然是不同的类型

    let (x, y, z) =  origin; // 报错，必须写明结构体的类型
    let Point(x, y, z) = origin; // 不会报错，解构 Point 结构体实例

    println!("{}", origin.0); // 使用 `.索引` 访问字段值
}
```

类单元结构体（unit-like structs）是一个没有任何字段的结构体。它们类似于 `()` 类型。类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。定义和实例化类单元结构体的语法极其简单，不需要花括号或圆括号。

```rust
struct AlwaysEqual; // 定义一个类单元结构体 AlwaysEqual

fn main() {
    let subject = AlwaysEqual; // 创建 AlwaysEqual 的实例
}
```

### 方法

方法（method）与函数类似：它们都使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。不过方法与函数也有不同，方法在结构体的上下文中被定义，或者是枚举或 trait 对象的上下文被定义，而且第一个参数总是 `self` ——代表调用该方法的结构体实例。

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // 为 Rectangle 结构体定义方法
    fn area(&self) -> u32 { // 方法的第一个参数是 &self，self 代表当前 impl 块类型（即 Rectangle）的实例
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // 采用方法语法（method syntax）调用 Rectangle 的 area 方法
    );
}
```

这里，使用一个 impl 块来定义 Rectangle 结构体的方法，这个 impl 块中的所有内容都将与 Rectangle 类型相关联。`&self` 实际上是 `self: &Self` 的缩写。在一个 `impl` 块中，`Self` 类型是 `impl` 块的类型的别名。方法的第一个参数必须有一个名为 `self` 的 `Self` 类型的参数，只能用 `self` 这个名字来简化。

之所以选择 `&self` 是因为我们并不想获取所有权，只希望能读取结构体中的数据，而不是写入。如果想要在方法中改变调用方法的实例，需要将第一个参数改为 `&mut self`。

方法的名称可以与结构体其中一个字段名相同。Rust 能区分何时调用方法（加圆括号），何时访问字段（不加圆括号）。

每个结构体都允许拥有多个 `impl` 块。

```rust
impl Rectangle {
    fn width(&self) -> bool { // 方法名称与某一字段名称 width 相同
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() { //这里调用 width 方法返回的布尔值
        println!("The rectangle has a nonzero width; it is {}", rect1.width); // 这里访问 width 字段的值
    }
}
```

Rust 具备自动引用和解引用（automatic referencing and dereferencing）的功能，方法调用是少数几种体现这种功能的地方之一。例如创建一个 `Rectangle` 实例 `rect1` 后再通过 `let rect = &rect1;` 来创建一个指向 `rect1` 的引用，此时能直接使用 `rect.area()` 来调用 `area` 方法，而不需要写成 `(*rect).area()`。Rust 会自动为我们添加解引用操作符。同理，Rust 还会自动添加 `&`、`&mut` 以便与方法签名匹配。

### 关联函数

所有在 `impl` 块中定义的函数被称为关联函数（associated functions），因为它们与 `impl` 后面的类型相关。我们可以定义不以 `self` 为第一参数的关联函数（因此就不是方法），因为它们并不作用于一个结构体的实例，而是作用于整个结构体类型。要调用这种关联函数，我们使用 `结构体名::关联函数名` 语法。

不是方法的关联函数经常被用作是返回一个结构体新实例的构造函数。它们总是被命名为 `new`，因为 `new` 不是关键字，可以作为函数名。

```rust
impl Rectangle {
    fn square(size: u32) -> Self { // 由于不以 self 为第一参数，它不是方法而是关联函数
        Self {
            width: size,
            height: size,
        } // 返回一个新的 Rectangle 实例，表明其作为构造函数的身份
    }
}

fn main() {
    let sq = Rectangle::square(3); // 调用关联函数 square，这里作为构造函数返回一个 Rectangle 实例

    println!(
        "The area of the square is {} square pixels.",
        sq.area()
    );
}
```

关键字 `Self` 在函数的返回类型和函数体中，都是对 `impl` 关键字后所示类型的别名。

> [!WARNING]
> 我们不能直接向外部类型（例如 `std` 中的 `u32`）附加新方法，但是 Rust 提供了一种采用扩展 trait 的方式。

## 枚举

结构体给予你将字段和数据聚合在一起的方法，而枚举给予你一个途径去声明某个值是一个集合中的一员。枚举出所有可能的值被称为枚举的变体（variants）。使用 `enum` 关键字来定义枚举。

```rust
enum IpAddrKind { // 定义一个枚举 IpAddrKind
    V4,
    V6, // V4 和 V6 是 IpAddrKind 的两个变体，没有关联任何数据
}
```

这样就可以创建变体的实例，例如 `let six = IpAddrKind::V6;`。枚举的变体位于其标识符的命名空间中，并使用两个冒号分开。

也可以定义函数来接收任何枚举类型的参数：`fn route(ip_kind: IpAddrKind) {}`。然后可以将任何 `IpAddrKind` 变体的实例传递给这个函数。

可以将枚举变体与数据一同放在结构体中，或者将数据直接放进每一个枚举变体而不是将枚举作为结构体的一部分。若是后者，每一个我们定义的枚举变体的名字也变成了一个构建枚举实例的函数。

```rust
enum IpAddr {
    V4(String), // V4 变体携带一个 String 类型的数据
    V6(String), // V6 变体携带一个 String 类型的数据
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1")); // IpAddr::V4() 是一个获取 String 参数并返回 IpAddr 类型实例的函数
    let loopback = IpAddr::V6(String::from("::1"));
}
```

枚举相比结构体还有另一个优势：每个变体可以处理不同类型和数量的数据。

```rust
struct V6Addr {
    // ...
}

enum IpAddr {
    V4(u8, u8, u8, u8), // V4 变体携带四个 u8 类型的数据
    V6(V6Addr), // V6 变体携带一个 V6Addr 结构体类型的数据
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
}
```

结构体和枚举还有另一个相似点：可以使用 `impl` 来在枚举上定义方法。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) { // 定义一个方法 call
        // ...
    }
}

fn main() {
    let m = Message::Write(String::from("hello")); // 创建 Message 枚举的一个实例
    m.call(); // 调用 Message 枚举实例的方法
}
```

### Option 枚举

`Option` 是标准库定义的另一个枚举。其应用广泛因为它编码了一个非常普遍的场景，即一个值要么有值要么没值。因为 Rust 并没有很多其他语言中有的空值功能，因为引入空值会带来很多问题，但是仍需要一种方式来表示因为某种原因目前无效或缺失的值。因此，Rust 拥有一个定义于标准库中，可以编码存在或不存在概念的枚举——`Option<T>`

```rust
enum Option<T> {
    None,
    Some(T),
}
```

无需将 `Option<T>` 显式引入作用域，它的变体也是如此：可以不需要 `Option::` 前缀来直接使用 `Some` 和 `None`，这是较之其他枚举的一个特殊之处。

`<T>` 语法是一个泛型类型参数，意味着 `Option` 枚举的 `Some` 变体可以包含任意类型的数据。

```rust
// 无需将 Option<T> 显式引入作用域
fn main() {
    let some_number = Some(5); // some_number 的类型是 Option<i32>
    let some_char = Some('e'); // some_char 的类型是 Option<char>

    let absent_number: Option<i32> = None;
    // 由于编译器只通过 None 值无法推断出 Some 变体保存的值的类型，因此需要显式指定类型
}
```

`Option<T>` 和 `T` 是不同的类型，它们之间无法直接操作，因此在对 `Option<T>` 进行运算之前必须将其转换为 `T`，通常这能帮助我们捕获到空值最常见的问题之一：假设某值不为空但实际上为空的情况。这限制了空值的泛滥以增加 Rust 代码的安全性。

### match 控制流结构

控制流运算符 `match` 允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码。模式可由字面值、变量、通配符和许多其他内容构成。

`match` 关键字后跟一个表达式（表达式可以是任意类型，并不像 `if` 那样一定是布尔值）。接下来花括号内是 `match` 的分支。一个分支有两个部分：一个模式和一些代码。`=>` 运算符将模式和将要运行的代码分开。分支之间使用逗号分隔。

当 `match` 后的表达式执行时，它将结果值按顺序与每一个分支的模式相比较。如果模式匹配了这个值，这个模式相关联的代码将被执行。每个分支相关联的代码是一个表达式，若该分支匹配成功，其结果将作为整个 `match` 表达式的值立即返回。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { // match 表达式是枚举实例 coin
        Coin::Penny => {
            println!("Lucky penny!");
            1 // 这里的表达式值为 1
        }
        Coin::Nickel => 5, // 这里的表达式值为 5
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

可以将 `match` 与枚举搭配，从而提取枚举变体中携带的数据。

```rust
#[derive(Debug)] // 这样可以立刻看到州的名称，因为我们使用了 {:?} 来打印枚举
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter 变体携带一个 UsState 类型的数据
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // 变量 state 将绑定到 Quarter 变体携带的数据上
            println!("State quarter from {state:?}!"); // 提取出 state，即枚举实例 coin 携带的数据
            25
        }
    }
}
// 如果调用 value_in_cents(Coin::Quarter(UsState::Alaska))
// state 就绑定在 UsState::Alaska 上
```

`match` 也可以匹配 `Option<T>` 枚举。

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None, // 如果 x 是 None，则返回 None
            Some(i) => Some(i + 1), // 如果 x 是 Some(i)，则返回 Some(i + 1)
                                    // 这里 i 绑定在 Some 内部的 i32 值上
        }
    }

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

match 需要注意：分支必须覆盖了所有的可能性，也就是说匹配必须是穷尽的，必须穷举到最后的可能性来使代码有效。可以使用通配模式来匹配所有尚未被列出的情况。

```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(), // 这里匹配了 3 和 7 两种情况
        other => move_player(other), // 使用我们命名的变量 other 来匹配其他所有情况
                                     // 即便我们没有列出 u8 所有情况
    }

    // 匹配结果是执行以下三个函数之一
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
```

必须将通配分支放在最后。如果我们在通配分支后添加其他分支，Rust 将会警告我们此后的分支永远不会被匹配到。

Rust 还提供了一个模式，当我们不想使用通配模式获取的值时，使用 `_` 可以匹配任意值而不绑定到该值。

```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // 使用 _ 来匹配其他所有情况，我们不需要此时的值
        // 或者这里用单元值：_ => (), 表示不执行任何操作
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
```

### if let / let else 控制流结构

`if let` 语法用于处理只匹配一个模式的值而忽略其他模式的情况，因为此时 `match`会显得冗长。获取通过等号分隔的一个模式和一个表达式。

```rust
fn main() {
    let config_max = Some(3u8);

    // 使用 match 语法来处理
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (), // 不需要此时的匹配值，也不需要执行任何操作
    }

    // 使用 if let 语法简化上面的 match
    if let Some(max) = config_max { // `if let 模式 = 表达式` 语法
        println!("The maximum is configured to be {max}");
    }
}
```

使用 `if let` 意味着编写更少代码，更少的缩进和更少的样板代码。但是会失去 `match` 强制要求的穷尽性检查。这是一个权衡取舍的问题。

如果需要对其他情况执行某些操作，可以添加一个 `else` 块。这相当于将 match 的 `_` 分支加入对应代码。

```rust
fn main() {
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1, // 对于其他情况不再是无操作
    }

    // 使用 if let 语法简化上面的 match
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else { // else 块中的代码与 match 的 `_` 分支相同
        count += 1;
    }
}
```

`let...else` 语法类似 `if let`，`let` 后跟 `模式 = 表达式`，不过它没有 `if` 分支，只有 `else` 分支。如果模式匹配，它会将匹配到的值绑定到外层作用域。如果模式不匹配，程序流会指向 `else` 分支。

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else { // let 模式 = 表达式 else
                                           // 如果模式匹配，会将匹配值绑定到 state 上，方便后续处理
                                           // 如果不匹配，程序流会直接指向 else 块
        println!("Not a quarter!");
        return None; // 返回 Option 枚举的 None 变体
    };

    Some(format!("State quarter from {state:?}!")) // 返回 Option 枚举的 Some 变体
    // 这里使用了前面与匹配值绑定的 state 变量
}
```

## 包、Crate 和模块

Rust 有许多功能可以让你管理代码的组织，包括哪些细节可以被公开，哪些细节作为私有部分，以及程序中各个作用域中有哪些名称。这些特性有时被统称为 “模块系统（the module system）”，包括：

1. 包（Packages）：Cargo 的一个功能，它允许你构建、测试和分享 crate。
2. Crates ：一个模块的树形结构，它形成了库或可执行文件项目。
3. 模块（Modules）和 use：允许你控制作用域和路径的私有性。
4. 路径（path）：一个为例如结构体、函数或模块等项命名的方式。

### 包和 Crate

crate 是 Rust 在编译时最小的代码单位。crate 有两种形式：二进制 crate 和库 crate。

1. 二进制 crate（Binary crates）可以被编译为可执行程序，它们必须有一个 `main` 函数。
2. 库 crate（Library crates）并没有 `main` 函数，也不会编译为可执行程序。相反它们定义了可供多个项目复用的功能模块。

crate root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块。

包（package）是提供一系列功能的一个或者多个 crate 的捆绑。一个包包含一个 `Cargo.toml` 文件，其阐述如何去构建这些 crate。

- Cargo 实际上就是一个包。
- 包中可以包含任意多个二进制 crate，至多包含一个库 crate，但必须至少包含一个 crate（无论是库的还是二进制的）。

使用 `cargo new` 指令创建一个包时，会自动生成 `Cargo.toml` 和 `src/main.rs` 等文件。

- Cargo 遵循着一个约定：`src/main.rs` 是一个与包同名的二进制 crate 的 crate 根。
- 如果包目录中包含 `src/lib.rs`，则包带有与其同名的库 crate，且 `src/lib.rs` 是 crate 根。
- 如果一个包同时含有 `src/main.rs` 和 `src/lib.rs`，则它有两个 crate：一个二进制的和一个库的，且名字都与包相同。
- crate 根文件将由 Cargo 传递给 rustc 来实际构建库或者二进制项目。

通过将文件放在 `src/bin` 目录下，一个包可以拥有多个二进制 crate，且每个 `src/bin` 下的文件都会被编译成一个独立的二进制 crate。

### 模块

模块让我们可以将一个 crate 中的代码进行分组，并且控制代码的私有性。

当编译一个 crate, 编译器首先在 crate 根文件（通常，对于一个库 crate 而言是 `src/lib.rs`，对于一个二进制 crate 而言是 `src/main.rs`）中寻找需要被编译的代码。

在 crate 根文件中，你可以使用 `mod 模块名;` 声明一个新模块。编译器会在下列路径中寻找模块代码：

1. 内联，用大括号替换 `mod 模块名` 后跟的分号。
2. 在文件 `src/模块名.rs`。
3. 在文件 `src/模块名/mod.rs`。

在除了 crate 根节点以外的任何文件中，你可以定义子模块。编译器会在以父模块命名的目录中寻找子模块代码。例如在 `src/模块名.rs` 中声明 `mod 子模块名;`，编译器会在下列路径中寻找子模块代码：

1. 内联，用大括号替换 `mod 子模块名` 后跟的分号。
2. 在文件 `src/模块名/子模块名.rs`。
3. 在文件 `src/模块名/子模块名/mod.rs`。

一旦一个模块是你 crate 的一部分，你可以在隐私规则允许的前提下，从同一个 crate 内的任意地方，通过代码路径引用该模块的代码。引用方式为 `crate::模块名::子模块名::...::项名`，例如 `crate::garden::vegetables::Asparagus`。

一个模块里的代码默认对其父模块私有。为了使一个模块公用，应当在声明时使用 `pub mod` 替代 `mod`。为了使一个公用模块内部的成员公用，应当在声明前使用 `pub`。

在一个作用域内，`use` 关键字创建了一个项的快捷方式，用来减少长路径的重复。在任何可以引用 `crate::模块名::子模块名::...::项名` 的作用域，你可以通过 `use crate::模块名::子模块名::...::项名;` 创建一个快捷方式，然后你就可以在作用域中只写 `项名` 来使用该类型。例如：

```
文件结构如下：
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs

其中，crate 根文件是 src/main.rs
```

```rust
// src/main.rs
use crate::garden::vegetables::Asparagus; // 这里用 use 创建了一个使用 Asparagus 的快捷方式
                                          // 这样后面就只需要写 Asparagus 了
                                          // 不需要每次都写完整路径 crate::garden::vegetables::Asparagus 了

pub mod garden; // 声明 garden 公用模块，编译器会在 src/garden.rs 中寻找模块代码并包含进来

fn main() {
    let plant = Asparagus {}; // 使用 Asparagus 里的代码
    println!("I'm growing {plant:?}!");
}
```

```rust
// src/garden.rs
pub mod vegetables; // 声明 vegetables 公用子模块，编译器会在 src/garden/vegetables.rs 中寻找模块代码并包含进来
```

```rust
// src/garden/vegetables.rs
#[derive(Debug)]
pub struct Asparagus {} // 定义一个公用结构体 Asparagus
```

`src/main.rs` 和 `src/lib.rs` 叫做 crate 根。之所以这样叫它们是因为这两个文件的内容都分别在 crate 模块结构的根组成了一个名为 `crate` 的模块，该结构被称为模块树（module tree）。

- 如果一个模块 `A` 被包含在模块 `B` 中，我们将模块 `A` 称为模块 `B` 的子（child）模块，模块 `B` 则是模块 `A` 的父（parent）模块。
- 互为兄弟（siblings）的模块定义在同一模块中。
- 整个模块树都植根于名为 `crate` 的隐式模块下。

```rust
// src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

```
上述代码的模块树如下所示：
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

### 路径与私有性

为了向 Rust 指示在模块树中从何处查找某个项，我们使用路径。路径有两种形式：

- 绝对路径（absolute path）是以 crate 根（root）开头的完整路径；对于外部 crate 的代码，是以 crate 名开头的绝对路径，对于当前 crate 的代码，则以字面值 `crate` 开头。
- 相对路径（relative path）从当前模块开始，以 `self`、`super` 或当前模块中的某个标识符开头。
- - 通过在路径的开头使用 `super`，从父模块开始构建相对路径

路径都使用双冒号 `::` 来分隔各部分。

在 Rust 中，所有项（函数、方法、结构体、枚举、模块和常量）默认对父模块都是私有的。如果希望创建一个如函数或结构体的私有项，可以将其放入一个模块。

父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用它们父模块中的项。

```rust
// src/lib.rs
mod front_of_house { // 定义一个模块 front_of_house，它与 eat_at_restaurant 同级
                     // 但是其子模块 hosting 是私有的
    mod hosting {
        fn add_to_waitlist() {}
    }
} // 虽然 front_of_house 模块并非公用的，但 eat_at_restaurant 函数可以访问它
  // 因为它们在同一个模块中（crate 根模块）

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist(); // 这里会报错，因为 hosting 是私有的

    // 相对路径
    front_of_house::hosting::add_to_waitlist(); // 这里也会报错
}
```

使模块公有并不能使其内容也是公有的。模块上的 `pub` 关键字只允许其父模块引用它，而不允许访问内部代码，因为模块是一个容器。

```rust
// 修改 src/lib.rs
mod front_of_house {
    pub mod hosting { // 使用 pub 关键字将 hosting 模块声明为公用
                      // 这样 eat_at_restaurant 函数就可以访问 hosting 模块了
                      // 但是 hosting 模块中的 add_to_waitlist 函数仍然是私有的
        fn add_to_waitlist() {}
    }
}
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist(); // 依旧会报错，因为 add_to_waitlist 是私有的

    // 相对路径
    front_of_house::hosting::add_to_waitlist(); // 这里也依旧会报错
}
```

```rust
// 再次修改 src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {} // 使用 pub 关键字将 add_to_waitlist 函数声明为公用
    }
}
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist(); // 编译成功

    // 相对路径
    front_of_house::hosting::add_to_waitlist(); // 这里也编译成功
}
```

可以使用 `pub` 来设计公有的结构体和枚举，不过关于在结构体和枚举上使用 `pub` 还有一些额外的区别：

- 结构体定义时使用 `pub`，这个结构体会变成公有的，但是这个结构体的字段仍然是私有的。我们可以根据情况决定每个字段是否公有（是否使用 `pub` 关键字）。
- 枚举定义时使用 `pub`，这个枚举和其所有的变体都会变成公有的。

### use 关键字

可以使用 `use` 关键字创建一个捷径，这样就不用每次都写出完整但冗长的路径了。注意通过 `use` 引入作用域的路径会检查私有性。

```rust
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; // 使用 use 创建一个快捷方式
                                    // 这样在作用域内就可以直接用 hosting 了

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // 这里直接使用 hosting 的 add_to_waitlist 函数
                                // 而不需要写成 crate::front_of_house::hosting::add_to_waitlist()
}
```

`use` 只能创建在其所在的特定作用域内的捷径。将某个名称导入当前作用域后，该名称对此作用域之外还是私有的。

```rust
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; // 使用 use 创建一个快捷方式
                                    // 它只作用在当前作用域内

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist(); // 编译错误，这里不在 use 的作用域内，不能直接使用 hosting
                                    // 应该写成完整路径
    }
}
```

一般我们都不会在 `use` 语句中精确到某个项，而是会引入一个模块。因为这清晰表明该项不是在本地定义的，同时使完整路径的重复度最小化。

- 例如上面的例子中使用 `use crate::front_of_house::hosting;`，而不是 `use crate::front_of_house::hosting::add_to_waitlist;`。因为后者会在代码中显示为 `add_to_waitlist()` 而非 `hosting::add_to_waitlist()`，这样就不清楚 `add_to_waitlist` 是在本地定义的还是通过 `use` 引入的。
- 如果别的模块中也有一个 `add_to_waitlist` 函数，那么同时在 `use` 中引入这两个函数就会引起冲突。

```rust
use std::fmt;
use std::io; // 引入两个不同的模块
             // 这两个模块中都有一个 Result 类型

fn function1() -> fmt::Result { // 使用 fmt 模块中的 Result 类型
    // ...
}

fn function2() -> io::Result<()> { // 使用 io 模块中的 Result 类型
    // ...
}
// 如果都写成 use std::fmt::Result; 和 use std::io::Result;
// 那么这两个 Result 类型同名会引起冲突
```

可以为 `use` 语句中的模块指定一个名称，使用 `as` 关键字来重命名。

```rust
use std::fmt::Result;
use std::io::Result as IoResult; // 使用 as 关键字为 io 模块中的 Result 类型指定别名 IoResult

fn function1() -> Result {
    // ...
}

fn function2() -> IoResult<()> { // 通过更名来避免冲突
    // ...
}
```

由于 `use` 只能创建在其所在的特定作用域内的捷径，为了让作用域之外的代码能够像在当前作用域中一样使用该名称，可以使用 `pub use`。这种技术被称为重导出（re-exporting）。

```rust
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting; // 使用 pub use 重导出 hosting 模块
                                        // 这样当前作用域之外也可以使用 hosting

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // 使用了重导出的 hosting 模块
}
```

### 使用外部包

为了在项目中使用外部包，在 `Cargo.toml` 中加入如下行：

```toml
包名 = "版本号"
```

这行告诉 Cargo 要从 crates.io 下载某外部包和其依赖，并使其可在项目代码中使用。为了能够具体使用，通过 `use` 将其中定义的项或模块引入项目包的作用域中。

```rust
use 外部包名::...::项名
```

`std` 标准库对于你的包来说也是外部 crate，但无需修改 `Cargo.toml`，直接在项目中引入所需的 `std` 中的具体项或模块即可。

```rust
use std::collections::HashMap;
```

如果需要用 use 引入很多项或模块，会占据较大篇幅。可以提取出相同的路径前缀，然后其余部分放在花括号里以逗号分隔。这种嵌套路径能使代码整洁简短。

```rust
use std::{cmp::Ordering, io};
// 等价于：
// use std::cmp::Ordering;
// use std::io;
```

可以在嵌套路径中使用 `self`，表示路径“到此为止”。

```rust
use std::cmp::{self, Ordering};
// 等价于：
// use std::cmp;
// use std::cmp::Ordering;
```

如果希望将一个路径下所有公有项引入作用域，可以在指定路径后跟 glob 运算符 `*`。

```rust
use std::collections::*;
```

## 常见集合

### 使用 Vec 储存列表

`Vec<T>`，即vector，允许我们在一个单独的数据结构中储存多于一个的值，它在内存中彼此相邻地排列所有的值。vector 只能储存相同类型的值。

- 新建一个空 vector 调用 `Vec::new` 函数。这里需要显式注明 `T` 类型。

```rust
let v: Vec<i32> = Vec::new(); // 注明 Vec<T> 存储 i32 元素
```

- 用初始值来创建 vector 时，Rust 提供了 `vec!` 宏后跟方括号的方式。编译器会根据初始值推断类型。

```rust
let v = vec![1, 2, 3];
```

- 更新 vector 时，先确保使用了 `mut` 关键字使其可变。
- - 使用 `push` 方法向 vector 增加新元素。

```rust
let mut v = vec![1, 2, 3];
v.push(4);
```

- 读取 vector 元素可以通过索引或使用 `get` 方法。
- - 使用 `&v[index]` 获取对应索引的元素的引用。若索引值超出现有元素范围，程序会 panic。
- - 当使用索引作为参数调用 `get` 方法时，会得到一个可以用于 `match` 的 `Option<&T>`。若索引值超出现有元素范围，会返回 `None`。

```rust
let third: &i32 = &v[2];
    println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

由于存在不能在相同作用域中同时存在可变和不可变引用的规则，因此创建了元素的引用后再向 vector 中增加元素会报错。因为 vector 中元素在内存相邻排列，剩余空间不足时可能会整体分配到新的内存中，这会导致原先的引用指向被释放的内存。

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6); // 编译错误

println!("The first element is: {first}");
```

- 遍历 vector 元素时，可以使用 `for` 循环。
- - 使用 `for ... in &v` 的结构，获取每个元素的引用并访问元素值。

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}
```

- - 使用 `for ... in &mut v` 的结构，获取每个元素的可变引用并对元素值进行操作。为了修改可变引用所指向的值，必须使用解引用运算符（`*`）获取元素值。

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50; // 使用 * 运算符获取值，并修改
}
```

- 删除元素时，使用 `pop` 方法会移除并返回 vector 的最后一个元素。
- vector 在其离开作用域时会被释放，此时所有其内容也会被丢弃。借用检查器会确保任何 vector 中内容的引用仅在 vector 本身有效时才可用。

由于 vector 只能储存相同类型的值，为了使其能储存一系列不同类型的值，可以使用枚举实例作为元素。

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String), // 枚举中使用不同类型
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12), // vector 中采用枚举类型的元素
                                   // 这样能够存放不同类型数据
];
```

### 字符串 string

Rust 的核心语言中只有一种字符串类型，字符串 slice `str`，它通常以被借用的形式（`&str`）出现。`String` 类型由 Rust 标准库提供，而不是编入核心语言，它是一种可增长、可变、可拥有、UTF-8 编码的字符串类型。

很多 `Vec<T>` 上可用的操作在 `String` 中同样可用，事实上 `String` 被实现为 `vector` 的封装。

- 使用 `new` 方法新建空的 `String`。
- 对字符串字面值使用 `to_string` 方法可以创建对应的 `String`。也可以使用 `String::from` 函数来从字符串字面值创建 `String`。

```rust
let data = "initial contents"; // 字符串字面值

let s = data.to_string(); // 将其转为 String

let s = "initial contents".to_string(); // 也可以直接转换

let s = String::from("hello"); // 使用函数进行创建
```

- `String` 可以用多种方法进行更新。
- - 通过 `push` 方法将一个单独的字符附加到 `String` 后。
- - 通过 `push_str` 方法来附加字符串 slice，使 `String` 增长。该方法不需要获取参数的所有权。
- - 使用 `+` 运算符将两个 `String` 拼接为一个新的 `String`。注意：这里前一个参数是 `String`，而后一个参数必须是 `&str` 或 `&String` 类型。并且前一个参数会发生所有权移动。
- - 也可以使用 `format!` 进行多个 `String` 的拼接。它会返回一个新 `String`。

```rust
let mut s1 = String::from("lo");
s1.push('l'); // 使用 push 附加单个字符

let mut s2 = String::from("foo");
let s3 = "bar";
s2.push_str(s3); // 使用 push_str 附加 &str
println!("s2 is {s2}"); // s2 is foobar

let s4 = String::from("Hello,");
let s5 = String::from("world!");
let s6 = s4 + &s5; // + 运算符连接 String 和 &String
// 这里 s4 不再有效，其所有权已移动。因为底层实现是 fn add(self, s: &str) -> String {
// s5 仍有效，因为使用的是它的引用，&String 会进行 Deref 强制转换成为 &str

let s7 = format!("{s4} {s5}"); // 宏 format! 会使用引用因此不会获取任何参数的所有权
```

> [!INFO]
> Rust 语言中有一种名为解引用强制转换（deref coercion）的特性，其与 `Deref` trait 相关。该 trait 定义在标准库的 `std::ops` 模块。

- 无法对字符串直接使用索引语法 `s[index]`。由于 `String` 是 `Vec<u8>` 的封装，实际存储的是各个字节的值，但由于各种字符所占字节数不尽相同，还存在一些特殊字符或辅助字符，Rust无法精确获取指定字符的对应字节，因此不允许使用单个值的索引。
- - 能在方括号中使用 range 来创建字符串 slice，但是必须注意索引要对应实际的字节。

```rust
let hello = "Здравствуйте"; // 这是一个包含 12 个西里尔字母的字符串
                            // 总共占 24 字节（每个字符 2 字节）
let s = &hello[0..4]; // 使用 range 获取前 4 个字节，即头两个字符“Зд”

let s = &hello[0..1]; // 运行时会发生 panic。只截取第一个字节是无效的
```

- 遍历字符串需要表明需要字符还是原始字节。对于单独的 Unicode 标量值使用 `chars` 方法，这样返回单个字符；对于原始字节使用 `bytes` 方法。

```rust
for c in "Зд".chars() { // 返回单个 Unicode 字符
    println!("{c}");
}
// 输出：
// З
// д

for b in "Зд".bytes() { // 返回单个字节值
    println!("{b}");
}
// 输出：
// 208
// 151
// 208
// 180
```

### Hash Map 存储键值对

`HashMap<K, V>` 类型储存了一个键类型 `K` 对应一个值类型 `V` 的映射。它通过一个哈希函数（hashing function）来实现映射。像 vector 一样，`HashMap` 将它的数据储存在堆上。`HashMap` 是同质的：所有的键必须是相同类型，值也必须都是相同类型。使用前需要声明路径 `std::collections::HashMap`。

- 使用 `new` 函数创建一个空的 `HashMap`，并使用 `insert` 方法增加键值对。

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

- 可以通过 `get` 方法并提供对应的键来从 `HashMap` 中获取值。此方法返回 `Option<&V>`（不存在对应键会返回 `None`），因此一般还会调用 `copied` 方法来获取一个 `Option<V>` 而不是 `Option<&V>`。

```rust
let team_name = String::from("Blue"); // 一个 HashMap 中存在的键
let score = scores.get(&team_name) // get 方法获取键对应的值
                  .copied()        // copied 方法将 Option<&V> 转为 Option<V>
                  .unwrap_or(0);   // 调用 unwrap_or 在 scores 中没有该键所对应的项时将其设置为零  
```

- 使用 `for` 循环遍历每一个键值对。

```rust
for (key, value) in &scores { // 这里注意使用引用
    println!("{key}: {value}");
}
```

将键值对插入 `HashMap` 时，对于整型浮点型这种实现 `Copy` Trait 的类型会直接进行拷贝；像 `String` 这样拥有所有权的值，将被移动并归 `HashMap` 所有。如果将值的引用插入，这些值本身将不会被移动进 `HashMap`。但是这些引用指向的值必须至少在 `HashMap` 有效时也是有效的。

`HashMap` 要求每个唯一的键只能同时关联一个值，但相等的值可以对应不同的键。

- 可以用 `insert` 方法将已有键值对的值替换成新的。

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25); // 这里键 Blue 对应的值被替换为 25
```

- 有时会出现一种情况：键没有对应的值。可以用 `entry` 方法检查这一点。该方法返回一个枚举 `Entry`。`Entry` 的 `or_insert` 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用。

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50); // 由于键 Yellow 没有对应值，因此将 50 作为其值一并插入
scores.entry(String::from("Blue")).or_insert(50); // 键 Blue 存在对应值

println!("{scores:?}");
```

可以运用这一点同时更新键值对。例如：

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{map:?}");
```

## 错误处理

Rust 将错误分为两大类：可恢复的（recoverable）和不可恢复的（unrecoverable）错误。`Result<T, E>` 类型用于处理可恢复的错误，还有 `panic!` 宏在程序遇到不可恢复的错误时停止执行。

### 用 `panic!` 处理不可恢复的错误

执行会造成代码 panic 的操作（比如访问超过数组结尾的内容）或者显式调用 `panic!` 宏。这两种情况都会使程序 panic。当出现 panic 时会打印出一个错误信息，程序默认会开始展开（unwinding），这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据；另一种选择是直接终止（abort），即不清理数据就退出程序，此时程序使用的内存需要由操作系统来清理。可以在 `Cargo.toml` 的 `[profile]` 部分增加 `panic = 'abort'` 由展开切换为终止。

```rust
fn main() {
    panic!("crash and burn"); // 圆括号内是输出的错误信息
}
```

可以使用 `panic!` 被调用的函数的 backtrace 来寻找代码中出问题的地方。backtrace 是一个执行到目前位置所有被调用的函数的列表，阅读 backtrace 的关键是从头开始读直到发现你编写的文件。

### 用 `Result` 处理可恢复的错误
















































## 泛型、Trait 和生命周期

### 泛型

泛型允许我们使用一个可以代表多种类型的占位符来替换特定类型，以此来减少代码冗余。

使用泛型定义函数时，本来在函数签名中指定参数和返回值的类型的地方，改用泛型来表示。当在函数签名中使用一个类型参数时，必须在使用它之前就声明它。类型参数声明位于函数名称与参数列表中间的尖括号 `<>` 中。

```rust
fn function<T>(list: &[T]) -> &T {
// 这个函数具有一个泛型类型 `T`
// 接受一个元素类型为 `T` 的 slice 作为参数
// 会返回一个 `T` 类型的引用
```

还需要对泛型类型进行限制，因为 `T` 并不一定适用所有可能的类型。

使用类似的语法来定义含泛型的结构体：在结构体名称后面的尖括号中声明泛型参数的名称，接着在结构体定义中指定具体数据类型的位置使用泛型类型。

```rust
struct Point<T> {
    x: T,
    y: T
}
```

可以使用多个类型参数来传入不同的类型。

```rust
struct Point<T, U> {
    x: T,
    y: U, // 传入的 x, y 类型可不相同
}
```

定义拥有泛型的枚举也遵循相似的语法。

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

为结构体或枚举实现方法时，也可以用泛型。要注意泛型的声明：如果要让定义的方法对任意 `T` 支持的类型都生效，`impl` 后就必须声明泛型，相当于声明该方法对任何实例（不论 `T` 实际是何类型）都生效。

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

也可以增加对泛型的限制：可以选择只为其中一种或几种类型的实例定义方法，而不是对任意类型 `T` 的实例。

```rust
impl Point<f32> { // 表明该 impl 块中的方法只对 Point<f32> 类型的实例生效
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

方法中可以使用与结构体或枚举定义时不同的泛型。

```rust
struct Point<X1, Y1> { // 结构体定义时使用泛型 X1, X2
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> { // 引入新泛型 Y2
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```

泛型参数一般不可以直接指定默认类型，但是可以在 trait 中为其指定。

> [!INFO]
> 泛型还可以与 trait 结合使用，即只允许对泛型使用实现某个或某些 trait 的类型，而不是任意类型。这叫做 trait 约束（trait bound）。

### Trait

trait 定义了某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共同行为。类似其他语言中的“接口”。

定义一个 trait 时，使用关键字 `trait`，后跟该 trait 的名称。花括号内声明描述实现这个 trait 的类型所需要的行为的方法签名，方法签名后跟分号，而不是在花括号中提供其实现（除非需要默认实现）。trait 体中可以有多个方法：一行一个方法签名且都以分号结尾。为了共享 trait，一般都用 `pub` 将其声明为公有。

只要某一类型实现了该 trait，它就必须提供花括号中所有方法的方法体。

```rust
pub trait Summary { 
    fn summarize(&self) -> String; // 任何实现 Summary 这个 trait 的类型
                                   // 都需要提供 summarize 方法的具体方法体
}
```

在类型上实现 trait 也要用到 `impl` 块，语法是 `impl trait名称 for 类型名`。

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle { // 类型 NewsArticle 实现了 Summary
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost { // 类型 SocialPost 实现了 Summary
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

只要该类型实现了 trait，之后就可以像调用自己的方法一样调用 trait 方法，但是使用前确保 trait 必须和类型一起引入作用域以便使用额外的 trait 方法。只有在 trait 或类型至少有一个属于当前 crate 时，我们才能对类型实现该 trait。不能为外部类型实现外部 trait，这被称为“孤儿原则”（orphan rule）。

> [!INFO] 孤儿原则与扩展 trait
> 考虑如下情形：
> 
> - Crate A 定义了 `IsEven` trait。
> - Crate B 为 `u32` 类型实现了 `IsEven` trait。
> - Crate C 也为 `u32` 类型提供了 `IsEven` trait 的（另一个不同的）实现。
> - Crate D 同时依赖于 Crate B 和 Crate C，并调用了 `1u32.is_even()` 方法。
> 
> 那么，应该使用哪个实现呢？是 Crate B 中定义的，还是 Crate C 中定义的？这个问题没有一个好的答案，因此引入了孤儿规则（orphan rule）来防止这种情况的发生。有了孤儿规则，无论是 Crate B 还是 Crate C 都将无法编译通过。

trait 可以为自己的某些或全部方法提供默认的实现，此时定义 trait 的花括号中不再只是方法签名，而是带有方法体。当为某个特定类型实现 trait 时，可以选择保留或重载每个方法的默认行为。要注意无法从相同方法的重载实现中调用默认方法。

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)") // 为 summarize 提供了默认实现
    }
}

impl Summary for NewsArticle {} //若沿用默认实现，只需要花括号内为空即可
```

虽然我们不能直接向外部类型附加新方法，但是 Rust 提供了一种采用扩展 trait 的方式。扩展 trait（extension trait）是一种其主要目的在于为外部类型（foreign types）——例如 `u32`——附加新方法的 trait。

简单来说就是定义一种新 trait，然后让外部类型实现该 trait 的方法。只要该 trait 还在作用域，就可以随时调用其方法。

#### Trait 作为参数

可以使用 `impl Trait` 语法，向一个函数传递任何实现 `Trait` 的类型的参数。例如：

```rust
pub fn notify(item: &impl Summary) { // item 可以是任何实现了 Summary 的类型
                                     // 例如 item 可以是 NewsArticle 也可以是 SocialPost
    println!("Breaking news! {}", item.summarize()); // 调用 Summary 这种 trait 的方法
}
```

上面的语法等价于下面的 trait bound 写法：

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

trait bound 写法适合更复杂的应用场景。传入多个参数时 `impl Trait` 语法无法控制各个参数的类型相同，但是 trait bound 写法可以。

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) { // ...
// item1 和 item2 的具体类型可以不相同
pub fn notify<T: Summary>(item1: &T, item2: &T) { // ...
// 强制 item1 和 item2 的具体类型相同
```

### 运算符重载

运算符重载指的是为 `+`、`-`、`*`、`/`、`==`、`!=` 等运算符定义自定义行为的能力。

在 Rust 中，运算符就是 trait。每个运算符都有一个对应的 trait，该 trait 定义了该运算符的行为。通过为你的类型实现该 trait，你就可以使用相应的运算符。例如，`PartialEq` trait 定义了 `==` 和 `!=` 运算符的行为：

```rust
// `PartialEq` trait 的定义，来自 Rust 标准库（为了便于理解，这里做了简化）
pub trait PartialEq {
    // 必须实现的方法
    fn eq(&self, other: &Self) -> bool;

    // 提供了默认实现的方法
    fn ne(&self, other: &Self) -> bool { ... }
}
```

当你编写 `x == y` 时，编译器会查找 `x` 和 `y` 的类型所对应的 `PartialEq` trait 实现，并将 `x == y` 替换为 `x.eq(y)`。这相当于是语法糖。`PartialEq::ne` 上的注释说明了 “`ne` 是一个提供了默认实现的方法”。这意味着 `PartialEq` 在其 trait 定义中为 `ne` 提供了一个默认实现——即定义代码片段中被省略的 `{ ... }` 代码块。如果我们展开这个被省略的代码块，它看起来是这样的：

```rust
pub trait PartialEq {
    fn eq(&self, other: &Self) -> bool;

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
```

这正是你所期望的：`ne` 是 `eq` 的否定。由于提供了默认实现，当你在为你的类型实现 `PartialEq` 时，可以跳过实现 `ne`。只实现 `eq` 就足够了。当然你也可以在实现 trait 时选择覆盖（override）它。

以下是主要运算符及其对应 trait 的关系：

| 运算符 | Trait |
| :--- | :--- |
| `+` | `Add` |
| `-` | `Sub` |
| `*` | `Mul` |
| `/` | `Div` |
| `%` | `Rem` |
| `==` 和 `!=` | `PartialEq` |
| `<`、`>`、`<=` 和 `>=` | `PartialOrd` |

算术运算符位于 `std::ops` 模块中，而比较运算符则位于 `std::cmp` 模块中。

### 派生宏

如果对元组、结构体、枚举等类型实现了 trait，当类型的定义发生变化时，需要更新 trait 的实现。为了避免出现潜在的问题，我们将其使用 `let` 解构。这样如果我们忘记更新实现，编译器会使得编译不通过，抱怨解构是有问题的。

```rust
impl PartialEq for Ticket { // Ticket 结构体实现了 PartialEq
    fn eq(&self, other: &Self) -> bool {
        let Ticket { // 使用 let 解构 self
            title,
            description,
            status,
        } = self;
        let Ticket { // 使用 let 解构 other
            title: other_title,
            description: other_description,
            status: other_status,
        } = other;
        // 后续步骤 ...
    }
}
```

但是以上步骤依旧繁琐。庆幸的是，Rust 提供派生宏用于为自定义类型自动实现常见的 trait。

宏相当于一种“代码生成器”，它们会根据你提供的输入生成新的 Rust 代码，然后这些生成的代码会与你程序的其余部分一起被编译。一些宏内置于 Rust 的标准库中，但你也可以编写自己的宏。一些 IDE 允许你展开宏以查看生成的代码，也可以使用 `cargo-expand`。

派生宏（derive macros）是 Rust 宏的一种特殊形式。它以属性（attribute）的形式指定在自定义类型的上方。

```rust
#[derive(PartialEq)] // 派生宏自动为 Ticket 实现了 PartialEq
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

将上面的宏展开后看起来像这样：

```rust
#[automatically_derived]
impl ::core::cmp::PartialEq for Ticket {
    #[inline]
    fn eq(&self, other: &Ticket) -> bool {
        self.title == other.title 
            && self.description == other.description
            && self.status == other.status
    }
}
```

只要有可能，编译器都会建议你使用派生宏来自动实现 trait。

### Trait 约束（trait bound）

我们可以将 trait 应用在泛型编程中。考虑对泛型增加限制：要求泛型必须实现某个或某些 trait。

使用 `where` 子句来指定 trait 约束。

```rust
fn print_if_even<T>(n: T) // 这是一个泛型函数
where
    T: IsEven + Debug // trait 约束，表示泛型 T 必须实现了 IsEven 并且也实现了 Debug
{
    if n.is_even() {
        println!("{n:?} is even");
    }
}
// 如果不加入 trait 约束，编译器就不知道 T 能做什么
// 它不知道 T 有一个 is_even 方法（由 IsEven 提供）
// 也不知道如何格式化 T 以便打印（由 debug 提供）
```

也有另一种内联式写法，直接内联在尖括号的类型参数旁边，这适用于 trait 约束较为简单的情形：

```rust
fn print_if_even<T: IsEven + Debug>(n: T) {
    // ...
}
```

### Deref trait

`Deref` trait 是 Rust 语言中一种名为解引用强制转换（deref coercion）的特性背后的机制。该 trait 定义在标准库的 `std::ops` 模块中。

```rust
pub trait Deref {
    type Target: ?Sized; // type Target 是一个关联类型（associated type）
                         // 它是一个具体类型的占位符，在实现该 trait 时必须被指定
    // 必须实现的方法
    fn deref(&self) -> &Self::Target;
}
```

如果类型 `T` 实现了 `Deref<Target = U>`，并且 `v` 是一个 `T` 类型的值，那么你是在告诉编译器 `&T` 和 `&U` 在某种程度上是可以互换的。即：

- 在不可变上下文中，`*v`（其中 `T` 既不是引用也不是原始指针）等同于 `*Deref::deref(&v)`。
- `&T` 类型的值会被强制转换为 `&U` 类型的值。
- `T` 会隐式地实现 `U` 类型中所有接收 `&self` 作为参数的方法（即可以在 `&T` 上调用所有定义在 `U` 上的、以 `&self` 作为输入的方法）。

`String` 实现了 `Deref`，其 `Target` 为 `str`。得益于这个实现和解引用强制转换，`&String` 在需要时会被自动转换为 `&str`。

```rust
impl Deref for String {
    type Target = str;
    
    fn deref(&self) -> &str {
        // ...
    }
}
```

例如，Rust 标准库为 `str`（字符串切片）类型定义了方法 `trim()`，其用于移除首尾空白字符。但是我们可以直接在 `&String` 类型实例上调用它，这是因为以引用的方式使用 `String`（即 `&String`）并调用一个它自身没有但 `str` 拥有的方法时，Rust 会自动将 `&String` 转换为 `&str`，然后调用相应的方法。

解引用强制转换是一个强大的特性，但它也可能导致混淆。因此不要滥用解引用强制转换。关于解引用强制转换有更安全的用例：智能指针。

### Sized trait

Rust 标准库定义了一个名为 `Sized` 的 trait。如果一个类型的大小在编译时是已知的，那么这个类型就是 `Sized` 的。换句话说，它不是一个动态大小类型（dynamically sized type, DST）。

- DST 是一种在编译时大小未知的类型。每当有一个指向 DST 的引用时，比如 `&str`，它就必须包含关于它所指向数据的额外信息。它是一个胖指针（fat pointer）。

```rust
pub trait Sized {
    // 这是一个空 trait，没有需要实现的方法
}
```

`Sized` 是一种标记 trait（marker trait）——一种不需要实现任何方法的 trait。它不定义任何行为，只用于标记一个类型具有某些属性。然后编译器会利用这个标记来启用某些行为或优化。

特别地，`Sized` 也是一个自动 trait（auto trait）。你不需要显式地去实现它；编译器会根据类型的定义自动为你实现。

`str` 不是 `Sized` 的。但是 `&str` 却是 `Sized` 的，我们在编译时就知道它的大小：两个 `usize`，一个用于指针，一个用于长度。

### From trait 和 Into trait

Rust 标准库在 `std::convert` 模块中为不会失败的转换定义了两个 trait：`From` 和 `Into`。

```rust
pub trait From<T>: Sized { // From<T>: Sized 语法意味着 From 是 Sized 的一个子 trait
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized { // Into<T>: Sized 也是如此
    fn into(self) -> T;
}
```

`From<T>: Sized`这个语法意味着 `From` 是 `Sized` 的一个子 trait（subtrait）：任何实现 `From` 的类型也必须实现 `Sized`。或者，你也可以说 `Sized` 是 `From` 的一个父 trait（supertrait）。同理于 `Into`。

要注意的是，每当你有一个泛型类型参数时，编译器会隐式地假定它是 `Sized` 的。这是一种隐式 trait 约束（implicit trait bounds）。

```rust
pub struct Foo<T> { // 等价于 pub struct Foo<T>: Sized {
    inner: T,
}

因此，由于隐式 trait 约束的存在，`T` 和实现 `From<T>` 的类型都必须是 `Sized` 的，即使前者的约束是隐式的。

可以使用负向 trait 约束（negative trait bound）来选择退出这个隐式的 `Sized` 约束。它允许你将 `T` 绑定到一个动态大小类型（DST）。要注意的是负向 trait 约束是 `Sized` 独有的，你不能将它们用于其他 trait。

```rust
pub struct Foo<T: ?Sized> {
               // ^^^^^^^
               // 这是一个负向 trait 约束，表示 T 可能是 Sized 的，也可能不是
    inner: T,
}

标准库中 `String` 为自身实现了 `From<&str>`，因此可以写 `let title = String::from("A title");`。但是我们同样可以使用 `let title = "A title".into()`，因为 `Into` 是通过一个毯式实现（blanket implementation）为任何实现了 `From` 的类型而实现的：

```rust
impl<T, U> Into<U> for T
where
    U: From<T>, // 只要类型 U 实现了 From<T>，那么 T 的 Into<U> 就会被自动实现
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

只要类型 `U` 实现了 `From<T>`，那么 `T` 的 `Into<U>` 就会被自动实现。换言之，`From` 和 `Into` 是一对对偶 trait。

每当你看到 `into()`，你都在见证一次类型间的转换。只要编译器能从上下文中明确无误地推断出目标类型，`into()` 就能直接工作。

### 泛型（generic）与关联类型（associated type）

对于 `Deref` 和 `From` / `Into` 的讨论引出了一个问题：为什么 `From` / `Into` 定义时是用泛型而 `Deref` 是用关联类型？

```rust
pub trait From<T> { // 这里使用的是泛型参数 T
    fn from(value: T) -> Self;
}

pub trait Deref {
    type Target: ?Sized; // 这里使用的是关联类型 Target
    
    fn deref(&self) -> &Self::Target;
}
```

`Deref` 用关联类型是因为解引用强制转换（deref coercion）的工作方式：对于一个给定类型只能有一种目标类型，`String` 只能解引用（deref）到 `str`。如果你可以为一个类型多次实现 `Deref`，那么编译器可能会对应该选择哪个 `Target` 类型产生歧义。而一个关联类型是由 trait 的实现唯一确定的，不会引起歧义。

另一方面，你可以为一个类型多次实现 `From`，只要输入的类型 `T` 不同即可。这里没有歧义：编译器可以根据被转换值的类型来确定使用哪个实现。

```rust
// 为了让 WrappingU32 能同时使用 u32 和 u16 作为输入类型而实现 From
impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { inner: value }
    }
}

impl From<u16> for WrappingU32 {
    fn from(value: u16) -> Self {
        WrappingU32 { inner: value.into() }
    }
}
```

泛型与关联类型可以同时出现在一个 trait 之中，见下节的 `Add` trait。

### 为泛型参数指定默认值

为泛型参数指定默认值的功能主要用在 trait 的定义中。这被称为默认类型参数（Default Type Parameters）。

例如标准库中的 `Add` trait：

```rust
pub trait Add<RHS = Self> { // 在尖括号内指定了泛型 RHS 的默认值为 Self
    type Output; // 关联类型 Output
    
    fn add(self, rhs: RHS) -> Self::Output; // 注意这里的 Self::Output 写法
}
```

`Add` trait 有一个泛型参数 `RHS`（Right-Hand Side，即右操作数）。如果你在实现这个 trait 时不指定 `RHS` 的具体类型，它将默认为 `Self`，也就是实现这个 trait 的类型本身。

这一 trait 同时使用了两种机制：泛型（generic）与关联类型（associated type）。在标准库中还会有类似这样的实现：

```rust
// + 运算符重载
impl Add<u32> for u32 { // 为 u32 类型实现 Add<u32>
    type Output = u32; // 运算结果是 u32
    
    fn add(self, rhs: u32) -> u32 {
                           // ^^^
        // 这里也可以写成 Self::Output
        // 编译器并不关心，只要你在这里指定的类型与你上面赋给 Output 的类型相匹配即可
        // ...
    }
}

impl Add<&u32> for u32 { // 为 u32 类型实现 Add<&u32>
    type Output = u32;
    
    fn add(self, rhs: &u32) -> u32 {
        // ...
    }
}
// 因此可以对一个 u32 和一个 &u32 做 + 运算，结果是 u32 类型
// 例如 5u32 + &5u32 + 6u32

impl Add<&u32> for &u32 { // 为 &u32 类型实现 Add<&u32>
    type Output = u32; // 表明两个 &u32 相加，结果是 u32

    fn add(self, rhs: &u32) -> u32 {
        // ...
    }
}
```

`Output` 不能是一个泛型参数。一旦操作数的类型确定，操作的结果类型必须是唯一确定的。这就是为什么它是一个关联类型。而我们为 `u32` 多次实现了 `Add` trait，这允许我们让 `u32`（左操作数）与 `u32` 或 `&u32`（右操作数）相加，因此 `RHS` 是一个泛型参数。

### Clone trait 和 Copy trait

由于所有权系统的存在，那些关于所有权和借用的限制可能会带来一些不便。有时我们可能需要调用一个会获取值所有权的函数，但在那之后我们仍然需要使用那个值。这时可以使用 `Clone` trait。

```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```

`Clone` 是 Rust 标准库中定义的一个 trait：它的 `clone` 方法接受一个对 `self` 的引用，并返回一个同类型的、拥有所有权的新实例。可以将 `clone` 理解为一种创建对象深拷贝（deep copy）的方式。

要使一个类型变得可以 Clone，我们必须为它实现 `Clone` trait。你几乎总是通过派生（deriving）它来实现 `Clone`：

```rust
#[derive(Clone)]
struct MyType {
    // 其他字段
}
```

这之后就可以对 `MyType` 实例直接调用 `clone` 方法了。

```rust
let instance = MyType {
    // ...
}

let another_instance = instance.clone();

consumer(another_instance); // another_instance 的所有权移动

// 此时 instance 依然可用
```

让我们考虑使用 `u32` 而不是 `String` 类型。

```rust
fn consumer(s: u32) { /* */ }

fn example() {
    let s: u32 = 5;
    consumer(s);
    let t = s + 1;
}
```

这段代码会毫无错误地编译通过。这表明 `String` 和 `u32` 之间有某种区别，使得后者在没有 `clone()` 的情况下也能工作。这种区别就是 `u32` 实现了 `Copy` 而 `String` 并没有。

`Copy` 是 Rust 标准库中定义的另一个 trait，它是一个标记 trait，没有需要实现的方法，就像 `Sized` 一样。

```rust
pub trait Copy: Clone { }
```

如果一个类型实现了 `Copy`，那么就不需要调用 `clone` 方法来创建该类型的新实例：Rust 会隐式地完成。`u32` 就是一个实现了 `Copy` 的类型，这就是为什么上面的例子能无错编译通过的原因：当 `consumer(s)` 被调用时，Rust 通过对 `s` 执行按位复制（bitwise copy）来创建一个新的 `u32` 实例，然后将这个新实例传递给 `consumer`。这一切都在幕后发生，你无需做任何事情。

要注意，一个类型必须满足一些要求才能被允许实现 `Copy`。

- 它必须实现 `Clone`，因为 `Copy` 是 `Clone` 的子 trait。这很合理：如果 Rust 可以隐式地创建一个类型的新实例，那么它也应该能够通过调用 `clone` 来显式地创建一个新实例。
- 该类型除了在内存中占用的 `std::mem::size_of` 字节之外，不管理任何额外的资源（例如堆内存、文件句柄等）。
- 该类型不是一个可变引用 (`&mut T`)。

如果这三个条件都满足，那么 Rust 就可以安全地通过对原始实例执行按位复制来创建一个新实例。

- `String` 是一个没有实现 `Copy` 的类型。因为它管理着一个额外的资源：存储字符串数据的堆分配内存缓冲区。

- `&mut u32` 也没有实现 `Copy`，即使 `u32` 实现了。这是因为所有权规则规定：在任何给定的时间点，一个值只能有一个可变借用。如果 `&mut u32` 实现了 `Copy`，你就可以为同一个值创建多个可变引用，并在多个地方同时修改它。那将违反 Rust 的借用规则。因此，无论 `T` 是什么，`&mut T` 都永远不会实现 `Copy`。

在大多数情况下不需要手动实现 `Copy`。你可以直接派生它，像这样：

```rust
#[derive(Copy, Clone)]
struct MyStruct {
    field: u32,
}
```

### Drop trait

当变量离开其作用域时，Rust 会自动隐式调用其析构函数（destructors）`std::mem::drop`，即：

- 回收该类型所占用的内存（即 `std::mem::size_of` 字节）。
- 清理该值可能管理的所有额外资源（例如 `String` 的堆缓冲区）。

`Drop` trait 是一种机制，让你能为你的类型定义额外的清理逻辑，这超出了编译器自动为你做的范畴。无论你在 `drop` 方法中放入什么逻辑，当该值离开作用域时，这些逻辑都将被执行。

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

> [!INFO] `Drop` trait 与 `Copy` trait
> 如果一个类型管理着超出其在内存中所占用的 `std::mem::size_of` 字节的额外资源，那么它就不能实现 `Copy` trait。编译器是如何知道一个类型是否管理着额外资源的呢？答案是通过 `Drop` trait 的实现。
> 
> 如果你的类型有一个显式的 `Drop` 实现，编译器就会假定你的类型附带有额外的资源，并且不会允许你实现 `Copy`。
>
> ```rust
> #[derive(Clone, Copy)]
> struct MyType; // 这是一个单元结构体
> 
> impl Drop for MyType {
>     fn drop(&mut self) {
>        // 这里只要有一个“空的” Drop 实现就足够了
>     }
> }
> // 编译会报错，因为MyType 实现了 Drop，即便方法体是空的
> // 编译器会判定 MyType 附带有额外资源无法实现 Copy
> ```

### Trait 用法

Rust 中的 trait 很强大，但不要滥用。请记住以下几条准则：

- 如果一个函数总是用同一种具体类型来调用，就不要将其泛型化。这会给你的代码库引入不必要的间接性，使其更难理解和维护。
- 如果一个 trait 只有一个实现，就不要创建它。这通常意味着这个 trait 是不必要的。
- 只要合理，就为你的自定义类型实现标准库中的 trait（如 `Debug`、`PartialEq` 等）。这会让你的类型更符合 Rust 的语言习惯，也更易于使用，从而解锁标准库和生态系统中其他 crate 提供的许多功能。
- 如果你需要第三方 crate 在其生态系统内所解锁的功能，就为你的类型实现这些 crate 中定义的 trait。
- 警惕不要仅仅为了在测试中使用模拟对象（mock）而将代码泛型化。这种方法的维护成本可能很高，通常采用不同的测试策略会更好。