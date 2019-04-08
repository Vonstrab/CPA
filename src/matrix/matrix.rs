use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub struct AdjMatrix {
    pub matrix: Vec<Vec<bool>>,
}

impl AdjMatrix {
    #[inline]

    pub fn new(file_reader: &mut BufReader<File>, nb_node: usize) -> AdjMatrix {
        let mut matrix: Vec<Vec<bool>> = Vec::new();

        for i in 0..nb_node {
            matrix.push(Vec::new());
            for _ in 0..nb_node {
                matrix[i].push(false);
            }
        }

        for l in file_reader.lines() {
            let line = l.unwrap();
            if line.starts_with("#") {
                continue;
            }
            let mut lsplit = line.split_whitespace();
            let node_source = lsplit.next().unwrap().parse::<usize>().unwrap();
            let node_target = lsplit.next().unwrap().parse::<usize>().unwrap();
            matrix[node_source][node_target] = true;
            matrix[node_target][node_source] = true;
        }

        AdjMatrix { matrix: matrix }
    }
    #[inline]

    pub fn mem_size(&self) -> usize {
        let mut size = std::mem::size_of::<self::AdjMatrix>();
        size += std::mem::size_of::<u8>() * self.matrix.len() * self.matrix.len();
        size
    }
}
