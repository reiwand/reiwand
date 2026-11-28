use rustix::fs::{MemfdFlags, SealFlags, ftruncate, memfd_create};
use rustix::mm::{MapFlags, ProtFlags, mmap, munmap};
use std::os::fd::OwnedFd;

pub const SLOTS: usize = 2;

pub struct Ring {
    pub fd: OwnedFd,
    pub size: (u32, u32),
    pub stride: u32,
    pub busy: [bool; SLOTS],
    pub sent: bool,
    map: *mut u8,
    len: usize,
}

impl Ring {
    pub fn new(size: (u32, u32), stride: u32) -> std::io::Result<Ring> {
        let slot = stride as usize * size.1 as usize;
        let len = slot * SLOTS;
        let fd = memfd_create("reiwand-web-frames", MemfdFlags::CLOEXEC | MemfdFlags::ALLOW_SEALING)?;
        ftruncate(&fd, len as u64)?;
        rustix::fs::fcntl_add_seals(&fd, SealFlags::SHRINK | SealFlags::GROW)?;
        let flags = ProtFlags::READ | ProtFlags::WRITE;
        let map = unsafe { mmap(std::ptr::null_mut(), len, flags, MapFlags::SHARED, &fd, 0) }?.cast();
        Ok(Ring { fd, size, stride, busy: [false; SLOTS], sent: false, map, len })
    }

    pub fn fits(&self, size: (u32, u32), stride: u32) -> bool {
        self.size == size && self.stride == stride
    }

    pub fn claim(&mut self) -> Option<(u32, *mut u8)> {
        let slot = self.busy.iter().position(|b| !b)?;
        self.busy[slot] = true;
        Some((slot as u32, unsafe { self.map.add(slot * (self.len / SLOTS)) }))
    }

    pub fn release(&mut self, slot: u32) {
        if let Some(busy) = self.busy.get_mut(slot as usize) {
            *busy = false;
        }
    }
}

impl Drop for Ring {
    fn drop(&mut self) {
        let _ = unsafe { munmap(self.map.cast(), self.len) };
    }
}
