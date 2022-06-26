# iterator trait
Rust某些类型实现了迭代器，和C++一样可以依次返回元素。迭代器都实现了一个叫做 Iterator 的定义于标准库的trait， trait定义如下：
```rust
pub trait Iterator {
    type Item; //关联类型, 实际类型在实现trait的impl中指定具体类型

    fn next(&mut self) -> Option<Self::Item>; //用于返回每个元素
}
```
迭代器trait常用的几个接口函数:
* * next(): 返回下一个值的不可变引用
* iter(): 生成一个不可变引用的迭代器
* into_inter(): 生成一个拥有变量所有权的迭代器

# 创建自定义迭代器
```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter { //实现迭代器
    type Item = u32; //指定Item为u32, 因为结构体中count是u32

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

# 迭代器的性能
闭包和迭代器是Rust 受函数式编程语言观念所启发的功能。他们对Rust以底层的性能来明确的表达高级概念的能力有很大贡献。**闭包和迭代器的实现达到了不影响运行时性能的程度**。这正是 Rust 竭力提供零成本抽象的目标的一部分。