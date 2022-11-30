use uuid::Uuid;

/// Tracks the changing state of the rebase file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
	id: Uuid,
	counter: usize,
}

/// A sentinel value version, that is useful as an initial non-version for caching.
pub const SENTINEL_VERSION: Version = Version::sentinel();

impl Version {
	/// Creates a sentinel [Self], that is useful as an initial non-version for caching. Any changes to this
	/// version will reset the internal state to a new value. Internally, this uses a [Uuid] with the maximum valid
	/// value, and an increment of [`usize::MAX`].
	///
	/// See [`SENTINEL_VERSION`] for a constant sentinel value.
	#[inline]
	#[must_use]
	pub const fn sentinel() -> Self {
		Self {
			id: Uuid::from_bytes([0xFF; 16]),
			counter: usize::MAX,
		}
	}

	/// Create a new [Self] instance with a random [Uuid] and an increment of 0.
	#[inline]
	#[must_use]
	pub fn new() -> Self {
		Self {
			id: Uuid::new_v4(),
			counter: 0,
		}
	}

	/// Reset to an initial state, with a new internal version tracker.
	#[inline]
	pub fn reset(&mut self) {
		self.id = Uuid::new_v4();
		self.counter = 0;
	}

	/// Increment the internal internal version, if the increment overflows, then the internal [Uuid] is recreated with
	/// a new value, and the increment is set to 0.
	///
	/// See [`Self::increment_wrap`] for a version that does not update the internal [Uuid] on overflow.
	#[inline]
	pub fn increment(&mut self) {
		self.counter = self.counter.wrapping_add(1);
		if self.counter == 0 {
			self.id = Uuid::new_v4();
		}
	}

	/// Increment the internal internal version, if the increment overflows, the increment resets to 0. This allows the
	/// internal [Uuid] to remain stable, and reusing older increments are okay.
	///
	/// See [`Self::increment`] for a version that refreshes the internal [Uuid] on overflow.
	#[inline]
	pub fn increment_wrap(&mut self) {
		self.counter = self.counter.wrapping_add(1);
	}

	/// Check if the internal [Uuid] is the same for this version and another. This is useful for checking if the state
	/// being treated by this instance is the same as another.
	///
	/// Note that using [`Self::reset`] or [`Self::increment`] can update the internal [Uuid], and result in this value
	/// returning false.
	#[inline]
	#[must_use]
	pub fn alike(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sentinel() {
		let version = Version::sentinel();
		assert_eq!(version.id.as_u128(), u128::MAX);
		assert_eq!(version.counter, usize::MAX);
	}

	#[test]
	fn new() {
		let version = Version::new();
		assert!(version.id.as_u128() > 0);
		assert_eq!(version.counter, 0);
	}

	#[test]
	fn reset() {
		let mut version = Version::new();
		version.counter = 42;
		let prev_id = version.id;
		version.reset();
		assert_ne!(version.id, prev_id);
		assert_eq!(version.counter, 0);
	}

	#[test]
	fn increment() {
		let mut version = Version::new();
		let prev_id = version.id;
		version.increment();
		assert_eq!(version.id, prev_id);
		assert_eq!(version.counter, 1);
	}

	#[test]
	fn increment_with_wrap() {
		let mut version = Version::new();
		version.counter = usize::MAX;
		let prev_id = version.id;
		version.increment();
		assert_ne!(version.id, prev_id);
		assert_eq!(version.counter, 0);
	}

	#[test]
	fn increment_wrap() {
		let mut version = Version::new();
		let prev_id = version.id;
		version.increment_wrap();
		assert_eq!(version.id, prev_id);
		assert_eq!(version.counter, 1);
	}

	#[test]
	fn increment_wrap_with_wrap() {
		let mut version = Version::new();
		version.counter = usize::MAX;
		let prev_id = version.id;
		version.increment_wrap();
		assert_eq!(version.id, prev_id);
		assert_eq!(version.counter, 0);
	}

	#[test]
	fn alike_match() {
		let version = Version::new();
		let other = version;
		assert!(other.alike(&version));
	}

	#[test]
	fn alike_mismatch() {
		let version = Version::new();
		let other = Version::new();
		assert!(!other.alike(&version));
	}
}
