/* Let F(n) be the number of connected graphs with blue edges (directed) and red edges (undirected) containing:
 * * two vertices of degree 1, one with a single outgoing blue edge and the other with a single incoming blue edge.
 * * vertices of degree 3, each of which has an incoming blue edge, a different outgoing blue edge and a red edge.
 *
 * F(4) = 5
 * F(8) = 319
 * F(50000) = ?
 */

// TODO: consider making vertices a multi-dimensional array for data locality. N needs to be a const though
// TODO: consider not using Option uint but a signed int and -1 as a sentinel value
// TODO: scrutinize profligate type conversions
// TODO: support parallelism?
pub struct Graph {
    // index represents vertex id
    // values are a tuple of (directed edge to, directed edge from, undirected edge to) vertex ids
    vertices: Vec<[Option<u16>; 3]>,
}

const DEBUG: bool = false;
const DEBUG_GRAPHVIZ: bool = false;
const PRINT_SOLUTIONS: bool = false;
const PRINT_SOLUTIONS_GRAPHVIZ: bool = false;

impl Graph {
    // generate the count of all unique Feynman diagrams for given n. If debug flags are set, print found results.
    pub fn generate(n: u16) -> u64 {
        if n % 2 == 1 {
            return 0;
        }

        let n_: usize = n.into();
        let mut g = Graph {
            vertices: vec![[None; 3]; n_ + 2],
        };
        let mut count: u64 = 0;
        Self::_generate(&mut g, 0, false, &mut count, n);
        return count;
    }

    fn _generate(g: &mut Graph, i: u16, used_sink: bool, count: &mut u64, n: u16) {
        if DEBUG {
            println!("PROCESSING i={}, n={}:\t\t\t\t{:?}", i, n, g.vertices);
            if DEBUG_GRAPHVIZ {
                println!("{}", g.to_graphviz());
            }
        }

        if i == n + 2 {
            // end reached.
            // we never generate unconnected graphs or graphs with incorrect edges. no check is needed.
            if PRINT_SOLUTIONS {
                println!("solution found:\t{:?}", g.vertices);
                if PRINT_SOLUTIONS_GRAPHVIZ {
                    println!("{}", g.to_graphviz());
                }
            }
            *count += 1;
            return;
        }
        let i_: usize = i.into();

        if i == 0 {
            // source vertex. place a single outgoing edge.
            g.vertices[i_][0] = Some(i + 1);
            g.vertices[i_ + 1][1] = Some(i);

            Self::_generate(g, i + 1, used_sink, count, n);
        } else {
            // if vertex is not connected, it will create an unconnected graph. abort.
            if g.vertices[i_][0].is_none()
                && g.vertices[i_][1].is_none()
                && g.vertices[i_][2].is_none()
            {
                return;
            }

            // place an outgoing edge and place an undirected edge if does not exist, then recurse.
            // restrict branching by only trying to connect the very next free vertex. rely on stable order of trying
            // directed and then undirected next.
            // TODO: avoid iteration of j and k by tracking remaining candidates explicitly?
            let mut used_unconnected_j_vertex = false;
            // directed edges can connect to previously seen vertices, unseen but connected vertices, or a single new
            // unconnected vertex.
            for j in 1..n + 2 {
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
                            break;
                        }
                        used_unconnected_j_vertex = true;
                    }

                    if g.vertices[i_][2].is_none() {
                        let mut used_unconnected_k_vertex = false;
                        // undirected edges can only connect to unseen but connected vertices or single new unconnected
                        // vertex.
                        for k in i + 1..n + 2 {
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
                                        break;
                                    }
                                    used_unconnected_k_vertex = true;
                                }

                                // update this vertex
                                // update the other side of the outgoing directed edge and undirected edge
                                g.vertices[i_][0] = Some(j);
                                g.vertices[i_][2] = Some(k);
                                g.vertices[j_][1] = Some(i);
                                g.vertices[k_][2] = Some(i);

                                // recurse and backtrack
                                Self::_generate(g, i + 1, used_sink, count, n);

                                g.vertices[i_][0] = None;
                                g.vertices[i_][2] = None;
                                g.vertices[j_][1] = None;
                                g.vertices[k_][2] = None;
                            }
                        }
                    } else {
                        // update this vertex
                        // update the other side of the outgoing directed edge
                        g.vertices[i_][0] = Some(j);
                        g.vertices[j_][1] = Some(i);

                        // recurse and backtrack
                        Self::_generate(g, i + 1, used_sink, count, n);

                        g.vertices[i_][0] = None;
                        g.vertices[j_][1] = None;
                    }
                }
            }

            // treat vertex as the sink and recurse.
            if !used_sink && i_ > 1 && g.vertices[i_][1].is_some() && g.vertices[i_][2].is_none() {
                Self::_generate(g, i + 1, true, count, n);
            }
        }

        return;
    }

    // produce graphviz dot file for visualization.
    pub fn to_graphviz(&self) -> String {
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