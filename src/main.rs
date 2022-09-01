use std::collections::HashMap;
use std::collections::HashSet;

/* Let F(n) be the number of connected graphs with blue edges (directed) and red edges (undirected) containing:
 * * two vertices of degree 1, one with a single outgoing blue edge and the other with a single incoming blue edge.
 * * vertices of degree 3, each of which has an incoming blue edge, a different outgoing blue edge and a red edge.
 */

struct Graph {
    // vertex id -> (directed edge to, directed edge from, undirected edge)
    // TODO: turn this into a vector of tuples. use the natural index of the vector subscript?
    // TODO: consider not using Option so that manually initing is easier to type...
    vertices: HashMap<u16, [Option<u16>; 3]>,
}

impl Graph {
    fn print(&self) {
        println!("{:?}", self.vertices);
    }

    // TODO: is_valid_graph. just to sanity check

    fn is_connected(&self, n: u16) -> bool {
        let mut visited = HashSet::new();
        let mut fringe = Vec::new();
        fringe.push(0);
        while !fringe.is_empty() {
            let cur = fringe.pop().unwrap();
            visited.insert(cur);

            // I think we only really need to check "edges to" if root behaves like I think. I don't have a proof that
            // that would be sufficient. Would also need to assume that we know the root (which we would, but make this fn
            // less general).
            for i in self.vertices[&cur] {
                if i.is_none() {
                    continue;
                }
                let i_some = i.unwrap();
                if !visited.contains(&i_some) {
                    visited.insert(i_some);
                    fringe.push(i_some);
                }
            }
        }
        return visited.len() == n.into();
    }

    fn has_valid_edges(&self) -> bool {
        let mut has_source = false;
        let mut has_sink = false;

        for v in self.vertices.values() {
            if v[0].is_some() && v[1].is_none() && v[2].is_none() {
                if has_source {
                    return false;
                }
                has_source = true;
                continue;
            }
            if v[0].is_none() && v[1].is_some() && v[2].is_none() {
                if has_sink {
                    return false;
                }
                has_sink = true;
                continue;
            }
            // else, has 1 incoming edge, 1 different outgoing edge, 1 undirected edge
            if v[0].is_none() || v[1].is_none() || v[2].is_none() {
                return false;
            }
        }

        return has_source && has_sink;
    }

    fn is_solution(&self, n: u16) -> bool {
        if self.vertices.len() != n.into() {
            return false;
        }
        return self.is_connected(n) && self.has_valid_edges();
    }
}

fn main() {
    // currently testing the solutions given in the example problem
    let gs: Vec<Graph> = vec![
        // where M = 0
        Graph {
            vertices: HashMap::from([(0, [Some(1), None, None]), (1, [None, Some(0), None])]),
        },
        // where M = 2
        Graph {
            vertices: HashMap::from([
                (0, [Some(1), None, None]),
                (1, [Some(2), Some(0), Some(2)]),
                (2, [Some(3), Some(1), Some(1)]),
                (3, [None, Some(2), None]),
            ]),
        },
        // where M = 4
        Graph {
            vertices: HashMap::from([
                (0, [Some(1), None, None]),
                (1, [Some(2), Some(1), Some(2)]),
                (2, [Some(3), Some(2), Some(1)]),
                (3, [Some(4), Some(3), Some(4)]),
                (4, [Some(5), Some(4), Some(3)]),
                (5, [None, Some(5), None]),
            ]),
        },
        Graph {
            vertices: HashMap::from([
                (0, [Some(1), None, None]),
                (1, [Some(2), Some(1), Some(3)]),
                (2, [Some(3), Some(2), Some(4)]),
                (3, [Some(4), Some(3), Some(1)]),
                (4, [Some(5), Some(4), Some(2)]),
                (5, [None, Some(5), None]),
            ]),
        },
        Graph {
            vertices: HashMap::from([
                (0, [Some(1), None, None]),
                (1, [Some(2), Some(1), Some(4)]),
                (2, [Some(3), Some(2), Some(3)]),
                (3, [Some(4), Some(3), Some(2)]),
                (4, [Some(5), Some(4), Some(1)]),
                (5, [None, Some(5), None]),
            ]),
        },
        Graph {
            vertices: HashMap::from([
                (0, [Some(1), None, None]),
                (1, [Some(2), Some(1), Some(3)]),
                (2, [None, Some(2), None]),
                (3, [Some(4), Some(5), Some(1)]),
                (4, [Some(5), Some(3), Some(5)]),
                (5, [Some(3), Some(4), Some(4)]),
            ]),
        },
        Graph {
            vertices: HashMap::from([
                (0, [Some(1), None, None]),
                (1, [Some(2), Some(0), Some(4)]),
                (2, [Some(3), Some(1), Some(5)]),
                (3, [None, Some(2), None]),
                (4, [Some(5), Some(5), Some(1)]),
                (5, [Some(4), Some(4), Some(2)]),
            ]),
        },
    ];

    for g in gs {
        g.print();
        println!("is_solution : {}", g.is_solution(g.vertices.len() as u16));
    }
}

/*
Ideas:
* [x] correctness check: connected, right number of edges, right edges
* [ ] permutation generation
*     [ ] we know how many edges there will be. n = 4 means 6 vertices, 5 blue edges, 2 red edges. limit permutations?
* [ ] backtracking
* [ ] isomorphic graphs. no redundant shapes
*/
