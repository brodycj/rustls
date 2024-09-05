/////// XXX TODO REPLACE ALL USE OF THIS MACRO WITH pub_api_trait_with_doc! (with doc fixed) & REMOVE THIS MACRO
/// pub trait - version with no doc - version that includes Send & Sync - supports use with alloc::sync::Arc
#[cfg(not(feature = "withrcalias"))]
macro_rules! pub_api_trait {
    ($name:ident, $body:tt) => {
        pub trait $name: core::fmt::Debug + Send + Sync $body
    }
}

/////// XXX TODO REPLACE ALL USE OF THIS MACRO WITH pub_api_trait_with_doc! (with doc fixed) & REMOVE THIS MACRO
/// pub trait - version with no doc - version with no Send / Sync - supports use with alloc::rc::Rc
#[cfg(feature = "withrcalias")]
macro_rules! pub_api_trait {
    ($name:ident, $body:tt) => {
        pub trait $name: core::fmt::Debug $body
    }
}
