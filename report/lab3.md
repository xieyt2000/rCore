# lab3

谢云桐 计83 2018011334



## 编程内容

在本次实验中我在完成指导书内容的基础上实现了 `sys_gettime, sys_set_priority` 这两个系统调用和 stride 调度算法，其中 stride 的实现较为复杂，需要实现对应的数据结构和一个堆来动态调度，同时为程序设置了运行时间片上限。



## 简答作业

### 1

简要描述这一章的进程调度策略。何时进行进程切换？如何选择下一个运行的进程？如何处理新加入的进程？

本章实现了抢占式调度和协作式调度，抢占式调度会在时间片结束后进行进程切换，而协作式则在用户程序 `yield` 时切换进程。

选择下一个进程可以使用 Round-Robin 和 stride 算法，Round-Robin 即顺序选择下一个可以执行执行的进程，stride 算法则每次会选择 `stride` 最小的进程，`stride` 是由 `pass` 累加而来的，`pass` 与优先级反比。

RR 算法中新加入的进程原则上最好排在队尾，但是以现在的实现方式来看，排在队尾资源消耗可能比较大，更高效的方法是插在队列中一个随机位置（如数组的最后、第一个空位）。而 sride 算法中设定好其 `pass, stride` 后直接入堆自动调度即可

### 2

在 C 版代码中，同样实现了类似 RR 的调度算法，但是由于没有 VecDeque 这样直接可用的数据结构（Rust很棒对不对），C 版代码的实现严格来讲存在一定问题。大致情况如下：C版代码使用一个进程池（也就是一个 struct proc 的数组）管理进程调度，当一个时间片用尽后，选择下一个进程逻辑在 [chapter３相关代码](https://github.com/DeathWish5/ucore-Tutorial/blob/ch3/kernel/proc.c#L60-L74) ，也就是当第 i 号进程结束后，会以 i -> max_num -> 0 -> i 的顺序遍历进程池，直到找到下一个就绪进程。C 版代码新进程在调度池中的位置选择见 [chapter5相关代码](https://github.com/DeathWish5/ucore-Tutorial/blob/ch5/kernel/proc.c#L90-L98) ，也就是从头到尾遍历进程池，找到第一个空位。

(2-1) 在目前这一章（chapter3）两种调度策略有实质不同吗？考虑在一个完整的 os 中，随时可能有新进程产生，这两种策略是否实质相同？

没有不同（事实上，本章也没有使用 `VecDeque`）。当随时可能产生新进程时，两种策略不同，见下一小问。

(2-2) 其实 C 版调度策略在公平性上存在比较大的问题，请找到一个进程产生和结束的时间序列，使得在该调度算法下发生：先创建的进程后执行的现象。你需要给出类似下面例子的信息（有更详细的分析描述更好，但尽量精简）。同时指出该序列在你实现的 stride 调度算法下顺序是怎样的？

> | 时间点   | 0              | 1    | 2       | 3       | 4       | 5       | 6       | 7    |
> | -------- | -------------- | ---- | ------- | ------- | ------- | ------- | ------- | ---- |
> | 运行进程 |                | p1   | p2      | p3      | p4      | p1      | p3      |      |
> | 事件     | p1、p2、p3产生 |      | p2 结束 | p4 产生 | p4 结束 | p1 结束 | p3 结束 |      |
>
> 产生顺序：p1、p2、p3、p4。第一次执行顺序: p1、p2、p3、p4。没有违反公平性。
>
> 其他细节：允许进程在其他进程执行时产生（也就是被当前进程创建）/结束（也就是被当前进程杀死）。

| 时间点   | 0              | 1    | 2       | 3           | 4    | 5    | 6    | 7    |
| -------- | -------------- | ---- | ------- | ----------- | ---- | ---- | ---- | ---- |
| 运行进程 |                | p1   | p2      | p3          | p5   | p1   | p4   |      |
| 事件     | p1、p2、p3产生 |      | p2 结束 | p4、p5 产生 |      |      |      |      |

在 p2 结束后后，新产生的 p4 占用了之前 p2 的位置，导致它处于实际上的队尾，p5 会在它前面执行

如果 stride 且这些进程的优先级（`pass`） 都相等时，顺序为 p1, p2, p3, p4, p5, p1...。在 p3 执行完毕后，p4、p5 的 `stride` 都是最小，根据我的实现 `pid` 更小的会在堆顶，因此先执行 p4.



### 3

stride 算法深入

stride算法原理非常简单，但是有一个比较大的问题。例如两个 pass = 10 的进程，使用 8bit 无符号整形储存 stride， p1.stride = 255, p2.stride = 250，在 p2 执行一个时间片后，理论上下一次应该 p1 执行。

- 实际情况是轮到 p1 执行吗？为什么？

  轮到 p2 执行，`ps.stride` 溢出了，值变为 4

我们之前要求进程优先级 >= 2 其实就是为了解决这个问题。可以证明，**在不考虑溢出的情况下**, 在进程优先级全部 >= 2 的情况下，如果严格按照算法执行，那么 STRIDE_MAX – STRIDE_MIN <= BigStride / 2。

- 为什么？尝试简单说明（传达思想即可，不要求严格证明）。

  考虑第一次出现 STRIDE_MAX – STRIDE_MIN > BigStride / 2 的时间片，很显然 STRIDE_MAX 对应的程序刚刚执行完，根据 stride 算法，其执行之前 `stride <= STRIDE_MIN`，也就是该程序的 `pass > BigStride / 2`，即其 `priority < 2`。 

已知以上结论，**考虑溢出的情况下**，我们可以通过设计 Stride 的比较接口，结合 BinaryHeap 的 pop 接口可以很容易的找到真正最小的 Stride。

- 请补全如下 `partial_cmp` 函数（假设永远不会相等）。

```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if Ordering::max(self.0, other.0) - Ordering::min(self.0, other.0) <= BigStride / 2 {
            other.0.partial_cmp(&self.0)
        } else {
            self.0.partial_cmp(&other.0)
        }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```

例如使用 8 bits 存储 stride, BigStride = 255, 则:

- (125 < 255) == false
- (129 < 255) == true