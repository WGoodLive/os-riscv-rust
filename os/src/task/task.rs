/* 
通过 #[derive(...)] 可以让编译器为你的类型提供一些 Trait 的默认实现。
    实现了 Clone Trait 之后就可以调用 clone 函数完成拷贝；
    实现了 PartialEq Trait 之后就可以使用 == 运算符比较该类型的两个实例，从逻辑上说只有 两个相等的应用执行状态才会被判为相等，而事实上也确实如此。
    Copy 是一个标记 Trait，决定该类型在按值传参/赋值的时候采用移动语义还是复制语义。
*/

#[derive(Clone, Copy,PartialEq)]
/// 任务状态
pub enum TaskStatus{
    UnInit, // 未初始化
    Ready, // 准备运行
    Running, // 正在运行
    Exited, // 已退出
}

use super::context::TaskContext;
#[derive(Clone,Copy)]
/// 任务控制块(很重要)
pub struct TaskControlBlock{
    pub task_status: TaskStatus,// 任务状态
    pub task_cx: TaskContext,
}
