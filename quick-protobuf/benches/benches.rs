use std::hint::black_box;
use std::time::Instant;

use lazy_static::lazy_static;
use quick_protobuf::{BytesReader, Writer};

const LEN: usize = 10_000;

lazy_static! {
    static ref BUFFER: Vec<u8> = {
        let mut buf = Vec::new();
        let mut writer = Writer::new(&mut buf);
        for i in 0..LEN {
            writer.write_int32(i as i32).unwrap();
        }
        buf
    };
}

fn run_bench(name: &str, iterations: u64, mut f: impl FnMut()) {
    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let elapsed = start.elapsed();
    let ns_per_iter = elapsed.as_nanos() as f64 / iterations as f64;
    println!("{:<28} {:>12.2} ns/iter", name, ns_per_iter);
}

fn read_varint32() {
    let mut reader = BytesReader::from_bytes(&BUFFER);
    for _ in 0..LEN {
        let _ = black_box(reader.read_varint32(&BUFFER).unwrap());
    }
    assert!(reader.is_eof());
}

fn read_varint64() {
    let mut reader = BytesReader::from_bytes(&BUFFER);
    for _ in 0..LEN {
        let _ = black_box(reader.read_varint64(&BUFFER).unwrap());
    }
    assert!(reader.is_eof());
}

fn read_varint64_and_is_eof() {
    let mut reader = BytesReader::from_bytes(&BUFFER);
    for _ in 0..LEN {
        assert!(!reader.is_eof());
        let _ = black_box(reader.read_varint64(&BUFFER).unwrap());
    }
}

fn main() {
    let iterations = std::env::var("BENCH_ITERS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(500);

    println!("Running quick-protobuf micro benches ({iterations} iterations each)");
    run_bench("read_varint32", iterations, read_varint32);
    run_bench("read_varint64", iterations, read_varint64);
    run_bench(
        "read_varint64_and_is_eof",
        iterations,
        read_varint64_and_is_eof,
    );
}
