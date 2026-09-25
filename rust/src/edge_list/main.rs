fn main() {
    let edges: Vec<(usize, usize)> = vec![(0, 1), (0, 2), (1, 2), (2, 3), (3, 1)];

    let vertex: usize = 1;
    let mut neighbors: Vec<usize> = Vec::new();

    for (src, dest) in edges {
        if src == vertex {
            neighbors.push(dest);
        }
    }

    println!("{:?}", neighbors)
}
