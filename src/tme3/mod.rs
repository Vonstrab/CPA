use crate::static_algs::static_calc;
use crate::tools::utils;

pub fn nb_node_edges(filename: &str, debug: bool) {
    let mut file = utils::reader_from_file(filename);
    let node_edges = static_calc::nb_node_and_edges(&mut file);
    if debug {
        println!(
            "\nFile :{} \n Nodes : {}\nEdges : {}",
            filename, node_edges.0, node_edges.1
        );
    }
}

pub fn qvalue(filename: &str, debug: bool) {
    let mut file = utils::reader_from_file(filename);
    let deg_filename = format!("{}-degre.txt", filename.get(0..filename.len() - 4).unwrap());

    let mut file_degres = utils::reader_from_file(deg_filename.as_str());

    let qvalue = static_calc::calculate_qvalue(&utils::read_degre(&mut file_degres), &mut file);
    if debug {
        println!("QValue {} {}", filename, qvalue);
    }
}

pub fn density(filename: &str, debug: bool) {
    let deg_filename = format!("{}-degre.txt", filename.get(0..filename.len() - 4).unwrap());
    let density_filename = format!(
        "{}-density.txt",
        filename.get(0..filename.len() - 4).unwrap()
    );
    if debug {
        println!("Filename for the Nodes density {}", density_filename);
    }

    let mut file_deg = utils::reader_from_file(deg_filename.as_str());
    let deg = utils::read_degre(&mut file_deg);

    let density = static_calc::degre_density(&deg);

    utils::output_density(density_filename.as_str(), &density);
}

pub fn tem3_tests(filename: &str, debug: bool) {
    if debug {
        println!("******************************");
        println!("Running fichier {}", filename);
    }
    let mut debut = std::time::SystemTime::now();

    nb_node_edges(filename, debug);
    if debug {
        println!(
            "\nTemps  nb node et edge {} s, {}ms\n",
            debut.elapsed().unwrap().as_secs(),
            debut.elapsed().unwrap().subsec_millis()
        );
    }

    debut = std::time::SystemTime::now();
    utils::degre(filename, debug);
    if debug {
        println!(
            "\nTemps degres {} s, {}ms\n",
            debut.elapsed().unwrap().as_secs(),
            debut.elapsed().unwrap().subsec_millis()
        );
    }
    qvalue(filename, debug);
    if debug {
        println!(
            "\nTemps qbvalue + degres {} s, {}ms\n",
            debut.elapsed().unwrap().as_secs(),
            debut.elapsed().unwrap().subsec_millis()
        );
    }
    debut = std::time::SystemTime::now();

    density(filename, debug);
    if debug {
        println!(
            "Temps density {} s, {}ms\n",
            debut.elapsed().unwrap().as_secs(),
            debut.elapsed().unwrap().subsec_millis()
        );
    }
    debut = std::time::SystemTime::now();

    let mem_size = utils::load_edges_list(filename).mem_size();
    if debug {
        println!(
            "Temps load Edge list {} s, {}ms\nPour une taille memoire {} octets",
            debut.elapsed().unwrap().as_secs(),
            debut.elapsed().unwrap().subsec_millis(),
            mem_size,
        );
    }

    let mut adjarray = utils::load_adj_array(filename);
    let mem_size = adjarray.mem_size();
    if debug {
        println!(
            "Temps adjacency array {} s, {}ms\nPour une taille memoire {} octets",
            debut.elapsed().unwrap().as_secs(),
            debut.elapsed().unwrap().subsec_millis(),
            mem_size,
        );
        println!("\nCalcul des composants");
    }

    let comps_brut = adjarray.components();
    let mut liste_node_iso: Vec<u32> = Vec::new();
    let mut comps: Vec<Vec<u32>> = Vec::new();

    for mut comp in comps_brut {
        if comp.len() == 1 {
            liste_node_iso.push(comp.pop().unwrap());
        } else {
            comps.push(comp);
        }
    }

    if debug {
        println!("noeuds isolés {:?}", liste_node_iso);
        println!("nombre de composant {}", comps.len());

        for i in 0..comps.len() {
            println!("taille du {} eme composant {}", i, comps[i].len());
        }
    }

    if debug {
        println!("\nCalcul du Diametre");
        println!("Lower bound diametre {}", adjarray.diam_lower_bound());
    }

    debut = std::time::SystemTime::now();
    let triangles = adjarray.get_triangles().len();
    if debug {
        println!("\nCalcul des triangles");
        println!("Nombre de triangles : {}", triangles);
        println!(
            "Temps adjacency triangle  {} s, {}ms",
            debut.elapsed().unwrap().as_secs(),
            debut.elapsed().unwrap().subsec_millis(),
        );
    }
}
