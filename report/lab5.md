# lab5



## 编程内容

实现了 `spawn` 系统调用，实现方式是 `fork` + `exec`

因为本次实验框架改动较大，重新实现了之前的一些功能



## 问答作业

### 1

fork + exec 的一个比较大的问题是 fork 之后的内存页/文件等资源完全没有使用就废弃了，针对这一点，有什么改进策略？

首先是直接放弃 fork+exec，编写一个新的方法来创建新进程。如果还使用该策略的话，可以修改 `fork`，使用 Copy on Write 技术，不复制父进程的内存空间而让子进程的相应地址直接映射到父进程的地址，在父/子进程修改数据时再拷贝

### 2

其实使用了题(1)的策略之后，fork + exec 所带来的无效资源的问题已经基本被解决了，但是今年来 fork 还是在被不断的批判，那么到底是什么正在”杀死”fork？可以参考 [论文](https://www.microsoft.com/en-us/research/uploads/prod/2019/04/fork-hotos19.pdf) ，**注意**：回答无明显错误就给满分，出这题只是想引发大家的思考，完全不要求看论文，球球了，别卷了。

`fork` 复制父进程状态这一说法非常模糊，当代的操作系统有很多很复杂的功能，如互斥锁、定时器、异步操作状态等，OS 需要做出专门的规定并且有时还要让用户控制，这实在太过复杂

### 3

fork 当年被设计并称道肯定是有其好处的。请使用 **带初始参数** 的 spawn 重写如下 fork 程序，然后描述 fork 有那些好处。注意:使用”伪代码”传达意思即可，spawn 接口可以自定义。可以写多个文件。

```rust
fn main() {
    let a = get_a();
    if fork() == 0 {
        let b = get_b();
        println!("a + b = {}", a + b);
        exit(0);
    }
    println!("a = {}", a);
    0
}
```

```rust
// parent.rs
fn main() {
    let a = get_a();
    spawn("child.rs", &[a.as_ptr()]);
    println!("a = {}", a);
    0
}

//child.rs
fn main(argc: usize, argv: &[&str]) {
		let a = argv[0].into();
  	let b = get_b();
  	println!("a + b = {}", a + b);
  	exit(0);
}
```

`fork` 可以让子进程访问父进程之前的数据，比如在本例中的 `a`

### 4

描述进程执行的几种状态，以及 fork/exec/wait/exit 对于状态的影响。

本实验中实现的进程状态有就绪、运行、等待

- `fork` 创建新进程，初始就绪态
- `exec` 由运行态到运行态
- `wait` 从运行态到等待态
- `exit` 使运行态到程序退出