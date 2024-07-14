// 对于系统调用而言， syscall 函数并不会实际处理系统调用，
// 而只是根据 syscall ID 分发到具体的处理函数
mod fs;
mod process;

use fs::sys_write;
use process::*;

use crate::task::TASKINFOARR;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_TASK_INFO:usize = 410;
// yield:屈服，让出，放弃

pub fn syscall(syscall_id:usize,args:[usize;3])->isize{
    let ti;
    unsafe {
        ti= TASKINFOARR.yy();
        (*ti.p_sys_time())[syscall_id]+=1;
    }
    // 用户级的系统输出
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[2], args[1] as *const u8),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(),
        SYSCALL_TASK_INFO=>{
            unsafe {
                sys_task_info(ti)
            }
        },
        _=>panic!("unsupported syscall_id:{}",syscall_id), 
    }
}