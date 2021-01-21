// Boost.Polygon library voronoi_visualizer.cpp file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

#include <iostream>
#include <vector>

#include <boost/polygon/polygon.hpp>
#include <boost/polygon/voronoi.hpp>
using namespace boost::polygon;

#include <unordered_map>
#include "voronoi_visual_utils.hpp"

void glVertex2f(int i, double x,double y) {
  std::cout << std::fixed;
  std::cout <<  "#" << i <<": x:";
  std::cout.precision(4);
  std::cout << x;
  std::cout << ", y:";
  std::cout.precision(4);
  std::cout << y << std::endl;
}

void debug_print(const voronoi_diagram<double> &vd) {
	std::unordered_map<const void*, int> m;

	auto o = [&](const void* p) {
	   if (p==0) {
		   return std::string("-");
	   } else if(m.count(p)>0){
		   return std::to_string(m[p]);
	   } else {
		   return std::string("?");
	   }
	};

	for (uint i=0; i < vd.cells().size(); i++) {
		m[(const void*) &vd.cells()[i] ] = i;//vd.cells()[i].source_index();
	}

	for (uint i=0; i<vd.vertices().size(); i++) {
		m[(const void*)&vd.vertices()[i]] = i;
	}
	for (uint i=0; i<vd.edges().size(); i++) {
		m[(const void*)&vd.edges()[i]] = i;
	}

	for (uint i=0; i<vd.cells().size(); i++) {
		auto c = &vd.cells()[i];
		std::cout << "Cell:#" << i
				  << "=>id:" << o(c)
				  << " ii:" << c->source_index()
				  << " ie:" << o(c->incident_edge());
				  //<< " c:" << c->raw_color();
		if (c->contains_point()) {
			std::cout << " point";
		} else if (c->contains_segment()) {
			std::cout << " segment";
		}
		std::cout << "  ptr=" << c << std::endl;
	}
	for (uint i=0; i<vd.vertices().size(); i++) {
		auto v = &vd.vertices()[i];
		std::cout << "Vertex:#" << i << "=>id:" << o(v)
			      << " x:" << v->x()
			      << " y:" << v->y()
			      << " ie:" <<  o(v->incident_edge())
				  << "  ptr=" << v
				  << std::endl;
	}
	for (uint i=0; i<vd.edges().size(); i++) {
		auto e = &vd.edges()[i];
		std::cout << "Edge:#" << i << "=>id:" << o(e)
				  << " cell:" << o(e->cell())
				  << " v0:" << o(e->vertex0())
				  //<< " v1:" << m[(const void*)e->vertex1()]
				  << " t:" << o(e->twin())
				  << " n:" << o(e->next())
				  << " p:" << o(e->prev())
				  //<< " rn:" << m[(const void*)e.rot_next()]
				  //<< " c:" << e->raw_color()
				  << "  ptr=" << e
				  << std::endl;
	}
	std::cout << std::endl;
}

class VorVisualizer{

 public:

  void build() {
    // Clear all containers.
    clear();

    read_data();

    // No data, don't proceed.
    //if (!brect_initialized_) {
    //  return;
    //}

    // Construct bounding rectangle.
    construct_brect();

    // Construct voronoi diagram.
    construct_voronoi(
        point_data_.begin(), point_data_.end(),
        segment_data_.begin(), segment_data_.end(),
        &vd_);

    debug_print(vd_);

    // Color exterior edges.
    for (const_edge_iterator it = vd_.edges().begin();
         it != vd_.edges().end(); ++it) {
      if (!it->is_finite()) {
        color_exterior(&(*it));
      }
    }

    // Update view port.
    //update_view_port();
    paintGL();
  }

  void show_primary_edges_only() {
    primary_edges_only_ ^= true;
  }

  void show_internal_edges_only() {
    internal_edges_only_ ^= true;
  }

 protected:

