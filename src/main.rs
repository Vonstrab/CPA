extern crate graph_lib;

use graph_lib::tme3;
use graph_lib::tme4;
use graph_lib::tme5;
use graph_lib::tme6;

use graph_lib::tools::utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Arguments {:?}", args);
    if args.contains(&"--clean".to_string()) {
        let mut filename_in = None;
        let mut filename_out = None;

        for i in 1..args.len() {
            if !args[i].starts_with("--") {
                if filename_in.is_none() {
                    filename_in = Some(args[i].clone());
                } else {
                    filename_out = Some(args[i].clone());
                }
            }
        }

        let str_in = filename_in.unwrap_or_else(|| panic!("pas assez d'arguments"));
        let str_out = filename_out.unwrap_or_else(|| panic!("pas assez d'arguments"));

        println!("Filename input{:?}", str_in);
        utils::degre(str_in.as_str(), true);
        let mut graph = utils::load(str_in.as_str(), true);
        graph.remove_lone_node();

        println!("Filename output{:?}", str_out);
        utils::output_edges(str_out.as_str(), graph.to_edges());
        return;
    }

    if args.contains(&"--tme3".to_string()) {
        let mut filename_in = None;

        for i in 1..args.len() {
            if !args[i].starts_with("--") {
                if filename_in.is_none() {
                    filename_in = Some(args[i].clone());
                }
            }
        }

        let str_in = filename_in.unwrap_or_else(|| panic!("pas assez d'arguments"));

        println!("Filename input{:?}", str_in);
        tme3::tem3_tests(str_in.as_str(), true);
        return;
    }

    if args.contains(&"--tme4rand".to_string()) {
        let mut p = None;
        let mut q = None;

        for i in 1..args.len() {
            if !args[i].starts_with("--") {
                if p.is_none() {
                    p = Some(args[i].clone());
                } else {
                    q = Some(args[i].clone());
                }
            }
        }

        let p_value = p
            .unwrap_or_else(|| panic!("pas assez d'arguments"))
            .parse::<f64>()
            .unwrap();
        let q_value = q
            .unwrap_or_else(|| panic!("pas assez d'arguments"))
            .parse::<f64>()
            .unwrap();

        let graph = graph_lib::adj::adj::AdjArray::new_rand(400, p_value, q_value);

        println!("Filename output{:?}", "rand.txt");

        utils::output_edges("rand.txt", graph.to_edges());
        std::process::Command::new("python3")
            .arg("src/tme4/netX.py")
            .output()
            .expect("Something went wrong with netX.py");

        return;
    }

    if args.contains(&"--tme4hist".to_string()) {
        let mut filename_in = None;
        let mut filename_out = None;

        for i in 1..args.len() {
            if !args[i].starts_with("--") {
                if filename_in.is_none() {
                    filename_in = Some(args[i].clone());
                } else {
                    filename_out = Some(args[i].clone());
                }
            }
        }

        let str_in = filename_in.unwrap_or_else(|| panic!("pas assez d'arguments"));
        let str_out = filename_out.unwrap_or_else(|| panic!("pas assez d'arguments"));

        println!("Filename input{:?}", str_in);

        let mut graph = utils::load(str_in.as_str(), true);

        let mut comunities: Vec<u32> = Vec::new();

        for i in 0..1000 {
            println!("Calcul N {}", i);
            comunities.push(tme4::nb_communities(&mut graph));
        }

        println!("Filename output{:?}", str_out);
        utils::output_hist(str_out.as_str(), &comunities);
        return;
    }

    if args.contains(&"--tme4comp".to_string()) {
        let mut filename_in = None;

        for i in 1..args.len() {
            if !args[i].starts_with("--") {
                if filename_in.is_none() {
                    filename_in = Some(args[i].clone());
                }
            }
        }

        let str_in = filename_in.unwrap_or_else(|| panic!("pas assez d'arguments"));

        println!("Filename input{:?}", str_in);
        let mut graph = utils::load(str_in.as_str(), true);

        println!("Calcul Label ");
        let comunities_label = tme4::nb_communities(&mut graph);
        println!("Nb com Label {}: ", comunities_label);

        println!("Calcul Louvain ");
        let comunities_louvain = graph.louvain().len();
        println!("Nb com Louvain {}: ", comunities_louvain);
    }

    if args.contains(&"--tme5".to_string()) {
        let mut filename_in = None;
        let mut filename_out = None;

        for i in 1..args.len() {
            if !args[i].starts_with("--") {
                if filename_in.is_none() {
                    filename_in = Some(args[i].clone());
                } else {
                    filename_out = Some(args[i].clone());
                }
            }
        }

        let str_in = filename_in.unwrap_or_else(|| panic!("pas assez d'arguments"));
        let str_out = filename_out.unwrap_or_else(|| panic!("pas assez d'arguments"));

        println!("Filename input{:?}", str_in);
        let mut graph = tme5::load_dir(str_in.as_str());

        println!("Calcul pagerank ");
        let page_rank = graph.power_page_rank(0.15);
        println!("pg {:?}", page_rank);
        utils::output_pagerank(str_out.as_str(), page_rank);
    }

    if args.contains(&"--tme6decomp".to_string()) {
        let mut filename_in = None;

        for i in 1..args.len() {
            if !args[i].starts_with("--") {
                if filename_in.is_none() {
                    filename_in = Some(args[i].clone());
                }
            }
        }

        let str_in = filename_in.unwrap_or_else(|| panic!("pas assez d'arguments"));
        tme6::k_core(str_in.as_str(), true);
    }

    if args.contains(&"--tme6scholar".to_string()) {
        let filename_in = "test/small/scholar/net.txt";

        tme6::k_core(filename_in, true);
    }

    return;
}
