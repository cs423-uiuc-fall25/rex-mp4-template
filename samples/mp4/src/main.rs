#![no_std]
#![no_main]

use core::fmt::Write;

use rex::kprobe::kprobe;
use rex::map::{RexQueue, RexRingBuf};
use rex::pt_regs::PtRegs;
use rex::task_struct::OwnedTaskStruct;
use rex::{Result, rex_kprobe, rex_map};
use rex::rex_printk;

#[rex_map]
static QUEUE: RexQueue<i32> = RexQueue::new(64, 0);

#[rex_map]
static RINGBUF: RexRingBuf = RexRingBuf::new(4096, 0);

#[rex_kprobe(function = "write_null")]
pub fn mp1(_obj: &kprobe, _ctx: &mut PtRegs) -> Result {
    Ok(0)
}
