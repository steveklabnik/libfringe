// This file is part of libfringe, a low-level green threading library.
// Copyright (c) Nathan Zadoks <nathan@nathan7.eu>
// See the LICENSE file included in this distribution.
//
use stack::Stack;

#[derive(Debug)]
pub struct StackPointer(*mut usize);

impl StackPointer {
  unsafe fn new(stack: &Stack) -> StackPointer {
    StackPointer(stack.top() as *mut usize)
  }

  unsafe fn push(&mut self, val: usize) {
    self.0 = self.0.offset(-1);
    *self.0 = val
  }
}

pub unsafe fn init(stack: &Stack, f: unsafe extern "C" fn(usize) -> !) -> StackPointer {
  let mut sp = StackPointer::new(stack);
  sp.push(0); // alignment
  sp.push(f as usize);
  sp
}


#[inline(always)]
pub unsafe fn swap(arg: usize, old_sp: &mut StackPointer, new_sp: &StackPointer) -> usize {
  let ret: usize;
  asm!(
    r#"
      ldr r1, [0]
    "#
    : "={r1}" (ret)
    : "{r1}" (arg)
      "{r2}" (old_sp)
      "{r3}" (new_sp)
    : "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "r9", "r10", "r11", "r12", //"r13", "r14"
      "cc", "flags", "memory"
    : "volatile");
  ret
}
