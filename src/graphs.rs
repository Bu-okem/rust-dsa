use std::collections::{HashMap, HashSet, VecDeque};

struct Graph {
    node: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            node: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, directed: bool) {
        self.node.entry(from).or_insert(Vec::new()).push(to);

        if !directed {
            self.node.entry(to).or_insert(Vec::new()).push(from);
        }
    }

    fn get_neighbours(&self, vertex: usize) -> Vec<usize> {
        match self.node.get(&vertex) {
            Some(v) => v.to_vec(),
            None => vec![],
        }
    }

    fn has_edge(&self, from: usize, to: usize) -> bool {
        match self.node.get(&from) {
            Some(v) => v.contains(&to),
            None => false,
        }
    }

    fn bfs(&self, start_node: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        visited.insert(start_node);
        queue.push_back(start_node);

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            result.push(node);
            for n in self.get_neighbours(node) {
                if !visited.contains(&n) {
                    visited.insert(n);
                    queue.push_back(n);
                }
            }
        }

        result
    }

    fn dfs(&self, start_node: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        let mut result = Vec::new();

        stack.push(start_node);

        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            if !visited.contains(&node) {
                visited.insert(node);
                result.push(node);
                for n in self.get_neighbours(node) {
                    stack.push(n);
                }
            }
        }

        result
    }
}
