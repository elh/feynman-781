mod graph;

// prints a csv header for timed runs
fn timed_header() {
    println!("case\tn\tresult\tdur_ms\tdur_pretty");
}

// prints a csv line for timed run
fn timed_generate<F>(func: F, n: u16)
where
    F: Fn(u16) -> u64,
{
    use std::time::Instant;
    let now = Instant::now();
    let res = func(n);
    let elapsed = now.elapsed();
    println!(
        "F({})\t{}\t{}\t{}\t{:.2?}",
        n,
        n,
        res,
        elapsed.as_millis(),
        elapsed
    );
}

fn main() {
    // generation
    timed_header();
    timed_generate(graph::Graph::generate, 0);
    timed_generate(graph::Graph::generate, 2);
    timed_generate(graph::Graph::generate, 4);
    timed_generate(graph::Graph::generate, 6);
    timed_generate(graph::Graph::generate, 8);
    timed_generate(graph::Graph::generate, 10);
    timed_generate(graph::Graph::generate, 12);
    timed_generate(graph::Graph::generate, 14);
    timed_generate(graph::Graph::generate, 16);
}
