# 创建新线程
如下所示, 可以使用`spawn`函数创建一个新的线程并运行, 返回`JoinHandle`对象(调用其join方法可以是主线程等到子线程结束):
```rust
use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(||{ //设置子线程函数, 这里用了闭包
        for i in 1..10{
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();//使主线程等待
}
```

# 传递所有权到线程
闭包会捕获环境中的所有权或引用, Rust编译器会自己推断需要如何捕获闭包中需要的变量. **但是Rust不知道新建线程会执行多久，所以无法知晓引用是否一直有效.**
解决办法是**在闭包之前增加`move`关键字，我们强制闭包获取其使用的值的所有权，而不是任由Rust编译器推断它应该借用值**.
```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```
所有权move到子线程后, 主线程不能再操作变量v了.

# 线程间消息通信
go和rust都推荐使用**消息传递**进行线程间通信,  这个思想来源于Go编程语言的口号:
>Do not communicate by sharing memory; instead, share memory by communicating.

## channel
Rust 中一个实现消息传递并发的主要工具是**信道（channel）**. 代码中的一部分调用发送者的方法以及希望发送的数据，另一部分则检查接收端收到的消息. 当**发送者或接收者**任一方被丢弃时可以认为信道被关闭（closed）了.
使用`mpsc::channel`函数创建一个新的信道；mpsc是多个生产者，单个消费者（multiple producer, single consumer）的缩写,即**一个信道可以有多个产生值的发送（sending）端，但只能有一个消费这些值的接收（receiving）端**. 
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();//子线程获取tx所有权, 并发送数据
    });
    //主线程接收数据
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```
注意:**send函数获取其参数的所有权并移动这个值归接收者所有**。这可以防止在发送后再次意外地使用这个值；所有权系统检查一切是否合乎规则.

## 接收者
接收端有如下几种方式接收数据
* `recv()`: 以阻塞方式接收数据, 等到有数据时返回
* `try_rev()`: 非阻塞方式接收数据, 当前没有数据立刻返回
* `for val in rx`: 将当作迭代器, 阻塞方式等待多个数据并循环处理

## 多发送者
` mpsc::channel()`默认返回一个发送者, 一个接收者, 可以通过`clone`创建多个发送者:
```rust
let (tx, rx) = mpsc::channel();
let tx1 = tx.clone();

thread::spawn(move || {
    let val = String::from("hi");
    tx1.send(val).unwrap();//使用tx1
});
```

# `Mutex<T>`与`Arc<T>`
共享内存类似于多所有权：多个线程可以同时访问相同的内存位置, 但是需要进行并发控制, 避免出现冲突.

任意时刻，mutex只允许一个线程访问某些数据, 线程首先需要通过获取互斥器的 锁（lock）来表明其希望访问数据.
互斥器以难以使用著称，因为你不得不记住(**在Rust中，得益于类型系统和所有权，我们不会在锁和解锁上出错**)：
* 在使用数据之前尝试获取锁。
* 处理完被互斥器所保护的数据之后，必须解锁数据，这样其他线程才能够获取锁。

`Mutex<T>`lock 调用返回一个的智能指针(类似C++的`std::lock_guard`, 但`std::lock_guard`不拥有数据)。这个智能指针实现 Deref来指向其内部数据；其也提供了一个Drop实现当MutexGuard离开作用域时自动释放锁.
```
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;

fn main() {
     //使用并发智能指针, 保证多个线程可以共享mutex, Rc不能用于多线程
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);//增加引用计数
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```
需要注意:
* `std::Arc`返回的不可变引用, 但是`Mutex<T>`本身提供修改功能, 所以可以执行lock操作(类似RefCell)
* 由于`Mutex<T>`的可变性, 需要注意死锁的问题(Rust提供了可修改性, 所以无法检测死锁)

