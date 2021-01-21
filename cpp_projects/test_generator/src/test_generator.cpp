// Boost.Polygon library voronoi_basic_tutorial.cpp file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

#include <cstdio>
#include <iostream>
#include <vector>
#include <unordered_map>
#include <cassert>

#include <boost/polygon/voronoi.hpp>
using boost::polygon::voronoi_builder;
using boost::polygon::voronoi_diagram;
using boost::polygon::x;
using boost::polygon::y;
using boost::polygon::low;
using boost::polygon::high;

//#include "voronoi_visual_utils.hpp"

struct Point {
  int a;
  int b;
  Point(int x, int y) : a(x), b(y) {}
};

struct Segment {
  Point p0;
  Point p1;
  Segment(int x1, int y1, int x2, int y2) : p0(x1, y1), p1(x2, y2) {}
};

namespace boost {
namespace polygon {

template <>
struct geometry_concept<Point> {
  typedef point_concept type;
};

template <>
struct point_traits<Point> {
  typedef int coordinate_type;

  static inline coordinate_type get(
      const Point& point, orientation_2d orient) {
    return (orient == HORIZONTAL) ? point.a : point.b;
  }
};

template <>
struct geometry_concept<Segment> {
  typedef segment_concept type;
};

template <>
struct segment_traits<Segment> {
  typedef int coordinate_type;
  typedef Point point_type;

  static inline point_type get(const Segment& segment, direction_1d dir) {
    return dir.to_int() ? segment.p1 : segment.p0;
  }
};
}  // polygon
}  // boost

// Traversing Voronoi edges using dumb edge iterator.
int iterate_primary_edges0(const voronoi_diagram<double>& vd) {
  int result = 0;
  for (voronoi_diagram<double>::const_edge_iterator it = vd.edges().begin();
       it != vd.edges().end(); ++it) {
      ++result;
  }
  return result;
}

// Traversing Voronoi edges using edge iterator.
int iterate_primary_edges1(const voronoi_diagram<double>& vd) {
  int result = 0;
  for (voronoi_diagram<double>::const_edge_iterator it = vd.edges().begin();
       it != vd.edges().end(); ++it) {
    if (it->is_primary())
      ++result;
  }
  return result;
}

// Traversing Voronoi edges using cell iterator.
int iterate_primary_edges2(const voronoi_diagram<double> &vd) {
  int result = 0;
  for (voronoi_diagram<double>::const_cell_iterator it = vd.cells().begin();
       it != vd.cells().end(); ++it) {
    const voronoi_diagram<double>::cell_type& cell = *it;
    const voronoi_diagram<double>::edge_type* edge = cell.incident_edge();
    if (edge) {
		// This is convenient way to iterate edges around Voronoi cell.
		do {
		  if (edge->is_primary())
			++result;
		  edge = edge->next();
		} while (edge != cell.incident_edge());
    }
  }
  return result;
}

// Traversing Voronoi edges using vertex iterator.
// As opposite to the above two functions this one will not iterate through
// edges without finite endpoints and will iterate only once through edges
// with single finite endpoint.
int iterate_primary_edges3(const voronoi_diagram<double> &vd) {
  int result = 0;
  for (voronoi_diagram<double>::const_vertex_iterator it =
       vd.vertices().begin(); it != vd.vertices().end(); ++it) {
    const voronoi_diagram<double>::vertex_type& vertex = *it;
    const voronoi_diagram<double>::edge_type* edge = vertex.incident_edge();
    // This is convenient way to iterate edges around Voronoi vertex.
    do {
      if (edge->is_primary())
        ++result;
      edge = edge->rot_next();
    } while (edge != vertex.incident_edge());
  }
  return result;
}

