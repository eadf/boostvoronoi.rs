use boostvoronoi::voronoi_builder as VB;
use boostvoronoi::voronoi_diagram as VD;
use boostvoronoi::voronoi_diagram::VoronoiEdgeIndex;
use boostvoronoi::voronoi_error::BVError;
//use boostvoronoi::voronoi_structures as VS;
use boostvoronoi::voronoi_visual_utils as VV;
use boostvoronoi::TypeConverter;
use boostvoronoi::{BigFloatType, BigIntType, InputType, OutputType};
//use num::NumCast;
//use num::Zero;
use geo::algorithm::simplify::Simplify;
use geo::{Coordinate, GeoFloat, Line, LineString, Point, Rect};
//use num::FromPrimitive;
//use num::ToPrimitive;
use ordered_float::OrderedFloat;
use std::marker::PhantomData;
use std::ops::Neg;

const EXTERNAL_COLOR: u32 = 1;
const SCREEN_WIDTH: f32 = 600.0;
const SCREEN_HEIGHT: f32 = 600.0;

type I1 = i32;
type F1 = f32;
type I2 = i64;
type F2 = f64;

//#[derive(Clone)]
struct VorVisualizer<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1> + GeoFloat,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    brect_: Rect<F1>,
    vd_: VD::VoronoiDiagram<I1, F1, I2, F2>,
    primary_edges_only_: bool,
    internal_edges_only_: bool,
    point_data_: Vec<Coordinate<I1>>,
    segment_data_: Vec<Line<I1>>,
    _pdo: PhantomData<F1>,
}

