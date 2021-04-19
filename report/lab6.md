# lab6



## 编程内容

实现了邮件数据结构 `Mail`，封装一个报文队列 `VecDeque<Vec<u8>>`，实现了 `read(_awailable), write(_available)` 方法，和 `stdio` 类似

实现了 `mailread, mailwrite` 的系统调用，在 `TaskControlBlockInner` 中维护 `Mail`，加入了根据 `pid` 查找进程的方法



## 问答作业

### 1

举出使用 pipe 的一个实际应用的例子

查看所有打开的 terminal：

```bash
who | grep tty
xieyuntong ttys001  Apr 18 22:23
xieyuntong ttys002  Apr 18 22:23
xieyuntong ttys003  Apr 18 22:23
```

### 2

假设我们的邮箱现在有了更加强大的功能，容量大幅增加而且记录邮件来源，可以实现“回信”。考虑一个多核场景，有 m 个核为消费者，n 个为生产者，消费者通过邮箱向生产者提出订单，生产者通过邮箱回信给出产品。

- 假设你的邮箱实现没有使用锁等机制进行保护，在多核情景下可能会发生哪些问题？单核一定不会发生问题吗？为什么？

  如果没有锁，多核线程不安全。如两个消费者都向邮箱只有一个空位的生产者提出订单，它们都先检查邮箱未满然后都向其中写入邮件导致超过原有容量

  在本实现下单核不会发生这种问题，因为内核态下不允许时钟中断。但是如果允许的话会产生同样的问题

- 请结合你在课堂上学到的内容，描述读者写者问题的经典解决方案，必要时提供伪代码。

  使用信号量实现读者优先的锁，伪代码如下：

  ```c
  void mail_write(){
      P(WriteMutex);
      write();
      V(WriteMutex);
  }
  
  void mail_read(){
      P(CountMutex);
      if(RCount == 0) {
          P(WriteMutex);
      }
      RCount++;
      V(CountMutex);
      read();
      P(CountMutex);
      RCount--;
      if(RCount == 0) {
          V(WriteMutex);
      }
      V(CountMutex);
  }
  ```

  `WriteMutex` 是控制互斥访问的信号量，`RCount` 是正在读的进程数目，`CountMutex` 是保护 `RCount` 的信号量

- 由于读写是基于报文的，不是随机读写，你有什么点子来优化邮箱的实现吗？

  对每个报文而不是整个邮箱加锁，提高效率