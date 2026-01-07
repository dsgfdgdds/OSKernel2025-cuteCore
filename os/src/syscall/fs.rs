use alloc::string::String;
use crate::fs::{open_dir, open_file_at, resolve_path, OpenFlags};
use crate::mm::{translated_byte_buffer, translated_str, UserBuffer};
use crate::task::{current_process, current_user_token, current_task};
use bitflags::bitflags;

use alloc::sync::Arc;
pub const AT_FDCWD: usize = 100usize.wrapping_neg();


// 已实现
// pub fn sys_getcwd(buf: *const u8, len: usize) -> *const u8 {
//     let token = current_user_token();
//     let process = current_process();
//     let inner = process.inner_exclusive_access();
//     let cwd = &inner.cwd;
//     if cwd.len() + 1 > len {
//         return core::ptr::null();
//     }
//     let mut buffer = UserBuffer::new(translated_byte_buffer(token, buf, len));
//     buffer.write_string(cwd);
//     buf
// }

pub fn sys_getcwd(buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let process = current_process();
    let inner = process.inner_exclusive_access();
    let cwd = &inner.cwd;
    if cwd.len() + 1 > len {
        // return core::ptr::null();
        return -34;
    }
    let mut buffer = UserBuffer::new(translated_byte_buffer(token, buf, len));
    buffer.write_string(cwd);
    buf as isize
}

// 已实现
pub fn sys_chdir(path: *const u8) -> isize {
    let process = current_process();
    let token = current_user_token();
    let path = translated_str(token, path);
    drop(process);
    if open_dir(path.as_str()).is_err() {
        println!("open_dir: {}", path);
        return -1;
    }
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    inner.cwd = resolve_path(path.as_str(), inner.cwd.as_str());
    0
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let process = current_process();
    let inner = process.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        if !file.readable() {
            return -1;
        }
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        file.read(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let process = current_process();
    let inner = process.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        if !file.writable() {
            return -1;
        }
        let file = file.clone();
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        file.write(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}

// pub fn sys_open(path: *const u8, flags: u32) -> isize {
//     let process = current_process();
//     let token = current_user_token();
//     let path = translated_str(token, path);
//     if let Some(inode) = open_file(path.as_str(), OpenFlags::from_bits(flags).unwrap()) {
//         let mut inner = process.inner_exclusive_access();
//         let fd = inner.alloc_fd();
//         inner.fd_table[fd] = Some(inode);
//         fd as isize
//     } else {
//         -1
//     }
// }

pub fn sys_openat(dirfd: usize, path: *const u8, flags: u32, mode: u32) -> isize {
    let task = current_task().unwrap();
    let token = task.get_user_token();
    let process = task.process.upgrade().unwrap();
    let mut inner = process.inner_exclusive_access();
    let path =  translated_str(token, path);
    let flags = match OpenFlags::from_bits(flags) {
        Some(flags) => flags,
        None => {
            return -1; //EINVAL;
        }
    };
    let mode = StatMode::from_bits(mode);
    let mut fd_table = inner.fd_table.pop();
    // let file_descriptor = inner.cwd ;
    let base_dir = inner.cwd.clone();

    // let new_file_descriptor = match open_file_at(&base_dir, &path, flags, mode.unwrap()) {
    //     Some(file_descriptor) => file_descriptor,
    //     None => return -1,
    // };
        if let Some(inode) = open_file_at(&base_dir, &path, flags, mode.unwrap()) {
            let fd = inner.alloc_fd();
            inner.fd_table[fd] = Some(inode);
            fd as isize
        } else {
            -1
        }
}

// pub fn sys_pipe2(pipefd: usize, flags: u32) -> isize {
//     const VALID_FLAGS: OpenFlags = OpenFlags::from_bits_truncate(
//

bitflags! {
    pub struct StatMode: u32 {
        ///bit mask for the file type bit field
        const S_IFMT    =   0o170000;
        ///socket
        const S_IFSOCK  =   0o140000;
        ///symbolic link
        const S_IFLNK   =   0o120000;
        ///regular file
        const S_IFREG   =   0o100000;
        ///block device
        const S_IFBLK   =   0o060000;
        ///directory
        const S_IFDIR   =   0o040000;
        ///character device
        const S_IFCHR   =   0o020000;
        ///FIFO
        const S_IFIFO   =   0o010000;

        ///set-user-ID bit (see execve(2))
        const S_ISUID   =   0o4000;
        ///set-group-ID bit (see below)
        const S_ISGID   =   0o2000;
        ///sticky bit (see below)
        const S_ISVTX   =   0o1000;

        ///owner has read, write, and execute permission
        const S_IRWXU   =   0o0700;
        ///owner has read permission
        const S_IRUSR   =   0o0400;
        ///owner has write permission
        const S_IWUSR   =   0o0200;
        ///owner has execute permission
        const S_IXUSR   =   0o0100;

        ///group has read, write, and execute permission
        const S_IRWXG   =   0o0070;
        ///group has read permission
        const S_IRGRP   =   0o0040;
        ///group has write permission
        const S_IWGRP   =   0o0020;
        ///group has execute permission
        const S_IXGRP   =   0o0010;

        ///others (not in group) have read, write,and execute permission
        const S_IRWXO   =   0o0007;
        ///others have read permission
        const S_IROTH   =   0o0004;
        ///others have write permission
        const S_IWOTH   =   0o0002;
        ///others have execute permission
        const S_IXOTH   =   0o0001;
    }
}
