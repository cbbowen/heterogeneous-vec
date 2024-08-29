#[macro_use]
extern crate heterogeneous_vec;

trait TestTrait {}
trait TestBound {}
struct TestType<A>(A);

heterogeneous_vec!(trait TestCollection = impl<T: TestBound> for (TestType<T>));
