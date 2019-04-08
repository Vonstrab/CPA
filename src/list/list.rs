use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub struct EdgeList {
    pub list: Vec<(u32, u32)>,
}

impl EdgeList {
    pub fn new(file_reader: &mut BufReader<File>) -> EdgeList {
        let mut liste: Vec<(u32, u32)> = Vec::new();

        for l in file_reader.lines() {
            let line = l.unwrap();
            if line.starts_with("#") {
                continue;
            }
            let mut lsplit = line.split_whitespace();
            let node_source = lsplit.next().unwrap().parse::<u32>().unwrap();
            let node_target = lsplit.next().unwrap().parse::<u32>().unwrap();
            liste.push((node_source, node_target));
        }

        EdgeList { list: liste }
    }

    pub fn mem_size(&self) -> usize {
        let mut size = std::mem::size_of::<self::EdgeList>();
        size += std::mem::size_of::<(usize, usize)>() * self.list.len();
        size
    }
}
