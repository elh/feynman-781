use std::collections::HashMap;
use std::collections::HashSet;

/* Let F(n) be the number of connected graphs with blue edges (directed) and red edges (undirected) containing:
 * * two vertices of degree 1, one with a single outgoing blue edge and the other with a single incoming blue edge.
 * * vertices of degree 3, each of which has an incoming blue edge, a different outgoing blue edge and a red edge.
 *
 * F(4) = 5
 * F(8) = 319
 * F(50000) = ???
 *
 * I think...
 * F(0) = 1
 * F(1) = 0
 * F(2) = 1
 * F(3) = 0
 */

struct Graph {
    // vertex id -> (directed edge to, directed edge from, undirected edge)
    // TODO: turn this into a vector of tuples. use the natural index of the vector subscript?
    //     * ^ this will really simplify generation so we are not having to check if not exists.
    // TODO: consider not using Option so that manually initing is easier to type..
    vertices: HashMap<u16, [Option<u16>; 3]>,
}

impl Graph {
    fn print(&self) {
        println!("{:?}", self.vertices);
    }

    // TODO: is_valid_graph. just to sanity check

    fn is_connected(&self, v_count: u16) -> bool {
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
        return visited.len() == v_count.into();
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
        if self.vertices.len() != (n + 2).into() {
            return false;
        }
        return self.is_connected(n + 2) && self.has_valid_edges();
    }

    fn generate(n: u16) {
        let mut g = Graph {
            vertices: HashMap::new(),
        };
        return Self::_generate(&mut g, 0, n);
    }

    // TODO: state in this traversal is messed up. we need to undo mutations or copy the graph
    // TODO: bound if we process an edge that is completely unconnected
    // TODO: track a argument used_sink. at any point, we can stop
    fn _generate(g: &mut Graph, i: u16, n: u16) {
        println!("PROCESSING i={}, n={}: {:?}", i, n, g.vertices);

        if i == n + 1 {
            println!("g: {:?}, is_solution: {:?}", g.vertices, g.is_solution(n));
            return;
        }

        if i == 0 {
            // source vertex. place a single outgoing edge
            g.vertices.insert(i, [Some(i + 1), None, None]);
            g.vertices.insert(i + 1, [None, Some(i), None]);

            Self::_generate(g, i + 1, n);
        } else {
            if g.vertices[&i][1].is_none() {
                println!("A");
                return;
            }

            // normal edge. place a outgoing edge and an undirected edge if does not exist.
            // lot of redundant isomorphic graphs here. should be able to restrict branching here
            for j in 1..n + 2 {
                // directed edge
                if i == j {
                    continue;
                }
                if g.vertices[&j][1].is_none() {
                    for k in 1..n + 2 {
                        // undirected edge
                        if i == k {
                            continue;
                        }

                        if g.vertices[&k][2].is_none() {
                            // update this vertex
                            *g.vertices.get_mut(&i).unwrap() =
                                [Some(j), g.vertices[&i][1], Some(k)];
                            // update the other side of the outgoing directed edge and undirected edge
                            *g.vertices.get_mut(&j).unwrap() =
                                [g.vertices[&j][0], Some(i), g.vertices[&j][2]];
                            *g.vertices.get_mut(&k).unwrap() =
                                [g.vertices[&k][0], g.vertices[&k][1], Some(i)];

                            Self::_generate(g, i + 1, n);
                        }
                    }
                }
            }
            println!("normal return");
        }

        return;
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
        println!(
            "is_solution : {}",
            g.is_solution((g.vertices.len() - 2) as u16) // janky
        );
    }

    // test generation
    println!("\n\n");
    println!("0 ///////////////////////////////////////////////////////////////////////////////////////////");
    Graph::generate(0);
    println!("1 ///////////////////////////////////////////////////////////////////////////////////////////");
    Graph::generate(1);
    println!("2 ///////////////////////////////////////////////////////////////////////////////////////////");
    Graph::generate(2);
}

/*
Ideas:
* [x] correctness check: connected, right number of edges, right edges
* [ ] permutation generation
*     [ ] we know how many edges there will be. n = 4 means 6 vertices, 5 blue edges, 2 red edges. limit permutations?
* [ ] backtracking
* [ ] isomorphic graphs. no redundant shapes
*/
