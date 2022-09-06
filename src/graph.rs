use std::fmt::Write;

/* Let F(n) be the number of connected graphs with blue edges (directed) and red edges (undirected) containing:
 * * two vertices of degree 1, one with a single outgoing blue edge and the other with a single incoming blue edge.
 * * vertices of degree 3, each of which has an incoming blue edge, a different outgoing blue edge and a red edge.
 *
 * F(4) = 5
 * F(8) = 319
 * F(50000) = ?
 */

// This defines the size of the underlying vertex array. Raise this limit to call generate(n) with larger n. This incurs
// a performance and memory overhead for small N but allows us to get sequential memory allocation.
const MAX_N: usize = 16;

// Print debugging information via Cargo features. See Cargo.toml.

#[derive(Default, Clone)]
pub struct Graph {
    // index represents vertex id
    // values are a tuple of (directed edge to, directed edge from, undirected edge to) vertex ids
    vertices: [[Option<u16>; 3]; MAX_N + 2],
}

impl Graph {
    // generate the count of all unique Feynman diagrams for given n. If debug flags are set, print found results.
    pub fn generate(n: u16) -> u64 {
        if n % 2 == 1 {
            return 0;
        }
        if usize::from(n) > MAX_N {
            panic!(
                "n ({}) cannot be greater than configured MAX_N ({})",
                n, MAX_N
            );
        }
        let mut g = Graph::default();
        let mut count: u64 = 0;
        Self::_generate(n, &mut g, &mut count, 0, false, 2, 2);
        count
    }

    // count is a mutable ref to a counter int of all found graphs so far
    // i is the vertex index to process in this call
    // used_sink tracks if we have already treated a prior vertex as the sink
    // j_0 is a cursor tracking the first vertex we need to consider for the directed edge
    // k_0 is a cursor tracking the first vertex we need to consider for the undirected edge
    // NOTE: j and k iteration could be optimized further but this is nice and simple
    fn _generate(
        n: u16,
        g: &mut Graph,
        count: &mut u64,
        i: u16,
        used_sink: bool,
        mut j_0: usize,
        mut k_0: usize,
    ) {
        if cfg!(feature = "debug") {
            println!("PROCESSING i={}, n={}:\t\t\t\t{:?}", i, n, g.vertices);
            if cfg!(feature = "debug-graphviz") {
                println!("{}", g.to_graphviz());
            }
        }

        if i == n + 2 {
            // end reached.
            if cfg!(feature = "print-solutions") {
                println!("solution found:\t{:?}", g.vertices);
                if cfg!(feature = "print-solutions-graphviz") {
                    println!("{}", g.to_graphviz());
                }
            }
            *count += 1;
            return;
        }
        let i_: usize = usize::from(i);

        if i == 0 {
            // source vertex. place a single outgoing edge.
            g.vertices[i_][0] = Some(i + 1);
            g.vertices[i_ + 1][1] = Some(i);

            Self::_generate(n, g, count, i + 1, used_sink, j_0, k_0);
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
            let mut used_unconnected_j_vertex = false;
            // directed edges can connect to previously seen vertices, unseen but connected vertices, or a single new
            // unconnected vertex.
            for j in (j_0 as u16)..n + 2 {
                if i == j {
                    continue;
                }
                let j_: usize = usize::from(j);

                if g.vertices[j_][1].is_some() {
                    // scoot the j_0 cursor over
                    if j_ == j_0 {
                        j_0 += 1;
                    }
                    continue;
                }

                if g.vertices[j_][0].is_none() && g.vertices[j_][2].is_none() {
                    if used_unconnected_j_vertex {
                        break;
                    }
                    used_unconnected_j_vertex = true;
                }

                if g.vertices[i_][2].is_none() {
                    let mut used_unconnected_k_vertex = false;
                    // start from the greater of the k_0 cursor or the next vertex
                    if i_ + 1 > k_0 {
                        k_0 = i_ + 1
                    }
                    for k in (k_0 as u16)..n + 2 {
                        if i == k {
                            continue;
                        }
                        let k_: usize = usize::from(k);

                        if g.vertices[k_][2].is_some() {
                            // scoot the k_0 cursor over
                            if k_ == k_0 {
                                k_0 += 1;
                            }
                            continue;
                        }

                        if k != j && g.vertices[k_][0].is_none() && g.vertices[k_][1].is_none() {
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
                        Self::_generate(n, g, count, i + 1, used_sink, j_0, k_0);

                        g.vertices[i_][0] = None;
                        g.vertices[i_][2] = None;
                        g.vertices[j_][1] = None;
                        g.vertices[k_][2] = None;
                    }
                } else {
                    // update this vertex
                    // update the other side of the outgoing directed edge
                    g.vertices[i_][0] = Some(j);
                    g.vertices[j_][1] = Some(i);

                    // recurse and backtrack
                    Self::_generate(n, g, count, i + 1, used_sink, j_0, k_0);

                    g.vertices[i_][0] = None;
                    g.vertices[j_][1] = None;
                }
            }

            // treat vertex as the sink and recurse.
            if !used_sink
                && (i_ > 1 || (n == 0 && i_ > 0))
                && g.vertices[i_][1].is_some()
                && g.vertices[i_][2].is_none()
            {
                Self::_generate(n, g, count, i + 1, true, j_0, k_0);
            }
        }
    }

    // produce graphviz dot file for visualization.
    pub fn to_graphviz(&self) -> String {
        let mut str = "digraph G {
\tedge [color=blue]"
            .to_owned();
        for (i, v) in self.vertices.iter().enumerate() {
            if v[0].is_some() {
                _ = write!(str, "\n\t{} -> {};", i, v[0].unwrap());
            }
            if matches!(v[2], Some(x) if usize::from(x) > i) {
                _ = write!(str, "\n\t{} -> {} [dir=none, color=red];", i, v[2].unwrap());
            }
        }
        str.push_str("\n}");
        str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let cases: Vec<(u16, u64)> = vec![
            (0, 1),
            (2, 1),
            (4, 5),
            (6, 35),
            (8, 319),
            (10, 3559),
            (12, 46841),
            (14, 709601),
            (16, 12156445),
        ];
        for case in cases {
            let got = Graph::generate(case.0);
            assert_eq!(
                got, case.1,
                "F({}) expected {}. got {}",
                case.0, case.1, got
            );
        }
    }
}
