//!# typed
//!
//! Type annotation to help rustc.
//! This is useful for code generation.
//!
//!# Usage
//!
//!This does not work without ufcs, because it's ambiguous.
//!
//!```rust
//!trait ChangeWatcher<T> {
//!  fn is_changed(&self) -> bool;
//!}
//!
//!struct DbConfig;
//!struct AppConfig;
//!
//!struct Context;
//!
//!impl ChangeWatcher<DbConfig> for Context {
//!  fn is_changed(&self) -> bool { false }
//!}
//!impl ChangeWatcher<AppConfig> for Context {
//!  fn is_changed(&self) -> bool { false }
//!}
//!
//!
//!
//!fn some<C: ChangeWatcher<DbConfig> + ChangeWatcher<AppConfig>>(c: C) {
//!  if <C as ChangeWatcher<DbConfig>>::is_changed(&c) {
//!
//!  }
//!}
//!
//!```
//!
//! But this works.
//!
//!```rust
//!extern crate typed;
//!use typed::{Type, type_of};
//!
//!
//!trait ChangeWatcher<T> {
//!  fn is_changed(&self, _: Type<T>) -> bool;
//!}
//!
//!struct DbConfig;
//!struct AppConfig;
//!
//!struct Context;
//!
//!impl ChangeWatcher<DbConfig> for Context {
//!  fn is_changed(&self, _: Type<DbConfig>) -> bool { false }
//!}
//!impl ChangeWatcher<AppConfig> for Context {
//!  fn is_changed(&self, _: Type<AppConfig>) -> bool { false }
//!}
//!
//!
//!fn some<C: ChangeWatcher<DbConfig> + ChangeWatcher<AppConfig>>(c: C) {
//!  if c.is_changed(type_of::<DbConfig>()) {
//!
//!  }
//!}
//!
//!# fn main(){}
//!```
//!


use std::marker::PhantomData;



pub struct Type<T: ?Sized>(PhantomData<T>);


pub fn type_of<T: ?Sized>() -> Type<T> {
    Type(PhantomData)
}

pub fn type_of_val<T: ?Sized>(_: &T) -> Type<T> {
    Type(PhantomData)
}
