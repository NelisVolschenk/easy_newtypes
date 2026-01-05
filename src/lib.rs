pub(crate) mod arith;

use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};


#[macro_export]
macro_rules! new_t {
    ($name: ident, $inner_type: tt) => {
        new_t!($name,$inner_type,derives = [],impls = []);
    };
    ($name: ident, $inner_type: tt, derives = [$($extra_derives: tt)*]) => {
        new_t!($name,$inner_type,derives = [$($extra_derives)*],impls = []);
    };
    ($name: ident, $inner_type: tt, impls = [$($extra_impls: tt)*]) => {
        new_t!($name,$inner_type,derives = [],impls = [$($extra_impls)*]);
    };
    ($name: ident, $inner_type: tt, derives = [$($extra_derives: tt),*], impls = [$($extra_impls: tt),*]) => {
        $(#[derive($extra_derives)])*
        struct $name($inner_type);
        $(impl_block!($extra_impls, $name, $inner_type);)*
    };
}

macro_rules! impl_block {
    ("arith", $name: ident, u8) => {
        crate::arith::unsigned_impl!($name);
    };
    ("arith", $name: ident, u16) => {
        crate::arith::unsigned_impl!($name);
    };
    ("arith", $name: ident, u32) => {
        crate::arith::unsigned_impl!($name);
    };
    ("arith", $name: ident, u64) => {
        crate::arith::unsigned_impl!($name);
    };
    ("arith", $name: ident, u128) => {
        crate::arith::unsigned_impl!($name);
    };
    ("arith", $name: ident, usize) => {
        crate::arith::unsigned_impl!($name);
    };
    ("arith", $name: ident, i8) => {
        crate::arith::unsigned_impl!($name);
        crate::arith::signed_impl!($name);
    };
    ("arith", $name: ident, i16) => {
        crate::arith::unsigned_impl!($name);
        crate::arith::signed_impl!($name);
    };
    ("arith", $name: ident, i32) => {
        crate::arith::unsigned_impl!($name);
        crate::arith::signed_impl!($name);
    };
    ("arith", $name: ident, i64) => {
        crate::arith::unsigned_impl!($name);
        crate::arith::signed_impl!($name);
    };
    ("arith", $name: ident, i128) => {
        crate::arith::unsigned_impl!($name);
        crate::arith::signed_impl!($name);
    };
    ("arith", $name: ident, isize) => {
        crate::arith::unsigned_impl!($name);
        crate::arith::signed_impl!($name);
    };

    ("arith_inner", $name: ident, $inner_type:tt) => {
        crate::arith::unsigned_impl_inner!($name, $inner_type);
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    new_t!(Unsigned, u32, derives = [Clone, Copy, Debug], impls = ["arith", "arith_inner"]);

    #[test]
    fn normal_add() {
        let val1 = Unsigned(5);
        let val2 = Unsigned(7);
        let v_res = val1 + val2;
        assert_eq!(v_res.0, 5+7)
    }

    #[test]
    fn add_assign() {
        let mut val1 = Unsigned(5);
        let val2 = Unsigned(7);
        val1 += val2;
        assert_eq!(val1.0, 5+7)
    }

    #[test]
    fn inner_add() {
        let val1 = Unsigned(5);
        let val2 = 7;
        let v_res = val1 + val2;
        assert_eq!(v_res.0, 5+7)
    }


}
