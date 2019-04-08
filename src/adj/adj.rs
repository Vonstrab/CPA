extern crate rand;
use rand::Rng;
use rand::*;

extern crate hashbrown;

use hashbrown::HashMap;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Debug)]
pub struct AdjArray {
    pub number_nodes: u32,
    pub number_edges: u32,
    pub adj: Vec<Vec<u32>>,
    pub is_sorted: bool,
    pub is_direct: bool,
    pub weigth: Option<Vec<HashMap<u32, u32>>>,
}

impl AdjArray {
    pub fn new(file_reader: &mut BufReader<File>, degre: &Vec<u32>) -> AdjArray {
        let number_nodes = degre.len();
        let mut number_edges = 0;
        let mut adj: Vec<Vec<u32>> = Vec::with_capacity(degre.len());

        for i in 0..number_nodes {
            let idegre = degre[i];
            number_edges += idegre;
            adj.push(Vec::with_capacity(idegre as usize));
        }

        for l in file_reader.lines() {
            let line = l.unwrap();
            if line.starts_with("#") {
                continue;
            }
            let mut lsplit = line.split_whitespace();
            if !line.is_empty() {
                let node_source = lsplit.next().unwrap().parse::<u32>().unwrap();
                let node_target = lsplit.next().unwrap().parse::<u32>().unwrap();
                if node_source != node_target {
                    adj[node_source as usize].push(node_target);
                    adj[node_target as usize].push(node_source);
                }
            }
        }

        AdjArray {
            number_nodes: number_nodes as u32,
            number_edges,
            adj,
            is_sorted: false,
            is_direct: false,
            weigth: None,
        }
    }

    pub fn new_rand(nb_node: usize, pc: f64, pp: f64) -> AdjArray {
        let mut number_edges = 0;
        let mut adj: Vec<Vec<u32>> = Vec::with_capacity(nb_node);

        for _ in 0..nb_node {
            adj.push(Vec::new());
        }

        let rng = rand::thread_rng();
        let mut srng = rand::rngs::SmallRng::from_rng(rng).unwrap();

        for u in 0..nb_node {
            let compu = (u / 100) as u8;
            for v in 0..u {
                let compv = (v / 100) as u8;

                let rand_prob: f64 = srng.gen_range(0.0, 1.0);
                if compu == compv {
                    if rand_prob < pc {
                        adj[u].push(v as u32);
                        adj[v].push(u as u32);
                        number_edges += 1;
                    }
                } else {
                    if rand_prob < pp {
                        adj[u].push(v as u32);
                        adj[v].push(u as u32);
                        number_edges += 1;
                    }
                }
            }
        }

        AdjArray {
            number_nodes: nb_node as u32,
            number_edges: number_edges,
            adj: adj,
            is_sorted: false,
            is_direct: false,

            weigth: None,
        }
    }

    pub fn mem_size(&self) -> usize {
        let mut size = std::mem::size_of::<self::AdjArray>();

        for vec in &self.adj {
            size += std::mem::size_of::<u32>() * vec.len();
        }
        size
    }

    pub fn get_neighbors(&self, node: u32) -> Vec<u32> {
        self.adj[node as usize].clone()
    }

    pub fn bfs_from(&self, node: u32) -> Vec<u32> {
        let mut output: Vec<u32> = Vec::new();
        let mut mark: Vec<bool> = Vec::with_capacity(self.adj.len());
        let mut fifo: Vec<u32> = Vec::new();

        for _ in 0..self.number_nodes {
            mark.push(false);
        }
        fifo.push(node);
        mark[node as usize] = true;

        while !fifo.is_empty() {
            let u = fifo[0];
            fifo.remove(0);
            output.push(u);

            for v in self.get_neighbors(u) {
                if !mark[v as usize] {
                    fifo.push(v);
                    mark[v as usize] = true;
                }
            }
        }

        output
    }

    pub fn dist_bfs(&self, node: u32, target: u32) -> u8 {
        if node == target {
            return 0;
        }
        let mut output: Vec<u32> = Vec::new();
        let mut mark: Vec<bool> = Vec::with_capacity(self.adj.len());
        let mut fifo: Vec<u32> = Vec::new();
        let mut dist: Vec<u8> = Vec::new();

        for _ in 0..self.number_nodes {
            mark.push(false);
            dist.push(0);
        }

        fifo.push(node);
        mark[node as usize] = true;

        while !fifo.is_empty() {
            let u = fifo[0];
            fifo.remove(0);
            output.push(u);

            for v in self.get_neighbors(u) {
                if !mark[v as usize] {
                    if v == target {
                        return dist[u as usize] + 1;
                    }
                    fifo.push(v);
                    mark[v as usize] = true;
                    dist[v as usize] = dist[u as usize] + 1;
                }
            }
        }

        dist[target as usize]
    }

