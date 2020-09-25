#[derive(Copy, Clone, Default)]
pub struct T;

#[derive(Copy, Clone, Default)]
pub struct F;

pub trait Bool {
    const VALUE: bool;
    const VALUE_I: usize;
}

impl Bool for T {
    const VALUE: bool = true;
    const VALUE_I: usize = 1;
}

impl Bool for F {
    const VALUE: bool = false;
    const VALUE_I: usize = 0;
}

pub struct BooleanMath;

pub trait TAnd<A, B> { type Result; }
#[macro_export]
macro_rules! tand (($a:ty, $b:ty)=>(<$crate::type_boolean::BooleanMath as $crate::type_integer::TAnd<$a, $b>>::Result));

impl TAnd<F, F> for BooleanMath { type Result = F; }

impl TAnd<F, T> for BooleanMath { type Result = F; }

impl TAnd<T, F> for BooleanMath { type Result = F; }

impl TAnd<T, T> for BooleanMath { type Result = T; }

pub trait TOr<A, B> { type Result; }
#[macro_export]
macro_rules! tor (($a:ty, $b:ty)=>(<$crate::type_boolean::BooleanMath as $crate::type_integer::TOr<$a, $b>>::Result));

impl TOr<F, F> for BooleanMath { type Result = F; }

impl TOr<F, T> for BooleanMath { type Result = T; }

impl TOr<T, F> for BooleanMath { type Result = T; }

impl TOr<T, T> for BooleanMath { type Result = T; }

pub trait TNot<T> { type Result; }
#[macro_export]
macro_rules! tnot (($a:ty)=>(<$crate::type_boolean::BooleanMath as $crate::type_integer::TNot<$a>>::Result));

impl TNot<F> for BooleanMath { type Result = T; }

impl TNot<T> for BooleanMath { type Result = F; }

pub trait TXor<A, B> { type Result; }
#[macro_export]
macro_rules! txor (($a:ty, $b:ty)=>(<$crate::type_boolean::BooleanMath as $crate::type_integer::TXor<$a, $b>>::Result));

impl TXor<F, F> for BooleanMath { type Result = F; }

impl TXor<F, T> for BooleanMath { type Result = T; }

impl TXor<T, F> for BooleanMath { type Result = T; }

impl TXor<T, T> for BooleanMath { type Result = F; }

pub trait TWhereTrue<T> {}

impl TWhereTrue<T> for BooleanMath {}

pub trait TWhereFalse<T> {}

impl TWhereFalse<F> for BooleanMath {}