  void paintGL() {

    draw_points();
    draw_segments();
    draw_vertices();
    draw_edges();
  }


 private:
  typedef double coordinate_type;
  typedef point_data<coordinate_type> point_type;
  typedef segment_data<coordinate_type> segment_type;
  typedef rectangle_data<coordinate_type> rect_type;
  typedef voronoi_builder<int> VB;
  typedef voronoi_diagram<coordinate_type> VD;
  typedef VD::cell_type cell_type;
  typedef VD::cell_type::source_index_type source_index_type;
  typedef VD::cell_type::source_category_type source_category_type;
  typedef VD::edge_type edge_type;
  typedef VD::cell_container_type cell_container_type;
  typedef VD::cell_container_type vertex_container_type;
  typedef VD::edge_container_type edge_container_type;
  typedef VD::const_cell_iterator const_cell_iterator;
  typedef VD::const_vertex_iterator const_vertex_iterator;
  typedef VD::const_edge_iterator const_edge_iterator;

  static const std::size_t EXTERNAL_COLOR = 1;

  void clear() {
    //brect_initialized_ = false;
    point_data_.clear();
    segment_data_.clear();
    vd_.clear();
  }

  void read_data() {
    // Preparing Input Geometries.
    //point_data_.push_back(point_type(12,14));
    //point_data_.push_back(point_type(4,5));
    //point_data_.push_back(point_type(100,400));
	{point_type l(498,224);
    point_type h(475,335);
    segment_data_.push_back(segment_type(l,h));
    }
    {point_type l(250,507);
     point_type h(60,77);
     segment_data_.push_back(segment_type(l,h));
    }

	std::size_t num_points = point_data_.size();
	std::size_t num_segments = segment_data_.size();

    for (std::size_t i = 0; i < num_points; ++i) {
      point_type p = point_data_[i];
      //update_brect(p);
      //point_data_.push_back(p);
        std::cout << "point:" <<i << "=(" << p.x()<<","<<p.y()<<")" << std::endl;
    }
    for (std::size_t i = 0; i < num_segments; ++i) {
      auto s = segment_data_[i];
        
      point_type lp = s.low();
      point_type hp = s.high();
      //update_brect(lp);
      //update_brect(hp);
      std::cout << "segment:" << i << "=(" << lp.x()<<","<<lp.y()<<")-("<< hp.x()<<","<<hp.y()<<")" << std::endl;
    }
    std::cout << "brect:" << "=(" << xl(brect_) <<","<<yl(brect_) << ")-("<< xh(brect_)<<","<<yh(brect_)<<")" << std::endl;
    std::cout << std::endl;
  }

  /*void update_brect(const point_type& point) {
    if (brect_initialized_) {
      encompass(brect_, point);
    } else {
      set_points(brect_, point, point);
      brect_initialized_ = true;
    }
  }*/

  void construct_brect() {
    std::cout << "->construct_brect()" << std::endl;
    encompass(brect_, point_type(0,0));
    encompass(brect_, point_type(600,600));

    //double side = (std::max)(xh(brect_) - xl(brect_), yh(brect_) - yl(brect_));
    //center(shift_, brect_);
    //
    //std::cout << "brect:" << "=(" << xl(brect_) <<","<<yl(brect_) << ")-("<< xh(brect_)<<","<<yh(brect_)<<")" << std::endl;
    //set_points(brect_, shift_, shift_);
    //std::cout << "brect:" << "=(" << xl(brect_) <<","<<yl(brect_) << ")-("<< xh(brect_)<<","<<yh(brect_)<<")" << std::endl;
    //std::cout << "side:" << "=" << side << " side*1.2 "<< side*1.2 <<std::endl;
    //bloat(brect_, side * 1.2);
    //std::cout << "brect:" << "=(" << xl(brect_) <<","<<yl(brect_) << ")-("<< xh(brect_)<<","<<yh(brect_)<<")" << std::endl;
    //std::cout << "shift:(" << shift_.x() <<"," << shift_.y()<<")" << std::endl;
    std::cout << "<-construct_brect()" << std::endl;
  }

