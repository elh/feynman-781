use std::collections::HashSet;

/* Let F(n) be the number of connected graphs with blue edges (directed) and red edges (undirected) containing:
 * * two vertices of degree 1, one with a single outgoing blue edge and the other with a single incoming blue edge.
 * * vertices of degree 3, each of which has an incoming blue edge, a different outgoing blue edge and a red edge.
 *
 * F(4) = 5
 * F(8) = 319
 * F(50000) = ???
 */

// TODO: consider making vertices a multi-dimensional array for data locality. N needs to be a const though
// TODO: consider not using Option so that manually initing is easier to type..
struct Graph {
    // index represents vertex id
    // values are a tuple of (directed edge to, directed edge from, undirected edge to) vertex ids
    vertices: Vec<[Option<u16>; 3]>,
}

const DEBUG: bool = false;
const PRINT_SOLUTIONS: bool = false;

impl Graph {
    // basic correctness sanity check for edges and their expected back pointers
    fn is_ok(&self) -> bool {
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

    // is a valid Feynman diagram graph
    fn is_solution(&self, n: u16) -> bool {
        if self.vertices.len() != (n + 2).into() {
            return false;
        }
        return self.is_connected(n + 2) && self.has_valid_edges();
    }

    // generate the count of all unique Feynman diagrams for given n. If debug flags are set, print found results.
    fn generate(n: u16) -> u64 {
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

    // TODO: consider granularity to assign into array and to store prior state for backtracking
    fn _generate(g: &mut Graph, i: u16, used_sink: bool, count: &mut u64, n: u16) {
        if DEBUG {
            println!("PROCESSING i={}, n={}:\t\t\t\t{:?}", i, n, g.vertices);
            println!("{}", g.to_graphviz());
        }

        if i == n + 2 {
            if g.is_solution(n) {
                if PRINT_SOLUTIONS {
                    println!("solution found:\t{:?}", g.vertices);
                }
                *count += 1;
            }
            return;
        }
        let i_: usize = i.into();

        if i == 0 {
            // source vertex. place a single outgoing edge
            g.vertices[i_] = [Some(i + 1), None, None];
            g.vertices[i_ + 1] = [None, Some(i), None];

            Self::_generate(g, i + 1, used_sink, count, n);
        } else {
            // place a outgoing edge and place an undirected edge if does not exist.
            // restrict branching by only try connecting the very next free vertex. rely on stable order of trying
            // directed and then undirected next
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
                            break;
                        }
                        used_unconnected_j_vertex = true;
                    }

                    if g.vertices[i_][2].is_none() {
                        let mut used_unconnected_k_vertex = false;
                        for k in 1..n + 2 {
                            // undirected edge
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

                                let old_i = g.vertices[i_];
                                let old_j = g.vertices[j_];
                                let old_k = g.vertices[k_];

                                // update this vertex
                                // update the other side of the outgoing directed edge and undirected edge
                                g.vertices[i_] = [Some(j), g.vertices[i_][1], Some(k)];
                                g.vertices[j_] = [g.vertices[j_][0], Some(i), g.vertices[j_][2]];
                                g.vertices[k_] = [g.vertices[k_][0], g.vertices[k_][1], Some(i)];

                                // recurse and backtrack
                                Self::_generate(g, i + 1, used_sink, count, n);

                                g.vertices[i_] = old_i;
                                g.vertices[j_] = old_j;
                                g.vertices[k_] = old_k;
                            }
                        }
                    } else {
                        let old_i = g.vertices[i_];
                        let old_j = g.vertices[j_];

                        // update this vertex
                        // update the other side of the outgoing directed edge
                        g.vertices[i_] = [Some(j), g.vertices[i_][1], g.vertices[i_][2]];
                        g.vertices[j_] = [g.vertices[j_][0], Some(i), g.vertices[j_][2]];

                        // recurse and backtrack
                        Self::_generate(g, i + 1, used_sink, count, n);

                        g.vertices[i_] = old_i;
                        g.vertices[j_] = old_j;
                    }
                }
            }
            // treat vertex as the sink and recurse
            if !used_sink && i_ > 1 && g.vertices[i_][1].is_some() && g.vertices[i_][2].is_none() {
                Self::_generate(g, i + 1, true, count, n);
            }
        }

        return;
    }

    // produce graphviz dot file for visualization
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

fn timed<F>(func: F)
where
    F: Fn(),
{
    use std::time::Instant;
    let now = Instant::now();
    func();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn main() {
    // generation
    timed(|| println!("F(0): {}", Graph::generate(0)));
    timed(|| println!("F(2): {}", Graph::generate(2)));
    timed(|| println!("F(4): {}", Graph::generate(4)));
    timed(|| println!("F(6): {}", Graph::generate(6)));
    timed(|| println!("F(8): {}", Graph::generate(8)));
    timed(|| println!("F(10): {}", Graph::generate(10)));
    timed(|| println!("F(12): {}", Graph::generate(12)));
}
