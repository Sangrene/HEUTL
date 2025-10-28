pub trait Merge<T> {
  fn merge(self, other: T) -> Self;
}
