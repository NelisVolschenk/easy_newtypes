
macro_rules! unsigned_impl {
    ($struct_name: ident, $rhs_type: ty, $inner_type: ty) => {
        impl core::ops::Add<$rhs_type> for $struct_name {
            type Output = Self;
        
            fn add(self, rhs: $rhs_type) -> Self::Output {
                Self(self.0.add(rhs.0))
            }
        }
        
        impl core::ops::Sub<$rhs_type> for $struct_name {
            type Output = Self;
        
            fn sub(self, rhs: $rhs_type) -> Self::Output {
                Self(self.0.sub(rhs.0))
            }
        }
        
        impl core::ops::Mul<$rhs_type> for $struct_name {
            type Output = Self;
        
            fn mul(self, rhs: $rhs_type) -> Self::Output {
                Self(self.0.mul(rhs.0))
            }
        }
        
        impl core::ops::Div<$rhs_type> for $struct_name {
            type Output = Self;
        
            fn div(self, rhs: $rhs_type) -> Self::Output {
                Self(self.0.div(rhs.0))
            }
        }
        
        impl core::ops::Rem<$rhs_type> for $struct_name {
            type Output = Self;
        
            fn rem(self, rhs: $rhs_type) -> Self::Output {
                Self(self.0.rem(rhs.0))
            }
        }
    };
}
pub(crate) use unsigned_impl;

macro_rules! signed_impl {
    ($struct_name: ident, $rhs_type: ty, $inner_type: ty) => {

    }
}

macro_rules! unsigned_impl_inner {
    ($struct_name: ident, $rhs_type: ty, $inner_type: ty) => {
        impl core::ops::Add<$inner_type> for $struct_name {
            type Output = Self;

            fn add(self, rhs: $inner_type) -> Self::Output {
                Self(self.0.add(rhs))
            }
        }

        impl core::ops::Sub<$inner_type> for $struct_name {
            type Output = Self;

            fn sub(self, rhs: $inner_type) -> Self::Output {
                Self(self.0.sub(rhs))
            }
        }

        impl core::ops::Mul<$inner_type> for $struct_name {
            type Output = Self;

            fn mul(self, rhs: $inner_type) -> Self::Output {
                Self(self.0.mul(rhs))
            }
        }

        impl core::ops::Div<$inner_type> for $struct_name {
            type Output = Self;

            fn div(self, rhs: $inner_type) -> Self::Output {
                Self(self.0.div(rhs))
            }
        }

        impl core::ops::Rem<$inner_type> for $struct_name {
            type Output = Self;

            fn rem(self, rhs: $inner_type) -> Self::Output {
                Self(self.0.rem(rhs))
            }
        }
    };
}
pub(crate) use unsigned_impl_inner;