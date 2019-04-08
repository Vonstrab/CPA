use crate::adj::adj::AdjArray;
use crate::tools::com::Comunity;
use crate::tools::utils;

extern crate hashbrown;
use hashbrown::HashMap;

pub fn nb_communities(graph: &mut AdjArray) -> u32 {
    let labels = graph.labels();
    let mut comunities: Vec<u32> = Vec::new();
    for lb in labels {
        if !comunities.contains(&lb) {
            comunities.push(lb);
        }
    }

    comunities.len() as u32
}

pub fn louvain(graph: &mut AdjArray) -> u32 {
    let coms = graph.louvain();

    coms.len() as u32
}

impl AdjArray {
    pub fn from_coms(
        graph: &self::AdjArray,
        com: &Vec<Comunity>,
        com_for_nodes: &Vec<u32>,
    ) -> AdjArray {
        let number_nodes = com.len();
        let mut number_edges = 0;
        let mut adj: Vec<Vec<u32>> = Vec::with_capacity(number_nodes);
        let mut weigth: Vec<HashMap<u32, u32>> = Vec::with_capacity(number_nodes);

        for _ in 0..number_nodes {
            adj.push(Vec::new());
            weigth.push(HashMap::new());
        }

        for (i, c) in com.iter().enumerate() {
            for node in &c.nodes {
                for voisins in graph.get_neighbors(*node as u32) {
                    let v = com_for_nodes[voisins as usize] as usize;
                    if v != i {
                        if !adj[i].contains(&(v as u32)) {
                            adj[v].push(i as u32);
                            adj[i].push(v as u32);
                            weigth[v].insert(i as u32, 1);
                            weigth[i].insert(v as u32, 1);
                            number_edges += 1;
                        } else {
                            let weigth_c = weigth.clone();
                            let value = weigth_c[i].get(&(v as u32)).unwrap();
                            weigth[i].insert(v as u32, value + 1);
                            let weigth_c = weigth.clone();
                            let value = weigth_c[v].get(&(i as u32)).unwrap();
                            weigth[v].insert(i as u32, value + 1);
                        }
                    } else {
                        if adj[i].contains(&(i as u32)) {
                            let weigth_c = weigth.clone();
                            let value = weigth_c[i].get(&(i as u32)).unwrap();
                            weigth[i].insert(i as u32, value + 1);
                        } else {
                            adj[i].push(i as u32);
                            weigth[i].insert(i as u32, 1);
                            number_edges += 1;
                        }
                    }
                }
            }
        }

        AdjArray {
            number_nodes: number_nodes as u32,
            number_edges,
            adj,
            is_sorted: false,
            is_direct: false,

            weigth: Some(weigth),
        }
    }

    pub fn labels(&mut self) -> Vec<u32> {
        fn h_freq(graph: &self::AdjArray, node: u32, freq: &Vec<u32>) -> u32 {
            let mut key: Vec<u32> = Vec::new();
            let mut value: Vec<u32> = Vec::new();
            let mut max = 0;

            for v in graph.get_neighbors(node) {
                let label = freq[v as usize];
                if key.contains(&label) {
                    let mut indice = 0;
                    for k in 0..key.len() {
                        if key[k] == label {
                            indice = k;
                            break;
                        }
                    }
                    value[indice as usize] += 1;
                    if value[max] < value[indice as usize] {
                        max = indice as usize
                    }
                } else {
                    key.push(label);
                    value.push(1);
                }
            }

            key[max] as u32
        }

        let mut labels: Vec<u32> = (0..self.number_nodes).collect();

        let mut cond = true;

        let mut nodes: Vec<u32> = (0..self.number_nodes).collect();

        while cond {
            utils::shuffle_fisher_yayes(&mut nodes);

            for n in &nodes {
                if self.get_neighbors(*n).is_empty() {
                    continue;
                }
                labels[*n as usize] = h_freq(&self, *n, &labels);
            }

            cond = false;

            for n in &nodes {
                if self.get_neighbors(*n).is_empty() {
                    continue;
                }
                if labels[*n as usize] != h_freq(&self, *n, &labels) {
                    cond = true;
                    break;
                }
            }
        }
        labels
    }

