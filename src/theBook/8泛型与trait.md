# 泛型<T>
和C++模板参数大致一样, 用于标记类型占位符, 真正的具体类型在使用时指定. 泛型通常用于如下几种结构中:

## 函数泛型参数
```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list); //自动推导出T的类型
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

## 结构体和impl泛型
```rust
struct Point<T> { //单泛型参数
    x: T,
    y: T,
}

struct Point<X1, Y1> {//多泛型参数
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> { //impl中也要指定泛型参数
    //方法里的X2,Y2表示类型与X1,Y1不一定一样
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```

## enum泛型
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

# trait接口
trait（类似C++纯虚函数）告诉编译器
* trait把函数签名放在一起, 实现它的某个类型就具有相应的功能
* trait抽象的定义共享行为
* **trait bound约束在某个类型上实现trait, 这个类型必须是本地crate里定义的, 无法为外部类型实现trait**. 这样规则的好处是:
    * 确保其他人的代码不能破坏本crate中的代码.
    * 两个crate无法为同一类型实现同一个trait, 避免冲突

## 定义trait
trait定义函数签名, 没有具体实现, 实现该trait的类型需要提供具体的实现.
类似C++定义class, 并添加纯虚函数, 供子类去继承实现
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```
当然, **也能在trait中添加函数的默认实现, 这样就变成了虚函数, 而不是纯虚函数**.

## 实现trait
即为某类型添加impl并实现定义中的各个函数. 即C++中继承纯虚函数类, 子类添加函数实现.
```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// use the trait in main.rs
use std::string::String;
use trait_tmp::{NewsArticle, Summary};

fn main() {
    let art = NewsArticle {
        headline: String::from("hello"),
        location: String::from("barret"),
        author: String::from("shidandan"),
        content: String::from("hello rust, content"),
    };

    println!("{}", art.summarize());
}
```

## trait作为参数
### 函数参数
trait作为函数参数, 就是**要求输入的参数类型必须实现了该trait, 不然不能调用函数**.
两种方式:
* `&impl trait_name`: 适用于简单情况
* trait bound: 用于复杂情况, 比如下面用where指定多个trait
```rust
//一个trait参数的示例, 两种方法都很简洁
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

//多trait
pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T) {}

// 多参数多trait, trait bound的方式就更清晰
fn some_function(t: &(impl Display + Clone), u:  &(impl Clone + Debug)){}
fn some_function<T, U>(t: &T, u: &U) -> i32
where T: Display + Clone,
      U: Clone + Debug
{}
```

### 函数返回值
使用`impl trait_name`作为函数返回值, 但要注意**函数内只能返回一种类型, 不能有多种返回类型**.
```rust
fn returns_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("hello"),
        location: String::from("barret"),
        author: String::from("shidandan"),
        content: String::from("hello rust, content"),
    }
}
```

# trait bound用于impl<T>
在泛型impl<T>外层使用trait bound, 可以限定泛型T, 从而有条件的为特定类型实现方法. 即C++中的**模板特化**.
```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}
//new应用于所有类型T
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
//cmp_display仅应用于实现了display和partialord trait的类型T
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```