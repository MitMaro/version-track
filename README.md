[![Crates.io](https://img.shields.io/crates/v/version-track.svg)](https://crates.io/crates/version-track)
[![docs](https://img.shields.io/docsrs/version-track/latest)](https://docs.rs/version-track/)
[![GitHub license](https://img.shields.io/github/license/MitMaro/version-track)](https://raw.githubusercontent.com/MitMaro/version-track/master/LICENSE)
[![Coverage Status](https://coveralls.io/repos/github/MitMaro/version-track/badge.svg?branch=main)](https://coveralls.io/github/MitMaro/version-track?branch=main)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.56.0+-green.svg)

# Version Track

This simple crate provides an easy way to track and compare changes to value that are too expensive to compare
directly. This crate will track mutable changes to a value, and automatically increment a version number on every
modification. Any comparison will then compare the internal version number, instead of having to do a full
comparison on a complex data structure.

Tracking is performed by two values, the first is a V4 [uuid::Uuid], and the second is a [usize] counter. The
[uuid::Uuid] is generated once¹ per value being tracked, and then each subsequent mutable reference access
increments the counter. This allows for tracking multiple distinct values independently, using the [uuid::Uuid] to
track the different values across increments, and the counter to track direct changes to the value.

The crate provides two ways to track versions. The first way is using a pointer-like wrapper around another value.
This is the easiest way to use this crate, but at times may result in extra unneeded version increments. To address
this it is possible to create a version tracker manually to be added as a field on a struct or stored separately.

¹ The [uuid::Uuid] value may be regenerated is the [usize] counter wraps back to zero.


## Installation and Usage

```toml
[dependencies]
version-track = "0.1.0"
```

## Basic Example

Any value can be wrapped with `Versioned<T>`, and because `Versioned<T>` implements [std::ops::Deref],
[std::ops::DerefMut], [std::convert::AsRef] and [std::convert::AsMut], the wrapped value can be used in most places
that the wrapped value is used.

```rust
use version_track::Versioned;

let mut tracked_value = Versioned::new(String::from("foo"));
let current_version = tracked_value.version();
tracked_value.push_str("bar");

assert_ne!(current_version, tracked_value.version());
assert_eq!(*tracked_value, "foobar");
```

## Direct Use Example

Sometimes more control over the version is desired, or wrapping the value to be tracked is not possible. In those
cases [Version] can be used directly. In this case, it is up to the developer to decide on when a change has
occurred. This can be useful to only track some modifications to a value, or to only increment once when multiple
mutable references are needed.

```rust
use version_track::Version;

let mut value = String::from("foo");
let mut version = Version::new();
let current_version = version;

value.push_str("bar");
version.increment();

assert_ne!(current_version, version);
assert_eq!(value, "foobar");
```

# Supported Rust Versions

This project will support Rust versions since 1.60.0.

Dropping support for a Rust version will result in a major version bump, following [Semantic Versioning](https://semver.org/).

## License

Version Track is released under the ISC license. See [LICENSE](LICENSE).

[uuid::Uuid]:https://docs.rs/uuid/latest/uuid/struct.Uuid.html
[usize]:https://doc.rust-lang.org/std/primitive.usize.html
[std::ops::Deref]:https://doc.rust-lang.org/std/ops/trait.Deref.html
[std::ops::DerefMut]:https://doc.rust-lang.org/std/ops/trait.DerefMut.html
[std::convert::AsRef]:https://doc.rust-lang.org/std/convert/trait.AsRef.html
[std::convert::AsMut]:https://doc.rust-lang.org/std/convert/trait.AsMut.html
