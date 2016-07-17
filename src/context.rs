// This file is part of libfringe, a low-level green threading library.
// Copyright (c) Nathan Zadoks <nathan@nathan7.eu>,
//               whitequark <whitequark@whitequark.org>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
use stack;
use debug;
use arch;

/// Context holds a suspended thread of execution along with a stack.
///
/// It can be swapped into and out of with the swap method,
/// and once you're done with it, you can get the stack back through unwrap.
///
/// Every operation is unsafe, because no guarantees can be made about
/// the state of the context.
#[derive(Debug)]
pub struct Context<Stack: stack::Stack> {
  stack:     Stack,
  stack_id:  debug::StackId,
  stack_ptr: arch::StackPointer
}

unsafe impl<Stack> Send for Context<Stack>
  where Stack: stack::Stack + Send {}

impl<Stack> Context<Stack> where Stack: stack::Stack {
  /// Creates a new Context. When it is swapped into, it will call
  /// `f(arg)`, where `arg` is the argument passed to `swap`.
  pub unsafe fn new(stack: Stack, f: unsafe extern "C" fn(usize) -> !) -> Context<Stack> {
    let stack_id  = debug::StackId::register(&stack);
    let stack_ptr = arch::init(&stack, f);
    Context {
      stack:     stack,
      stack_id:  stack_id,
      stack_ptr: stack_ptr
    }
  }

  /// Unwraps the context, returning the stack it contained.
  pub unsafe fn unwrap(self) -> Stack {
    self.stack
  }
}

impl<OldStack> Context<OldStack> where OldStack: stack::Stack {
  /// Switches to `in_ctx`, saving the current thread of execution to `out_ctx`.
  #[inline(always)]
  pub unsafe fn swap<NewStack>(old_ctx: *mut Context<OldStack>,
                               new_ctx: *const Context<NewStack>,
                               arg: usize) -> usize
      where NewStack: stack::Stack {
    arch::swap(arg, &mut (*old_ctx).stack_ptr, &(*new_ctx).stack_ptr)
  }
}
