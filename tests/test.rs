use core::mem;

use simple_scan::IteratorSimpleScanExt;

use crate::for_test::*;

#[test]
fn trace() {
    with_empty();
    with_normal();

    fn with_empty() {
        let target = empty_iter();
        let not_called = |_s: &i32, _x: i32| {
            assert!(false);
            0
        };
        let mut result = target.trace(0, not_called);
        assert_eq!(result.next(), None);
    }

    fn with_normal() {
        let result = sample_iter().trace(0, |s, x| s + x);
        let expect = sample_iter().scan(0, |s, x| {
            *s += x;
            Some(*s)
        });

        assert!(result.eq(expect));
    }
}

#[test]
fn trace2() {
    with_empty();
    with_normal();

    fn with_empty() {
        let not_called = |_s: &i32, _x: i32| {
            assert!(false);
            0
        };
        let mut result = empty_iter().trace2(0, not_called);
        assert_eq!(result.next(), None);
    }

    fn with_normal() {
        let result = sample_iter().trace2(0, |s, x| s + x);
        let expect = sample_iter().scan(0, |s, x| {
            let prev = *s;
            *s += x;
            Some((prev, *s))
        });

        assert!(result.eq(expect));
    }
}

#[test]
fn diff() {
    with_empty();
    with_normal();

    fn with_empty() {
        let not_called = |_c: i32, _p: i32| {
            assert!(false);
            0
        };
        let mut result = empty_iter().diff(0, not_called);
        assert_eq!(result.next(), None);
    }

    fn with_normal() {
        const INI_VAL: i32 = 3;
        let result = sample_iter().diff(INI_VAL, |c, p| c - p);
        let expect = sample_iter().scan(INI_VAL, |s, x| {
            let prev = mem::replace(s, x);
            Some(x - prev)
        });

        assert!(result.eq(expect));
    }
}

mod for_test {
    const SAMPLE_SIZE: i32 = 10;

    pub fn empty_iter() -> impl Iterator<Item = i32> {
        0..0
    }

    pub fn sample_iter() -> impl Iterator<Item = i32> {
        0..SAMPLE_SIZE
    }
}
