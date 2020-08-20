pub struct T;

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

pub struct Math;

pub trait TAnd<A, B> { type Result; }
#[macro_export]
macro_rules! tand (($a:ty, $b:ty)=>(<$crate::Math as $crate::TAnd<$a, $b>>::Result));

impl TAnd<F, F> for Math { type Result = F; }

impl TAnd<F, T> for Math { type Result = F; }

impl TAnd<T, F> for Math { type Result = F; }

impl TAnd<T, T> for Math { type Result = T; }

pub trait TOr<A, B> { type Result; }
#[macro_export]
macro_rules! tor (($a:ty, $b:ty)=>(<$crate::Math as $crate::TOr<$a, $b>>::Result));

impl TOr<F, F> for Math { type Result = F; }

impl TOr<F, T> for Math { type Result = T; }

impl TOr<T, F> for Math { type Result = T; }

impl TOr<T, T> for Math { type Result = T; }

pub trait TNot<T> { type Result; }
#[macro_export]
macro_rules! tnot (($a:ty)=>(<$crate::Math as $crate::TNot<$a>>::Result));

impl TNot<F> for Math { type Result = T; }

impl TNot<T> for Math { type Result = F; }

pub trait TXor<A, B> { type Result; }
#[macro_export]
macro_rules! txor (($a:ty, $b:ty)=>(<$crate::Math as $crate::TXor<$a, $b>>::Result));

impl TXor<F, F> for Math { type Result = F; }

impl TXor<F, T> for Math { type Result = T; }

impl TXor<T, F> for Math { type Result = T; }

impl TXor<T, T> for Math { type Result = F; }

pub trait TWhereTrue<T> {}

impl TWhereTrue<T> for Math {}

pub trait TWhereFalse<T> {}

impl TWhereFalse<F> for Math {}

pub type Int8<A0, A1, A2, A3, A4, A5, A6, A7> = (A0, A1, A2, A3, A4, A5, A6, A7);

pub type Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af> = (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af);

pub type Int32<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>
= (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f);

pub trait Integer {
    const VALUE: usize;
}

