#[cfg(feature = "arcshare")]
pub(crate) use alloc::sync::Arc;

#[cfg(not(feature = "arcshare"))]
pub(crate) use alloc::rc::Rc as Arc;