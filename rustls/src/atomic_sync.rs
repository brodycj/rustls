// NOTE: This `atomic_sync` module is intended to make it easier for a fork to use
// another implementation of `Arc` such as portable-atomic-util::Arc
// as may be needed to support targets with no atomic pointer.
// This module also makes it really easy for CI to over-write for build & unit testing.

pub(crate) use alloc::sync::Arc;
