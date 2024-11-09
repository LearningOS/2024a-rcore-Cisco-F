# 实现功能
* `sys_spawn`：从程序列表中找到传入的path对应的项目并将其转化为elf数据，之后调用new方法生成一个进程并令起开始运行即可。要注意维护父子进程之间的关系
* `sys_set_priority`：在tcb中加入priority字段，同时manager初始化时要注意对应成员的赋值
* `stride调度算法`：为了实现算法，除了priority字段，再为tcb加入stride字段。同时修改fetch方法，之前为从就绪队列头取出一个元素，现在需要遍历每一个任务，找到stride最小的任务返回。同时，task每被调度一次，要为其stride加上对应的pass值

# 问答作业
1. 不是。8bit无符号整形最大值为255，p2执行完一个时间片后stride+= pass结果为260，溢出，内存中stride实际值为4，比p1小，仍然会调度p2运行
2. 假设有两个进程 p1 和 p2，其初始 stride 值分别为stride1和stride2。由于 priority >= 2，pass值最小为 BigStride / 2。执行一个时间片后，stride1 增加 pass1，stride2 增加 pass2。由于 priority >= 2，pass1 和 pass2 均不小于 BigStride / 2。假设 stride1 和 stride2 的初始差值为 D，即 D = stride1 - stride2。执行一个时间片后，新的差值为 D' = (stride1 + pass1) - (stride2 + pass2)。由于 pass1 和 pass2 均不小于 BigStride / 2，因此 D' 的绝对值不会超过 BigStride / 2。
3. 补全代码如下,此处令`BigStride = u64::MAX`
```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let half_max = u64::MAX / 2;
        if self.0 == other.0 {
            None
        } else if (self.0 < other.0 && other.0 - self.0 <= half_max) || (self.0 > other.0 && self.0 - other.0 > half_max) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```
# 荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

> 无

此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

> [项目文档](https://learningos.cn/rCore-Tutorial-Book-v3/chapter1/3first-instruction-in-kernel1.html) 

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
