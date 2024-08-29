#[macro_use]
extern crate heterogeneous_vec;
trait TestTrait {}
trait TestBound {}
struct TestType<A>(A);
trait TestCollectionFunctor {
    type Result<T: TestBound>;
    fn apply<T: TestBound>(&mut self, v: TestType<T>) -> Self::Result<T>;
}
trait TestCollection {
    type Push<T: TestBound>: TestCollection;
    fn push<T: TestBound>(self, v: TestType<T>) -> Self::Push<T>;
    type Map<F: TestCollectionFunctor>;
    fn map<F: TestCollectionFunctor>(self, f: &mut F) -> Self::Map<F>;
}
impl TestCollection for () {
    type Push<T: TestBound> = (T,);
    fn push<T: TestBound>(self, v: TestType<T>) -> Self::Push<T> {
        (v,)
    }
    type Map<F: TestCollectionFunctor> = ();
    fn map<F: TestCollectionFunctor>(self, f: &mut F) -> Self::Map<F> {
        ()
    }
}
impl<T0: TestBound> TestCollection for (T0,) {
    type Push<T: TestBound> = (T0, T);
    fn push<T: TestBound>(self, v: TestType<T>) -> Self::Push<T> {
        (self.0, v)
    }
    type Map<F: TestCollectionFunctor> = (F::Result<T0>,);
    fn map<F: TestCollectionFunctor>(self, f: &mut F) -> Self::Map<F> {
        (f.apply(self.0),)
    }
}
impl<T0: TestBound, T1: TestBound> TestCollection for (T0, T1) {
    type Push<T: TestBound> = (T0, T1, T);
    fn push<T: TestBound>(self, v: TestType<T>) -> Self::Push<T> {
        (self.0, self.1, v)
    }
    type Map<F: TestCollectionFunctor> = (F::Result<T0>, F::Result<T1>);
    fn map<F: TestCollectionFunctor>(self, f: &mut F) -> Self::Map<F> {
        (f.apply(self.0), f.apply(self.1))
    }
}
impl<T0: TestBound, T1: TestBound, T2: TestBound> TestCollection for (T0, T1, T2) {
    type Push<T: TestBound> = ();
    fn push<T: TestBound>(self, v: TestType<T>) -> Self::Push<T> {
        ::core::panicking::panic("not implemented");
    }
    type Map<F: TestCollectionFunctor> = (F::Result<T0>, F::Result<T1>, F::Result<T2>);
    fn map<F: TestCollectionFunctor>(self, f: &mut F) -> Self::Map<F> {
        (f.apply(self.0), f.apply(self.1), f.apply(self.2))
    }
}
