# 测试模块与测试函数
和C++ attributes类似, rust可以通过`#[]`格式对代码进行属性修饰.
要使用`cargo test`自动允许测试用例, 需要通过以下属性让cargo知道测试模块和测试函数在哪里
* `#[cfg(test)]`: 设置module属性, 标记为测试模块. **测试模块里也能添加普通函数**.
* `#[test]`: 设置函数属性, 标记为测试用例函数

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

# 判断测试结果
## 断言宏
|macro|作用|
|:-|:-|
|assert!()|判断bool, 测试结果为true为通过;结果为false, 断言失败|
|assert_eq!|判断两个参数是否相等|
|assert_ne!|安定两个参数是否不相等|

这几个断言宏都可以自定义断言失败的错误信息, 第一个参数是要判断的语句, 后面是要输出的错误和参数(参照format!的格式)

## 检查panic
使用` #[should_panic]`可以检查代码中的panic, 在test过程中,不会真正让程序panic, 而是显示should panic提示.
同时还可以给panic检查添加自定义信息(` #[should_panic(expected="")`), 在panic发生时也会在test结果中打印出来
```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

```

## 测试函数返回值
我们可以直接给测试函数设置返回值 Result<T,E>, 这样就不需要用断言宏判断结果了.
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

# 集成测试集
上面的测试模块可以放在src下的任何rs文件中，表示它是一个单元测试模块，可以测试本项目所有的public和private代码。
而我们还可以创建一个单独的tests目录（与src同级），这一目录里保存集成测试的函数。在此目录里可以不显式使用`#[cfg(test)]`直接定义测试函数, 测试函数只能测试src下的public代码(use package导入src的crate). tests也叫集成测试目录.

使用`cargo test`会默认执行单元测试和集成测试所有的函数, 如果要执行集成测试下的特定函数,需要下面方法
* 运行一个特定的集成测试: `cargo test 函数名`
* 运行某个rs文件内所有的测试: `cargo test --test filename`

注意: 
* **tests下的子目录不会被作为单独的crate进行编译, 不会被认为是一个集成测试文件. 子目录的内容可以被tests下的测试文件引用.**
* **bianry crate不能创建tests集成测试**, 因为binary crate是独立运行, 不会暴露函数给外部使用, 集成测试无法引入.
