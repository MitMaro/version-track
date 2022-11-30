use std::ops::{Deref, DerefMut};

use crate::Version;

/// A wrapper around data, that tracks mutable changes using mutable dereference counting.
#[derive(Debug)]
pub struct Versioned<T> {
	value: T,
	version: Version,
}

impl<T> Versioned<T> {
	/// Create a new instance of [Self] with an internal version.
	#[inline]
	pub fn new(value: T) -> Self {
		Self::with_version(value, Version::new())
	}

	/// Creates a new instance of [Self] with a provided version.
	#[inline]
	pub fn with_version(value: T, version: Version) -> Self {
		Self { value, version }
	}

	/// Returns current version.
	#[inline]
	pub fn version(&self) -> Version {
		self.version
	}
}

impl<T> Default for Versioned<T>
where T: Default
{
	/// Proxy default call, that creates a version tracker `T` with a default value.
	#[inline]
	fn default() -> Self {
		Self::new(T::default())
	}
}

#[allow(clippy::expl_impl_clone_on_copy)]
impl<T> Clone for Versioned<T>
where T: Clone
{
	/// Proxy clone call, that creates a new tracked version of `T` by cloning `T`. THe version is not cloned, and a
	/// new version is created.
	#[inline]
	fn clone(&self) -> Self {
		Self::new(self.value.clone())
	}
}

impl<T> Copy for Versioned<T> where T: Copy {}

impl<T> Deref for Versioned<T> {
	type Target = T;

	/// Dereferences the value. Does not increment version.
	#[must_use]
	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

impl<T> AsRef<T> for Versioned<T> {
	/// Returns reference to the value. Does not increment version.
	#[must_use]
	fn as_ref(&self) -> &T {
		self.deref()
	}
}

impl<T> DerefMut for Versioned<T> {
	/// Mutably dereferences the value. Increments version.
	#[must_use]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.version.increment();
		&mut self.value
	}
}

impl<T> AsMut<T> for Versioned<T> {
	/// Returns mutable reference to the value. Increments version.
	#[must_use]
	fn as_mut(&mut self) -> &mut T {
		self.deref_mut()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::SENTINEL_VERSION;

	#[derive(Debug, PartialEq, Copy, Clone)]
	struct TestType {
		value: usize,
	}

	impl Default for TestType {
		fn default() -> Self {
			Self { value: 42 }
		}
	}

	#[test]
	fn new() {
		let versioned_value = Versioned::new(42);
		assert_eq!(*versioned_value, 42);
	}

	#[test]
	fn with_version() {
		let versioned_value = Versioned::with_version(42, SENTINEL_VERSION);
		assert_eq!(*versioned_value, 42);
		assert_eq!(versioned_value.version(), SENTINEL_VERSION);
	}

	#[test]
	fn version() {
		let versioned_value = Versioned::with_version(42, SENTINEL_VERSION);
		assert_eq!(versioned_value.version(), SENTINEL_VERSION);
	}

	#[test]
	fn default_impl() {
		let versioned_value = Versioned::<TestType>::default();
		assert_eq!(*versioned_value, TestType::default());
	}

	#[test]
	#[allow(clippy::clone_on_copy)]
	fn clone_impl() {
		let versioned_value = Versioned::new(TestType::default());
		let initial_version = versioned_value.version();
		let cloned = versioned_value.clone();
		assert_eq!(*cloned, *versioned_value);
		assert_ne!(initial_version, cloned.version());
	}

	#[test]
	fn deref() {
		let versioned_value = Versioned::new(TestType::default());
		let initial_version = versioned_value.version();
		assert_eq!(versioned_value.deref(), &TestType::default());
		assert_eq!(initial_version, versioned_value.version());
	}

	#[test]
	fn as_ref() {
		let versioned_value = Versioned::new(TestType::default());
		let initial_version = versioned_value.version();
		assert_eq!(versioned_value.as_ref(), &TestType::default());
		assert_eq!(initial_version, versioned_value.version());
	}

	#[test]
	fn deref_mut() {
		let mut versioned_value = Versioned::new(TestType::default());
		let initial_version = versioned_value.version();
		let mut value = versioned_value.deref_mut();
		value.value = 99;
		assert_eq!(*versioned_value, TestType { value: 99 });
		assert_ne!(initial_version, versioned_value.version());
	}

	#[test]
	fn as_mut() {
		let mut versioned_value = Versioned::new(TestType::default());
		let initial_version = versioned_value.version();
		let mut value = versioned_value.as_mut();
		value.value = 99;
		assert_eq!(*versioned_value, TestType { value: 99 });
		assert_ne!(initial_version, versioned_value.version());
	}
}