    pub fn louvain(&mut self) -> Vec<Comunity> {
        fn deltaq(
            graph: &mut self::AdjArray,
            node: u32,
            test_com: u32,
            com_for_nodes: &Vec<u32>,
            coms: &mut Vec<Comunity>,
        ) -> f64 {
            let icom = com_for_nodes[node as usize];
            let som_in: u64 = coms[icom as usize].nb_links_in as u64;
            let som_tot: u64 = coms[icom as usize].nb_links_tot as u64;
            let mut ki: u64 = 0;
            let mut kiin: u64 = 0;
            let m: f64 = graph.number_edges as f64;

            for n in graph.get_neighbors(node) {
                if com_for_nodes[n as usize] == test_com {
                    match &graph.weigth {
                        Some(weigth) => {
                            let map = &weigth[n as usize];
                            ki += *map.get(&node).unwrap() as u64;
                            kiin += *map.get(&node).unwrap() as u64;
                        }
                        None => {
                            ki += 1;
                            kiin += 1;
                        }
                    }
                } else {
                    match &graph.weigth {
                        Some(weigth) => {
                            let map = &weigth[n as usize];
                            ki += *map.get(&node).unwrap() as u64;
                        }
                        None => {
                            ki += 1;
                        }
                    }
                }
            }

            let mut delta: f64 = (som_in + kiin) as f64 / (2.0 * m);
            delta -= ((som_tot + ki) as f64 / (2.0 * m)) * ((som_tot + ki) as f64 / (2.0 * m));

            coms[test_com as usize].add_node(graph, node, test_com, com_for_nodes);

            let som_in: u64 = coms[test_com as usize].nb_links_in as u64;
            let som_tot: u64 = coms[test_com as usize].nb_links_tot as u64;

            delta -= (som_in as f64 / (2.0 * m))
                - ((som_tot as f64 / 2.0 * m) * (som_tot as f64 / 2.0 * m))
                - ((ki as f64 / 2.0 * m) * (ki as f64 / 2.0 * m));

            coms[test_com as usize].remove_node(graph, node, test_com, com_for_nodes);

            delta
        }

        fn aux_louvain(
            graph: &mut AdjArray,
            nb_com: usize,
            nb_rounds: usize,
            pred_qvalue: &mut f64,
        ) -> Vec<Comunity> {
            println!("\n ************Round {}****************", nb_rounds);
            println!("nb nodes restants {}", graph.number_nodes);
            println!("nb edges restants {}", graph.number_edges);
            println!("qvalue {}", pred_qvalue);

            let mut coms: Vec<Comunity> = Vec::with_capacity(graph.number_nodes as usize);
            let mut com_for_nodes: Vec<u32> = (0..graph.number_nodes).collect();

            println!("creating coms");
            for i in 0..graph.number_nodes {
                coms.push(Comunity::new(graph, i));
            }

            let mut cond = true;
            println!("in while");

            let mut qvalue: f64 = 0.0;

            while cond {
                cond = false;
                for n in 0..graph.number_nodes {
                    let voisins = graph.get_neighbors(n);
                    if voisins.is_empty() {
                        continue;
                    }
                    let mut max_q: f64 = 0.0;
                    let mut com_max = 0;

                    for voi in graph.get_neighbors(n) {
                        let jcom = com_for_nodes[voi as usize];

                        let q = deltaq(graph, n, jcom, &com_for_nodes, &mut coms);

                        if q > 0.0 && q > max_q {
                            com_max = jcom;
                            max_q = q;
                        }
                    }

                    if max_q > qvalue {
                        // println!("max q find");

                        coms[com_max as usize].add_node(graph, n, com_max, &com_for_nodes);
                        coms[com_for_nodes[n as usize] as usize].remove_node(
                            graph,
                            n,
                            com_for_nodes[n as usize],
                            &com_for_nodes,
                        );
                        com_for_nodes[n as usize] = com_max;
                        cond = true;
                        qvalue = max_q;
                    }
                }
            }

            let mut out_coms: Vec<Comunity> = Vec::new();
            let mut com_for_nodes: Vec<u32> = (0..graph.number_nodes).collect();
            println!("cleaning coms");

            for c in coms {
                if !c.nodes.is_empty() {
                    out_coms.push(c);
                    for node in &out_coms.last().unwrap().nodes {
                        com_for_nodes[*node] = (out_coms.len() - 1) as u32;
                    }
                }
            }

            if out_coms.len() < nb_com && qvalue > *pred_qvalue * 0.1 {
                // println!("creating new graph");

                let mut new_graph = AdjArray::from_coms(graph, &out_coms, &com_for_nodes);
                out_coms = aux_louvain(&mut new_graph, out_coms.len(), nb_rounds + 1, &mut qvalue);
            }

            out_coms
        }

        aux_louvain(self, std::usize::MAX, 0, &mut 0.0)
    }
}
