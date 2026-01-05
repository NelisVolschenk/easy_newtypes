
macro_rules! unsigned_impl {
    ($struct_name: ident) => {
        impl core::ops::Add<$struct_name> for $struct_name {
            type Output = Self;
        
            fn add(self, rhs: $struct_name) -> Self::Output {
                Self(self.0.add(rhs.0))
            }
        }
        
        impl core::ops::Sub<$struct_name> for $struct_name {
            type Output = Self;
        
            fn sub(self, rhs: $struct_name) -> Self::Output {
                Self(self.0.sub(rhs.0))
            }
        }
        
        impl core::ops::Mul<$struct_name> for $struct_name {
            type Output = Self;
        
            fn mul(self, rhs: $struct_name) -> Self::Output {
                Self(self.0.mul(rhs.0))
            }
        }
        
        impl core::ops::Div<$struct_name> for $struct_name {
            type Output = Self;
        
            fn div(self, rhs: $struct_name) -> Self::Output {
                Self(self.0.div(rhs.0))
            }
        }
        
        impl core::ops::Rem<$struct_name> for $struct_name {
            type Output = Self;
        
            fn rem(self, rhs: $struct_name) -> Self::Output {
                Self(self.0.rem(rhs.0))
            }
        }

        impl core::ops::AddAssign<$struct_name> for $struct_name {

            fn add_assign(&mut self, rhs: $struct_name){
                self.0 = self.0.add(rhs.0)
            }
        }

        impl core::ops::SubAssign<$struct_name> for $struct_name {

            fn sub_assign(&mut self, rhs: $struct_name){
                self.0 = self.0.sub(rhs.0)
            }
        }

        impl core::ops::MulAssign<$struct_name> for $struct_name {

            fn mul_assign(&mut self, rhs: $struct_name){
                self.0 = self.0.mul(rhs.0)
            }
        }

        impl core::ops::DivAssign<$struct_name> for $struct_name {

            fn div_assign(&mut self, rhs: $struct_name){
                self.0 = self.0.div(rhs.0)
            }
        }

        impl core::ops::RemAssign<$struct_name> for $struct_name {

            fn rem_assign(&mut self, rhs: $struct_name){
                self.0 = self.0.rem(rhs.0)
            }
        }


    };
}
pub(crate) use unsigned_impl;

macro_rules! signed_impl {
    ($struct_name: ident) => {
        impl core::ops::Neg for $struct_name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self(self.0.neg())
            }
        }
    };
}
pub(crate) use signed_impl;

macro_rules! unsigned_impl_inner {
    ($struct_name: ident, $inner_type: ty) => {
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

        impl core::ops::AddAssign<$inner_type> for $struct_name {

            fn add_assign(&mut self, rhs: $inner_type){
                self.0 = self.0.add(rhs)
            }
        }

        impl core::ops::SubAssign<$inner_type> for $struct_name {

            fn sub_assign(&mut self, rhs: $inner_type){
                self.0 = self.0.sub(rhs)
            }
        }

        impl core::ops::MulAssign<$inner_type> for $struct_name {

            fn mul_assign(&mut self, rhs: $inner_type){
                self.0 = self.0.mul(rhs)
            }
        }

        impl core::ops::DivAssign<$inner_type> for $struct_name {

            fn div_assign(&mut self, rhs: $inner_type){
                self.0 = self.0.div(rhs)
            }
        }

        impl core::ops::RemAssign<$inner_type> for $struct_name {

            fn rem_assign(&mut self, rhs: $inner_type){
                self.0 = self.0.rem(rhs)
            }
        }
    };
}
pub(crate) use unsigned_impl_inner;