  void color_exterior(const VD::edge_type* edge) {
    if (edge->color() == EXTERNAL_COLOR) {
      return;
    }
    edge->color(EXTERNAL_COLOR);
    edge->twin()->color(EXTERNAL_COLOR);
    const VD::vertex_type* v = edge->vertex1();
    if (v == NULL || !edge->is_primary()) {
      return;
    }
    v->color(EXTERNAL_COLOR);
    const VD::edge_type* e = v->incident_edge();
    do {
      color_exterior(e);
      e = e->rot_next();
    } while (e != v->incident_edge());
  }

  void draw_points() {
    // Draw input points and endpoints of the input segments.
    std::cout << "->draw_points points" << std::endl;
    int j=0;
    for (std::size_t i = 0; i < point_data_.size(); ++i) {
      point_type point = point_data_[i];
      //deconvolve(point, shift_);
      glVertex2f(j++, point.x(), point.y());
    }
    std::cout << "->draw_points segments" << std::endl;
    for (std::size_t i = 0; i < segment_data_.size(); ++i) {
      point_type lp = low(segment_data_[i]);
      //lp = deconvolve(lp, shift_);
      glVertex2f(j, lp.x(), lp.y());
      point_type hp = high(segment_data_[i]);
      //hp = deconvolve(hp, shift_);
      glVertex2f(j++, hp.x(), hp.y());
    }
    std::cout << "<-draw_points" << std::endl;
    std::cout << std::endl;
  }

  void draw_segments() {
    // Draw input segments.
    std::cout << "->draw_segments" << std::endl;
    int j=0;
    for (std::size_t i = 0; i < segment_data_.size(); ++i) {
      point_type lp = low(segment_data_[i]);
      //lp = deconvolve(lp, shift_);
      glVertex2f(j, lp.x(), lp.y());
      point_type hp = high(segment_data_[i]);
      //hp = deconvolve(hp, shift_);
      glVertex2f(j++, hp.x(), hp.y());
      //std::cout << std::endl;
    }
    std::cout << "<-draw_segments" << std::endl;
    std::cout << std::endl;
  }

  void draw_vertices() {
    // Draw voronoi vertices.
    std::cout << "->draw_vertices" << std::endl;
    int j=0;
    for (const_vertex_iterator it = vd_.vertices().begin();
         it != vd_.vertices().end(); ++it) {
      if (internal_edges_only_ && (it->color() == EXTERNAL_COLOR)) {
        continue;
      }
      point_type vertex(it->x(), it->y());
      //vertex = deconvolve(vertex, shift_);
      glVertex2f(j++, vertex.x(), vertex.y());
    }
    std::cout << "<-draw_vertices" << std::endl;
    std::cout << std::endl;
  }

  void draw_edges() {
    // Draw voronoi edges.
    std::cout << "->draw_edges" << std::endl;
    int j=0;
    for (const_edge_iterator it = vd_.edges().begin();
         it != vd_.edges().end(); ++it) {
      if (primary_edges_only_ && !it->is_primary()) {
        continue;
      }
      if (internal_edges_only_ && (it->color() == EXTERNAL_COLOR)) {
        continue;
      }
      if (j>=2){
    	  std::cout << "";
      }
      std::vector<point_type> samples;
      if (!it->is_finite()) {
        clip_infinite_edge(*it, &samples);
      } else {
        point_type vertex0(it->vertex0()->x(), it->vertex0()->y());
        samples.push_back(vertex0);
        point_type vertex1(it->vertex1()->x(), it->vertex1()->y());
        samples.push_back(vertex1);
        if (it->is_curved()) {
          sample_curved_edge(*it, &samples);
        }
      }
      for (std::size_t i = 0; i < samples.size(); ++i) {
        point_type vertex = samples[i];//deconvolve(samples[i], shift_);
        glVertex2f(j, vertex.x(), vertex.y());
      }
      j++;
    }
    std::cout << "<-draw_edges" << std::endl;
  }

