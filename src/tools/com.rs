use crate::adj::adj::AdjArray;

#[derive(Debug)]
pub struct Comunity {
    pub nb_links_tot: usize,
    pub nb_links_in: usize,
    pub nodes: Vec<usize>,
}

impl Comunity {
    pub fn new(graph: &AdjArray, node: u32) -> Comunity {
        let mut node_in = 0;
        let mut node_out = 0;

        for n in graph.get_neighbors(node) {
            if n == node {
                match &graph.weigth {
                    Some(weigth) => {
                        let map = &weigth[n as usize];
                        node_in += map.get(&node).unwrap();
                    }
                    None => {
                        node_in += 1;
                    }
                }
            } else {
                match &graph.weigth {
                    Some(weigth) => {
                        let map = &weigth[n as usize];
                        node_out += map.get(&node).unwrap();
                    }
                    None => {
                        node_out += 1;
                    }
                }
            }
        }

        Comunity {
            nb_links_tot: (node_out + node_out) as usize,
            nb_links_in: node_in as usize,
            nodes: vec![node as usize],
        }
    }
    pub fn add_node(
        &mut self,
        graph: &mut AdjArray,
        node: u32,
        com_index: u32,
        com_for_nodes: &Vec<u32>,
    ) {
        let mut node_in = 0;
        let mut node_out = 0;

        for n in graph.get_neighbors(node) {
            if com_for_nodes[n as usize] == com_index {
                match &graph.weigth {
                    Some(weigth) => {
                        let map = &weigth[n as usize];
                        node_in += map.get(&node).unwrap();
                    }
                    None => {
                        node_in += 1;
                    }
                }
            } else {
                match &graph.weigth {
                    Some(weigth) => {
                        let map = &weigth[n as usize];
                        node_out += map.get(&node).unwrap();
                    }
                    None => {
                        node_out += 1;
                    }
                }
            }
        }

        self.nb_links_tot += node_out as usize;
        self.nb_links_in += node_in as usize;
        self.nodes.push(node as usize);
    }

    pub fn remove_node(
        &mut self,
        graph: &mut AdjArray,
        node: u32,
        com_index: u32,
        com_for_nodes: &Vec<u32>,
    ) {
        let mut node_in = 0;
        let mut node_out = 0;

        for n in graph.get_neighbors(node) {
            if com_for_nodes[n as usize] == com_index {
                match &graph.weigth {
                    Some(weigth) => {
                        let map = &weigth[n as usize];
                        node_in += map.get(&node).unwrap();
                    }
                    None => {
                        node_in += 1;
                    }
                }
            } else {
                match &graph.weigth {
                    Some(weigth) => {
                        let map = &weigth[n as usize];
                        node_out += map.get(&node).unwrap();
                    }
                    None => {
                        node_out += 1;
                    }
                }
            }
        }

        self.nb_links_tot -= node_out as usize;
        self.nb_links_in -= node_in as usize;

        let index = self
            .nodes
            .iter()
            .position(|r| *r as u32 == node)
            .unwrap_or_default();

        self.nodes.remove(index);
    }
}