impl<A0: Bool, A1: Bool, A2: Bool, A3: Bool, A4: Bool, A5: Bool, A6: Bool, A7: Bool> Integer for Int8<A0, A1, A2, A3, A4, A5, A6, A7>
{
    const VALUE: usize =
        A0::VALUE_I << 7
            | A1::VALUE_I << 6
            | A2::VALUE_I << 5
            | A3::VALUE_I << 4
            | A4::VALUE_I << 3
            | A5::VALUE_I << 2
            | A6::VALUE_I << 1
            | A7::VALUE_I << 0;
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af> Integer for Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af>
    where Int8<A0, A1, A2, A3, A4, A5, A6, A7>: Integer,
          Int8<A8, A9, Aa, Ab, Ac, Ad, Ae, Af>: Integer
{
    const VALUE: usize = <Int8<A0, A1, A2, A3, A4, A5, A6, A7> as Integer>::VALUE << 0x8
        | <Int8<A8, A9, Aa, Ab, Ac, Ad, Ae, Af> as Integer>::VALUE;
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f> Integer
for Int32<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>
    where Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af>: Integer,
          Int16<A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>: Integer
{
    const VALUE: usize = <Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af> as Integer>::VALUE << 0x10
        | <Int16<A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f> as Integer>::VALUE;
}

pub trait TIsZero<T> { type Result; }
#[macro_export]
macro_rules! tiszero {
    ($T:ty)=>(<$crate::Math as $crate::TIsZero<$T>>::Result)
}

impl<A0, A1, A2, A3, A4, A5, A6, A7> TIsZero<Int8<A0, A1, A2, A3, A4, A5, A6, A7>> for Math
    where Math: TOr<A0, A1> + TOr<A2, A3> + TOr<A4, A5> + TOr<A6, A7>,
          Math: TOr<tor!(A0,A1), tor!(A2,A3)> + TOr<tor!(A4,A5), tor!(A6,A7)>,
          Math: TOr<tor!(tor!(A0,A1), tor!(A2,A3)), tor!(tor!(A4,A5), tor!(A6,A7))>,
          Math: TNot<tor!(tor!(tor!(A0,A1), tor!(A2,A3)), tor!(tor!(A4,A5), tor!(A6,A7)))>,
{
    type Result = tnot!(tor!(tor!(tor!(A0,A1), tor!(A2,A3)), tor!(tor!(A4,A5), tor!(A6,A7))));
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af> TIsZero<Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af>> for Math
    where Math: TIsZero<Int8<A0, A1, A2, A3, A4, A5, A6, A7>> + TIsZero<Int8<A8, A9, Aa, Ab, Ac, Ad, Ae, Af>>,
          Math: TAnd<tiszero!(Int8<A0, A1, A2, A3, A4, A5, A6, A7>), tiszero!(Int8<A8, A9, Aa, Ab, Ac, Ad, Ae, Af>)>
{
    type Result = tand!(tiszero!(Int8<A0, A1, A2, A3, A4, A5, A6, A7>), tiszero!(Int8<A8, A9, Aa, Ab, Ac, Ad, Ae, Af>));
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>
TIsZero<Int32<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>>
for Math
    where Math: TIsZero<Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af>> + TIsZero<Int16<A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>>,
          Math: TAnd<tiszero!(Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af>), tiszero!(Int16<A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>)>
{
    type Result = tand!(tiszero!(Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af>), tiszero!(Int16<A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>));
}

pub trait TIsNeg<T> { type Result; }
#[macro_export]
macro_rules! tisneg {
    ($T:ty)=>(<$crate::Math as $crate::TIsNeg<$T>>::Result)
}

impl<A0, A1, A2, A3, A4, A5, A6, A7> TIsNeg<Int8<A0, A1, A2, A3, A4, A5, A6, A7>> for Math
{
    type Result = A0;
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af> TIsNeg<Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af>> for Math
{
    type Result = A0;
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>
TIsNeg<Int32<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>>
for Math
{
    type Result = A0;
}

type UInt8AddIntermediate<A0, A1, A2, A3, A4, A5, A6, A7, B0, B1, B2, B3, B4, B5, B6, B7> = ((A0, B0), (A1, B1), (A2, B2), (A3, B3), (A4, B4), (A5, B5), (A6, B6), (A7, B7));

pub trait TAdd<A, B, C> {
    type Result;
    type Carry;
}

#[macro_export]
macro_rules! tadd {
    ($A:ty, $B:ty)=>(<$crate::Math as $crate::TAdd<$A, $B, $crate::F>>::Result)
}

pub trait TMinus<T> { type Result; }
#[macro_export]
macro_rules! tminus {
    ($T:ty)=>(<$crate::Math as $crate::TMinus<$T>>::Result)
}

impl<A0, A1, A2, A3, A4, A5, A6, A7> TMinus<Int8<A0, A1, A2, A3, A4, A5, A6, A7>> for Math
    where Math: TNot<A0> + TNot<A1> + TNot<A2> + TNot<A3> + TNot<A4> + TNot<A5> + TNot<A6> + TNot<A7>,
          Math: TAdd<Int8<tnot!(A0), tnot!(A1), tnot!(A2), tnot!(A3), tnot!(A4), tnot!(A5), tnot!(A6), tnot!(A7)>, Int8<F, F, F, F, F, F, F, T>, F>
{
    type Result = tadd!(Int8<tnot!(A0),tnot!(A1),tnot!(A2),tnot!(A3),tnot!(A4),tnot!(A5),tnot!(A6),tnot!(A7)>,Int8<F,F,F,F,F,F,F,T>);
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af> TMinus<Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af>> for Math
    where Math: TNot<A0> + TNot<A1> + TNot<A2> + TNot<A3> + TNot<A4> + TNot<A5> + TNot<A6> + TNot<A7> + TNot<A8> + TNot<A9> + TNot<Aa> + TNot<Ab> + TNot<Ac> + TNot<Ad> + TNot<Ae> + TNot<Af>,
          Math: TAdd<Int16<tnot!(A0), tnot!(A1), tnot!(A2), tnot!(A3), tnot!(A4), tnot!(A5), tnot!(A6), tnot!(A7), tnot!(A8), tnot!(A9), tnot!(Aa), tnot!(Ab), tnot!(Ac), tnot!(Ad), tnot!(Ae), tnot!(Af)>, Int16<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T>, F>
{
    type Result = tadd!(Int16<tnot!(A0), tnot!(A1), tnot!(A2), tnot!(A3), tnot!(A4), tnot!(A5), tnot!(A6), tnot!(A7), tnot!(A8), tnot!(A9), tnot!(Aa), tnot!(Ab), tnot!(Ac), tnot!(Ad), tnot!(Ae), tnot!(Af)>, Int16<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T>);
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f> TMinus<Int32<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>> for Math
    where Math: TNot<A0> + TNot<A1> + TNot<A2> + TNot<A3> + TNot<A4> + TNot<A5> + TNot<A6> + TNot<A7> + TNot<A8> + TNot<A9> + TNot<Aa> + TNot<Ab> + TNot<Ac> + TNot<Ad> + TNot<Ae> + TNot<Af> + TNot<A10> + TNot<A11> + TNot<A12> + TNot<A13> + TNot<A14> + TNot<A15> + TNot<A16> + TNot<A17> + TNot<A18> + TNot<A19> + TNot<A1a> + TNot<A1b> + TNot<A1c> + TNot<A1d> + TNot<A1e> + TNot<A1f>,
          Math: TAdd<Int32<tnot!(A0), tnot!(A1), tnot!(A2), tnot!(A3), tnot!(A4), tnot!(A5), tnot!(A6), tnot!(A7), tnot!(A8), tnot!(A9), tnot!(Aa), tnot!(Ab), tnot!(Ac), tnot!(Ad), tnot!(Ae), tnot!(Af), tnot!(A10), tnot!(A11), tnot!(A12), tnot!(A13), tnot!(A14), tnot!(A15), tnot!(A16), tnot!(A17), tnot!(A18), tnot!(A19), tnot!(A1a), tnot!(A1b), tnot!(A1c), tnot!(A1d), tnot!(A1e), tnot!(A1f)>, Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T>, F>
{
    type Result = tadd!(Int32<tnot!(A0), tnot!(A1), tnot!(A2), tnot!(A3), tnot!(A4), tnot!(A5), tnot!(A6), tnot!(A7), tnot!(A8), tnot!(A9), tnot!(Aa), tnot!(Ab), tnot!(Ac), tnot!(Ad), tnot!(Ae), tnot!(Af), tnot!(A10), tnot!(A11), tnot!(A12), tnot!(A13), tnot!(A14), tnot!(A15), tnot!(A16), tnot!(A17), tnot!(A18), tnot!(A19), tnot!(A1a), tnot!(A1b), tnot!(A1c), tnot!(A1d), tnot!(A1e), tnot!(A1f)>, Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T>);
}

macro_rules! tm_gen {
    ($G0:ty,$P0:ty,$G1:ty)=>{tor!($G0,tand!($P0,$G1))};
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, B0, B1, B2, B3, B4, B5, B6, B7, C> TAdd<Int8<A0, A1, A2, A3, A4, A5, A6, A7>, Int8<B0, B1, B2, B3, B4, B5, B6, B7>, C> for Math
    where Math: TAnd<A0, B0> + TAnd<A1, B1> + TAnd<A2, B2> + TAnd<A3, B3> + TAnd<A4, B4> + TAnd<A5, B5> + TAnd<A6, B6> + TAnd<A7, B7>,
          Math: TOr<A0, B0> + TOr<A1, B1> + TOr<A2, B2> + TOr<A3, B3> + TOr<A4, B4> + TOr<A5, B5> + TOr<A6, B6> + TOr<A7, B7>,
          Math: TXor<A0, B0> + TXor<A1, B1> + TXor<A2, B2> + TXor<A3, B3> + TXor<A4, B4> + TXor<A5, B5> + TXor<A6, B6> + TXor<A7, B7>,
          Math: TAnd<tor!(A7, B7), C> + TOr<tand!(A7, B7), tand!(tor!(A7, B7), C)>,
          MathI: UInt8Add0<UInt8AddIntermediate<
              tand!(A0, B0),
              tand!(A1, B1),
              tand!(A2, B2),
              tand!(A3, B3),
              tand!(A4, B4),
              tand!(A5, B5),
              tand!(A6, B6),
              tm_gen!(tand!(A7, B7), tor!(A7, B7), C),
              // tm_gen/!(tand!(A7, B7), tor!(A7, B7), C),
              tor!(A0, B0),
              tor!(A1, B1),
              tor!(A2, B2),
              tor!(A3, B3),
              tor!(A4, B4),
              tor!(A5, B5),
              tor!(A6, B6),
              tor!(A7, B7)
          >>
          + UInt8Add3<
              Int8<
                  txor!(A0, B0),
                  txor!(A1, B1),
                  txor!(A2, B2),
                  txor!(A3, B3),
                  txor!(A4, B4),
                  txor!(A5, B5),
                  txor!(A6, B6),
                  txor!(A7, B7),
              >,
              <MathI as UInt8Add0<UInt8AddIntermediate<
                  tand!(A0, B0),
                  tand!(A1, B1),
                  tand!(A2, B2),
                  tand!(A3, B3),
                  tand!(A4, B4),
                  tand!(A5, B5),
                  tand!(A6, B6),
                  tm_gen!(tand!(A7, B7), tor!(A7, B7), C),
                  tor!(A0, B0),
                  tor!(A1, B1),
                  tor!(A2, B2),
                  tor!(A3, B3),
                  tor!(A4, B4),
                  tor!(A5, B5),
                  tor!(A6, B6),
                  tor!(A7, B7)
              >>>::Result
              , C
          >
{
    type Result = <MathI as UInt8Add3<
        Int8<
            txor!(A0,B0),
            txor!(A1,B1),
            txor!(A2,B2),
            txor!(A3,B3),
            txor!(A4,B4),
            txor!(A5,B5),
            txor!(A6,B6),
            txor!(A7,B7),
        >,
        <MathI as UInt8Add0<UInt8AddIntermediate<
            tand!(A0, B0),
            tand!(A1, B1),
            tand!(A2, B2),
            tand!(A3, B3),
            tand!(A4, B4),
            tand!(A5, B5),
            tand!(A6, B6),
            tm_gen!(tand!(A7, B7), tor!(A7, B7), C),
            tor!(A0, B0),
            tor!(A1, B1),
            tor!(A2, B2),
            tor!(A3, B3),
            tor!(A4, B4),
            tor!(A5, B5),
            tor!(A6, B6),
            tor!(A7, B7)
        >>>::Result,
        C
    >>::Result;
    type Carry = <MathI as UInt8Add3<
        Int8<
            txor!(A0,B0),
            txor!(A1,B1),
            txor!(A2,B2),
            txor!(A3,B3),
            txor!(A4,B4),
            txor!(A5,B5),
            txor!(A6,B6),
            txor!(A7,B7),
        >,
        <MathI as UInt8Add0<UInt8AddIntermediate<
            tand!(A0, B0),
            tand!(A1, B1),
            tand!(A2, B2),
            tand!(A3, B3),
            tand!(A4, B4),
            tand!(A5, B5),
            tand!(A6, B6),
            tm_gen!(tand!(A7, B7), tor!(A7, B7), C),
            tor!(A0, B0),
            tor!(A1, B1),
            tor!(A2, B2),
            tor!(A3, B3),
            tor!(A4, B4),
            tor!(A5, B5),
            tor!(A6, B6),
            tor!(A7, B7)
        >>>::Result,
        C
    >>::Carry;
}


impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, Ba, Bb, Bc, Bd, Be, Bf, C>
TAdd<Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af>, Int16<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, Ba, Bb, Bc, Bd, Be, Bf>, C> for Math
    where Math: TAdd<Int8<A8, A9, Aa, Ab, Ac, Ad, Ae, Af>, Int8<B8, B9, Ba, Bb, Bc, Bd, Be, Bf>, C>,
          Math: TAdd<Int8<A0, A1, A2, A3, A4, A5, A6, A7>, Int8<B0, B1, B2, B3, B4, B5, B6, B7>, <Math as TAdd<Int8<A8, A9, Aa, Ab, Ac, Ad, Ae, Af>, Int8<B8, B9, Ba, Bb, Bc, Bd, Be, Bf>, C>>::Carry>,
          MathI: Concat<<Math as TAdd<Int8<A0, A1, A2, A3, A4, A5, A6, A7>, Int8<B0, B1, B2, B3, B4, B5, B6, B7>, <Math as TAdd<Int8<A8, A9, Aa, Ab, Ac, Ad, Ae, Af>, Int8<B8, B9, Ba, Bb, Bc, Bd, Be, Bf>, C>>::Carry>>::Result, <Math as TAdd<Int8<A8, A9, Aa, Ab, Ac, Ad, Ae, Af>, Int8<B8, B9, Ba, Bb, Bc, Bd, Be, Bf>, C>>::Result>,
{
    type Result = <MathI as Concat<<Math as TAdd<
        Int8<A0, A1, A2, A3, A4, A5, A6, A7>,
        Int8<B0, B1, B2, B3, B4, B5, B6, B7>,
        <Math as TAdd<
            Int8<A8, A9, Aa, Ab, Ac, Ad, Ae, Af>,
            Int8<B8, B9, Ba, Bb, Bc, Bd, Be, Bf>,
            C
        >>::Carry
    >>::Result,
        <Math as TAdd<
            Int8<A8, A9, Aa, Ab, Ac, Ad, Ae, Af>,
            Int8<B8, B9, Ba, Bb, Bc, Bd, Be, Bf>,
            C
        >>::Result
    >>::Result;
    type Carry = <Math as TAdd<
        Int8<A0, A1, A2, A3, A4, A5, A6, A7>,
        Int8<B0, B1, B2, B3, B4, B5, B6, B7>,
        <Math as TAdd<
            Int8<A8, A9, Aa, Ab, Ac, Ad, Ae, Af>,
            Int8<B8, B9, Ba, Bb, Bc, Bd, Be, Bf>,
            C
        >>::Carry
    >>::Carry;
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, Ba, Bb, Bc, Bd, Be, Bf, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B1a, B1b, B1c, B1d, B1e, B1f, C>
TAdd<Int32<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>, Int32<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, Ba, Bb, Bc, Bd, Be, Bf, B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B1a, B1b, B1c, B1d, B1e, B1f>, C> for Math
    where Math: TAdd<Int16<A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>, Int16<B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B1a, B1b, B1c, B1d, B1e, B1f>, C>,
          Math: TAdd<Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af>, Int16<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, Ba, Bb, Bc, Bd, Be, Bf>, <Math as TAdd<Int16<A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>, Int16<B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B1a, B1b, B1c, B1d, B1e, B1f>, C>>::Carry>,
          MathI: Concat<<Math as TAdd<Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af>, Int16<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, Ba, Bb, Bc, Bd, Be, Bf>, <Math as TAdd<Int16<A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>, Int16<B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B1a, B1b, B1c, B1d, B1e, B1f>, C>>::Carry>>::Result, <Math as TAdd<Int16<A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>, Int16<B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B1a, B1b, B1c, B1d, B1e, B1f>, C>>::Result>,
{
    type Result = <MathI as Concat<<Math as TAdd<
        Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af>,
        Int16<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, Ba, Bb, Bc, Bd, Be, Bf>,
        <Math as TAdd<
            Int16<A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>,
            Int16<B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B1a, B1b, B1c, B1d, B1e, B1f>,
            C
        >>::Carry
    >>::Result,
        <Math as TAdd<
            Int16<A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>,
            Int16<B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B1a, B1b, B1c, B1d, B1e, B1f>,
            C
        >>::Result
    >>::Result;
    type Carry = <Math as TAdd<
        Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af>,
        Int16<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, Ba, Bb, Bc, Bd, Be, Bf>,
        <Math as TAdd<
            Int16<A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>,
            Int16<B10, B11, B12, B13, B14, B15, B16, B17, B18, B19, B1a, B1b, B1c, B1d, B1e, B1f>,
            C
        >>::Carry
    >>::Carry;
}

pub struct MathI;

pub trait Concat<A, B> { type Result; }

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af> Concat<Int8<A0, A1, A2, A3, A4, A5, A6, A7>, Int8<A8, A9, Aa, Ab, Ac, Ad, Ae, Af>> for MathI {
    type Result = Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af>;
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>
Concat<Int16<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af>, Int16<A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>> for MathI {
    type Result = Int32<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, Aa, Ab, Ac, Ad, Ae, Af, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A1a, A1b, A1c, A1d, A1e, A1f>;
}

pub trait UInt8Add0<T> { type Result; }

impl<A0, A1, A2, A3, A4, A5, A6, A7, B0, B1, B2, B3, B4, B5, B6, B7> UInt8Add0<UInt8AddIntermediate<A0, A1, A2, A3, A4, A5, A6, A7, B0, B1, B2, B3, B4, B5, B6, B7>> for MathI
    where Math: TAnd<B0, A1> + TOr<A0, tand!(B0,A1)> + TAnd<B2, A3> + TOr<A2, tand!(B2,A3)> + TAnd<B4, A5> + TOr<A4, tand!(B4,A5)> + TAnd<B6, A7> + TOr<A6, tand!(B6,A7)>,
          Math: TAnd<B0, B1> + TAnd<B2, B3> + TAnd<B4, B5> + TAnd<B6, B7>,
          MathI: UInt8Add1<UInt8AddIntermediate<tm_gen!(A0,B0,A1), A1, tm_gen!(A2,B2,A3), A3, tm_gen!(A4,B4,A5), A5, tm_gen!(A6,B6,A7), A7, tand!(B0,B1), B1, tand!(B2,B3), B3, tand!(B4,B5), B5, tand!(B6,B7), B7>>
{
    type Result = <MathI as UInt8Add1<UInt8AddIntermediate<tm_gen!(A0,B0,A1), A1, tm_gen!(A2,B2,A3), A3, tm_gen!(A4,B4,A5), A5, tm_gen!(A6,B6,A7), A7, tand!(B0,B1), B1, tand!(B2,B3), B3, tand!(B4,B5), B5, tand!(B6,B7), B7>>>::Result;
}

pub trait UInt8Add1<T> { type Result; }

impl<A0, A1, A2, A3, A4, A5, A6, A7, B0, B1, B2, B3, B4, B5, B6, B7> UInt8Add1<UInt8AddIntermediate<A0, A1, A2, A3, A4, A5, A6, A7, B0, B1, B2, B3, B4, B5, B6, B7>> for MathI
    where Math: TAnd<B0, A2> + TOr<A0, tand!(B0,A2)> + TAnd<B1, A2> + TOr<A1, tand!(B1,A2)> + TAnd<B4, A6> + TOr<A4, tand!(B4,A6)> + TAnd<B5, A6> + TOr<A5, tand!(B5,A6)>,
          Math: TAnd<B0, B2> + TAnd<B1, B2> + TAnd<B4, B6> + TAnd<B5, B6>,
          MathI: UInt8Add2<UInt8AddIntermediate<tm_gen!(A0,B0,A2), tm_gen!(A1,B1,A2), A2, A3, tm_gen!(A4,B4,A6), tm_gen!(A5,B5,A6), A6, A7, tand!(B0,B2), tand!(B1,B2), B2, B3, tand!(B4,B6), tand!(B5,B6), B6, B7>>
{
    type Result = <MathI as UInt8Add2<UInt8AddIntermediate<tm_gen!(A0,B0,A2), tm_gen!(A1,B1,A2), A2, A3, tm_gen!(A4,B4,A6), tm_gen!(A5,B5,A6), A6, A7, tand!(B0,B2), tand!(B1,B2), B2, B3, tand!(B4,B6), tand!(B5,B6), B6, B7>>>::Result;
}

pub trait UInt8Add2<T> { type Result; }

impl<A0, A1, A2, A3, A4, A5, A6, A7, B0, B1, B2, B3, B4, B5, B6, B7> UInt8Add2<UInt8AddIntermediate<A0, A1, A2, A3, A4, A5, A6, A7, B0, B1, B2, B3, B4, B5, B6, B7>> for MathI
    where Math: TAnd<B0, A4> + TOr<A0, tand!(B0,A4)> + TAnd<B1, A4> + TOr<A1, tand!(B1,A4)> + TAnd<B2, A4> + TOr<A2, tand!(B2,A4)> + TAnd<B3, A4> + TOr<A3, tand!(B3,A4)>
{
    type Result = Int8<tm_gen!(A0,B0,A4), tm_gen!(A1,B1,A4), tm_gen!(A2,B2,A4), tm_gen!(A3,B3,A4), A4, A5, A6, A7>;
}

pub trait UInt8Add3<S, T, C> {
    type Result;
    type Carry;
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, B0, B1, B2, B3, B4, B5, B6, B7, C> UInt8Add3<Int8<A0, A1, A2, A3, A4, A5, A6, A7>, Int8<B0, B1, B2, B3, B4, B5, B6, B7>, C> for MathI
    where Math: TXor<A0, B1> + TXor<A1, B2> + TXor<A2, B3> + TXor<A3, B4> + TXor<A4, B5> + TXor<A5, B6> + TXor<A6, B7> + TXor<A7, C>
{
    type Result = Int8<txor!(A0,B1), txor!(A1,B2), txor!(A2,B3), txor!(A3,B4), txor!(A4,B5), txor!(A5,B6), txor!(A6,B7), txor!(A7,C)>;
    type Carry = B0;
}

#[cfg(test)]
mod tests {
    use crate::{Bool, F, Int16, Int32, Int8, Integer, Math, T, TAdd};

    #[test]
    fn it_works() {
        assert_eq!(<Int8<T, T, T, T, T, T, T, T> as Integer>::VALUE, 255);
        assert_eq!(<Int8<F, F, F, F, F, F, F, T> as Integer>::VALUE, 1);
        assert_eq!(<Int8<F, F, F, F, F, F, T, F> as Integer>::VALUE, 2);
        assert_eq!(<Int8<F, F, F, F, F, T, F, F> as Integer>::VALUE, 4);
        assert_eq!(<Int8<F, F, F, F, T, F, F, F> as Integer>::VALUE, 8);
        assert_eq!(<Int8<F, F, F, T, F, F, F, F> as Integer>::VALUE, 16);
        assert_eq!(<Int8<F, F, T, F, F, F, F, F> as Integer>::VALUE, 32);
        assert_eq!(<Int8<F, T, F, F, F, F, F, F> as Integer>::VALUE, 64);
        assert_eq!(<Int8<T, F, F, F, F, F, F, F> as Integer>::VALUE, 128);
        assert_eq!(<Int16<F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F> as Integer>::VALUE, 256);
        assert_eq!(<Int16<F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 512);
        assert_eq!(<Int16<F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 1024);
        assert_eq!(<Int16<F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 2048);
        assert_eq!(<Int16<F, F, F, T, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 4096);
        assert_eq!(<Int16<F, F, T, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 8192);
        assert_eq!(<Int16<F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 16384);
        assert_eq!(<Int16<T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 32768);
        assert_eq!(<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 65536);
        assert_eq!(<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 131072);
        assert_eq!(<Int32<F, F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 262144);
        assert_eq!(<Int32<F, F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 524288);
        assert_eq!(<Int32<F, F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 1048576);
        assert_eq!(<Int32<F, F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 2097152);
        assert_eq!(<Int32<F, F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 4194304);
        assert_eq!(<Int32<F, F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 8388608);
        assert_eq!(<Int32<F, F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 16777216);
        assert_eq!(<Int32<F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 33554432);
        assert_eq!(<Int32<F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 67108864);
        assert_eq!(<Int32<F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 134217728);
        assert_eq!(<Int32<F, F, F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 268435456);
        assert_eq!(<Int32<F, F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 536870912);
        assert_eq!(<Int32<F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 1073741824);
        assert_eq!(<Int32<T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F> as Integer>::VALUE, 2147483648);

        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, F, F, F, T>, F>>::Result as Integer>::VALUE, 0);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, F, F, T, F>, F>>::Result as Integer>::VALUE, 1);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, F, T, F, F>, F>>::Result as Integer>::VALUE, 3);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, T, F, F, F>, F>>::Result as Integer>::VALUE, 7);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, T, F, F, F, F>, F>>::Result as Integer>::VALUE, 15);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, T, F, F, F, F, F>, F>>::Result as Integer>::VALUE, 31);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, T, F, F, F, F, F, F>, F>>::Result as Integer>::VALUE, 63);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<T, F, F, F, F, F, F, F>, F>>::Result as Integer>::VALUE, 127);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, F, F, F, F>, F>>::Result as Integer>::VALUE, 255);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, F, F, F, T>, Int8<T, T, T, T, T, T, T, T>, F>>::Result as Integer>::VALUE, 0);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, F, F, T, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Result as Integer>::VALUE, 1);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, F, T, F, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Result as Integer>::VALUE, 3);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, T, F, F, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Result as Integer>::VALUE, 7);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, T, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Result as Integer>::VALUE, 15);
        assert_eq!(<<Math as TAdd<Int8<F, F, T, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Result as Integer>::VALUE, 31);
        assert_eq!(<<Math as TAdd<Int8<F, T, F, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Result as Integer>::VALUE, 63);
        assert_eq!(<<Math as TAdd<Int8<T, F, F, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Result as Integer>::VALUE, 127);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Result as Integer>::VALUE, 255);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, F, F, F, T>, T>>::Result as Integer>::VALUE, 1);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, F, F, T, F>, T>>::Result as Integer>::VALUE, 2);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, F, T, F, F>, T>>::Result as Integer>::VALUE, 4);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, T, F, F, F>, T>>::Result as Integer>::VALUE, 8);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, T, F, F, F, F>, T>>::Result as Integer>::VALUE, 16);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, T, F, F, F, F, F>, T>>::Result as Integer>::VALUE, 32);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, T, F, F, F, F, F, F>, T>>::Result as Integer>::VALUE, 64);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<T, F, F, F, F, F, F, F>, T>>::Result as Integer>::VALUE, 128);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, F, F, F, F>, T>>::Result as Integer>::VALUE, 0);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, F, F, F, T>, Int8<T, T, T, T, T, T, T, T>, T>>::Result as Integer>::VALUE, 1);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, F, F, T, F>, Int8<T, T, T, T, T, T, T, T>, T>>::Result as Integer>::VALUE, 2);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, F, T, F, F>, Int8<T, T, T, T, T, T, T, T>, T>>::Result as Integer>::VALUE, 4);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, T, F, F, F>, Int8<T, T, T, T, T, T, T, T>, T>>::Result as Integer>::VALUE, 8);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, T, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, T>>::Result as Integer>::VALUE, 16);
        assert_eq!(<<Math as TAdd<Int8<F, F, T, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, T>>::Result as Integer>::VALUE, 32);
        assert_eq!(<<Math as TAdd<Int8<F, T, F, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, T>>::Result as Integer>::VALUE, 64);
        assert_eq!(<<Math as TAdd<Int8<T, F, F, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, T>>::Result as Integer>::VALUE, 128);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, T>>::Result as Integer>::VALUE, 0);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, F, F, F, T>, F>>::Result as Integer>::VALUE, 0);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, F, F, T, F>, F>>::Result as Integer>::VALUE, 1);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, F, T, F, F>, F>>::Result as Integer>::VALUE, 3);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, T, F, F, F>, F>>::Result as Integer>::VALUE, 7);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, T, F, F, F, F>, F>>::Result as Integer>::VALUE, 15);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, T, F, F, F, F, F>, F>>::Result as Integer>::VALUE, 31);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, T, F, F, F, F, F, F>, F>>::Result as Integer>::VALUE, 63);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<T, F, F, F, F, F, F, F>, F>>::Result as Integer>::VALUE, 127);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, F, F, F, F>, F>>::Result as Integer>::VALUE, 255);

        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, F, F, F, T>, Int8<T, T, T, T, T, T, T, T>, F>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, F, F, T, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, F, T, F, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, T, F, F, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, T, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<F, F, T, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<F, T, F, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<T, F, F, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Carry as Bool>::VALUE, false);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, F, F, F, T>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, F, F, T, F>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, F, T, F, F>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, T, F, F, F>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, T, F, F, F, F>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, T, F, F, F, F, F>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, T, F, F, F, F, F, F>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<T, F, F, F, F, F, F, F>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<T, T, T, T, T, T, T, T>, Int8<F, F, F, F, F, F, F, F>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, F, F, F, T>, Int8<T, T, T, T, T, T, T, T>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, F, F, T, F>, Int8<T, T, T, T, T, T, T, T>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, F, T, F, F>, Int8<T, T, T, T, T, T, T, T>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, T, F, F, F>, Int8<T, T, T, T, T, T, T, T>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, T, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<F, F, T, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<F, T, F, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<T, F, F, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, T>>::Carry as Bool>::VALUE, true);
        assert_eq!(<<Math as TAdd<Int8<F, F, F, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, T>>::Carry as Bool>::VALUE, true);

        assert_eq!(std::mem::size_of::<<Math as TAdd<Int8<F, F, F, F, F, F, F, F>, Int8<T, T, T, T, T, T, T, T>, F>>::Result>(), 0);
    }
}
