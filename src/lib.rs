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
