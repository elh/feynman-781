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
    //     * ^ will also make printing of them stable.
    //     * ^ more amenable to simple backtracking
    // TODO: later improve perf by making this a multi-dimensional array for data locality. N needs to be a const though
    // TODO: consider not using Option so that manually initing is easier to type..
    // pre-allocate all top level vectors. this is in line with the final array soln and simplifies some ops.
    vertices: Vec<[Option<u16>; 3]>,
}

// TODO: scrutinize heavy use of `into`

const DEBUG: bool = false;

impl Graph {
    fn is_valid(&self) -> bool {
        for (i, v) in self.vertices.iter().enumerate() {
            if v[0].is_some() {
                let v0: usize = v[0].unwrap().into();
                if self.vertices[v0][1].is_none() || usize::from(self.vertices[v0][1].unwrap()) != i
                {
                    return false;
                }
            }
            if v[1].is_some() {
                let v1: usize = v[1].unwrap().into();
                if self.vertices[v1][0].is_none() || usize::from(self.vertices[v1][0].unwrap()) != i
                {
                    return false;
                }
            }
            if v[2].is_some() {
                let v2: usize = v[2].unwrap().into();
                if self.vertices[v2][2].is_none() || usize::from(self.vertices[v2][2].unwrap()) != i
                {
                    return false;
                }
            }
        }
        return true;
    }

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
            for i in self.vertices[cur] {
                if i.is_none() {
                    continue;
                }
                let i_some = i.unwrap().into();
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

        for v in &self.vertices {
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
        if n % 2 == 1 {
            return;
        }

        let n_: usize = n.into();
        let mut g = Graph {
            vertices: vec![[None; 3]; n_ + 2],
        };
        return Self::_generate(&mut g, 0, n);
    }

    // TODO: bound if we process an edge that is completely unconnected
    // TODO: track a argument used_sink. at any point, we can stop
    fn _generate(g: &mut Graph, i: u16, n: u16) {
        if DEBUG {
            println!("PROCESSING i={}, n={}:\t\t\t\t{:?}", i, n, g.vertices);
            println!("{}", g.to_graphviz());
        }

        if i == n + 1 {
            if g.is_solution(n) {
                println!(
                    "g: is_valid: {}, is_solution: {:?}!\t\t{:?}",
                    g.is_valid(),
                    true,
                    g.vertices
                );
                println!("{}", g.to_graphviz());
            } else if DEBUG {
                println!(
                    "g: is_valid: {}, is_solution: {:?}!\t\t{:?}",
                    g.is_valid(),
                    false,
                    g.vertices
                );
            }
            return;
        }
        let i_: usize = i.into();

        if i == 0 {
            // source vertex. place a single outgoing edge
            g.vertices[i_] = [Some(i + 1), None, None];
            g.vertices[i_ + 1] = [None, Some(i), None];

            Self::_generate(g, i + 1, n);
        } else {
            if !g.vertices.len() <= i_ {
                if DEBUG {
                    println!("unconnected vertex. abort");
                }
                return;
            }

            // normal edge. place a outgoing edge and an undirected edge if does not exist.
            // TODO: lot of redundant isomorphic graphs here. should be able to restrict branching here. only try connecting the
            // very next free vertex. rely on stable order of trying directed and then undirected next
            let mut used_unconnected_j_vertex = false;
            for j in 1..n + 2 {
                // directed edge
                if i == j {
                    continue;
                }
                let j_: usize = j.into();

                if g.vertices[j_][1].is_none() {
                    if g.vertices[j_][0].is_none()
                        && g.vertices[j_][1].is_none()
                        && g.vertices[j_][2].is_none()
                    {
                        if used_unconnected_j_vertex {
                            if DEBUG {
                                println!("breaking because of unconnected j {}", j);
                            }
                            break;
                        }
                        used_unconnected_j_vertex = true;
                        if DEBUG {
                            println!("saw unconnected j {}", j)
                        }
                    }

                    // TODO: clean this up. no need to add undirected edge if we already have one. compicates the prior iteration a bit.
                    if g.vertices[i_][2].is_none() {
                        let mut used_unconnected_k_vertex = false;
                        for k in 1..n + 2 {
                            // undirected edge
                            // TODO: handle case where we decide this should be the sink
                            if i == k {
                                continue;
                            }
                            let k_: usize = k.into();

                            if g.vertices[k_][2].is_none() {
                                if k != j
                                    && g.vertices[k_][0].is_none()
                                    && g.vertices[k_][1].is_none()
                                    && g.vertices[k_][2].is_none()
                                {
                                    if used_unconnected_k_vertex {
                                        if DEBUG {
                                            println!("breaking because of unconnected k {}", k);
                                        }
                                        break;
                                    }
                                    used_unconnected_k_vertex = true;
                                    if DEBUG {
                                        println!("saw unconnected k {}", k)
                                    }
                                }

                                let old_i = g.vertices[i_];
                                let old_j = g.vertices[j_];
                                let old_k = g.vertices[k_];

                                // TODO: consider assigning cell by cell?
                                // update this vertex
                                g.vertices[i_] = [Some(j), g.vertices[i_][1], Some(k)];
                                // update the other side of the outgoing directed edge and undirected edge
                                g.vertices[j_] = [g.vertices[j_][0], Some(i), g.vertices[j_][2]];
                                g.vertices[k_] = [g.vertices[k_][0], g.vertices[k_][1], Some(i)];

                                Self::_generate(g, i + 1, n);

                                // TODO: backtrack w/ a ton of state...
                                g.vertices[i_] = old_i;
                                g.vertices[j_] = old_j;
                                g.vertices[k_] = old_k;
                            }
                        }
                    } else {
                        let old_i = g.vertices[i_];
                        let old_j = g.vertices[j_];

                        // TODO: consider assigning cell by cell?
                        // update this vertex
                        g.vertices[i_] = [Some(j), g.vertices[i_][1], g.vertices[i_][2]];
                        // update the other side of the outgoing directed edge
                        g.vertices[j_] = [g.vertices[j_][0], Some(i), g.vertices[j_][2]];

                        Self::_generate(g, i + 1, n);

                        // TODO: backtrack w/ a ton of state...
                        g.vertices[i_] = old_i;
                        g.vertices[j_] = old_j;
                    }
                }
            }
            if DEBUG {
                println!("iteration ended. back to:\t\t\t{:?}", g.vertices);
            }
        }

        return;
    }

    fn to_graphviz(&self) -> String {
        let mut str = "digraph G {
\tedge [color=blue]"
            .to_owned();
        for (i, v) in self.vertices.iter().enumerate() {
            if v[0].is_some() {
                str.push_str(&format!("\n\t{} -> {};", i, v[0].unwrap()));
            }
            if v[2].is_some() {
                if usize::from(v[2].unwrap()) > i {
                    str.push_str(&format!(
                        "\n\t{} -> {} [dir=none, color=red];",
                        i,
                        v[2].unwrap()
                    ));
                }
            }
        }
        str.push_str("\n}");
        return str;
    }
}

fn main() {
    // // currently testing the solutions given in the example problem
    // let gs: Vec<Graph> = vec![
    //     // where M = 0
    //     Graph {
    //         vertices: vec![[Some(1), None, None], [None, Some(0), None]],
    //     },
    //     // where M = 2
    //     Graph {
    //         vertices: vec![
    //             [Some(1), None, None],
    //             [Some(2), Some(0), Some(2)],
    //             [Some(3), Some(1), Some(1)],
    //             [None, Some(2), None],
    //         ],
    //     },
    //     // where M = 4
    //     Graph {
    //         vertices: vec![
    //             [Some(1), None, None],
    //             [Some(2), Some(0), Some(2)],
    //             [Some(3), Some(1), Some(1)],
    //             [Some(4), Some(2), Some(4)],
    //             [Some(5), Some(3), Some(3)],
    //             [None, Some(4), None],
    //         ],
    //     },
    //     Graph {
    //         vertices: vec![
    //             [Some(1), None, None],
    //             [Some(2), Some(0), Some(3)],
    //             [Some(3), Some(1), Some(4)],
    //             [Some(4), Some(2), Some(1)],
    //             [Some(5), Some(3), Some(2)],
    //             [None, Some(4), None],
    //         ],
    //     },
    //     Graph {
    //         vertices: vec![
    //             [Some(1), None, None],
    //             [Some(2), Some(0), Some(4)],
    //             [Some(3), Some(1), Some(3)],
    //             [Some(4), Some(2), Some(2)],
    //             [Some(5), Some(3), Some(1)],
    //             [None, Some(4), None],
    //         ],
    //     },
    //     Graph {
    //         vertices: vec![
    //             [Some(1), None, None],
    //             [Some(2), Some(0), Some(3)],
    //             [None, Some(1), None],
    //             [Some(4), Some(5), Some(1)],
    //             [Some(5), Some(3), Some(5)],
    //             [Some(3), Some(4), Some(4)],
    //         ],
    //     },
    //     Graph {
    //         vertices: vec![
    //             [Some(1), None, None],
    //             [Some(2), Some(0), Some(4)],
    //             [Some(3), Some(1), Some(5)],
    //             [None, Some(2), None],
    //             [Some(5), Some(5), Some(1)],
    //             [Some(4), Some(4), Some(2)],
    //         ],
    //     },
    // ];

    // for g in gs {
    //     let n = g.vertices.len() - 2;
    //     println!(
    //         "g(n={}) is_valid: {}, is_solution: {:?}!\t{:?}",
    //         n,
    //         g.is_valid(),
    //         g.is_solution(n as u16),
    //         g.vertices
    //     );
    // }

    // test generation
    use std::time::Instant;
    let now = Instant::now();
    println!("0 ///////////////////////////////////////////////////////////////////////////////////////////");
    Graph::generate(0);
    println!("1 ///////////////////////////////////////////////////////////////////////////////////////////");
    Graph::generate(1);
    println!("2 ///////////////////////////////////////////////////////////////////////////////////////////");
    Graph::generate(2);
    println!("3 ///////////////////////////////////////////////////////////////////////////////////////////");
    Graph::generate(3);
    println!("4 ///////////////////////////////////////////////////////////////////////////////////////////");
    Graph::generate(4);
    // println!("5 ///////////////////////////////////////////////////////////////////////////////////////////");
    // Graph::generate(5);
    // println!("6 ///////////////////////////////////////////////////////////////////////////////////////////");
    // Graph::generate(6);
    // println!("7 ///////////////////////////////////////////////////////////////////////////////////////////");
    // Graph::generate(7);
    // println!("8 ///////////////////////////////////////////////////////////////////////////////////////////");
    // Graph::generate(8);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // first timings:
    // n=4 160.95µs
    // n=6 3.10ms
    // n=8 145.28ms
    // n=10 10.36s oh boy...
}

/*
Ideas:
* [x] correctness check: connected, right number of edges, right edges
* [ ] permutation generation
*     [ ] we know how many edges there will be. n = 4 means 6 vertices, 5 blue edges, 2 red edges. limit permutations?
* [ ] backtracking
* [ ] isomorphic graphs. no redundant shapes
* [ ] parallelize?
*/
