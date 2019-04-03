#[macro_use] extern crate criterion;
extern crate sorensen;
extern crate rand;


use criterion::{
    Criterion,
};


fn intersection_2sym_bench(c: &mut Criterion) {
    c.bench_function("2 symbols string", |b| {
        b.iter(|| {
            sorensen::distance(
                "ac".as_bytes(),
                "bc".as_bytes()
            )
        })
    });
}

fn intersection_20sym_bench(c: &mut Criterion) {
    c.bench_function("20 symbols string", |b| {
        let str1 = (0..20).map(|_| { rand::random::<char>() }).collect::<String>();
        let str2 = (0..20).map(|_| { rand::random::<char>() }).collect::<String>();

        b.iter(|| {
            sorensen::distance(
                str1.as_bytes(),
                str2.as_bytes()
            )
        })
    });
}

fn intersection_200_long_bench(c: &mut Criterion) {
    c.bench_function("200 symbols string", |b| {
        let str1 = (0..200).map(|_| { rand::random::<char>() }).collect::<String>();
        let str2 = (0..200).map(|_| { rand::random::<char>() }).collect::<String>();

        b.iter(|| {
            sorensen::distance(
                str1.as_bytes(),
                str2.as_bytes()
            )
        })
    });
}

fn intersection_2000_long_bench(c: &mut Criterion) {
    c.bench_function("2000 symbols string", |b| {
        let str1 = (0..2000).map(|_| { rand::random::<char>() }).collect::<String>();
        let str2 = (0..2000).map(|_| { rand::random::<char>() }).collect::<String>();

        b.iter(|| {
            sorensen::distance(
                str1.as_bytes(),
                str2.as_bytes()
            )
        })
    });
}

fn intersection_20000_long_bench(c: &mut Criterion) {
    c.bench_function("20000 symbols string", |b| {
        let str1 = (0..20000).map(|_| { rand::random::<char>() }).collect::<String>();
        let str2 = (0..20000).map(|_| { rand::random::<char>() }).collect::<String>();

        b.iter(|| {
            sorensen::distance(
                str1.as_bytes(),
                str2.as_bytes()
            )
        })
    });
}

fn intersection_200000_long_bench(c: &mut Criterion) {
    c.bench_function("200000 symbols string", |b| {
        let str1 = (0..200000).map(|_| { rand::random::<char>() }).collect::<String>();
        let str2 = (0..200000).map(|_| { rand::random::<char>() }).collect::<String>();

        b.iter(|| {
            sorensen::distance(
                str1.as_bytes(),
                str2.as_bytes()
            )
        })
    });
}

fn intersection_2000000_long_bench(c: &mut Criterion) {
    c.bench_function("2000000 symbols string", |b| {
        let str1 = (0..2000000).map(|_| { rand::random::<char>() }).collect::<String>();
        let str2 = (0..2000000).map(|_| { rand::random::<char>() }).collect::<String>();

        b.iter(|| {
            sorensen::distance(
                str1.as_bytes(),
                str2.as_bytes()
            )
        })
    });
}

fn intersection_long_vec_bench(c: &mut Criterion) {
    c.bench_function(" 1_000_000 vec", |b| {
        let str1 = (0..100000).map(|_| { rand::random::<char>() }).collect::<Vec<char>>();
        let str2 = (0..100000).map(|_| { rand::random::<char>() }).collect::<Vec<char>>();
        b.iter(|| sorensen::distance(&str1, &str2));
    });

    c.bench_function(" 10_000_000 vec", |b| {
        let str1 = (0..1000000).map(|_| { rand::random::<char>() }).collect::<Vec<char>>();
        let str2 = (0..1000000).map(|_| { rand::random::<char>() }).collect::<Vec<char>>();
        b.iter(|| sorensen::distance(&str1, &str2));
    });
}

criterion_group!(
    benches,
    intersection_2sym_bench,
    intersection_20sym_bench,
    intersection_200_long_bench,
    intersection_2000_long_bench,
    intersection_20000_long_bench,
    intersection_200000_long_bench,
    intersection_2000000_long_bench,
    intersection_long_vec_bench
);
criterion_main!(benches);