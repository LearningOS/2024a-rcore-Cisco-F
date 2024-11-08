# 实现功能
对于`syscall_times`，在内核的syscall（）中维护。在进入具体的处理函数之前，更新当前任务的系统调用数
对于`time`，在每次调用`sys_task_info()`时，将当前时间与任务创建时记录的时间相减即可
对于`status`，用相同的方法获取即可；也可直接赋值为`TaskStatus::Running`

# 简答作业
1. sbi版本为`RustSBI-QEMU Version 0.2.0-alpha.2`
使用S态特权指令：[kernel] IllegalInstruction in application, kernel killed it.直接判断指令不合法，内核报错退出。
访问S态寄存器：[kernel] IllegalInstruction in application, kernel killed it.直接判断指令不合法，内核报错退出。
访问错误地址(0x0)：[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003ac, kernel killed it.出现了PageFault,同样报错退出

2.  a. a0代表了内核栈的栈顶。__restore既可以用来启动新的应用（在系统启动或者一个应用结束需要启动另一个程序时），也可以用来从trap_handler运行后恢复context
    b. 特殊处理了sstatus,sepc和sscratch。sstatus保存了Trap发生前cpu所处于的特权级；sepc保存了用户态从trap恢复后继续运行的指令地址，因此很重要。sscratch保存了进入内核栈的栈顶地址，对于下一次触发trap很重要。
    c. x2为sp，因为在__alltrap触发之前就已经被保存在了sscratch中，因此无需保存。x4为tp，常用于多核情况，在当前环境下也不需要保存
    d. 该指令交换了sscratch和sp的值，即在进入用户态前把sp设置为用户态栈顶，并将sscratch 设置为内核态栈顶。
    e. sret指令。其用于从S态返回到U态
    f. 发生之后，sp为内核栈栈顶，sscratch为用户栈栈顶。
    g. user_lib中调用ecall方法之后即trap进内核

# 荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

> 无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

> ![项目文档](https://learningos.cn/rCore-Tutorial-Book-v3/chapter1/3first-instruction-in-kernel1.html) 

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。