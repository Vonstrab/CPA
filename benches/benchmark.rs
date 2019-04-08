#[macro_use]
extern crate criterion;

use criterion::Criterion;

extern crate graph_lib;

use graph_lib::tme3;
use graph_lib::tme4;

use graph_lib::tme6;

use graph_lib::tools::utils;

fn load_benchmark(c: &mut Criterion) {
    c.bench_function("test1 edge list", |b| {
        b.iter(|| utils::load_edges_list("test/small/test/test1.txt"))
    });
    c.bench_function("test1 adj list", |b| {
        b.iter(|| utils::load_adj_array("test/small/test/test1.txt"))
    });
    c.bench_function("test1 matrix list", |b| {
        b.iter(|| utils::load_matrix("test/small/test/test1.txt"))
    });

    c.bench_function("test2 edge list", |b| {
        b.iter(|| utils::load_edges_list("test/small/test/test2.txt"))
    });
    c.bench_function("test2 adj list", |b| {
        b.iter(|| utils::load_adj_array("test/small/test/test2.txt"))
    });
    c.bench_function("test2 matrix list", |b| {
        b.iter(|| utils::load_matrix("test/small/test/test2.txt"))
    });

    c.bench_function("test3 edge list", |b| {
        b.iter(|| utils::load_edges_list("test/small/test/test3.txt"))
    });
    c.bench_function("test3 adj list", |b| {
        b.iter(|| utils::load_adj_array("test/small/test/test3.txt"))
    });
    c.bench_function("test3 matrix list", |b| {
        b.iter(|| utils::load_matrix("test/small/test/test3.txt"))
    });
}

fn test1_all_test_benchmark(c: &mut Criterion) {
    c.bench_function("test1 tme3 all", |b| {
        b.iter(|| tme3::tem3_tests("test/small/test/test1.txt", false))
    });
}

fn test2_all_test_benchmark(c: &mut Criterion) {
    c.bench_function("test2 tme3 all", |b| {
        b.iter(|| tme3::tem3_tests("test/small/test/test2.txt", false))
    });
}

fn test3_all_test_benchmark(c: &mut Criterion) {
    c.bench_function("test3 tme3 all", |b| {
        b.iter(|| tme3::tem3_tests("test/small/test/test3.txt", false))
    });
}

criterion_group!(
    tme3,
    test1_all_test_benchmark,
    test2_all_test_benchmark,
    test3_all_test_benchmark,
    load_benchmark,
);

fn test1_comunities(c: &mut Criterion) {
    let mut graph = utils::load("test/small/test/test1.txt", false);

    c.bench_function("test1 tme4 communities", move |b| {
        b.iter(|| tme4::nb_communities(&mut graph));
    });
}

fn test2_comunities(c: &mut Criterion) {
    let mut graph = utils::load("test/small/test/test2.txt", false);

    c.bench_function("test2 tme4 communities", move |b| {
        b.iter(|| tme4::nb_communities(&mut graph));
    });
}

fn test3_comunities(c: &mut Criterion) {
    let mut graph = utils::load("test/small/test/test3.txt", false);

    c.bench_function("test3 tme4 communities", move |b| {
        b.iter(|| tme4::nb_communities(&mut graph));
    });
}

fn test1_louvain(c: &mut Criterion) {
    let mut graph = utils::load("test/small/test/test1.txt", false);

    c.bench_function("test1 tme4 communities louvain", move |b| {
        b.iter(|| tme4::louvain(&mut graph));
    });
}

fn test2_louvain(c: &mut Criterion) {
    let mut graph = utils::load("test/small/test/test2.txt", false);

    c.bench_function("test2 tme4 communities louvain", move |b| {
        b.iter(|| tme4::louvain(&mut graph));
    });
}

fn test3_louvain(c: &mut Criterion) {
    let mut graph = utils::load("test/small/test/test3.txt", false);

    c.bench_function("test3 tme4 communities louvain", move |b| {
        b.iter(|| tme4::louvain(&mut graph));
    });
}

criterion_group!(
    tme4,
    test1_comunities,
    test2_comunities,
    test3_comunities,
    test1_louvain,
    test2_louvain,
    test3_louvain
);

fn test1_core(c: &mut Criterion) {
    c.bench_function("test1 k-core", |b| {
        b.iter(|| tme6::k_core("test/small/test/test1.txt", false))
    });
}

fn test2_core(c: &mut Criterion) {
    c.bench_function("test2 k-core", |b| {
        b.iter(|| tme6::k_core("test/small/test/test2.txt", false))
    });
}

fn test3_core(c: &mut Criterion) {
    c.bench_function("test3 k-core", |b| {
        b.iter(|| tme6::k_core("test/small/test/test3.txt", false))
    });
}

criterion_group!(tme6, test1_core, test2_core, test3_core);
criterion_main!(tme3, tme4, tme6);
