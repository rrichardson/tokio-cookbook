#![warn(rust_2018_idioms)]

use futures::future::{
    Ready,
    ready,
};
use futures::stream::{
    futures_unordered::FuturesUnordered,
    StreamExt
};
use tokio::{
    self,
    runtime,
};

fn main() {
    // First we create a custom runtime that includes a threadpool
    // large enough to parallellize our computations
    let rt = runtime::Builder::new()
        .blocking_threads(4)
        .core_threads(4)
        .name_prefix("compute")
        .build()
        .unwrap();

    let futs = (1..20).map(|i| expensive_future(i)).collect::<FuturesUnordered<_>>();

    let results = rt.block_on(futs.collect::<Vec<usize>>());
    println!("results: {:?}", results);
}

// The Ready struct is returned by `ready` and it implements Future
fn expensive_future(i: usize) -> Ready<usize> {
    ready(usize::pow(i, 4))
}