  void clip_infinite_edge(
    const edge_type& edge, std::vector<point_type>* clipped_edge) {
    const cell_type& cell1 = *edge.cell();
    const cell_type& cell2 = *edge.twin()->cell();
    point_type origin, direction;
    // Infinite edges could not be created by two segment sites.
    if (cell1.contains_point() && cell2.contains_point()) {
      point_type p1 = retrieve_point(cell1);
      point_type p2 = retrieve_point(cell2);
      origin.x((p1.x() + p2.x()) * 0.5);
      origin.y((p1.y() + p2.y()) * 0.5);
      direction.x(p1.y() - p2.y());
      direction.y(p2.x() - p1.x());
    } else {
      origin = cell1.contains_segment() ?
          retrieve_point(cell2) :
          retrieve_point(cell1);
      segment_type segment = cell1.contains_segment() ?
          retrieve_segment(cell1) :
          retrieve_segment(cell2);
      coordinate_type dx = high(segment).x() - low(segment).x();
      coordinate_type dy = high(segment).y() - low(segment).y();
      if ((low(segment) == origin) ^ cell1.contains_point()) {
        direction.x(dy);
        direction.y(-dx);
      } else {
        direction.x(-dy);
        direction.y(dx);
      }
    }
    coordinate_type side = xh(brect_) - xl(brect_);
    coordinate_type koef =
        side / (std::max)(fabs(direction.x()), fabs(direction.y()));
    if (edge.vertex0() == NULL) {
      clipped_edge->push_back(point_type(
          origin.x() - direction.x() * koef,
          origin.y() - direction.y() * koef));
    } else {
      clipped_edge->push_back(
          point_type(edge.vertex0()->x(), edge.vertex0()->y()));
    }
    if (edge.vertex1() == NULL) {
      clipped_edge->push_back(point_type(
          origin.x() + direction.x() * koef,
          origin.y() + direction.y() * koef));
    } else {
      clipped_edge->push_back(
          point_type(edge.vertex1()->x(), edge.vertex1()->y()));
    }
  }

  void sample_curved_edge(
      const edge_type& edge,
      std::vector<point_type>* sampled_edge) {
    coordinate_type max_dist = 1E-3 * (xh(brect_) - xl(brect_));
    point_type point = edge.cell()->contains_point() ?
        retrieve_point(*edge.cell()) :
        retrieve_point(*edge.twin()->cell());
    segment_type segment = edge.cell()->contains_point() ?
        retrieve_segment(*edge.twin()->cell()) :
        retrieve_segment(*edge.cell());
    voronoi_visual_utils<coordinate_type>::discretize(
        point, segment, max_dist, sampled_edge);
  }

  point_type retrieve_point(const cell_type& cell) {
    source_index_type index = cell.source_index();
    source_category_type category = cell.source_category();
    if (category == SOURCE_CATEGORY_SINGLE_POINT) {
      return point_data_[index];
    }
    index -= point_data_.size();
    if (category == SOURCE_CATEGORY_SEGMENT_START_POINT) {
      return low(segment_data_[index]);
    } else {
      return high(segment_data_[index]);
    }
  }

  segment_type retrieve_segment(const cell_type& cell) {
    source_index_type index = cell.source_index() - point_data_.size();
    return segment_data_[index];
  }

  //point_type shift_;
  std::vector<point_type> point_data_;
  std::vector<segment_type> segment_data_;
  rect_type brect_;
  VB vb_;
  VD vd_;
  //bool brect_initialized_= false;
  bool primary_edges_only_= false;
  bool internal_edges_only_ = false;
};

int main() {
   VorVisualizer vb;
   vb.build();
}
