#[cfg(not(feature = "withrcalias"))]
macro_rules! pub_api_trait {
    ($name:ident, $body:tt) => {
        pub trait $name: core::fmt::Debug + Send + Sync $body
    }
}

#[cfg(feature = "withrcalias")]
macro_rules! pub_api_trait {
    ($name:ident, $body:tt) => {
        pub trait $name: core::fmt::Debug $body
    }
}

#[cfg(not(feature = "withrcalias"))]
macro_rules! internal_generic_state_trait {
    // XXX TBD HACKISH - MAY WANT TO RECONSIDER
    ($name:ident, $generic_parameter:ident, $body:tt) => {
        pub(crate) trait $name<$generic_parameter>: Send + Sync $body
    }
}

#[cfg(feature = "withrcalias")]
macro_rules! internal_generic_state_trait {
    // XXX TBD HACKISH - MAY WANT TO RECONSIDER
    ($name:ident, $generic_parameter:ident, $body:tt) => {
        pub(crate) trait $name<$generic_parameter> $body
    }
}
