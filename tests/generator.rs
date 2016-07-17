// This file is part of libfringe, a low-level green threading library.
// Copyright (c) whitequark <whitequark@whitequark.org>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
extern crate fringe;

use fringe::OsStack;
use fringe::generator::Generator;

#[test]
fn generator() {
  let stack = OsStack::new(0).unwrap();
  let mut gen = Generator::new(stack, move |yielder| {
    for i in 1..4 {
      yielder.generate(i);
    }
  });
  assert_eq!(gen.next(), Some(1));
  assert_eq!(gen.next(), Some(2));
  assert_eq!(gen.next(), Some(3));
  assert_eq!(gen.next(), None);
}

#[test]
fn move_after_new() {
  let stack = OsStack::new(0).unwrap();
  let mut gen = Generator::new(stack, move |yielder| {
    for i in 1..4 {
      yielder.generate(i);
    }
  });
  assert_eq!(gen.next(), Some(1));

  #[inline(never)]
  fn rest(mut gen: Generator<u32, OsStack>) {
    assert_eq!(gen.next(), Some(2));
    assert_eq!(gen.next(), Some(3));
    assert_eq!(gen.next(), None);
  }
  rest(gen);
}
