use std::marker::PhantomData;

pub type Zero = ();
pub type Suc<T> = (T, );

pub struct Lambda<T>(PhantomData<T>);

pub struct ItemIndex<Num>(PhantomData<Num>);

pub struct ItemValue<Value>(PhantomData<Value>);

pub struct Call<F, A>(PhantomData<(F, A)>);

pub trait Shift<Depth, N, S> {
    type Result;
}

impl<Depth, N, S, T> Shift<Depth, N, S> for Lambda<T>
    where T: Shift<Suc<Depth>, N, S> {
    type Result = Lambda<<T as Shift<Suc<Depth>, N, S>>::Result>;
}

impl<Depth, N, S, Num> Shift<Suc<Depth>, Suc<N>, S> for ItemIndex<Num>
    where ItemIndex<Num>: Shift<Depth, N, S> {
    type Result = <ItemIndex<Num> as Shift<Depth, N, S>>::Result;
}

impl<Depth, S, Num> Shift<Suc<Depth>, Zero, S> for ItemIndex<Num> {
    type Result = ItemIndex<Num>;
}

impl<N, S, Num> Shift<Zero, N, Suc<S>> for ItemIndex<Num>
    where ItemIndex<Suc<Num>>: Shift<Zero, N, S> {
    type Result = <ItemIndex<Suc<Num>> as Shift<Zero, N, S>>::Result;
}

impl<N, Num> Shift<Zero, N, Zero> for ItemIndex<Num> {
    type Result = ItemIndex<Num>;
}

impl<Depth, N, S, Value> Shift<Depth, N, S> for ItemValue<Value> {
    type Result = ItemValue<Value>;
}

impl<Depth, N, S, F, A> Shift<Depth, N, S> for Call<F, A>
    where F: Shift<Depth, N, S>,
          A: Shift<Depth, N, S> {
    type Result = Call<<F as Shift<Depth, N, S>>::Result, <A as Shift<Depth, N, S>>::Result>;
}

pub trait AlphaTransform<Depth, Value> { type Result; }

impl<Depth, Value, T> AlphaTransform<Depth, Value> for Lambda<T>
    where T: AlphaTransform<Suc<Depth>, Value> {
    type Result = Lambda<<T as AlphaTransform<Suc<Depth>, Value>>::Result>;
}

impl<Depth, Value, F, A> AlphaTransform<Depth, Value> for Call<F, A>
    where F: AlphaTransform<Depth, Value>,
          A: AlphaTransform<Depth, Value> {
    type Result = Call<<F as AlphaTransform<Depth, Value>>::Result, <A as AlphaTransform<Depth, Value>>::Result>;
}

impl<Depth, Value, Num> AlphaTransform<Depth, Value> for ItemIndex<Num>
    where ItemIndex<Num>: ItemIndexAlphaTransform<Depth, Num, Value> {
    type Result = <ItemIndex<Num> as ItemIndexAlphaTransform<Depth, Num, Value>>::Result;
}

pub trait ItemIndexAlphaTransform<Depth, Num, Value> { type Result; }

impl<Depth, Num, Value, N> ItemIndexAlphaTransform<Suc<Depth>, Suc<Num>, Value> for ItemIndex<N>
    where ItemIndex<N>: ItemIndexAlphaTransform<Depth, Num, Value> {
    type Result = <ItemIndex<N> as ItemIndexAlphaTransform<Depth, Num, Value>>::Result;
}

impl<Depth, Value, N> ItemIndexAlphaTransform<Suc<Depth>, Zero, Value> for ItemIndex<N> {
    type Result = ItemIndex<N>;
}

impl<Num, Value, N> ItemIndexAlphaTransform<Zero, Suc<Num>, Value> for ItemIndex<Suc<N>> {
    type Result = ItemIndex<N>;
}

impl<Value, N> ItemIndexAlphaTransform<Zero, Zero, Value> for ItemIndex<N>
    where Value: Shift<Zero, N, N> {
    type Result = <Value as Shift<Zero, N, N>>::Result;
}

impl<Depth, Value, V> AlphaTransform<Depth, Value> for ItemValue<V> {
    type Result = ItemValue<V>;
}

pub trait BetaSimplify<D> {
    type Result;
}

impl<T, D> BetaSimplify<Suc<D>> for Lambda<T>
    where T: BetaSimplify<D> {
    type Result = <T as BetaSimplify<D>>::Result;
}

impl<Num, D> BetaSimplify<D> for ItemIndex<Num> {
    type Result = ItemIndex<Num>;
}

impl<Value, D> BetaSimplify<D> for ItemValue<Value> {
    type Result = ItemValue<Value>;
}

impl<F, A, D> BetaSimplify<D> for Call<Lambda<F>, A>
    where F: AlphaTransform<Zero, A> {
    type Result = <F as AlphaTransform<Zero, A>>::Result;
}

impl<Num, A, D> BetaSimplify<Suc<D>> for Call<ItemIndex<Num>, A>
    where ItemIndex<Num>: BetaSimplify<D>,
          A: BetaSimplify<D> {
    type Result = Call<<ItemIndex<Num> as BetaSimplify<D>>::Result, <A as BetaSimplify<D>>::Result>;
}

impl<Value, A, D> BetaSimplify<Suc<D>> for Call<ItemValue<Value>, A>
    where A: BetaSimplify<D> {
    type Result = Call<ItemValue<Value>, <A as BetaSimplify<D>>::Result>;
}

impl<F, A1, A2, D> BetaSimplify<Suc<D>> for Call<Call<F, A1>, A2>
    where Call<F, A1>: BetaSimplify<D>,
          A2: BetaSimplify<D> {
    type Result = Call<<Call<F, A1> as BetaSimplify<D>>::Result, <A2 as BetaSimplify<D>>::Result>;
}

pub trait BetaSimplifyRecursive<Count, Depth> {
    type Result;
}

impl<T, Count, Depth> BetaSimplifyRecursive<Suc<Count>, Depth> for T
    where T: BetaSimplify<Depth>,
          <T as BetaSimplify<Depth>>::Result: BetaSimplifyRecursive<Count, Depth> {
    type Result = <<T as BetaSimplify<Depth>>::Result as BetaSimplifyRecursive<Count, Depth>>::Result;
}

impl<T, Depth> BetaSimplifyRecursive<Zero, Depth> for T
    where T: BetaSimplify<Depth> {
    type Result = <T as BetaSimplify<Depth>>::Result;
}