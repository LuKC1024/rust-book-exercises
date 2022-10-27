//! In this file, you will implement two simple algorithms.
//! The goal is to familiarize you with the basics of working with references.
//!
//! Both of these problems involve the `Vec` datatype. I would take a look the `Vec` documentation:
//! https://doc.rust-lang.org/std/vec/struct.Vec.html

/// P1a: `insort` is a function that takes a sorted vector `v`, and inserts an element `n` into `v`
/// such that `v` remains sorted.
///
/// You may assume that `v` is already sorted, and do not need to check this fact.
///
/// Run `cargo test insort` to check your answers.
pub fn insort(v: &mut Vec<i32>, n: i32) {
    let mut i = 0;
    while i < v.len() && v[i] < n {
        i += 1;
    }
    v.insert(i, n);
}

type Node = i32;

/// P1b: `connected` is a function that takes an edge-list representation `edges` of a *directed* graph
/// (i.e. edges has the form `[(&from, &to), ...]`) as well as a source `src` and destination `dst`.
/// `connected` returns true if there exists a path from `src` to `dst` in `edges`.
///
/// Note: in this graph representation, references to nodes are not e.g. indices into a vector, but actual
/// Rust references. You need to be very careful when comparing two nodes for equality. For example, in the program:
///
///    let x = 1; let y = 1;
///    assert!(&x == &y)
///
/// Then this assertion passes because Rust does an implicit dereference on equality checks. You will need
/// to use the [`std::ptr::eq`](https://doc.rust-lang.org/std/ptr/fn.eq.html) function to implement `connected`.
///
/// Run `cargo test connected` to check your answers.
pub fn connected(edges: &[(&Node, &Node)], src: &Node, dst: &Node) -> bool {
    let mut visited_nodes: Vec<&Node> = vec![src];
    fn is_visited(visited_nodes: &Vec<&Node>, node: &Node) -> bool {
        visited_nodes.iter().any(|n| std::ptr::eq(*n, node))
    }
    let mut old_size = 0;
    while old_size != visited_nodes.len() && !is_visited(&visited_nodes, dst) {
        old_size = visited_nodes.len();
        for (src0, dst0) in edges {
            if is_visited(&visited_nodes, src0) && !is_visited(&visited_nodes, dst0) {
                visited_nodes.push(dst0);
            }
        }
    }
    is_visited(&visited_nodes, dst)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insort_test() {
        let mut v = vec![1, 5, 8];

        insort(&mut v, 0);
        assert_eq!(v, vec![0, 1, 5, 8]);

        insort(&mut v, 3);
        assert_eq!(v, vec![0, 1, 3, 5, 8]);

        insort(&mut v, 9);
        assert_eq!(v, vec![0, 1, 3, 5, 8, 9]);
    }

    #[test]
    fn connected_test() {
        let nodes = vec![1, 1, 1];
        let edges = vec![(&nodes[0], &nodes[1]), (&nodes[1], &nodes[2])];
        assert!(connected(&edges, &nodes[0], &nodes[2]));
        assert!(!connected(&edges, &nodes[2], &nodes[0]))
    }
}
