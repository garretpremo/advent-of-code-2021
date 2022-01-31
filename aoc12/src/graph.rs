use std::collections::{HashMap, HashSet};

pub struct Edge {
    from: String,
    to: String
}

#[allow(dead_code)]
struct Node {
    id: String,
    edges: HashSet<String>
}

pub struct Graph {
    nodes: HashMap<String, Node>,
}

impl Edge {
    pub fn new(from: String, to: String) -> Edge {
        Edge { from, to }
    }
}

impl Node {
    pub fn new(id: &String) -> Node {
        Node { id: id.clone(), edges: HashSet::new() }
    }

    fn _get_id(&self) -> &String {
        &self.id
    }

    pub fn add_edge(&mut self, id: &String) {
        self.edges.insert(id.clone());
    }

    fn is_big(id: &String) -> bool {
        id.to_uppercase() == id.clone()
    }
}

impl Graph {
    pub fn new() -> Graph {
        Graph { nodes: HashMap::new() }
    }

    pub fn count_distinct_paths(&self) -> u32 {
        let start = self.nodes.get("start").unwrap();
        let path: HashSet<String> = HashSet::from([String::from("start")]);

        let mut distinct_paths = 0;

        for edge in start.edges.iter() {
            distinct_paths += self.find_all_distinct_paths(&path, edge);
        }

        distinct_paths
    }

    fn find_all_distinct_paths(&self, path: &HashSet<String>, id: &String) -> u32 {
        if id.clone() == String::from("end") {
            return 1;
        }

        let mut path_copy = path.clone();
        path_copy.insert(id.clone());

        let mut distinct_paths = 0;

        let node = self.nodes.get(id.as_str()).unwrap();

        for edge in node.edges.iter() {
            if Node::is_big(edge) || !path.contains(edge) {
                distinct_paths += self.find_all_distinct_paths(&path_copy, edge);
            }
        }

        distinct_paths
    }

    pub fn add_edges(&mut self, edges: &Vec<Edge>) {
        edges.iter().for_each(|edge| self.add_edge(edge));
    }

    fn add_edge(&mut self, edge: &Edge) {
        if !self.has_node(&edge.from) {
            self.create_node(&edge.from);
        }

        if !self.has_node(&edge.to) {
            self.create_node(&edge.to);
        }

        self.nodes.get_mut(&edge.from).unwrap().add_edge(&edge.to);
        self.nodes.get_mut(&edge.to).unwrap().add_edge(&edge.from);
    }

    fn create_node(&mut self, id: &String) {
        self.nodes.insert(id.clone(), Node::new(id));
    }

    fn has_node(&self, id: &String) -> bool {
        self.nodes.contains_key(id)
    }
}
