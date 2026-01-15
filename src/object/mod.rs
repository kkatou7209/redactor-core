//! This module contains PDF object representations.
mod boolean;
mod name;

use crate::object::boolean::Boolean;
use crate::object::name::Name;

/// PDF Object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    /// PDF `Boolean` object.
    Boolean(Boolean),
    /// PDF `Name` object.
    Name(Name),
}