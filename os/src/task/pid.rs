//! # PID 分配器模块
//!
//! ## Overview
//! 本模块实现了内核中的 **进程 ID（PID）分配与回收机制**。
//! 使用了一个简单的 **回收式递增分配器（RecycleAllocator）** 来管理 PID。
//!
//! 功能包括：
//! - 为新进程分配唯一 PID
//! - 回收退出进程的 PID 以供重用
//! - 保证每个活动 PID 唯一且不重复
//!
//! ## Assumptions
//! - 系统运行在单处理器环境
//! - 所有对 PID 分配器的访问通过 `UPIntrFreeCell` 保护
//! - PID 从 0 开始递增，其中 `IDLE_PID = 0` 保留给空闲任务
//!
//! ## Safety
//! - 回收机制保证不会重复分配同一个 PID
//! - 对回收 PID 的访问必须在单线程 / 关中断保护下进行
//!
//! ## Invariants
//! - 活动 PID 集合中无重复值
//! - 回收的 PID 必然小于 `current`
//! - `recycled` 中的 PID 不在活动任务中
//!
//! ## Behavior
//! - `pid_alloc()`：
//!   - 返回新的 PID 句柄
//!   - 优先使用回收的 PID，其次递增分配
//! - `PidHandle` drop：
//!   - 自动将 PID 回收
//! - `RecycleAllocator`：
//!   - 内部记录当前最大 PID 与回收池

use crate::sync::UPIntrFreeCell;
use alloc::vec::Vec;
use lazy_static::lazy_static;

lazy_static! {
    /// 全局 PID 分配器
    ///
    /// ## Overview
    /// 管理系统中所有进程的 PID 分配与回收
    static ref PID_ALLOCATOR: UPIntrFreeCell<RecycleAllocator> =
        unsafe { UPIntrFreeCell::new(RecycleAllocator::new()) };
}

/// 空闲任务 PID 保留值
pub const IDLE_PID: usize = 0;

/// PID 句柄
///
/// ## Overview
/// 封装了 PID 值，并在 drop 时自动回收
pub struct PidHandle(pub usize);

/// 分配一个新的 PID
///
/// ## Behavior
/// - 从回收池中获取 PID（若有）
/// - 否则递增生成新的 PID
pub fn pid_alloc() -> PidHandle {
    PidHandle(PID_ALLOCATOR.exclusive_access().alloc())
}

impl Drop for PidHandle {
    /// PID 回收
    ///
    /// ## Behavior
    /// - 将 PID 返回到回收池
    fn drop(&mut self) {
        PID_ALLOCATOR.exclusive_access().dealloc(self.0);
    }
}

/// 可回收的递增 PID 分配器
///
/// ## Fields
/// - `current`：
///   - 当前最大 PID（未分配的下一个 PID）
/// - `recycled`：
///   - 回收池，存放已经释放的 PID
pub struct RecycleAllocator {
    current: usize,
    recycled: Vec<usize>,
}

impl RecycleAllocator {
    /// 创建新的 PID 分配器
    ///
    /// ## Invariants
    /// - 初始时 `current = 0`，`recycled` 为空
    pub fn new() -> Self {
        RecycleAllocator {
            current: 0,
            recycled: Vec::new(),
        }
    }

    /// 分配一个 PID
    ///
    /// ## Behavior
    /// - 优先返回回收 PID
    /// - 若无回收 PID，递增分配新 PID
    pub fn alloc(&mut self) -> usize {
        if let Some(id) = self.recycled.pop() {
            id
        } else {
            self.current += 1;
            self.current - 1
        }
    }

    /// 回收一个 PID
    ///
    /// ## Safety
    /// - PID 必须已经分配过
    /// - PID 不能重复回收
    ///
    /// ## Panics
    /// - PID 超出 `current` 或已在回收池中存在
    pub fn dealloc(&mut self, id: usize) {
        assert!(id < self.current);
        assert!(
            !self.recycled.iter().any(|i| *i == id),
            "id {} has been deallocated!",
            id
        );
        self.recycled.push(id);
    }
}