    pub fn dfs_from_to(&self, node: u32) -> Vec<u32> {
        let mut output: Vec<u32> = Vec::new();
        let mut mark: Vec<bool> = Vec::with_capacity(self.adj.len());
        let mut stack: Vec<u32> = Vec::new();

        for _ in 0..self.number_nodes {
            mark.push(false);
        }
        stack.push(node);
        mark[node as usize] = true;

        while !stack.is_empty() {
            let u = stack.pop().unwrap();

            if !mark[u as usize] {
                mark[u as usize] = true;
                for v in self.get_neighbors(u) {
                    stack.push(v);
                }
                output.push(u);
            }
        }

        output
    }

    pub fn diam_lower_bound(&self) -> u8 {
        //println!("First BFS");
        let component = self.components();
        let mut lower_bound = 0;
        for comp in component {
            if comp.len() == 1 {
                continue;
            }
            let v = *comp.last().unwrap();
            // println!("Second BFS");
            let w = *self.bfs_from(v).last().unwrap();
            // println!("Calc Dist");
            let distvw = self.dist_bfs(v, w);
            lower_bound = lower_bound.max(distvw);
        }
        lower_bound
    }

    pub fn components(&self) -> Vec<Vec<u32>> {
        let mut output: Vec<Vec<u32>> = Vec::new();
        let mut mark: Vec<bool> = Vec::with_capacity(self.adj.len());
        let mut fifo: Vec<u32> = Vec::new();

        for _ in 0..self.number_nodes {
            mark.push(false);
        }

        for node in 0..self.number_nodes {
            if mark[node as usize] {
                continue;
            }

            let mut component: Vec<u32> = Vec::new();
            fifo.push(node);
            mark[node as usize] = true;

            while !fifo.is_empty() {
                let u = fifo[0];
                fifo.remove(0);
                component.push(u);

                for v in self.get_neighbors(u) {
                    if !mark[v as usize] {
                        fifo.push(v);
                        mark[v as usize] = true;
                    }
                }
            }
            output.push(component);
        }

        output
    }
    #[inline]

    pub fn remove_lone_node(&mut self) {
        let mut i = 0;

        while i < self.adj.len() {
            if self.adj[i].len() == 0 {
                //println!("Noeud Seul trouvé indice : {}", i);
                if i < self.adj.len() - 1 {
                    for k in 0..self.adj.len() {
                        for j in 0..self.adj[k].len() {
                            if self.adj[k][j] > i as u32 {
                                self.adj[k][j] -= 1 as u32;
                            }
                        }
                    }
                }
                self.adj.remove(i);
                self.number_nodes -= 1;
            } else {
                i += 1;
            }
        }
    }

    pub fn sort_by_deg(&mut self) {
        // println!("Tri par degres");
        let mut n_idexes: Vec<u32> = (0..self.number_nodes + 1).collect();
        let mut new_adj = self.adj.clone();

        //overflows on large graphes
        // utils::quicksort_deg_index(& mut new_adj, 0, (self.number_nodes-1) as usize, & mut n_idexes);
        new_adj.sort_by(|a, b| (&b.len()).cmp(&a.len()));

        //  println!("Contruction de l'index");
        for (i, adj) in self.adj.iter().enumerate() {
            let ad = adj.clone();
            let index = new_adj.iter().position(|r| *r == ad).unwrap();
            n_idexes[i] = index as u32;
        }

        // println!("Reindexage des nodes");
        for i in 0..new_adj.len() {
            for j in 0..new_adj[i].len() {
                new_adj[i][j] = n_idexes[new_adj[i][j] as usize];
            }
        }

        self.adj = new_adj;
    }

    pub fn get_triangles(&mut self) -> Vec<(u32, u32, u32)> {
        self.sort_by_deg();
        let mut tri_list: Vec<(u32, u32, u32)> = Vec::new();
        let edge_list = self.to_edges();
        for (u, v) in edge_list {
            let vois_u = self.get_neighbors(u);
            let vois_v = self.get_neighbors(v);
            for e in vois_u {
                if e < u && vois_v.contains(&e) {
                    tri_list.push((u, v, e));
                }
            }
        }

        tri_list
    }

    pub fn to_edges(&self) -> Vec<(u32, u32)> {
        let mut edge_list: Vec<(u32, u32)> = Vec::with_capacity(self.number_edges as usize);

        for n in 0..self.number_nodes {
            for v in &self.adj[n as usize] {
                if *v > n {
                    edge_list.push((n, *v));
                }
            }
        }
        edge_list
    }
}
