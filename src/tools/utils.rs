use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use crate::adj::adj;
use crate::list::list;
use crate::matrix::matrix;
use crate::tme5;

use crate::static_algs::static_calc;
use rand::*;

pub fn load(filename: &str, debug: bool) -> adj::AdjArray {
    degre(filename, debug);
    load_adj_array(filename)
}

pub fn degre(filename: &str, debug: bool) {
    let mut file = reader_from_file(filename);
    let new_filename = format!("{}-degre.txt", filename.get(0..filename.len() - 4).unwrap());

    if debug {
        println!("Filename for the Nodes degres {}", new_filename);
    }

    let node_edges = static_calc::nb_node_and_edges(&mut file);

    file = reader_from_file(filename);
    let degres = static_calc::degre_node(&mut file, node_edges.0 as usize);
    output_degre(new_filename.as_str(), degres);
}

pub fn degre_dir(filename: &str, debug: bool) {
    let mut file = reader_from_file(filename);
    let new_filename = format!("{}-degre_dir.txt", filename.get(0..filename.len() - 4).unwrap());

    if debug {
        println!("Filename for the directed Nodes degres {}", new_filename);
    }

    let node_edges = static_calc::nb_node_and_edges(&mut file);

    file = reader_from_file(filename);
    let degres = tme5::degre_node_dir(&mut file, node_edges.0 as usize);
    output_degre(new_filename.as_str(), degres);
}

pub fn load_edges_list(filename: &str) -> list::EdgeList {
    let mut file = reader_from_file(filename);
    list::EdgeList::new(&mut file)
}
pub fn load_adj_array(filename: &str) -> adj::AdjArray {
    let mut file = reader_from_file(filename);
    let deg_filename = format!("{}-degre.txt", filename.get(0..filename.len() - 4).unwrap());

    let mut file_deg = reader_from_file(deg_filename.as_str());
    let deg = read_degre(&mut file_deg);

    adj::AdjArray::new(&mut file, &deg)
}
pub fn load_matrix(filename: &str) -> matrix::AdjMatrix {
    let mut file = reader_from_file(filename);
    let deg_filename = format!("{}-degre.txt", filename.get(0..filename.len() - 4).unwrap());

    let mut file_deg = reader_from_file(deg_filename.as_str());
    let deg = read_degre(&mut file_deg);

    matrix::AdjMatrix::new(&mut file, deg.len())
}
pub fn shuffle_fisher_yayes(vec: &mut [u32]) {
    let rng = rand::thread_rng();
    let mut srng = rand::rngs::SmallRng::from_rng(rng).unwrap();
    for i in 1..vec.len() {
        let j: usize = srng.gen_range(0, i);
        let tmp = vec[i];
        vec[i] = vec[j];
        vec[j] = tmp;
    }
}

pub fn partition_deg(
    vec: &mut Vec<Vec<u32>>,
    low: usize,
    high: usize,
    indexes: &mut [u32],
) -> usize {
    let pivot = vec[high].len();
    let mut i: i64 = low as i64 - 1;
    for j in low..high {
        if vec[j].len() <= pivot {
            i += 1;
            indexes[i as usize] = j as u32;
            let tmp = vec[j].clone();
            vec[j] = vec[i as usize].clone();
            vec[i as usize] = tmp;
        }
    }
    indexes[i as usize + 1] = high as u32;
    let tmp = vec[high].clone();
    vec[high] = vec[i as usize + 1].clone();
    vec[i as usize + 1] = tmp;

    i as usize + 1
}

pub fn quicksort_deg_index(vec: &mut Vec<Vec<u32>>, low: usize, high: usize, indexes: &mut [u32]) {
    if low < high {
        let pi: usize = partition_deg(vec, low, high, indexes);

        quicksort_deg_index(vec, low, pi - 1, indexes);
        quicksort_deg_index(vec, pi + 1, high, indexes);
    }
}

pub fn output_hist(filename: &str, comunites: &Vec<u32>) {
    let path = Path::new(filename);
    let mut com = comunites.clone();
    let mut file = File::create(&path).expect("Impossible to create file.");
    while !com.is_empty() {
        let value = com.remove(0);
        let mut nb_fois = 1;
        let mut i = 0;
        while i < com.len() {
            if com[i] == value {
                nb_fois += 1;
                com.remove(i);
            } else {
                i += 1;
            }
        }

        let _result = write!(file, "{} {}\n", nb_fois, value);
    }
}

pub fn output_pagerank (filename: &str, pagerank: Vec<f32>){
    let path = Path::new(filename);
    let mut file = File::create(&path).expect("Impossible to create file.");
    for (i, d) in pagerank.iter().enumerate() {
        let _result = write!(file, "{} {}\n", i, d);
    }
}

pub fn reader_from_file(filename: &str) -> BufReader<File> {
    let path = Path::new(filename);
    let file = File::open(&path).expect("Impossible to open file.");
    BufReader::new(file)
}

pub fn write_in(filename: &str, text: &String) {
    let path = Path::new(filename);
    let mut file = File::create(&path).expect("Impossible to create file.");
    let _result = write!(file, "{}", text);
}

pub fn output_degre(filename: &str, degre: Vec<u32>) {
    let path = Path::new(filename);
    let mut file = File::create(&path).expect("Impossible to create file.");
    for (i, d) in degre.iter().enumerate() {
        let _result = write!(file, "{} {}\n", i, d);
    }
}

pub fn output_edges(filename: &str, edges: Vec<(u32, u32)>) {
    let path = Path::new(filename);
    let mut file = File::create(&path).expect("Impossible to create file.");
    for e in edges {
        let _result = write!(file, "{} {}\n", e.0, e.1);
    }
}

pub fn read_degre(file: &mut BufReader<File>) -> Vec<u32> {
    let mut out: Vec<u32> = Vec::new();

    for l in file.lines() {
        let line = l.unwrap();

        if line.starts_with('#') {
            continue;
        }
        let mut lsplit = line.split_whitespace();
        let _node = lsplit.next().unwrap().parse::<u64>().unwrap();
        let degre = lsplit.next().unwrap().parse::<u32>().unwrap();
        out.push(degre);
    }
    out
}

pub fn output_density(filename: &str, density: &Vec<u32>) {
    let path = Path::new(filename);
    let mut file = File::create(&path).expect("Impossible to create file.");
    for i in 0..density.len() {
        let _result = write!(file, "{} {}\n", i, density[i]);
    }
}
