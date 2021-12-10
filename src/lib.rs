#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    nonstandard_style,
    unused,
    future_incompatible,
    non_camel_case_types,
    unused_parens,
    non_upper_case_globals,
    unused_qualifications,
    unused_results,
    unused_imports,
    unused_variables,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    elided_lifetimes_in_paths
)]

#![doc(issue_tracker_base_url = "https://github.com/eadf/boostvoronoi.rs/issues")]

//! The `boostvoronoi` Rust library provides functionality to construct a Voronoi diagram of a set
//! of points and linear segments in 2D space with the following set of limitations:
//!
//! * Coordinates of the input points and endpoints of the input segments should have integral type.
//!   The `i32` and `i64` data types are supported by the default implementation.
//!
//! * Input points and segments should not overlap except their endpoints.
//!   This means that input point should not lie inside the input segment and input segments
//!   should not intersect except their endpoints.
//!
//! This library is a port of the C++ boost voronoi implementation
//! <https://www.boost.org/doc/libs/1_76_0/libs/polygon/doc/voronoi_main.htm>

#[doc(hidden)]
pub mod prelude {
    pub use boostvoronoi_core::builder::Builder;
    pub use boostvoronoi_core::diagram::{
        Cell, CellIndex, ColorType, Diagram, Edge, EdgeIndex, Vertex, VertexIndex,
    };
    pub use boostvoronoi_core::geometry::*;
    pub use boostvoronoi_core::{cast, try_cast, BvError, InputType, OutputType};
}

pub use boostvoronoi_core::builder::Builder;
pub use boostvoronoi_core::diagram::{
    Cell, CellIndex, ColorType, Diagram, Edge, EdgeIndex, SourceCategory, SourceIndex, Vertex,
    VertexIndex,
};
pub use boostvoronoi_core::file_reader::{read_boost_input_buffer, read_boost_input_file};
pub use boostvoronoi_core::geometry::*;
pub use boostvoronoi_core::sync_diagram::SyncDiagram;
pub use boostvoronoi_core::visual_utils::*;
pub use boostvoronoi_core::{cast, try_cast, BvError, InputType, OutputType};

#[cfg(feature = "cgmath")]
// Allowing integration tests access to `cgmath` without needing to add `cgmath` to dev-dependencies.
pub use boostvoronoi_core::cgmath;

#[cfg(feature = "mint")]
// Allowing integration tests access to `mint` without needing to add `mint` to dev-dependencies.
pub use boostvoronoi_core::mint;

#[cfg(feature = "geo")]
// Allowing integration tests access to `geo` without needing to add `geo` to dev-dependencies.
pub use boostvoronoi_core::geo;

#[cfg(feature = "glam")]
// Allowing integration tests access to `glam` without needing to add `glam` to dev-dependencies.
pub use boostvoronoi_core::glam;
