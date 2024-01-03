use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

#[derive(Debug, Clone)]
struct Graph<'a> {
    edges: HashMap<&'a str, HashMap<&'a str, usize>>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: &'a str, v: &'a str, w: usize) {
        self.edges.entry(u).or_default().insert(v, w);
        self.edges.entry(v).or_default().insert(u, w);
    }

    fn update_edge(&mut self, s: &'a str, t: &'a str, w: usize) {
        *self.edges.entry(s).or_default().entry(t).or_insert(0) += w;
        *self.edges.entry(t).or_default().entry(s).or_insert(0) += w;
    }

    fn remove(&mut self, node: &'a str) {
        self.edges.remove(node);

        self.edges.iter_mut().for_each(|(_, edges)| {
            edges.remove(node);
        });
    }
}

fn minimum_cut_phase<'a>(graph: &Graph<'a>) -> (&'a str, &'a str, usize) {
    let mut queue = PriorityQueue::new();

    graph.edges.keys().for_each(|node| {
        queue.push(node, 0);
    });

    let mut cut_weight = 0;
    let mut s = "";
    let mut t = "";

    while let Some((node, weight)) = queue.pop() {
        s = t;
        t = node;
        cut_weight = weight;

        for (e, w) in graph.edges.get(node).unwrap().iter() {
            queue.change_priority_by(e, |current_weight| *current_weight += w);
        }
    }

    (s, t, cut_weight)
}

// https://dl.acm.org/doi/pdf/10.1145/263867.263872
fn stoer_wagner<'a>(graph: &'a Graph) -> usize {
    let mut contracted_graph = graph.clone();

    let mut best_iteration = 0;
    let mut min_cut_weight = usize::MAX;
    let mut contractions = Vec::new();
    println!("initial size: {}", contracted_graph.edges.len());

    for i in 0..graph.edges.keys().count() - 1 {
        let (s, t, cut_weight) = minimum_cut_phase(&contracted_graph);



        contractions.push((s, t));

        if let Some(edges) = contracted_graph.edges.get(t) {
            for (e, w) in edges.clone() {
                contracted_graph.update_edge(s, e, w);
            }
        }

        contracted_graph.remove(t);

        if cut_weight < min_cut_weight {
            best_iteration = i;
            min_cut_weight = cut_weight;
        }
    }

    let mut split_graph: HashMap<&'a str, HashSet<&'a str>> = HashMap::new();
    for (s, t) in contractions.iter().take(best_iteration) {
        split_graph
            .entry(*s)
            .or_insert_with(HashSet::new)
            .insert(*t);
        split_graph
            .entry(*t)
            .or_insert_with(HashSet::new)
            .insert(*s);
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(contractions[best_iteration].1);

    while let Some(node) = queue.pop_front() {
        if visited.contains(node) {
            continue;
        }

        visited.insert(node);
        if let Some(edges) = split_graph.get(node) {
            for edge in edges {
                queue.push_back(*edge);
            }
        }
    }

    visited.len() * (graph.edges.len() - visited.len())
}

fn main() {
    let input = include_str!("../input.txt");

    let mut graph: Graph = Graph::new();
    for line in input.lines() {
        let (label, connections_str) = line.split_once(": ").unwrap();
        for connection in connections_str.split_whitespace() {
            graph.add_edge(label, connection, 1);
        }
    }
    let start = Instant::now();
    println!("Solution: {}", stoer_wagner(&graph));
    let end = start.elapsed();
    println!("Took: {} ms", end.as_millis());
}
