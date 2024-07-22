

use crate::println;

use super::{page_table::PageTableEntry, PAGE_SIZE, PAGE_SIZE_BITS, PA_WIDTH_SV39, PPN_WIDTH_SV39};
use alloc::fmt::Debug;
use alloc::fmt::Formatter;
use alloc::fmt;
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// 物理地址56bits
pub struct PhysAddr(pub usize); // 元组结构体

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// 虚拟地址
pub struct VirtAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// 物理页号PPN
pub struct PhysPageNum(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// 虚拟页号
pub struct VirtPageNum(pub usize);


/* 
From之间的转换是独立的

impl From<T> for U：
U::from(_:T) -> T

impl Into<T> for U:
let t:T = U.into(); // 必须指定转换类型
*/

impl From<usize> for PhysAddr { // usize -> PhysAddr
    fn from(value: usize) -> Self {
        Self(value & ((1<<PA_WIDTH_SV39) -1))
    }
}

impl From<usize> for PhysPageNum { 
    fn from(value: usize) -> Self {
        println!("...");
        println!("{:#x}",value & ( (1 << PPN_WIDTH_SV39) - 1 ));
        Self(value & ( (1 << PPN_WIDTH_SV39) - 1 )) 
    }
}

impl From<PhysAddr> for usize { // PhyaAddr -> usize
    fn from(v: PhysAddr) -> Self {
        v.0 
    }
}
impl From<PhysPageNum> for usize {
    fn from(v: PhysPageNum) -> Self {
        v.0 
    }
}

impl PhysAddr {
    pub fn page_offset(&self) ->usize{
        self.0 & (PAGE_SIZE-1)
    }
    
    // 这个不能直接右移，为啥
    pub fn floor(&self) -> PhysPageNum { 
        PhysPageNum(self.0 / PAGE_SIZE)  // 不知道这里为啥不能右移
    }

    /// PhysAddr -> PhysPageNum
    pub fn ceil(&self) -> PhysPageNum {  
        PhysPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE) 
    }
}


impl From<PhysAddr> for PhysPageNum {
    /// 实现从物理地址到物理页号(PPN)的转换
    fn from(value: PhysAddr) -> Self {
        assert_eq!(value.page_offset(),0);
        value.floor()
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(v: PhysPageNum) -> Self { 
        Self(v.0 << PAGE_SIZE_BITS) 
    }
}

impl PhysPageNum {
    /// 一个字节数组的可变引用，可以以字节为粒度对物理页帧上的数据进行访问
    pub fn get_bytes_array(&self) -> &'static mut [u8]{
        let pa:PhysAddr = (*self).into();
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut u8, PAGE_SIZE)
        }
    }

    /// 返回一个页表项定长数组的可变引用，代表多级页表中的一个节点
    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry]{
        // 实际上这个代码如果不用'static，就要来回考虑借用规则，生命周期了，或者
        // 使用unsafe，不能保证不是空指针，但是'static能保证访问的地方一定有数据
        let pa:PhysAddr = self.clone().into();
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, 512)
        }
    }

    // 泛型函数，可以获取一个恰好放在一个物理页帧开头的类型为 T 的数据的可变引用
    pub fn get_mut<T>(&self) -> &'static mut T {
        let pa: PhysAddr = self.clone().into();
        unsafe {
            (pa.0 as *mut T).as_mut().unwrap()
        }
    }

    
}

impl VirtPageNum{
    /// 返回虚拟地址的低29位  
    /// 因为是恒等映射，所以就29位VPN=PPN
    pub fn indexes(&self) ->[usize;3]{
        let mut vpn = self.0;
        let mut idx = [0usize; 3];
        for i in (0..3).rev() {
            idx[i] = vpn & 511;
            vpn >>= 9;
        }
        idx
    }
}

impl Debug for VirtAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VA:{:#x}", self.0))
    }
}
impl Debug for VirtPageNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VPN:{:#x}", self.0))
    }
}
impl Debug for PhysAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PA:{:#x}", self.0))
    }
}
impl Debug for PhysPageNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PPN:{:#x}", self.0))
    }
}