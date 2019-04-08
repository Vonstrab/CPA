use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn nb_node_and_edges(file: &mut BufReader<File>) -> (i64, i64) {
    let mut nb_node: i64 = 0;
    let mut nb_edge = 0;

    for l in file.lines() {
        let line = l.unwrap();
        if line.starts_with("#") {
            continue;
        }
        let mut lsplit = line.split_whitespace();
        // println!(" lsplit {:?}", lsplit);

        if !line.is_empty() {
            let node_source = lsplit.next().unwrap().parse::<i64>().unwrap();
            let node_target = lsplit.next().unwrap().parse::<i64>().unwrap();
            nb_node = nb_node.max(node_source);
            nb_node = nb_node.max(node_target);
            nb_edge += 1;
        }
    }

    return (nb_node + 1, nb_edge);
}

pub fn degre_node(file: &mut BufReader<File>, nb_node: usize) -> Vec<u32> {
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
            let node_source = lsplit.next().unwrap().parse::<i64>().unwrap();
            let node_target = lsplit.next().unwrap().parse::<i64>().unwrap();
            //println!("u{} v{}", node_source, node_target);

            degre[node_target as usize] += 1;
            degre[node_source as usize] += 1;
        }
    }

    degre
}

pub fn calculate_qvalue(degres: &Vec<u32>, file: &mut BufReader<File>) -> u64 {
    let mut value = 0;
    for l in file.lines() {
        let line = l.unwrap();

        if line.starts_with("#") {
            continue;
        }
        let mut lsplit = line.split_whitespace();
        let nodeu = lsplit.next().unwrap().parse::<i64>().unwrap();
        let nodev = lsplit.next().unwrap().parse::<i64>().unwrap();
        value += (degres[nodeu as usize] * degres[nodev as usize]) as u64;
    }
    value
}

pub fn degre_density(degres: &Vec<u32>) -> Vec<u32> {
    let mut maxdeg = 0;

    for i in 0..degres.len() {
        maxdeg = maxdeg.max(degres[i]);
    }

    let mut density: Vec<u32> = Vec::with_capacity(maxdeg as usize);

    for _ in 0..maxdeg + 1 {
        density.push(0);
    }

    for val in degres {
        density[*val as usize] += 1;
    }

    let mut max = 0;
    for i in 0..density.len() {
        max = max.max(density[i]);
    }

    // println!("desity size density {}", density.len());
    // println!("maximum density {}", max);

    density
}
