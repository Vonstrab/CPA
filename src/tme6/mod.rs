use crate::adj::adj::AdjArray;
use crate::tools::tasmin::State;
use crate::tools::utils;

extern crate hashbrown;

use hashbrown::HashMap;

use std::collections::BinaryHeap;

pub fn k_core(filename: &str, debug: bool) {
    if debug {
        println!("Filename input{:?}", filename);
    }
    utils::degre(filename, false);
    let graph = utils::load(filename, false);
    if debug {
        println!("Calcul decomposition");
    }
    let density_score = graph.get_core_info();
    if debug {
        println!("density : {:?}", density_score);
    }
}

impl AdjArray {
    pub fn core_decomposition(&self) -> (HashMap<u32, u16>, HashMap<u32, u32>) {
        let mut i = self.number_nodes;
        let mut c: u32 = 0;
        let mut k_core: HashMap<u32, u16> = HashMap::with_capacity(self.number_nodes as usize);
        let mut eta: HashMap<u32, u32> = HashMap::with_capacity(self.number_nodes as usize);

        let mut tas: BinaryHeap<State> = BinaryHeap::with_capacity(self.number_nodes as usize);

        for j in 0..self.number_nodes {
            tas.push(State {
                density: self.adj[j as usize].len(),
                node: j,
            })
        }

        while let Some(State { density, node }) = tas.pop() {
            c = c.max(density as u32);
            let voisins = self.get_neighbors(node);
            let mut new_tas: BinaryHeap<State> =
                BinaryHeap::with_capacity(self.number_nodes as usize);

            for State {
                density: old_den,
                node: tas_node,
            } in tas.drain()
            {
                if voisins.contains(&tas_node) {
                    new_tas.push(State {
                        density: old_den - 1,
                        node: tas_node,
                    });
                } else {
                    new_tas.push(State {
                        density: old_den,
                        node: tas_node,
                    });
                }
            }
            tas = new_tas;
            k_core.insert(node, c as u16);
            eta.insert(i, node);
            i -= 1;
            println!("i : {}", i);
        }

        (k_core, eta)
    }

    pub fn get_core_info(&self) -> Vec<(u32, f64, u32, f64)> {
        let res = self.core_decomposition();
        let k_core = res.0;
        let eta = res.1;

        let mut density_score: Vec<u32> = Vec::with_capacity(self.number_nodes as usize);
        for i in 0..self.number_nodes as usize {
            density_score.push(self.adj[i].len() as u32);
        }

        let mut max_core = 0;

        for i in 0..k_core.len() {
            let core = k_core[&(i as u32)];
            if core > max_core {
                max_core = core;
            }
        }

        let mut core = Vec::with_capacity((max_core + 1) as usize);
        for _ in 0..max_core + 1 {
            core.push((0, 0, 0));
        }

        for i in 1..self.number_nodes + 1 {
            println!("i : {}", i);

            let node = eta[&i];
            let kcore = k_core[&node] as usize;

            for voisin in self.get_neighbors(node as u32) {
                let vkcore = k_core[&voisin] as usize;

                if vkcore < (i as usize) {
                    core[kcore] = (core[kcore].1, core[kcore].1 + 1, core[kcore].2 + 1);
                } else {
                    core[kcore] = (core[kcore].1, core[kcore].1, core[kcore].2 + 1);
                }
            }
        }

        let mut premier = (0, 0.0, 0, 0.0);
        let mut deux = (0, 0.0, 0, 0.0);
        let mut trois = (0, 0.0, 0, 0.0);
        let mut quatre = (0, 0.0, 0, 0.0);
        let mut cinq = (0, 0.0, 0, 0.0);

        for i in 0..core.len() {
            let deg = (core[i].1 as f64) / (core[i].0 as f64);
            let size = core[i].0;
            let edge = (core[i].1 as f64) / (core[i].2 as f64);
            if deg > premier.1 {
                cinq = quatre;
                quatre = trois;
                trois = deux;
                premier = deux;

                premier.0 = eta[&(i as u32)];
                premier.1 = deg;
                premier.2 = size;
                premier.3 = edge;
            } else if deg > deux.1 {
                cinq = quatre;
                quatre = trois;
                trois = deux;

                deux.0 = eta[&(i as u32)];
                deux.1 = deg;
                deux.2 = size;
                deux.3 = edge;
            } else if deg > trois.1 {
                cinq = quatre;
                quatre = trois;

                trois.0 = eta[&(i as u32)];
                trois.1 = deg;
                trois.2 = size;
                trois.3 = edge;
            } else if deg > quatre.1 {
                cinq = quatre;

                quatre.0 = eta[&(i as u32)];
                quatre.1 = deg;
                quatre.2 = size;
                quatre.3 = edge;
            } else if deg > cinq.1 {
                cinq.0 = eta[&(i as u32)];
                cinq.1 = deg;
                cinq.2 = size;
                cinq.3 = edge;
            }
        }

        vec![premier, deux, trois, quatre, cinq]
    }
}
