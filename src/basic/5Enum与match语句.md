
# enum类型

## int型enum
enum枚举定义和C++基本一样，但不需要最后的分号。使用方式类似C++ `enum class`：
```rust
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind, //作为struct的成员
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
}
```

## 非int型enum
和C++`enum class`一样，enum默认是int类型，当然也可以自定义其他类型。Rust enum比C++强大的是：
- rust enum可以单独为每个enum值定义类型，**不同enum值类型可以不一样**（C++不行）
- rust enum可以为同一个enum值**定义任意类型**，使用`()`
- rust enum可以为enum值定义struct类型，使用`{}`

```rust
enum Messgae {
    Quit, //默认类型int
    Move { x: i32, y: i32}, //结构体类型
    Write(String), //字符串
    ChangeColor(i32, i32, i32), //tuple类型
}
//使用:
let q = Message::Quit;
let m = Message::Move{x: 12, y: 15};
let w = Message::Write(String::from("hello"));
let c = Message::ChangeColor(0, 255, 255);
```

# Option enum
Option定义在标准库中, 表示**某个值可能存在(即有效的某种类型的值)或不存在, 用于取代C++中的NULL**. Option的原型定义如下:
```rust
enum Option<T>{
    Some(T), //表示某类型的值
    None, //不存在值, 类似NULL
}
```
注意:
* Option<T>和T是不同的类型, 不能当作T来使用, **避免了C++中直接操作NULL带来的错误**
* 要想使用T,需要使用`unwrap()`成员函数从Option<T>中获取T. 如果是None调用unwrap会导致panic

# match模式匹配
## enum匹配
**switch 语法很经典，但在 Rust 中并不支持**，很多语言摒弃 switch 的原因都是因为 switch 容易存在因忘记添加 break 而产生的串接运行问题。Rust 通过 match 语句来实现分支结构：
**注意, match必须穷举所有可能性, 不能省略. 类似C++的default, 可以用`_`表示其他未列出的可能值**.
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        _ => 25, //即C++中的switch default分支
    }
}
```

## 绑定值的enum匹配
enum可以定义非int类型，那么match语句也需要支持其他类型的enum值，比如：
```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {//匹配到此类型的enum，state这里表示参数名
            println!("State quarter from {:?}!", state);//打印enum的具体值
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));//指定enum值
}
```

## Option<t>匹配
```rust
fn add_one(x: Option<i32>) -> Option<i32>{
    match x {
        Some(i) => Some(i+1),
        None => None,
    }
}
```

# if let语句
如果**只判断某一个enum值**，用match就有点多余。在C++我们可以直接使用if语句判断，而在rust中这是不允许的，会报错`an implementation of 'std::cmp::PartialEq' might be missing for "XXX"`。
Rust中可以使用`if let`判断一个enum值：
```rust
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin { //类似C++的if判断
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
```