void dbg_output(const voronoi_diagram<double> &vd) {
	std::unordered_map<const void*, int> m;
    //std::vector<voronoi_diagram<double>::cell_type*> sorted_cells;

	auto o = [&](const void* p) {
	   if (p==0) {
		   return std::string("-");
	   } else if(m.count(p)>0){
		   return std::to_string(m[p]);
	   } else {
		   return std::string("?");
	   }
	};
	//sorted_cells.resize(vd.cells().size());
	/*for (uint i=0; i<vd.cells().size(); i++) {
		sorted_cells[vd.cells()[i].source_index()] = (voronoi_diagram<double>::cell_type*) &(vd.cells()[i]);
	}
	for (uint i=0; i < sorted_cells.size(); i++) {
		assert(sorted_cells[i]);
		m[(const void*) sorted_cells[i] ] = sorted_cells[i]->source_index();
	}

	if(sorted_cells.size() >0) {
		assert(sorted_cells[0]->source_index() == 0);
	}*/
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
				  << " ie:" << o(c->incident_edge())
				  // << " c:" << c->raw_color()
				  << "  ptr=" << c
				  << std::endl;
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
				  // << " v1:" << m[(const void*)e->vertex1()]
				  << " t:" << o(e->twin())
				  << " n:" << o(e->next())
				  << " p:" << o(e->prev())
				  // << " rn:" << m[(const void*)e.rot_next()]
				  // << " c:" << e->raw_color()
				  << "  ptr=" << e
				  << std::endl;
	}
	std::cout << std::endl;
}

void generate_test(const voronoi_diagram<double> &vd, bool make_test=true, bool small_test=true) {
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
				  << " ie:" << o(c->incident_edge())
				  // << " c:" << c->raw_color()
				  << "  ptr=" << c
				  << std::endl;
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
				  // << " v1:" << m[(const void*)e->vertex1()]
				  << " t:" << o(e->twin())
				  << " n:" << o(e->next())
				  << " p:" << o(e->prev())
				  // << " rn:" << m[(const void*)e.rot_next()]
				  // << " c:" << e->raw_color()
				  << "  ptr=" << e
				  << std::endl;
	}
	std::cout << std::endl;

	if (make_test){
		int i=0;
		std::cout << "assert_eq!(output.cells().len()," << vd.cells().size() << ");" << std::endl;

		for (voronoi_diagram<double>::const_cell_iterator it =
				   vd.cells().begin(); it != vd.cells().end(); ++it) {
			auto cell = &(*it);
			std::cout << "let cell = output.cells()[" << i <<"].get();" << std::endl;
			std::cout << "assert_eq!(cell.get_id()," << i << ");" << std::endl;
			std::cout << "assert_eq!(cell.source_index()," << cell->source_index() << ");" << std::endl;
            std::cout << "assert_eq!(cell.is_degenerate()," << (cell->is_degenerate()?"true":"false") << ");" << std::endl;
            std::cout << "assert_eq!(cell.contains_point()," << (cell->contains_point()?"true":"false") << ");" << std::endl;
            std::cout << "assert_eq!(cell.contains_segment()," << (cell->contains_segment()?"true":"false") << ");" << std::endl;
			i++;
		}

		i=0;
		for (voronoi_diagram<double>::const_vertex_iterator it =
			   vd.vertices().begin(); it != vd.vertices().end(); ++it) {

			m[(const void*)&(*it)] = i;
			i++;
		}
		std::cout << "assert_eq!(output.vertices().len()," << vd.vertices().size() << ");" <<std::endl;
		i=0;
		for (voronoi_diagram<double>::const_edge_iterator it = vd.edges().begin();
			   it != vd.edges().end(); ++it) {
			m[(const void*)&(*it)] = i;
			i++;
		}
		std::cout << "assert_eq!(output.edges().len()," << vd.edges().size() << ");" <<std::endl;

		i=0;
		for (voronoi_diagram<double>::const_vertex_iterator it =
				   vd.vertices().begin(); it != vd.vertices().end(); ++it) {
			auto v = &(*it);
			std::cout << "let v = output.vertices()[" << i <<"].get();" << std::endl;
			std::cout.precision(7);
			std::cout << std::showpoint << "assert!(almost_equal(v.x(), "
					  << std::fixed << v->x() << ", v.y(), "
					  << std::fixed << v->y() << "));" << std::endl;
            if (v->incident_edge())
              std::cout << "assert_eq!(v.get_incident_edge().unwrap().0," << o(v->incident_edge()) << ");" << std::endl;
            else
              std::cout << "assert!(v.get_incident_edge().is_none());" << std::endl;
			i++;
		}

		i=0;
		if (!small_test)
			for (voronoi_diagram<double>::const_edge_iterator it = vd.edges().begin();
					   it != vd.edges().end(); ++it) {
				auto e = &(*it);

				std::cout << "assert_eq!(output.edges().get(" << i
						  << ").unwrap().get().cell().unwrap().0,"
						  << o(e->cell())
						  << ");" << std::endl;
				if (e->vertex0()) {
					std::cout << "assert_eq!(output.edges().get(" << i
							  << ").unwrap().get().vertex0().unwrap().0,"
							  << o(e->vertex0()) << ");" << std::endl;
				} else {
					std::cout << "assert!(output.edges().get(" << i
							  << ").unwrap().get().vertex0().is_none());" << std::endl;
				}
				std::cout << "assert_eq!(output.edges().get(" << i
						  << ").unwrap().get().twin().unwrap().0,"
						  << o(e->twin()) << ");" << std::endl;
				std::cout << "assert_eq!(output.edges().get(" << i
						  << ").unwrap().get().next().unwrap().0,"
						  << o(e->next()) << ");"  << std::endl;
				std::cout << "assert_eq!(output.edges().get(" << i
						  << ").unwrap().get().prev().unwrap().0,"
						  << o(e->prev()) << ");"  << std::endl;
				std::cout << "let e = output.edges()[" << i <<"].get();" << std::endl;
				if (e->vertex1()) {
					std::cout << "assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0," << o(e->vertex1()) << ");" << std::endl;
				} else {
					std::cout << "assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());" << std::endl;
				}
				if (e->rot_next()) {
					std::cout << "assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0," << o(e->rot_next()) << ");" << std::endl;
				} else {
					std::cout << "assert!(output.edge_rot_next(Some(e.get_id())).is_none());" << std::endl;
				}
				if (e->rot_prev()) {
					std::cout << "assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0," << o(e->rot_prev()) << ");" << std::endl;
				} else {
					std::cout << "assert!(output.edge_rot_prev(Some(e.get_id())).is_none());" << std::endl;
				}
				std::cout << "assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap()," << (e->is_finite()?"true":"false") << ");" << std::endl;
				std::cout << "assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap()," << (e->is_infinite()?"true":"false") << ");" << std::endl;
				std::cout << "assert_eq!(e.is_linear()," << (e->is_linear()?"true":"false") << ");" << std::endl;
				std::cout << "assert_eq!(e.is_curved()," << (e->is_curved()?"true":"false") << ");" << std::endl;
				std::cout << "assert_eq!(e.is_primary()," << (e->is_primary()?"true":"false") << ");" << std::endl;
				std::cout << "assert_eq!(e.is_secondary()," << (e->is_secondary()?"true":"false") << ");" << std::endl;
				i++;
			}
	}
}


