const FD_STDOUT: usize = 1;
use crate::batch::user_addr;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if !user_addr(buf as usize, len) {
        log::error!("[kernel] sys_write access denied: buf = {:#x}, len = {}", buf as usize, len);
        return -1;
    }
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        },
        _ => {
            log::error!("[kernel] Unsupported fd in sys_write!");
            -1
        }
    }
}