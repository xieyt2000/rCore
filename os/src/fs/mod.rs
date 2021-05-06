mod pipe;
mod stdio;
mod mail;
mod inode;

use crate::mm::UserBuffer;

pub trait File: Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
    fn get_stat(&self) -> Option<Stat> {
        None
    }
}

pub use pipe::{Pipe, make_pipe};
pub use stdio::{Stdin, Stdout};
pub use mail::Mail;
pub use inode::{link, list_apps, open_file, unlink, OSInode, OpenFlags, Stat};