int main() {
  // Preparing Input Geometries.
  std::vector<Point> points;
  /*int pts[4][2] = {{582, 779}, {683, 1329}, {741, 1155}, {1239, 1102}};
  for(auto &&pt: pts) {
      points.push_back(Point(pt[0],pt[1]));
  }*/
  //points.push_back(Point(12,14));
  //points.push_back(Point(1,1));
  //points.push_back(Point(11,11));

  //points.push_back(Point(10, 11));
  //points.push_back(Point(1, 3));
  //points.push_back(Point(10, 11));

  //points.push_back(Point(45, 1));
  //points.push_back(Point(8, 23));

  //points.push_back(Point(10, 10));
  //points.push_back(Point(1, 1));
  //points.push_back(Point(1, 6));

  //points.push_back(Point(10, 18));
  //points.push_back(Point(12, 3));
  //points.push_back(Point(4, 21));
  //points.push_back(Point(8, 3));

  //points.push_back(Point(8, 9));
  //points.push_back(Point(2, 14));
  //points.push_back(Point(1, 15));
  //points.push_back(Point(4, 16));
  //points.push_back(Point(9, 8));
  //
  //points.push_back(Point(10,16));
  //points.push_back(Point(12, 3));
  //points.push_back(Point(4, 12));
  //points.push_back(Point(8, 10));
  //points.push_back(Point(7, 18));
  //points.push_back(Point(8, 9));
  //points.push_back(Point(9, 8));
  //points.push_back(Point(11, 11));

  //points.push_back(Point(4, 5));

  std::vector<Segment> segments;
  if (true){
	  int sgs[5][4] = {{200,200,200,400},{200,400,400,400},{400,400,400,200},{400,200,200,200},{529,242,367,107}};
	  for(auto &&sg: sgs) {
		  segments.push_back(Segment(sg[0],sg[1],sg[2],sg[3]));
	  }
  }
  //[[442, 215, 438, 355],[129, 559, 141, 60]];
  //int c =300;
  //segments.push_back(Segment(c,c,c,200+c));
  //segments.push_back(Segment(0+c,200+c,200+c,200+c));
  //segments.push_back(Segment(200+c,200+c,200+c, c));
  //segments.push_back(Segment(200+c,c,c,c));
    
  //segments.push_back(Segment(5, 6, 3, 1));

  // Construction of the Voronoi Diagram.
  voronoi_diagram<double> vd;
  construct_voronoi(points.begin(), points.end(),
                    segments.begin(), segments.end(),
                    &vd);

  // Traversing Voronoi Graph.
  {
    printf("Traversing Voronoi graph.\n");
    printf("Number of visited primary edges using dumb edge iterator: %d\n",
            iterate_primary_edges0(vd));
    printf("Number of visited primary edges using edge iterator: %d\n",
        iterate_primary_edges1(vd));
    printf("Number of visited primary edges using cell iterator: %d\n",
        iterate_primary_edges2(vd));
    printf("Number of visited primary edges using vertex iterator: %d\n",
        iterate_primary_edges3(vd));
    printf("\n");
  }

  // Using color member of the Voronoi primitives to store the average number
  // of edges around each cell (including secondary edges).
  {
    printf("Number of edges (including secondary) around the Voronoi cells:\n");
    for (voronoi_diagram<double>::const_edge_iterator it = vd.edges().begin();
         it != vd.edges().end(); ++it) {
      std::size_t cnt = it->cell()->color();
      it->cell()->color(cnt + 1);
    }
    for (voronoi_diagram<double>::const_cell_iterator it = vd.cells().begin();
         it != vd.cells().end(); ++it) {
      printf("%lu ", it->color());
    }
    printf("\n");
    printf("\n");
  }

  // Linking Voronoi cells with input geometries.
  {
    unsigned int cell_index = 0;
    for (voronoi_diagram<double>::const_cell_iterator it = vd.cells().begin();
         it != vd.cells().end(); ++it) {
      if (it->contains_point()) {
        if (it->source_category() ==
            boost::polygon::SOURCE_CATEGORY_SINGLE_POINT) {
          std::size_t index = it->source_index();
          Point p = points[index];
          printf("Cell #%u contains a point: (%d, %d).\n",
                 cell_index, x(p), y(p));
        } else if (it->source_category() ==
                   boost::polygon::SOURCE_CATEGORY_SEGMENT_START_POINT) {
          std::size_t index = it->source_index() - points.size();
          Point p0 = low(segments[index]);
          printf("Cell #%u contains segment start point: (%d, %d).\n",
                 cell_index, x(p0), y(p0));
        } else if (it->source_category() ==
                   boost::polygon::SOURCE_CATEGORY_SEGMENT_END_POINT) {
          std::size_t index = it->source_index() - points.size();
          Point p1 = high(segments[index]);
          printf("Cell #%u contains segment end point: (%d, %d).\n",
                 cell_index, x(p1), y(p1));
        }
      } else {
        std::size_t index = it->source_index() - points.size();
        Point p0 = low(segments[index]);
        Point p1 = high(segments[index]);
        printf("Cell #%u contains a segment: ((%d, %d), (%d, %d)). \n",
               cell_index, x(p0), y(p0), x(p1), y(p1));
      }
      ++cell_index;
    }
  }
  {
	 printf("Num vertices %lu\n", vd.num_vertices());
     unsigned int vertec_index = 0;
     for (voronoi_diagram<double>::const_vertex_iterator vt = vd.vertices().begin();
          vt != vd.vertices().end(); ++vt) {
    	 printf("Vertex #%u contains a point: (%f, %f).\n",
    			 vertec_index, vt->x(), vt->y() );
     }
  }
  std::cout << std::endl;
  //dbg_output(vd);
  generate_test(vd, true, true);
  return 0;
}
