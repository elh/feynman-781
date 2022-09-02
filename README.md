# Feynman 781 ⚛️

A simple generator of "Feynman diagram" permutations (as defined [Project Euler problem 781](https://projecteuler.net/problem=781)). Incrementally builds candidate graphs in a compact adjacency representation with backtracking and no isomorphic duplicates.

Implemented quickly to play around with performance improvements, graph traversals, and to try out Rust for the first time. 🦀

### Problem 781?

This is not a solution for the problem, but you may find the enumeration and visualization of small N solutions useful for your own thinking. Good luck.

This does not leverage any simplification needed to directly calculate the goal F(50000), but actually generates graph permutation and counts them. This is clearly not scalable, but is fun! The hope was to first generate data to study, then maybe find inspiration for a solution later...

> Let F(n) be the number of connected graphs with blue edges (directed) and red edges (undirected) containing:
> * two vertices of degree 1, one with a single outgoing blue edge and the other with a single incoming blue edge.
> * vertices of degree 3, each of which has an incoming blue edge, a different outgoing blue edge and a red edge.

_(example where N = 8)_
<p>
    <img width="200px" alt="example graph" src="https://user-images.githubusercontent.com/1035393/188107684-5556d282-7403-48ce-be82-849cfed8a9eb.png">
</p>
