mod graph;

// prints a csv header for timed runs
fn timed_header() {
    println!("case\tn\tresult\tdur_ms\tdur_pretty");
}

// prints a csv line for timed run
fn timed_generate<F>(func: F, n: u16)
where
    F: FnOnce(u16) -> u64,
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
    for x in (0..17).step_by(2) {
        timed_generate(graph::Graph::generate, x);
    }
}
