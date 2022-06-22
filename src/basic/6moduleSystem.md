# 模块系统各级概念
- 包（Packages）： Cargo 的一个功能，它允许你构建、测试和分享 crate。
- Crates ：一个树形结构的模块树，它形成了库或二进制文件。
- 模块（Modules）和 use： 允许你控制作用域和路径的私有性。
- 路径（path）：一个命名结构体、函数或模块等项的方式

# Package
## crate类型
使用`cargo new`可以创建两种crate：

- binary二进制文件：`cargo new name`，此方式会在src下创建**main.rs**文件
   - 如果将文件放在 **src/bin** 目录下，一个包可以拥有多个二进制 crate：每个 src/bin 下的文件都会被编译成一个独立的二进制crate  
- lib库文件：`cargo new --lib name`，此方式会在src下创建**lib.rs**文件
   - 在src下的rs文件，默认会生成**与文件名同名的模块**，可以在其他文件中使用`mod name;`导入使用（C++的前置声明）。
   - 多级嵌套模块在src下对应多级目录。

如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：一个库和一个二进制项，且名字都与包同。

## package目录结构
Cargo的package目录结构如下：
```cpp
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```

- Cargo.toml与Cargo.lock存储在项目的根目录中。
- 源代码src目录, **一个package至少包含一个crate(library或binary)**
- 默认库文件是src/lib.rs. **一个package只能有0-1个library**
- 默认的可执行文件是src/main.rs。
- 其他可执行文件可以放入src/bin/*.rs。**binary数量可以任意数量**
- 集成测试为tests目录（单元测试进入他们正在测试的每个文件中）。
- 示例可执行文件放在examples目录中。
- 基准测试进入benches目录

# module与Path
## module私有性
在一个crate中,使用module对代码进行分组, 可以增加可读性,易于复用, 还可以可以控制项目的私有性:
* 声明默认是private的, 需要用`pub`声明为public
    * **`pub struct`表示struct是public的, 但里面的成员还是private, 需要单独加`pub`**
    * `pub enum`表示enum和里面的值都是public的
* 父模块无法访问子模块的私有item
* 子模块可以使用所有祖先模块的item(使用`super`关键字)

## 引用模块path
- 绝对路径（absolute path）从crate根开始，以`crate名`或者`字面值crate`开头。**推荐使用绝对路径**.
- 相对路径（relative path）从当前模块开始，以 self(本模块开始)、 super(父模块开始)或当前模块的标识符开头
- 绝对路径和相对路径都后跟一个或多个由双冒号（ :: ）分割的标识符
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
//绝对路径
crate::front_of_house::hosting::add_to_waitlist();
//相对路径，本函数与上面的模块在同一层级
front_of_house::hosting::add_to_waitlist();
self::front_of_house::hosting::add_to_waitlist();
```

## use导入模块
直接使用路径太繁琐了，我们可以使用`use`导入需要的模块，简化调用，类似C++的`include + using`。
```rust
//导入了hosting模块，可以直接调用，不需要那么长的路径了
use crate::front_of_house::hosting; //use后面绝对路径和相对路径都可以
hosting::add_to_waitlist();
```
**其他技巧**:
- 和C++的`using`一样，`use`也可以定义别名，格式为：`use XXX as xxx;`
- `use`还可以把多个模块在同一行中导入，减少行数：
    - `use std::{mod1,mod2};` 两个独立路径
    - `use std::{self, mod2};` mod2是self的子路径
- 使用`use`导入的item在当前作用域内是private的, 只能在本文件内使用.如果需要让其他文件也可能访问当前文件导入的item, 需要使用`pub use`

# 拆分module到不同文件
类似与C++的前置声明, 我们可以在总的模块定义文件中(比如lib.rs)使用格式`mod module_name;`做前置声明, 具体的module定义放在其他文件中.
需要注意有一下规则:
* 存放具体module定义的文件,文件名必须是module_name
* 如果是存放多级module, 文件也需要保存在多级目录下, 文件路径与module path一致(src为根路径)

举例:
```rust
// src/lib.rs
mod front_of_house; //前置声明, 具体定义在front_of_house.rs中

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// src/front_of_house.rs
pub mod hosting; //子模块前置声明

// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {} //实际的定义位置
```