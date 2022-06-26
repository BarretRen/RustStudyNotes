**闭包=匿名函数+状态**， 类似C++中的捕获了外部变量的lambda表达式，或者使用`std::bind`和`std::function`返回的函数和参数绑定的对象.
闭包的主要特点:
1. 是匿名函数
1. 保存为变量, 可以作为参数传递
1. 可以在一个地方创建闭包, 在另一个地方调用
1. **从其定义的作用域捕获变量值**

# closure定义
如下是rust中函数和closure定义的语法
```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 } //函数定义, 必须指定参数和返回值类型
let add_one_v2 = |x: u32| -> u32 { x + 1 }; //闭包定义, 显式指定类型, 括号变为竖线
let add_one_v3 = |x|             { x + 1 }; //不写类型, 编译器自动推导
let add_one_v4 = |x|               x + 1  ; //函数体只有一句, 可以不写{}
```
规则:
* 闭包不需要显式的标注参数和返回值的类型
* 编译器可以通过上下文自动推导类型, 但**只会推导出一种类型, 不能一个闭包对应多种类型**
* 可以手动的添加类型, 和函数语法一样

# closure的类型
每一个闭包实例有其自己独有的匿名类型：即便两个闭包有着相同的签名，他们的类型仍然可以被认为是不同。
**Fn 系列 trait 由标准库提供。所有的闭包都实现了trait Fn、FnMut 或 FnOnce 中的一个（函数也是）**。 所以我么可以用这三个trait表示一个闭包或函数，类似C++的函数指针和`std::function<>`.
* FnOnce:从周围作用域捕获的变量，闭包周围的作用域被称为环境。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
* FnMut 获取可变的借用值所以可以改变其环境
* Fn 从其环境获取不可变的借用值

# 带有泛型和 Fn trait的闭包
利用Fn trait这样的函数指针, 我们可以给匿名函数保存状态, 从而成为真正的**有状态闭包**.

下面例子, 我们保存一个value和一个函数指针,函数指针用于计算value,但是只计算一次.如果value有值了, 就不再计算. 具体的计算函数由外部传入.
## C++的实现
使用C++, 那我们直接使用函数指针就好了, 为了便于维持value和函数的关系, 我们把它们放在一个类里:
```cpp
#include <chrono>
#include <cstdlib>
#include <functional>
#include <iostream>
#include <thread>

class Cacher {
private:
  std::function<int(int)> caculate;//保存函数指针
  int value;//保存value
  bool isCaculated;//标记value是否被计算过

public:
  Cacher(std::function<int(int)> func) {
    caculate = func;
    value = 0;
    isCaculated = false;
  }

  int Value(int intensity) {
    if (!isCaculated) {
      isCaculated = true;
      value = caculate(intensity);
    }
    return value;
  }
};
//创建Cacher对象并调用函数计算value
void genrate_workout(int intensity, int random_numer) {
  auto cache = Cacher([](int a) { //传入lambda匿名函数作为计算value的函数
    printf("calculating slowly\n");
    std::this_thread::sleep_for(std::chrono::seconds(2));
    return a;
  });

  if (intensity < 25) {
    printf("today, do %d pushups\n", cache.Value(intensity));
    printf("next, do %d pushups\n", cache.Value(intensity));
  } else {
    if (random_numer == 3)
      printf("Take a break today! remember to stay hydrated!\n");
    else {
      printf("today, run for %d minutes\n", cache.Value(intensity));
    }
  }
}
```

## rust实现
对于rust, 我们需要用Fn trait来表示函数或闭包, 它没有直接的函数指针表示. 而介绍trait时提到过, 将trait作为参数时, 比较好的用法是用**trait bound**语法.
所欲类比上边的C++代码, rust实现如下
```rust
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32, //trait bound, 标记哪种Fn trait可以被使用,其实就是函数指针
{
    calculation: T,
    value: Option<u32>, //option表示value, none表示value没有计算过.不需要单独的bool flag
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> { //创建对象的函数
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| { //传入一个闭包作为计算value的函数
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
```

# closure捕获变量
C++中lambda表达式有多种捕获变量的方式,需要在[]中指定捕获哪些变量, 以什么形式捕获.
Rust closure中,当创建一个闭包时，**编译器根据其如何使用环境中变量来推断我们希望如何引用环境**, 不需要像C++一样用[]显式指定:
* 由于所有闭包都可以被调用至少一次，所以所有闭包都实现了 FnOnce 。
* 那些并没有移动被捕获变量的所有权到闭包内的闭包也实现了 FnMut
* 不需要对被捕获的变量进行可变访问的闭包则也实现了 Fn

大部分需要指定一个Fn系列trait bound的时候，可以从 Fn 开始，而编译器会根据闭包体中的情况告诉你是否需要 FnMut 或 FnOnce.