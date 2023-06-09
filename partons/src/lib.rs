//! Ship partons distributions to the user.
//!
//! The overall structure is divided in two different hierarchies: the runtime library and the data
//! management.
//!
//! The runtime library is centered organized as follows:
//! ```text
//! Set
//!   Member
//!     Block
//!   Quantity
//!     Block
//!   Info
//! ```
//! - `Set` is a collection of homogeneous objects
//! - `Member` the element of the collection
//! - `Block` is the interpolation atom, essentially an N-dimensional array to be interpolated
//! - `Quantity` further interpolated quantities (e.g. `\alpha_s`)
//! - `Info` additional metadata
//!
//! The data layer itself accomplishes two separate roles:
//!
//! - fetching data from remote sources
//!     - supporting multiple sources
//! - providing the data to the runtime library
//!     - supporting multiple formats
//!
//! Partons is configurable through its own [config files](configs), only required to persist
//! information about remote sources (but may contain further optional configurations).
//! The alternative is to always specify the full remote locator (through the API or CLI).

mod block;
pub mod configs;
mod member;
mod set;

pub mod data;
mod engine;
