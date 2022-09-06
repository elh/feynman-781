#![feature(test)]
extern crate test;

use test::Bencher;
mod graph;

// benchmark Graph::generate with N from 0 to 12
#[bench]
fn benchmark_generate(b: &mut Bencher) {
    b.iter(|| {
        for x in (0..13).step_by(2) {
            graph::Graph::generate(x);
        }
    })
}
