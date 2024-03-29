// enable all rustc's built-in lints
#![deny(
	future_incompatible,
	nonstandard_style,
	rust_2018_compatibility,
	rust_2018_idioms,
	rust_2021_compatibility,
	unused,
	warnings
)]
// rustc's additional allowed by default lints
#![deny(
	absolute_paths_not_starting_with_crate,
	deprecated_in_future,
	elided_lifetimes_in_paths,
	explicit_outlives_requirements,
	ffi_unwind_calls,
	keyword_idents,
	let_underscore_drop,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_abi,
	missing_copy_implementations,
	missing_debug_implementations,
	missing_docs,
	non_ascii_idents,
	noop_method_call,
	pointer_structural_match,
	rust_2021_incompatible_closure_captures,
	rust_2021_incompatible_or_patterns,
	rust_2021_prefixes_incompatible_syntax,
	rust_2021_prelude_collisions,
	single_use_lifetimes,
	trivial_casts,
	trivial_numeric_casts,
	unreachable_pub,
	unsafe_code,
	unsafe_op_in_unsafe_fn,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
	unused_lifetimes,
	unused_macro_rules,
	unused_qualifications,
	unused_results,
	unused_tuple_struct_fields,
	variant_size_differences
)]
// enable all of Clippy's lints
#![deny(clippy::all, clippy::cargo, clippy::pedantic, clippy::restriction)]
#![cfg_attr(include_nightly_lints, deny(clippy::nursery))]
#![allow(
	clippy::arithmetic_side_effects,
	clippy::arithmetic_side_effects,
	clippy::blanket_clippy_restriction_lints,
	clippy::bool_to_int_with_if,
	clippy::default_numeric_fallback,
	clippy::else_if_without_else,
	clippy::expect_used,
	clippy::float_arithmetic,
	clippy::implicit_return,
	clippy::indexing_slicing,
	clippy::map_err_ignore,
	clippy::missing_docs_in_private_items,
	clippy::missing_trait_methods,
	clippy::mod_module_files,
	clippy::module_name_repetitions,
	clippy::new_without_default,
	clippy::non_ascii_literal,
	clippy::option_if_let_else,
	clippy::pub_use,
	clippy::question_mark_used,
	clippy::redundant_pub_crate,
	clippy::ref_patterns,
	clippy::std_instead_of_alloc,
	clippy::std_instead_of_core,
	clippy::tabs_in_doc_comments,
	clippy::tests_outside_test_module,
	clippy::too_many_lines,
	clippy::unwrap_used
)]
#![deny(
	rustdoc::bare_urls,
	rustdoc::broken_intra_doc_links,
	rustdoc::invalid_codeblock_attributes,
	rustdoc::invalid_html_tags,
	rustdoc::missing_crate_level_docs,
	rustdoc::private_doc_tests,
	rustdoc::private_intra_doc_links
)]
// allow some things in tests
#![cfg_attr(
	test,
	allow(
		let_underscore_drop,
		clippy::cognitive_complexity,
		clippy::let_underscore_must_use,
		clippy::let_underscore_untyped,
		clippy::needless_pass_by_value,
		clippy::panic,
		clippy::shadow_reuse,
		clippy::shadow_unrelated,
		clippy::undocumented_unsafe_blocks,
		clippy::unimplemented,
		clippy::unreachable
	)
)]
// allowable upcoming nightly lints
#![cfg_attr(include_nightly_lints, allow(clippy::pub_with_shorthand))]

//! # Version Track
//!
//! This simple crate provides an easy way to track and compare changes to value that are too expensive to compare
//! directly. This crate will track mutable changes to a value, and automatically increment a version number on every
//! modification. Any comparison will then compare the internal version number, instead of having to do a full
//! comparison on a complex data structure.
//!
//! Tracking is performed by two values, the first is a V4 [`uuid::Uuid`], and the second is a [usize] counter. The
//! [`uuid::Uuid`] is generated once¹ per value being tracked, and then each subsequent mutable reference access
//! increments the counter. This allows for tracking multiple distinct values independently, using the [`uuid::Uuid`] to
//! track the different values across increments, and the counter to track direct changes to the value.
//!
//! The crate provides two ways to track versions. The first way is using a pointer-like wrapper around another value.
//! This is the easiest way to use this crate, but at times may result in extra unneeded version increments. To address
//! this it is possible to create a version tracker manually to be added as a field on a struct or stored separately.
//!
//! ¹ The [`uuid::Uuid`] value may be regenerated is the [usize] counter wraps back to zero.
//!
//! ## Basic Example
//!
//! Any value can be wrapped with [`Versioned<T>`], and because [`Versioned<T>`] implements [`std::ops::Deref`],
//! [`std::ops::DerefMut`], [`std::convert::AsRef`] and [`std::convert::AsMut`], the wrapped value can be used in most
//! places that the wrapped value is used.
//!
//! ```
//! use version_track::Versioned;
//!
//! let mut tracked_value = Versioned::new(String::from("foo"));
//! let current_version = tracked_value.version();
//! tracked_value.push_str("bar");
//!
//! assert_ne!(current_version, tracked_value.version());
//! assert_eq!(*tracked_value, "foobar");
//! ```
//!
//! ## Direct Use Example
//!
//! Sometimes more control over the version is desired, or wrapping the value to be tracked is not possible. In those
//! cases [Version] can be used directly. In this case, it is up to the developer to decide on when a change has
//! occurred. This can be useful to only track some modifications to a value, or to only increment once when multiple
//! mutable references are needed.
//!
//! ```
//! use version_track::Version;
//!
//! let mut value = String::from("foo");
//! let mut version = Version::new();
//! let current_version = version;
//!
//! value.push_str("bar");
//! version.increment();
//!
//! assert_ne!(current_version, version);
//! assert_eq!(value, "foobar");
//! ```

mod version;
mod versioned;

pub use crate::{
	version::{Version, SENTINEL_VERSION},
	versioned::Versioned,
};
