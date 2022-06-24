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