impl<I1, F1, I2, F2> VorVisualizer<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1> + GeoFloat,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    pub fn new() -> Self {
        Self {
            brect_: Rect::<F1>::new(
                Point::<F1>::new(F1::from(0.0).unwrap(), F1::from(0.0).unwrap()),
                Point::<F1>::new(
                    F1::from(SCREEN_WIDTH).unwrap(),
                    F1::from(SCREEN_HEIGHT).unwrap(),
                ),
            ),
            vd_: VD::VoronoiDiagram::<I1, F1, I2, F2>::new(0),
            primary_edges_only_: false,
            internal_edges_only_: false,
            point_data_: Vec::<Coordinate<I1>>::new(),
            segment_data_: Vec::<Line<I1>>::new(),
            _pdo: PhantomData,
        }
    }

    pub fn build(&mut self) -> Result<String, BVError> {
        // Clear all containers.
        self.clear();

        self.read_data();

        let mut point_data = Vec::<Coordinate<I1>>::new();
        let mut segment_data = Vec::<Line<I1>>::new();
        std::mem::swap(&mut self.point_data_, &mut point_data);
        std::mem::swap(&mut self.segment_data_, &mut segment_data);

        let mut vb = VB::VoronoiBuilder::<I1, F1, I2, F2>::new();
        vb.with_vertices(point_data.iter())?;
        vb.with_segments(segment_data.iter())?;
        std::mem::swap(&mut point_data, &mut self.point_data_);
        std::mem::swap(&mut segment_data, &mut self.segment_data_);

        // Construct voronoi diagram.
        self.vd_ = vb.construct()?;

        // Color exterior edges.
        for it in self.vd_.edges().iter() {
            let edge_id = it.get().get_id();
            //assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
            if !self.vd_.edge_is_finite(Some(edge_id)).unwrap() {
                self.color_exterior(Some(it.get().get_id()));
            }
        }

        // Update view port.
        //update_view_port();
        self.paint_gl();
        Result::Ok("".to_string())
    }

    #[allow(dead_code)]
    fn show_primary_edges_only(&mut self) {
        self.primary_edges_only_ ^= true;
    }

    #[allow(dead_code)]
    fn show_internal_edges_only(&mut self) {
        self.internal_edges_only_ ^= true;
    }

    fn paint_gl(&self) {
        self.draw_points();
        self.draw_segments();
        self.draw_vertices();
        self.draw_edges();
    }

    fn clear(&mut self) {
        self.point_data_.clear();
        self.segment_data_.clear();
        self.vd_.clear();
    }

    fn read_data(&mut self) {
        let i32_i1 = |x| I1::from(x).unwrap();
        let to_points = |points: &[[i32; 2]]| {
            let mut rv = Vec::new();
            for p in points.iter() {
                rv.push(Coordinate::<I1> {
                    x: i32_i1(p[0]),
                    y: i32_i1(p[1]),
                });
            }
            rv
        };
        let to_segments = |points: &[[i32; 4]]| {
            let mut rv = Vec::new();
            for p in points.iter() {
                rv.push(Line::<I1>::new(
                    Coordinate::<I1> {
                        x: i32_i1(p[0]),
                        y: i32_i1(p[1]),
                    },
                    Coordinate::<I1> {
                        x: i32_i1(p[2]),
                        y: i32_i1(p[3]),
                    },
                ));
            }
            rv
        };

        let points: [[i32; 2]; 0] = [];
        let segments: [[i32; 4]; 2] = [[498, 224, 475, 335], [250, 507, 60, 77]];

        // Preparing Input Geometries.
        self.point_data_.append(&mut to_points(&points));
        self.segment_data_.append(&mut to_segments(&segments));
    }

    fn color_exterior(&self, edge_id: Option<VD::VoronoiEdgeIndex>) {
        if edge_id.is_none() {
            return;
        }
        if self.vd_.edge_get_color(edge_id).unwrap() == EXTERNAL_COLOR {
            return;
        }
        self.vd_.edge_set_color(edge_id, EXTERNAL_COLOR);
        self.vd_
            .edge_set_color(self.vd_.edge_get_twin(edge_id), EXTERNAL_COLOR);

        let v1 = self.vd_.edge_get_vertex1(edge_id);
        if v1.is_some() || !self.vd_.get_edge(edge_id.unwrap()).get().is_primary() {
            return;
        }
        self.vd_.vertex_set_color(v1, EXTERNAL_COLOR);
        let mut e = self.vd_.vertex_get_incident_edge(v1);
        let v1_incident_edge = e;
        while e.is_some() {
            self.color_exterior(e);
            e = self.vd_.edge_rot_next(e);
            if e == v1_incident_edge {
                break;
            }
        }
    }

    fn draw_points(&self) {
        // Draw input points and endpoints of the input segments.
        println!(
            "brect:({},{})-({},{})",
            self.brect_.min().x,
            self.brect_.min().y,
            self.brect_.max().x,
            self.brect_.max().y
        );
        println!("->draw_points points");
        let mut j: usize = 0;
        for i in self.point_data_.iter() {
            let point = Self::cast_point_io(i);
            Self::gl_vertex2f(j, &point);
            j += 1;
        }
        println!("->draw_points segments");
        for i in self.segment_data_.iter() {
            let lp = Self::cast_coord_io(&i.start);
            Self::gl_vertex2f(j, &lp);
            let hp = Self::cast_coord_io(&i.end);
            Self::gl_vertex2f(j, &hp);
            j += 1;
        }
        println!("<-draw_points");
        println!();
    }

    fn draw_segments(&self) {
        // Draw input segments.
        println!("->draw_segments");
        for (j, i) in self.segment_data_.iter().enumerate() {
            let lp = Self::cast_coord_io(&i.start);
            Self::gl_vertex2f(j, &lp);
            let hp = Self::cast_coord_io(&i.end);
            Self::gl_vertex2f(j, &hp);
        }
        println!("<-draw_segments");
        println!();
    }

    fn draw_vertices(&self) {
        // Draw voronoi vertices.
        println!("->draw_vertices");
        for (i, it) in self.vd_.vertex_iter().enumerate() {
            let it = it.get();
            if self.internal_edges_only_ && it.get_color() == EXTERNAL_COLOR {
                continue;
            }
            let point = Coordinate {
                x: it.x(),
                y: it.y(),
            };
            Self::gl_vertex2f(i, &point);
        }
        println!("<-draw_vertices");
        println!();
    }

    fn draw_edges(&self) {
        // Draw voronoi edges.
        println!("->draw_edges");
        for it in self.vd_.edges().iter().enumerate() {
            let edge_id = VoronoiEdgeIndex(it.0);
            let edge = it.1.get();
            assert_eq!(edge.get_id(), edge_id);

            if self.primary_edges_only_ && !edge.is_primary() {
                continue;
            }
            if self.internal_edges_only_ && (edge.get_color() == EXTERNAL_COLOR) {
                continue;
            }

            let mut samples = Vec::<Coordinate<F1>>::new();
            if !self.vd_.edge_is_finite(Some(edge_id)).unwrap() {
                self.clip_infinite_edge(edge_id, &mut samples);
            } else {
                let vertex0 = self.vd_.vertex_get(edge.vertex0()).unwrap().get();
                let vertex0 = Coordinate {
                    x: vertex0.x(),
                    y: vertex0.y(),
                };
                samples.push(vertex0);
                let vertex1 = self.vd_.edge_get_vertex1(Some(edge_id));
                let vertex1 = self.vd_.vertex_get(vertex1).unwrap().get();
                let vertex1 = Coordinate {
                    x: vertex1.x(),
                    y: vertex1.y(),
                };
                samples.push(vertex1);
                if edge.is_curved() {
                    self.sample_curved_edge(VoronoiEdgeIndex(it.0), &mut samples);
                    // Optional simplification of the sampled curve
                    let linestring = LineString::from(samples).simplify(&F1::from(1.0).unwrap());
                    samples = linestring.0;
                }
            }
            for i in samples.iter() {
                Self::gl_vertex2f(edge_id.0, i);
            }
        }
        println!("<-draw_edges");
    }

    fn clip_infinite_edge(
        &self,
        edge_id: VD::VoronoiEdgeIndex,
        clipped_edge: &mut Vec<Coordinate<F1>>,
    ) {
        let edge = self.vd_.get_edge(edge_id);
        //const cell_type& cell1 = *edge.cell();
        let cell1_id = self.vd_.edge_get_cell(Some(edge_id)).unwrap();
        let cell1 = self.vd_.get_cell(cell1_id).get();
        //const cell_type& cell2 = *edge.twin()->cell();
        let cell2_id = self
            .vd_
            .edge_get_twin(Some(edge_id))
            .and_then(|e| self.vd_.edge_get_cell(Some(e)))
            .unwrap();
        let cell2 = self.vd_.get_cell(cell2_id).get();

        let mut origin: Coordinate<F1> = Coordinate {
            x: F1::default(),
            y: F1::default(),
        };
        let mut direction: Coordinate<F1> = Coordinate {
            x: F1::default(),
            y: F1::default(),
        };
        // Infinite edges could not be created by two segment sites.
        if cell1.contains_point() && cell2.contains_point() {
            let p1 = Self::cast_point_io(&self.retrieve_point(cell1_id));
            let p2 = Self::cast_point_io(&self.retrieve_point(cell2_id));
            origin.x = (p1.x + p2.x) * Self::castf32_o(0.5);
            origin.y = (p1.y + p2.y) * Self::castf32_o(0.5);
            direction.x = p1.y - p2.y;
            direction.y = p2.x - p1.x;
        } else {
            origin = if cell1.contains_segment() {
                Self::cast_point_io(&self.retrieve_point(cell2_id))
            } else {
                Self::cast_point_io(&self.retrieve_point(cell1_id))
            };
            let segment = if cell1.contains_segment() {
                self.retrieve_segment(cell1_id)
            } else {
                self.retrieve_segment(cell2_id)
            };
            let dx = Self::cast_io(segment.end.x - segment.start.x);
            let dy = Self::cast_io(segment.end.y - segment.start.y);
            if (Self::cast_coord_io(&segment.start) == origin) ^ cell1.contains_point() {
                direction.x = dy;
                direction.y = -dx;
            } else {
                direction.x = -dy;
                direction.y = dx;
            }
        }
        let side = self.brect_.max().x - self.brect_.min().x;
        let koef = side / Self::max(direction.x.abs(), direction.y.abs());

        let vertex0 = edge.get().vertex0();
        if vertex0.is_none() {
            clipped_edge.push(Coordinate {
                x: origin.x - direction.x * koef,
                y: origin.y - direction.y * koef,
            });
        } else {
            let vertex0 = self.vd_.vertex_get(vertex0).unwrap().get();
            clipped_edge.push(Coordinate {
                x: vertex0.x(),
                y: vertex0.y(),
            });
        }
        let vertex1 = self.vd_.edge_get_vertex1(Some(edge_id));
        if vertex1.is_none() {
            clipped_edge.push(Coordinate {
                x: origin.x + direction.x * koef,
                y: origin.y + direction.y * koef,
            });
        } else {
            let vertex1 = self.vd_.vertex_get(vertex1).unwrap().get();
            clipped_edge.push(Coordinate {
                x: vertex1.x(),
                y: vertex1.y(),
            });
        }
    }

    fn sample_curved_edge(
        &self,
        edge_id: VD::VoronoiEdgeIndex,
        sampled_edge: &mut Vec<Coordinate<F1>>,
    ) {
        let max_dist = Self::castf32_o(1E-3) * (self.brect_.max().x - self.brect_.min().x);

        let cell_id = self.vd_.edge_get_cell(Some(edge_id)).unwrap();
        let cell = self.vd_.get_cell(cell_id).get();
        let twin_id = self.vd_.edge_get_twin(Some(edge_id)).unwrap();
        let twin_cell_id = self.vd_.edge_get_cell(Some(twin_id)).unwrap();

        let point = if cell.contains_point() {
            self.retrieve_point(cell_id)
        } else {
            self.retrieve_point(twin_cell_id)
        };
        let segment = if cell.contains_point() {
            self.retrieve_segment(twin_cell_id)
        } else {
            self.retrieve_segment(cell_id)
        };
        VV::VoronoiVisualUtils::<I1, F1, I2, F2>::discretize(
            &point,
            segment,
            max_dist,
            sampled_edge,
        );
    }

    fn retrieve_point(&self, cell_id: VD::VoronoiCellIndex) -> Coordinate<I1> {
        let (index, cat) = self.vd_.get_cell(cell_id).get().source_index_2();
        match cat {
            VD::SourcePointCategory::SinglePoint =>
                self.point_data_[index],
            VD::SourcePointCategory::SegmentStart =>
                self.segment_data_[index-self.point_data_.len()].start,
            VD::SourcePointCategory::SegmentEnd =>
                self.segment_data_[index-self.point_data_.len()].end
        }
    }

    fn retrieve_segment(&self, cell_id: VD::VoronoiCellIndex) -> &Line<I1> {
        let cell = self.vd_.get_cell(cell_id).get();
        let index = cell.source_index() - self.point_data_.len();
        &self.segment_data_[index]
    }

    fn cast_point_io(value: &Coordinate<I1>) -> Coordinate<F1> {
        Coordinate::<F1> {
            x: Self::cast_io(value.x),
            y: Self::cast_io(value.y),
        }
    }

    fn cast_coord_io(value: &Coordinate<I1>) -> Coordinate<F1> {
        Coordinate::<F1> {
            x: Self::cast_io(value.x),
            y: Self::cast_io(value.y),
        }
    }

    #[inline(always)]
    fn max(a: F1, b: F1) -> F1 {
        OrderedFloat(a).max(OrderedFloat(b)).into_inner()
    }

    #[inline(always)]
    pub fn cast_io(value: I1) -> F1 {
        TypeConverter::<I1, F1, I2, F2>::i1_to_f1(value)
    }

    #[inline(always)]
    pub fn castf32_o(value: f32) -> F1 {
        TypeConverter::<I1, F1, I2, F2>::f32_to_f1(value)
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn cast_oi(value: F1) -> I1 {
        TypeConverter::<I1, F1, I2, F2>::f1_to_i1(value)
    }

    fn gl_vertex2f(i: usize, c: &Coordinate<F1>) {
        println!("#{}: x:{:.4}, y:{:.4}", i, c.x, c.y);
    }
}

fn main() {
    let mut vd = VorVisualizer::<I1, F1, I2, F2>::new();
    if let Err(e) = vd.build() {
        eprintln!("Oh noes, {}", e);
    }
}
