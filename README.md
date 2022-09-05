# Feynman diagram generator ⚛️

A simple generator of "Feynman diagram" permutations (as defined by [problem 781](https://projecteuler.net/problem=781)). Incrementally builds isomorphically unique graphs in a compact adjacency representation using backtracking at over 13M results a second on my 2020 Macbook.

Implemented quickly to play around with performance improvements, graph traversals, and to try out Rust for the first time. 🦀

### Problem 781?

This is NOT a solution for the problem whose math is left as an exercise for the reader. Good luck.

This does not leverage any simplification needed to directly calculate the goal F(50000), but actually generates graph permutation and counts them. This is clearly not scalable, but is fun! I was, at the time, interested in tiny chess engines and there is a similar underlying problem of rapidly generating states. The enumeration and visualization of small N graphs sparked ideas for the solution, but let's keep those to ourselves. 🤫

> Let F(n) be the number of connected graphs with blue edges (directed) and red edges (undirected) containing:
> * two vertices of degree 1, one with a single outgoing blue edge and the other with a single incoming blue edge.
> * vertices of degree 3, each of which has an incoming blue edge, a different outgoing blue edge and a red edge.

_(example where N = 4)_
<p>
    <img width="200px" alt="example graph" src="https://user-images.githubusercontent.com/1035393/188370348-79beb4cf-4c1f-42d5-91b9-cc6fd75c1dce.png">
</p>
