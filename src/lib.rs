pub(crate) mod arith;

use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};


#[macro_export]
macro_rules! new_t {
    ($name: ident, $inner_type: ty) => {
        new_t!($name,$inner_type,derives = [],impls = []);
    };
    ($name: ident, $inner_type: ty, derives = [$($extra_derives: tt)*]) => {
        new_t!($name,$inner_type,derives = [$($extra_derives)*],impls = []);
    };
    ($name: ident, $inner_type: ty, impls = [$($extra_impls: tt)*]) => {
        new_t!($name,$inner_type,derives = [],impls = [$($extra_impls)*]);
    };
    ($name: ident, $inner_type: ty, derives = [$($extra_derives: tt),*], impls = [$($extra_impls: tt),*]) => {
        $(#[derive($extra_derives)])*
        struct $name($inner_type);
        $(impl_block!($extra_impls, $name, $inner_type);)*
    };
}

macro_rules! impl_block {
    ("arith", $name: ident, $inner_type:ty) => {
        crate::arith::unsigned_impl!($name, $name, $inner_type);
    };
    ("arith_inner", $name: ident, $inner_type:ty) => {
        crate::arith::unsigned_impl_inner!($name, $inner_type, $inner_type);
    }
}

#[derive(Clone)]
struct Test(u8);

impl Add for Test {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

//
// impl Add<u8> for Test {
//     type Output = Self;
//
//     fn add(self, rhs: u8) -> Self::Output {
//         Self(self.0 + rhs)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    new_t!(Basic, u8);
    new_t!(WithDer, u8, derives = [Clone, Copy]);
    new_t!(ModbusId, u8, impls = ["arith", "arith_inner"]);

    #[test]
    fn manual() {
        let t1 = Test(5);
        let t2 = Test(7);
        let tres = t1 + t2;
        assert_eq!(tres.0, 5+7)
    }

    #[test]
    fn normal_add() {
        let val1 = ModbusId(5);
        let val2 = ModbusId(7);
        let v_res = val1 + val2;
        assert_eq!(v_res.0, 5+7)
    }

    #[test]
    fn inner_add() {
        let val1 = ModbusId(5);
        let val2 = 7;
        let v_res = val1 + val2;
        assert_eq!(v_res.0, 5+7)
    }
}
