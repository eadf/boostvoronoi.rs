// Boost.Polygon library voronoi_builder.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

#ifndef BOOST_POLYGON_VORONOI_BUILDER
#define BOOST_POLYGON_VORONOI_BUILDER

#include <algorithm>
#include <map>
#include <queue>
#include <utility>
#include <vector>
#include <unordered_map>

#include "detail/voronoi_ctypes.hpp"
#include "detail/voronoi_predicates.hpp"
#include "detail/voronoi_structures.hpp"

#include "voronoi_geometry_type.hpp"
#include <iostream>

namespace boost {
namespace polygon {

template <typename T>
std::string debug_number_padding (T x)
{
    const char* sign = x < 0 ? "-" : "";
    const auto mag = std::abs (x);
    if (mag < 10)  return sign + std::string (" " + std::to_string(mag));
    //if (mag < 100) return sign + std::string (" " + std::to_string(mag));
    return std::to_string(x);
}

// GENERAL INFO:
// The sweepline algorithm implementation to compute Voronoi diagram of
// points and non-intersecting segments (excluding endpoints).
// Complexity - O(N*logN), memory usage - O(N), where N is the total number
// of input geometries.
//
// CONTRACT:
// 1) Input geometries should have integral (e.g. int32, int64) coordinate type.
// 2) Input geometries should not intersect except their endpoints.
//
// IMPLEMENTATION DETAILS:
// Each input point creates one input site. Each input segment creates three
// input sites: two for its endpoints and one for the segment itself (this is
// made to simplify output construction). All the site objects are constructed
// and sorted at the algorithm initialization step. Priority queue is used to
// dynamically hold circle events. At each step of the algorithm execution the
// leftmost event is retrieved by comparing the current site event and the
// topmost element from the circle event queue. STL map (red-black tree)
// container was chosen to hold state of the beach line. The keys of the map
// correspond to the neighboring sites that form a bisector and values map to
// the corresponding Voronoi edges in the output data structure.
template <typename T,
          typename CTT = detail::voronoi_ctype_traits<T>,
          typename VP = detail::voronoi_predicates<CTT> >
class voronoi_builder {
public:
typedef typename CTT::int_type int_type;
typedef typename CTT::fpt_type fpt_type;

voronoi_builder() : index_(0) {
}

// Each point creates a single site event.
std::size_t insert_point(const int_type& x, const int_type& y) {
	site_events_.push_back(site_event_type(x, y));
	site_events_.back().initial_index(index_);
	site_events_.back().source_category(SOURCE_CATEGORY_SINGLE_POINT);
	return index_++;
}

// Each segment creates three site events that correspond to:
//   1) the start point of the segment;
//   2) the end point of the segment;
//   3) the segment itself defined by its start point.
std::size_t insert_segment(
	const int_type& x1, const int_type& y1,
	const int_type& x2, const int_type& y2) {
	// Set up start point site.
	point_type p1(x1, y1);
	site_events_.push_back(site_event_type(p1));
	site_events_.back().initial_index(index_);
	site_events_.back().source_category(SOURCE_CATEGORY_SEGMENT_START_POINT);

	// Set up end point site.
	point_type p2(x2, y2);
	site_events_.push_back(site_event_type(p2));
	site_events_.back().initial_index(index_);
	site_events_.back().source_category(SOURCE_CATEGORY_SEGMENT_END_POINT);

	// Set up segment site.
	if (point_comparison_(p1, p2)) {
		site_events_.push_back(site_event_type(p1, p2));
		site_events_.back().source_category(SOURCE_CATEGORY_INITIAL_SEGMENT);
	}
	else {
		site_events_.push_back(site_event_type(p2, p1));
		site_events_.back().source_category(SOURCE_CATEGORY_REVERSE_SEGMENT);
	}
	site_events_.back().initial_index(index_);
	return index_++;
}

// Run sweepline algorithm and fill output data structure.
template <typename OUTPUT>
void construct(OUTPUT* output) {
	// Init structures.
	output->_reserve(site_events_.size());
	init_sites_queue();
	init_beach_line(output);

	//dbg();

	// The algorithm stops when there are no events to process.
	event_comparison_predicate event_comparison;
	while (!circle_events_.empty() ||
	       !(site_event_iterator_ == site_events_.end())) {
		dbg_beachline();
		std::cout <<"################################################" << std::endl;
		std::cout << "loop circle_events_:" << circle_events_.size()
				  << " num_vertices:" << output->num_vertices()
				  << " site_events:" << site_events_.size()
				  << " beachline:" << beach_line_.size()
				  << " index:" << index_
				  << " debug_site_counter:" << debug_site_counter
				  << " debug_circle_counter:" << debug_circle_counter << std::endl;
		if (debug_circle_counter>=27) {
			std::cout << "";
		}
		if (debug_site_counter>=8) {
			std::cout << "";
		}
		if (circle_events_.empty()) {
			process_site_event(output);
		}
		else if (site_event_iterator_ == site_events_.end()) {
			process_circle_event(output);
		}
		else {
			// int <-> float comparison
			if (event_comparison(*site_event_iterator_,
			                     circle_events_.top().first)) {
				process_site_event(output);
			}
			else {
				process_circle_event(output);
			}
		}
		while (!circle_events_.empty() &&
		       !circle_events_.top().first.is_active()) {
			circle_events_.pop();
		}
	}
	beach_line_.clear();

	// Finish construction.
	output->_build();
}

void clear() {
	index_ = 0;
	site_events_.clear();
}

private:
typedef detail::point_2d<int_type> point_type;
typedef detail::site_event<int_type> site_event_type;
typedef typename std::vector<site_event_type>::const_iterator
        site_event_iterator_type;
typedef detail::circle_event<fpt_type> circle_event_type;
typedef typename VP::template point_comparison_predicate<point_type>
        point_comparison_predicate;
typedef typename VP::
        template event_comparison_predicate<site_event_type, circle_event_type>
        event_comparison_predicate;
typedef typename VP::
        template circle_formation_predicate<site_event_type, circle_event_type>
        circle_formation_predicate_type;
typedef void edge_type;
typedef detail::beach_line_node_key<site_event_type> key_type;
typedef detail::beach_line_node_data<edge_type, circle_event_type>
        value_type;
typedef typename VP::template node_comparison_predicate<key_type>
        node_comparer_type;
typedef std::map< key_type, value_type, node_comparer_type > beach_line_type;
typedef typename beach_line_type::iterator beach_line_iterator;
typedef std::pair<circle_event_type, beach_line_iterator> event_type;
typedef struct {
	bool operator()(const event_type& lhs, const event_type& rhs) const {
		return predicate(rhs.first, lhs.first);
	}
	event_comparison_predicate predicate;
} event_comparison_type;
typedef detail::ordered_queue<event_type, event_comparison_type>
        circle_event_queue_type;
typedef std::pair<point_type, beach_line_iterator> end_point_type;

void init_sites_queue() {
	// Sort site events.
	std::sort(site_events_.begin(), site_events_.end(),
	          event_comparison_predicate());

	// Remove duplicates.
	site_events_.erase(std::unique(
				   site_events_.begin(), site_events_.end()), site_events_.end());

	// Index sites.
	for (std::size_t cur = 0; cur < site_events_.size(); ++cur) {
		site_events_[cur].sorted_index(cur);
	}

	// Init site iterator.
	site_event_iterator_ = site_events_.begin();
}

template <typename OUTPUT>
void init_beach_line(OUTPUT* output) {
	if (site_events_.empty()) {
		return;
	}
	if (site_events_.size() == 1) {
		// Handle single site event case.
		output->_process_single_site(site_events_[0]);
		++site_event_iterator_;
	}
	else {
		int skip = 0;

		while (site_event_iterator_ != site_events_.end() &&
		       VP::is_vertical(site_event_iterator_->point0(),
		                       site_events_.begin()->point0()) &&
		       VP::is_vertical(*site_event_iterator_)) {
			++site_event_iterator_;
			++skip;
		}

		if (skip == 1) {
			// Init beach line with the first two sites.
			init_beach_line_default(output);
		}
		else {
			// Init beach line with collinear vertical sites.
			init_beach_line_collinear_sites(output);
		}
	}
}

// Init beach line with the two first sites.
// The first site is always a point.
template <typename OUTPUT>
void init_beach_line_default(OUTPUT* output) {
	// Get the first and the second site event.
	site_event_iterator_type it_first = site_events_.begin();
	site_event_iterator_type it_second = site_events_.begin();
	++it_second;
	insert_new_arc(
		*it_first, *it_first, *it_second, beach_line_.end(), output);
	// The second site was already processed. Move the iterator.
	++site_event_iterator_;
}

// Init beach line with collinear sites.
template <typename OUTPUT>
void init_beach_line_collinear_sites(OUTPUT* output) {
	site_event_iterator_type it_first = site_events_.begin();
	site_event_iterator_type it_second = site_events_.begin();
	++it_second;
	while (it_second != site_event_iterator_) {
		// Create a new beach line node.
		key_type new_node(*it_first, *it_second);

		// Update the output.
		edge_type* edge = output->_insert_new_edge(*it_first, *it_second).first;

		// Insert a new bisector into the beach line.
		beach_line_.insert(beach_line_.end(),
		                   std::pair<key_type, value_type>(new_node, value_type(edge)));
		//dbg_beachline();
		// Update iterators.
		++it_first;
		++it_second;
	}
}

void deactivate_circle_event(value_type* value) {
	if (debug_site_counter>=6){
		dbg_beachline();
		std::cout << "debug_site_counter:" << debug_site_counter
				  << ", circle_events:" << circle_events_.size()
				  << ", circle_event: " << value->circle_event() << std::endl;
	}
	if (value->circle_event()) {
		value->circle_event()->deactivate();
		value->circle_event(NULL);
	}
}

template <typename OUTPUT>
void process_site_event(OUTPUT* output) {

    if (debug_site_counter>=6){
    	dbg_beachline();
    	std::cout << "debug_site_counter:" << debug_site_counter << std::endl;
    }
    debug_site_counter++;

	// Get next site event to process.
	site_event_type site_event = *site_event_iterator_;
    std::cout << "site_event:"; dbg(&site_event); std::cout << std::endl;

	// Move site iterator.
	site_event_iterator_type last = site_event_iterator_ + 1;

	// If a new site is an end point of some segment,
	// remove temporary nodes from the beach line data structure.
	if (!site_event.is_segment()) {
		std::cout << "b4 erase";dbg_beachline();
		if (!end_points_.empty()){
		   std::cout << "peek end_points_.size()=" << end_points_.size() << std::endl;;
		   std::cout << "site_event.point0()=" << site_event.point0() << std::endl;
		   std::cout << "peek.p=" << end_points_.top().first << std::endl;
		   std::cout << "peek.L=" <<end_points_.top().second->first.left_site() << std::endl;
		   std::cout << "peek.R=" <<end_points_.top().second->first.right_site() << std::endl;
		}
		while (!end_points_.empty() &&
		       end_points_.top().first == site_event.point0()) {
			beach_line_iterator b_it = end_points_.top().second;
			std::cout << "erasing:"; dbg_beachline(&(b_it->first));std::cout << std::endl;
			end_points_.pop();
			beach_line_.erase(b_it);
			std::cout << "after erase, end_points_.size()=" << end_points_.size() << std::endl;;
			dbg_beachline();
		}
	}
	else {
		while (last != site_events_.end() &&
		       last->is_segment() && last->point0() == site_event.point0()) {
			++last;
		}
	}

	// Find the node in the binary search tree with left arc
	// lying above the new site point.
	key_type new_key(*site_event_iterator_);

	if (debug_site_counter>=6){
		dbg_beachline();
		std::cout << "debug_site_counter:" << debug_site_counter << std::endl;
	}
	std::cout << "new_key_"; dbg_beachline(&new_key);std::cout << std::endl;
	if (debug_site_counter>=7){
		std::cout << "";
		dbg_beachline_with_cmp(&new_key);
	}

	beach_line_iterator right_it = beach_line_.lower_bound(new_key);
	if (debug_site_counter>=7){
		dbg_beachline();
		std::cout << "debug_site_counter:" << debug_site_counter << std::endl;
		std::cout << "right_it==";
		if (right_it == beach_line_.end())
			std::cout << "None" << std::endl;
		else {
			dbg_beachline(&(right_it->first));std::cout << std::endl;
		}
	}

/*
	std::cout << std::endl << std::endl <<"site_event_iterator_:"; dbg(&(*site_event_iterator_));
	std::cout << "last:"; dbg(&(*last)); std::cout << std::endl;
	std::cout << std::endl;
*/
	for (; site_event_iterator_ != last; ++site_event_iterator_) {
		site_event = *site_event_iterator_;
		beach_line_iterator left_it = right_it;

		if (debug_site_counter>=6){
			dbg_beachline();
			std::cout << "debug_site_counter:" << debug_site_counter << std::endl;
		}

		// Do further processing depending on the above node position.
		// For any two neighboring nodes the second site of the first node
		// is the same as the first site of the second node.
		if (right_it == beach_line_.end()) {
			// The above arc corresponds to the second arc of the last node.
			// Move the iterator to the last node.
			--left_it;

			// Get the second site of the last node
			const site_event_type& site_arc = left_it->first.right_site();

			// Insert new nodes into the beach line. Update the output.
			right_it = insert_new_arc(
				site_arc, site_arc, site_event, right_it, output);

			// Add a candidate circle to the circle event queue.
			// There could be only one new circle event formed by
			// a new bisector and the one on the left.
			activate_circle_event(left_it->first.left_site(),
			                      left_it->first.right_site(),
			                      site_event, right_it);
		}
		else if (right_it == beach_line_.begin()) {
			// The above arc corresponds to the first site of the first node.
			const site_event_type& site_arc = right_it->first.left_site();

			// Insert new nodes into the beach line. Update the output.
			left_it = insert_new_arc(
				site_arc, site_arc, site_event, right_it, output);

			// If the site event is a segment, update its direction.
			if (site_event.is_segment()) {
				site_event.inverse();
			}

			// Add a candidate circle to the circle event queue.
			// There could be only one new circle event formed by
			// a new bisector and the one on the right.
			activate_circle_event(site_event, right_it->first.left_site(),
			                      right_it->first.right_site(), right_it);
			right_it = left_it;
			std::cout << "right_it==";dbg_beachline(&(right_it->first));std::cout << std::endl;
		}
		else {
			std::cout << "right_it==";dbg_beachline(&(right_it->first));std::cout << std::endl;
			// The above arc corresponds neither to the first,
			// nor to the last site in the beach line.
			const site_event_type& site_arc2 = right_it->first.left_site();
			const site_event_type& site3 = right_it->first.right_site();

			// Remove the candidate circle from the event queue.
			deactivate_circle_event(&right_it->second);
			--left_it;
			dbg_beachline();
			std::cout << "left_it==";dbg_beachline(&(left_it->first));std::cout << std::endl;

			const site_event_type& site_arc1 = left_it->first.right_site();
			const site_event_type& site1 = left_it->first.left_site();

			// Insert new nodes into the beach line. Update the output.
			beach_line_iterator new_node_it =
				insert_new_arc(site_arc1, site_arc2, site_event, right_it, output);
			dbg_beachline();

			// Add candidate circles to the circle event queue.
			// There could be up to two circle events formed by
			// a new bisector and the one on the left or right.
			activate_circle_event(site1, site_arc1, site_event, new_node_it);

			// If the site event is a segment, update its direction.
			if (site_event.is_segment()) {
				site_event.inverse();
			}
			activate_circle_event(site_event, site_arc2, site3, right_it);
			right_it = new_node_it;
		}
	}
}

// In general case circle event is made of the three consecutive sites
// that form two bisectors in the beach line data structure.
// Let circle event sites be A, B, C, two bisectors that define
// circle event are (A, B), (B, C). During circle event processing
// we remove (A, B), (B, C) and insert (A, C). As beach line comparison
// works correctly only if one of the nodes is a new one we remove
// (B, C) bisector and change (A, B) bisector to the (A, C). That's
// why we use const_cast there and take all the responsibility that
// map data structure keeps correct ordering.
template <typename OUTPUT>
void process_circle_event(OUTPUT* output) {
	// Get the topmost circle event.
	const event_type& e = circle_events_.top();
	const circle_event_type& circle_event = e.first;
	std::cout << "->process_circle_event()" << std::endl;
	std::cout << "Found circle event:" << e.first << std::endl;
	if (debug_circle_counter>=36) {
		dbg_beachline();
	}
	beach_line_iterator it_first = e.second;
	beach_line_iterator it_last = it_first;

	// Get the C site.
	site_event_type site3 = it_first->first.right_site();

	// Get the half-edge corresponding to the second bisector - (B, C).
	edge_type* bisector2 = it_first->second.edge();
	std::cout << "it_first pre="; dbg_beachline(&(it_first->first));std::cout << std::endl;
	// Get the half-edge corresponding to the first bisector - (A, B).
	--it_first;
	std::cout << "it_first post="; dbg_beachline(&(it_first->first));std::cout << std::endl;

	edge_type* bisector1 = it_first->second.edge();
	std::cout << "bisector1=" << output->dbg_edge(bisector1) << std::endl;
	std::cout << "bisector2=" << output->dbg_edge(bisector2) << std::endl;

	// Get the A site.
	site_event_type site1 = it_first->first.left_site();

	if (!site1.is_segment() && site3.is_segment() &&
	    site3.point1() == site1.point0()) {
		site3.inverse();
	}

	if (debug_circle_counter>=27) {
		std::cout << "";
		dbg_beachline();
	}
	// Change the (A, B) bisector node to the (A, C) bisector node.
	const_cast<key_type&>(it_first->first).right_site(site3);
	if (debug_circle_counter>=27) {
		std::cout << "";
		dbg_beachline_with_self_cmp();
		dbg_beachline();
	}
	// Insert the new bisector into the beach line.
	it_first->second.edge(output->_insert_new_edge(
				      site1, site3, circle_event, bisector1, bisector2).first);
	dbg_beachline();
	// Remove the (B, C) bisector node from the beach line.
	beach_line_.erase(it_last);
	dbg_beachline();
	it_last = it_first;

	// Pop the topmost circle event from the event queue.
	circle_events_.pop();
	std::cout << "circle_events_.size()=" << circle_events_.size() << std::endl;

	// Check new triplets formed by the neighboring arcs
	// to the left for potential circle events.
	if (it_first != beach_line_.begin()) {
		deactivate_circle_event(&it_first->second);
		--it_first;
		const site_event_type& site_l1 = it_first->first.left_site();
		activate_circle_event(site_l1, site1, site3, it_last);
	}

	// Check the new triplet formed by the neighboring arcs
	// to the right for potential circle events.
	//dbg_beachline();
	dbg_beachline();
	std::cout << "it_last->first(pre++):"; dbg_beachline(&it_last->first);
	std::cout << std::endl;
	++it_last;
	std::cout << "it_last->first(post++):"; dbg_beachline(&it_last->first);
	std::cout << std::endl;
	if (debug_circle_counter>=7) {
			std::cout << "";
	}
	if (it_last != beach_line_.end()) {
		deactivate_circle_event(&it_last->second);
		const site_event_type& site_r1 = it_last->first.right_site();
		activate_circle_event(site1, site3, site_r1, it_last);
	}
}

// Insert new nodes into the beach line. Update the output.
template <typename OUTPUT>
beach_line_iterator insert_new_arc(
	const site_event_type& site_arc1, const site_event_type &site_arc2,
	const site_event_type& site_event, beach_line_iterator position,
	OUTPUT* output) {

	std::cout << "insert_new_arc" << std::endl;

	std::cout << "site_arc1:"; dbg(&site_arc1); std::cout << std::endl;
	std::cout << " site_arc2:"; dbg(&site_arc2); std::cout << std::endl;
	std::cout << " site_event:"; dbg(&site_event); std::cout << std::endl;

	// Create two new bisectors with opposite directions.
	key_type new_left_node(site_arc1, site_event);
	key_type new_right_node(site_event, site_arc2);

	// Set correct orientation for the first site of the second node.
	if (site_event.is_segment()) {
		new_right_node.left_site().inverse();
	}

	// Update the output.
	std::pair<edge_type*, edge_type*> edges =
		output->_insert_new_edge(site_arc2, site_event);
	position = beach_line_.insert(position,
	                              typename beach_line_type::value_type(
					      new_right_node, value_type(edges.second)));
	dbg_beachline();
	if (site_event.is_segment()) {
		// Update the beach line with temporary bisector, that will
		// disappear after processing site event corresponding to the
		// second endpoint of the segment site.
		key_type new_node(site_event, site_event);
		new_node.right_site().inverse();
		std::cout << "newnode:"; dbg_beachline(&new_node); std::cout << std::endl;
		dbg_beachline();
		position = beach_line_.insert(position,
		                              typename beach_line_type::value_type(new_node, value_type(NULL)));
		dbg_beachline();
		// Update the data structure that holds temporary bisectors.
		{
		   std::cout << "end_points_.push point=" << site_event.point1()<<std::endl;
		   std::cout << " positionL=" << position->first.left_site()<<std::endl;
		   std::cout << " positionR=" << position->first.right_site()<<std::endl;
		   end_points_.push(std::make_pair(site_event.point1(), position));
		   std::cout << "peek.p=" << end_points_.top().first << std::endl;
		}
	}

	position = beach_line_.insert(position,
	                              typename beach_line_type::value_type(
					      new_left_node, value_type(edges.first)));
	dbg_beachline();
	return position;
}

// Add a new circle event to the event queue.
// bisector_node corresponds to the (site2, site3) bisector.
void activate_circle_event(const site_event_type& site1,
                           const site_event_type& site2,
                           const site_event_type& site3,
                           beach_line_iterator bisector_node) {
	dbg_beachline();
	std::cout << "{Activate_circle_event:" << debug_circle_counter << std::endl;
	dbg(site1,site2,site3,bisector_node);
	std::cout << "}" << std::endl;
	if (debug_circle_counter>=6){
		dbg_beachline();
		std::cout << "";
	}
	debug_circle_counter += 1;

	circle_event_type c_event;
	// Check if the three input sites create a circle event.
	if (circle_formation_predicate_(site1, site2, site3, c_event)) {
		// Add the new circle event to the circle events queue.
		// Update bisector's circle event iterator to point to the
		// new circle event in the circle event queue.
		std::cout << "++++ activate_circle_event() +++++++++++" << std::endl;
		std::cout << c_event << std::endl; //"circle: cx:"<< c_event.x()<<" cy:"<< c_event.y() << " lx:" << c_event.lower_x() << std::endl;

		event_type& e = circle_events_.push(
			std::pair<circle_event_type, beach_line_iterator>(
				c_event, bisector_node));
		bisector_node->second.circle_event(&e.first);
		std::cout << e.first << std::endl;;//"circle: cx:"<< e.first.x()<<" cy:"<< e.first.y() << " lx:" << e.first.lower_x() << std::endl;
		std::cout << " circle_events_.size()=" << circle_events_.size() << std::endl;
	}
}

private:
point_comparison_predicate point_comparison_;
struct end_point_comparison {
	bool operator() (const end_point_type& end1,
	                 const end_point_type& end2) const {
		return point_comparison(end2.first, end1.first);
	}
	point_comparison_predicate point_comparison;
};

void dbg() {
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

	for (uint i=0; i<site_events_.size(); i++) {
		m[(const void*)& site_events_[i] ] = i;
	}

	for (uint i=0; i<site_events_.size(); i++) {
		auto s = &site_events_[i];
		std::cout << "Site:" << i
				  << "==" << o(s)
				  << " p0:(" << debug_number_padding(s->point0().x())
				  << "," << debug_number_padding(s->point0().y())
				  << ") p1:(" << debug_number_padding(s->point1().x())
				  << "," << debug_number_padding(s->point1().y())
				  << ") si:" << s->sorted_index()
				  << " ii:" << s->initial_index()
				  << " f:" <<s->flags()
				  << std::endl;
	}

}

void dbg_beachline() {
	std::cout << "-----beachline----" << beach_line_.size() << std::endl;
	int i=0;
	for (beach_line_iterator b_it = beach_line_.begin(); b_it != beach_line_.end(); b_it++){
		std::cout << "#" << i <<":";
		dbg_beachline(&(b_it->first));

		if (b_it->second.circle_event()){
			std::cout << " -> CircleEvent(";
			std::cout << "x=" << b_it->second.circle_event()->x();
			std::cout << ",y=" << b_it->second.circle_event()->y();
			std::cout << ",lx=" << b_it->second.circle_event()->lower_x() << ")";
		}else {
			std::cout << " -> CircleEvent=-";
		}
		if (!b_it->second.edge()){
			std::cout << ", Temporary bisector";
		}
	    std::cout << std::endl;
	    i++;
	}
	std::cout << "------------------" << std::endl;
}

void dbg_beachline_with_cmp(const key_type* key=NULL) {
	//auto comp = beach_line_.value_comp();
    node_comparer_type comp;

	std::cout << "-----beachline----" << beach_line_.size() << std::endl;
	int i=0;
	std::cout << "Key =" << i <<":"; dbg_beachline(key); std::cout << std::endl;
	for (beach_line_iterator b_it = beach_line_.begin(); b_it != beach_line_.end(); b_it++){
		std::cout << "#" << i <<":";
		dbg_beachline( &(b_it->first));
		if (b_it->second.circle_event()){
			std::cout << " -> CircleEvent!";
		}else {
			std::cout << " -> CircleEvent=-";
		}
		if (key) {
			std::cout << " cmp=" << (comp(*(&(b_it->first)), *key)?"TRUE":"FALSE");
		}
	    std::cout << std::endl;
	    i++;
	}
	std::cout << "------------------" << std::endl;
}

void dbg_beachline_with_self_cmp() {
	//auto comp = beach_line_.value_comp();
    node_comparer_type comp;

	std::cout << "-----beachline----" << beach_line_.size() << std::endl;
	int i=1;
	beach_line_iterator b_it = beach_line_.begin();
	//if (b_it)
	std::cout << "#0:"; dbg_beachline(&(b_it->first)); std::cout << std::endl;
	beach_line_iterator prev = b_it;
	b_it++;
	beach_line_iterator next = b_it;
	if(next != beach_line_.end()) {
		next++;
	}

	for ( ; b_it != beach_line_.end(); b_it++){
		bool cmp = comp(*(&(b_it->first)),*(&(prev->first)));
		bool cmp2 = comp(*(&(b_it->first)),*(&(next->first)));

		std::cout << "#" << i <<":";
		dbg_beachline( &(b_it->first));
		if (b_it->second.circle_event()){
			std::cout << " -> CircleEvent!";
		}else {
			std::cout << " -> CircleEvent=-";
		}
		if (prev!= beach_line_.end()) {
			std::cout << "     cmp=" << (cmp?"TRUE":"FALSE") << " cmp2=" << (cmp2?"TRUE":"FALSE");
		}
	    std::cout << std::endl;
	    i++;
	    prev++;
	    if(next != beach_line_.end()) {
			next++;
		}
	}
	std::cout << "------------------" << std::endl;
}

void dbg_beachline(const key_type* bi) {
	std::cout << "L:";
	dbg(&bi->left_site());
	std::cout << ",R:";
	dbg(&bi->right_site());
}

void dbg(const site_event_type *site){
	if (site) {
		/*std::cout << "#" << site->sorted_index();
		if (site->is_segment()){
			std::cout << "(" << debug_number_padding(site->x0())
					  << "," << debug_number_padding(site->y0())
					  << ")" << (site->is_inverse()?"¿":"-")
					  << "(" << debug_number_padding(site->x1())
					  << "," << debug_number_padding(site->y1())
					  << ")";
			} else {
				std::cout << "("
				<< debug_number_padding(site->x0())
				<< ","
				<< debug_number_padding(site->y0()) << ")";
			}
		std::cout << ",ii:" << site->initial_index_;
		std::cout << ",f:" << site->flags();
		*/
		std::cout << *site;
	} else {
		std::cout << "NULL";
	}
}

void dbg(const site_event_type& site1,
	   const site_event_type& site2,
	   const site_event_type& site3,
	   beach_line_iterator bisector_node) {

	std::cout << "site1:"; dbg(&site1);std::cout << std::endl;
	std::cout << "site2:"; dbg(&site2);std::cout << std::endl;
	std::cout << "site3:"; dbg(&site3);std::cout << std::endl;
	std::cout << "L:"; dbg(&bisector_node->first.left_site());
	std::cout << std::endl;
	std::cout << "R:"; dbg(&bisector_node->first.right_site());
	std::cout << std::endl;
	std::cout << std::endl;
}

std::vector<site_event_type> site_events_;
site_event_iterator_type site_event_iterator_;
std::priority_queue< end_point_type, std::vector<end_point_type>,
                     end_point_comparison > end_points_;
circle_event_queue_type circle_events_;
beach_line_type beach_line_;
circle_formation_predicate_type circle_formation_predicate_;
std::size_t index_;
int debug_circle_counter = 0;
int debug_site_counter = 0;

// Disallow copy constructor and operator=
voronoi_builder(const voronoi_builder&);
void operator=(const voronoi_builder&);
};

typedef voronoi_builder<detail::int32> default_voronoi_builder;
}  // polygon
}  // boost

#endif  // BOOST_POLYGON_VORONOI_BUILDER
