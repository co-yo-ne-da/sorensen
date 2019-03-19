#[macro_use] extern crate criterion;
extern crate sorensen;
extern crate rand;

use criterion::{
    Criterion,
    Bencher,
    ParameterizedBenchmark,
//    PlotConfiguration,
//    AxisScale
};


//fn intersection_2sym_bench(b: &mut Bencher) {
//    b.iter(|| {
//        sorensen(
//            "ac".as_bytes(),
//            "bc".as_bytes()
//        )
//    })
//}
//
//fn intersection_20sym_bench(b: &mut Bencher) {
//    let str1 = (0..20).map(|_| { rand::random::<char>() }).collect::<String>();
//    let str2 = (0..20).map(|_| { rand::random::<char>() }).collect::<String>();
//
//    b.iter(|| {
//        sorensen(
//            str1.as_bytes(),
//            str2.as_bytes()
//        )
//    })
//}
//
//fn intersection_200_long_bench(b: &mut Bencher) {
//    let str1 = (0..200).map(|_| { rand::random::<char>() }).collect::<String>();
//    let str2 = (0..200).map(|_| { rand::random::<char>() }).collect::<String>();
//
//    b.iter(|| {
//        sorensen(
//            str1.as_str().as_bytes(),
//            str2.as_str().as_bytes()
//        )
//    })
//}
//
//fn intersection_2000_long_bench(b: &mut Bencher) {
//    let str1 = (0..2000).map(|_| { rand::random::<char>() }).collect::<String>();
//    let str2 = (0..2000).map(|_| { rand::random::<char>() }).collect::<String>();
//
//    b.iter(|| {
//        sorensen(
//            str1.as_str().as_bytes(),
//            str2.as_str().as_bytes()
//        )
//    })
//}
//
//fn intersection_20000_long_bench(b: &mut Bencher) {
//    let str1 = (0..20000).map(|_| { rand::random::<char>() }).collect::<String>();
//    let str2 = (0..20000).map(|_| { rand::random::<char>() }).collect::<String>();
//
//    b.iter(|| {
//        sorensen(
//            str1.as_str().as_bytes(),
//            str2.as_str().as_bytes()
//        )
//    })
//}
//
//fn intersection_200000_long_bench(b: &mut Bencher) {
//    let str1 = (0..200000).map(|_| { rand::random::<char>() }).collect::<String>();
//    let str2 = (0..200000).map(|_| { rand::random::<char>() }).collect::<String>();
//
//    b.iter(|| {
//        sorensen(
//            str1.as_str().as_bytes(),
//            str2.as_str().as_bytes()
//        )
//    })
//}
//
//fn intersection_2000000_long_bench(b: &mut Bencher) {
//    let str1 = (0..2_000_000).map(|_| { rand::random::<char>() }).collect::<String>();
//    let str2 = (0..2_000_000).map(|_| { rand::random::<char>() }).collect::<String>();
//
//    b.iter(move || {
//        sorensen(
//            str1.as_str().as_bytes(),
//            str2.as_str().as_bytes()
//        )
//    })
//}

fn intersection_1000000_long_vec_bench(c: &mut Criterion) {
    c.bench_function(" 1_000_000 vec", |b| {
        let str1 = (0..100000).map(|_| { rand::random::<char>() }).collect::<Vec<char>>();
        let str2 = (0..100000).map(|_| { rand::random::<char>() }).collect::<Vec<char>>();
        b.iter(|| sorensen::sorensen(&str1, &str2));
    });
}

criterion_group!(benches, intersection_1000000_long_vec_bench);
criterion_main!(benches);