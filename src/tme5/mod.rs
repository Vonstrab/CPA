use crate::adj::adj::AdjArray;

use crate::tools::utils;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn degre_node_dir(file: &mut BufReader<File>, nb_node: usize) -> Vec<u32> {
    let mut degre: Vec<u32> = Vec::with_capacity(nb_node);

    for _ in 0..nb_node {
        degre.push(0);
    }

    for l in file.lines() {
        let line = l.unwrap();
        //println!("line : {}", line);

        if line.starts_with("#") {
            continue;
        }
        let mut lsplit = line.split_whitespace();
        if !line.is_empty() {
            let _node_source = lsplit.next().unwrap().parse::<i64>().unwrap();
            let node_target = lsplit.next().unwrap().parse::<i64>().unwrap();

            degre[node_target as usize] += 1;
        }
    }

    degre
}

pub fn load_dir(filename: &str) -> AdjArray {
    let mut file = utils::reader_from_file(filename);
    let deg_filename = format!("{}-degre.txt", filename.get(0..filename.len() - 4).unwrap());

    println!("doing degre");
    utils::degre(filename, false);

    let mut file_deg = utils::reader_from_file(deg_filename.as_str());
    let deg = utils::read_degre(&mut file_deg);
    println!("doing graph");

    AdjArray::new_dir(&mut file, &deg)
}

impl AdjArray {
    pub fn prod_mat_vec(&self, vec: &Vec<f32>,t :&Vec<Vec<f32>>) -> Vec<f32> {
        let mut b: Vec<f32> = Vec::with_capacity(vec.len());

        for i in 0..vec.len() {
            b.push(0.0);
            for j in 0..vec.len() {
                let degu = t[i][j];
                // let degu = self.deg_out(j as u32) as f32;
                    b[i] += degu * vec[i];
            }
        }
        b
    }

    pub fn new_dir(file_reader: &mut BufReader<File>, degre: &Vec<u32>) -> AdjArray {
        let number_nodes = degre.len();
        let mut number_edges = 0;
        let mut adj: Vec<Vec<u32>> = Vec::with_capacity(degre.len());

        for i in 0..number_nodes {
            let idegre = degre[i];
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
                    number_edges += 1;
                }
            }
        }

        AdjArray {
            number_nodes: number_nodes as u32,
            number_edges,
            adj,
            is_sorted: false,
            is_direct: true,
            weigth: None,
        }
    }

    pub fn deg_out(&self, node: u32) -> u32 {
        assert!(self.is_direct);
        self.adj[node as usize].len() as u32
    }

    pub fn transition_matrix(&self) -> Vec<Vec<f32>> {
        let mut matrix = Vec::with_capacity(self.number_nodes as usize);
        for i in 0..self.number_nodes {
            matrix.push(Vec::with_capacity(self.number_nodes as usize));

            for _ in 0..self.number_nodes {
                matrix[i as usize].push(0.0);
            }
        }

        for (u, v) in self.to_edges() {
            let degu = self.deg_out(u) as f32;
            if degu == 0.0 {
                matrix[u as usize][v as usize] = 1.0 / (self.number_nodes as f32);
            } else {
                matrix[u as usize][v as usize] = 1.0 / degu;
            }
        }

        matrix
    }

    pub fn power_page_rank(&mut self, alpha: f32) -> Vec<f32> {
        let t = self.transition_matrix();
        let mut p: Vec<f32> = Vec::with_capacity(self.number_nodes as usize);

        let entry: f32 = 1.0 / self.number_nodes as f32;
        for _ in 0..self.number_nodes {
            p.push(entry);
        }

        let mut cond = true;
        let mut round = 1;

        while cond {
            cond = false;
            println!("** round : {} **", round);
            let mut new_p = self.prod_mat_vec(&p,&t);
            for j in 0..self.number_nodes {
                new_p[j as usize] = (1.0 - alpha) * ((p[j as usize] + alpha) as f32) * entry;
                // new_p[j as usize] += (1.0 - p[j as usize]) / (self.number_nodes as usize) as f32;

                if (new_p[j as usize] - p[j as usize]).abs() > 0.0 {
                    cond = true;
                }
            }
            if cond {
                p = new_p;
            }

            round += 1;
        }

        p
    }
}
