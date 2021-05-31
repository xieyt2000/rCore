# lab8



## 失败测例分析

### ch8_01

#### 表现

app 结束后内核崩溃

```bash
>> ch8_01
Shell: Process 1 exited with code 0
>> 
>> [kernel] Panicked at src/mm/page_table.rs:97 called `Option::unwrap()` on a `None` value
```

#### 原因

`fork` 进程数过多，内存不足，发生崩溃

#### 解决方案

在 `fork` 申请内存时进行检查，允许 `fork` 失败，返回 `-1`

### ch8_04

#### 表现

内核崩溃

```bash
>> ch8_04
fname0
GOOD LUCK
[kernel] Panicked at /media/psf/Home/Documents/code/course/os/labs-2018011334/easy-fs/src/layout.rs:419 range end index 60 out of range for slice of length 28
```

#### 原因

文件名过长，`DirEntry` 放不下，即使修复这一点，之后还会有链接不存在的文件的问题，也会导致崩溃

#### 解决方案

创建文件时检查文件名是否过长，如果超过最大长度（27）则返回 `-1`

链接时检查原文件是否存在，不存在则返回 `-1`

### ch8_05

#### 表现

内核崩溃

```bash
>> ch8_05
[kernel] Panicked at src/trap/mod.rs:113 a trap Exception(StorePageFault) from kernel!
```

#### 原因

同 `ch8_01`

#### 解决方案

同 `ch8_01`

### ch8_06

#### 表现

```bash
>> ch8_06
mmap ...
[kernel] Panicked at src/trap/mod.rs:113 a trap Exception(StorePageFault) from kernel!
```

#### 原因 

子进程中没有父进程用 `mmap` 申请的内存区域，访问时会出错

#### 解决方案

`fork` 时复制 `mmap` 生成的区域



## 编程实现

加入 `fork` 时的内存检查，内存不足返回 `-1`。这样理论上可以通过 ch8_01 和 ch8_05。

首先修改 `sys_fork` 顶层实现：

```rust
pub fn sys_fork() -> isize {
    let current_task = current_task().unwrap();
    let new_task = match current_task.fork() {
        Ok(task) => task,
        Err(()) => return -1,
    };
  ..............
```

这要求 `TaskControlBlock.fork` 检查是否失败，在其申请 `MemorySet` 和 `KernelStack` 时进行检查

```rust
        let memory_set = MemorySet::from_existed_user(&parent_inner.memory_set)?;
...............................................
        let kernel_stack = KernelStack::new(&pid_handle)?;
```

当然，也需要为 `MemorySet` 和 `KernelStack` 加入检查，并且相应地原来调用这些方法的地方要加入检查/直接 `unwrap`

结果如下：

```bash
>> ch8_01      
Shell: Process 1 exited with code 0
>> ch8_05
Heavy fork test iteration 0 success.
Heavy fork test iteration 1 success.
Heavy fork test iteration 2 success.
Heavy fork test iteration 3 success.
Heavy fork test iteration 4 success.
Heavy fork test iteration 5 success.
Heavy fork test iteration 6 success.
Heavy fork test iteration 7 success.
Heavy fork test iteration 8 success.
Heavy fork test iteration 9 success.
Heavy fork test iteration 10 success.
Heavy fork test iteration 11 success.
Heavy fork test iteration 12 success.
Heavy fork test iteration 13 success.
Heavy fork test iteration 14 success.
Heavy fork test iteration 15 success.
Heavy fork test iteration 16 success.
Heavy fork test iteration 17 success.
Heavy fork test iteration 18 success.
Heavy fork test iteration 19 success.
Heavy fork test iteration 20 success.
Heavy fork test iteration 21 success.
Heavy fork test iteration 22 success.
Heavy fork test iteration 23 success.
Heavy fork test iteration 24 success.
Heavy fork test iteration 25 success.
Heavy fork test iteration 26 success.
Heavy fork test iteration 27 success.
Heavy fork test iteration 28 success.
Heavy fork test iteration 29 success.
Shell: Process 118 exited with code 0
```

