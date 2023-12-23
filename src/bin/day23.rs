use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let maze: Vec<Vec<u8>> = BufReader::new(File::open("input/day23.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();
    let nodes = nodes(&maze);
    let edges = edges(&maze, &nodes);
    let adj = adjacence(&edges);

    let mut distances = vec![0; nodes.len()];
    let mut stack: Vec<(usize, usize)> = adj[0].iter().map(|edge| (edge.to, edge.len)).collect();
    while let Some((n, d)) = stack.pop() {
        if d > distances[n] {
            distances[n] = d;
            for edge in &adj[n] {
                stack.push((edge.to, d + edge.len));
            }
        }
    }
    println!("Day 23 Part One: {}", distances[1]);

    let edges: Vec<Edge> = edges
        .into_iter()
        .flat_map(|edge| {
            [
                Edge {
                    from: edge.to,
                    to: edge.from,
                    len: edge.len,
                },
                edge,
            ]
        })
        .collect();
    let adj = adjacence(&edges);
    let answer = longest_path(&adj, 0, 1, &mut vec![false; nodes.len()], 0);
    println!("Day 23 Part Two: {}", answer);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    x: usize,
    y: usize,
}

struct Edge {
    from: usize,
    to: usize,
    len: usize,
}

fn nodes(maze: &[Vec<u8>]) -> Vec<Node> {
    let rows = maze.len();
    let cols = maze[0].len();
    let mut nodes = vec![
        Node { x: 0, y: 1 },
        Node {
            x: rows - 1,
            y: cols - 2,
        },
    ];
    assert_eq!(maze[nodes[0].x][nodes[0].y], b'.');
    assert_eq!(maze[nodes[1].x][nodes[1].y], b'.');
    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if maze[r][c] == b'.'
                && neighbours(r, c)
                    .into_iter()
                    .all(|(x, y, _)| maze[x][y] != b'.')
            {
                nodes.push(Node { x: r, y: c });
            }
        }
    }
    nodes
}

fn edges(maze: &[Vec<u8>], nodes: &[Node]) -> Vec<Edge> {
    let mut edges = vec![];
    for (i, node) in nodes.iter().enumerate() {
        if node.x == 0 {
            edges.push(walk(maze, nodes, i, (1, 1)));
        } else if node.x != maze.len() - 1 {
            for (x, y, c) in neighbours(node.x, node.y) {
                if maze[x][y] == c || maze[x][y] == b'.' {
                    edges.push(walk(maze, nodes, i, (x, y)));
                }
            }
        }
    }
    edges
}

fn adjacence(edges: &[Edge]) -> Vec<Vec<&Edge>> {
    let mut adj: Vec<Vec<&Edge>> = vec![];
    for edge in edges {
        if edge.from >= adj.len() {
            adj.resize(edge.from + 1, vec![]);
        }
        adj[edge.from].push(edge);
    }
    adj
}

fn walk(maze: &[Vec<u8>], nodes: &[Node], node: usize, start: (usize, usize)) -> Edge {
    let mut d = 1;
    let mut prev = (nodes[node].x, nodes[node].y);
    let mut curr = start;
    let nodes: HashMap<(usize, usize), usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| ((n.x, n.y), i))
        .collect();
    while !nodes.contains_key(&curr) {
        let mut iter = neighbours(curr.0, curr.1)
            .into_iter()
            .filter(|&(x, y, _)| (x, y) != prev && maze[x][y] != b'#');
        let (x, y, _) = iter.next().unwrap();
        assert!(iter.next().is_none());
        d += 1;
        prev = curr;
        curr = (x, y);
    }
    Edge {
        from: node,
        to: nodes[&curr],
        len: d,
    }
}

fn longest_path(
    adj: &[Vec<&Edge>],
    from: usize,
    to: usize,
    visited: &mut [bool],
    len: usize,
) -> usize {
    let mut result = 0;
    visited[from] = true;
    for edge in &adj[from] {
        let len = len + edge.len;
        if edge.to == to {
            result = result.max(len);
        } else if !visited[edge.to] {
            let l = longest_path(adj, edge.to, to, visited, len);
            result = result.max(l);
        }
    }
    visited[from] = false;
    result
}

fn neighbours(x: usize, y: usize) -> [(usize, usize, u8); 4] {
    [
        (x - 1, y, b'^'),
        (x + 1, y, b'v'),
        (x, y - 1, b'<'),
        (x, y + 1, b'>'),
    ]
}
