#[macro_use]
extern crate bencher;

use bencher::Bencher;

fn index_map(b: &mut Bencher) {
    use heapless::FnvIndexMap;
    use rand::prelude::*;
    use rand_pcg::Pcg64;

    let mut map = FnvIndexMap::<&str, u8, 6>::new();
    map.insert("ab", 1).unwrap();
    map.insert("ac", 2).unwrap();
    map.insert("ad", 3).unwrap();
    map.insert("ae", 4).unwrap();

    b.iter(move || {
        let mut rng = Pcg64::seed_from_u64(2);

        let keys = ["ab", "ac", "ad", "ae"].iter().cloned().cycle();

        for key in keys.take(5_000) {
            map.insert(key, rng.gen::<u8>()).unwrap();
        }
    });
}

fn btree_map(b: &mut Bencher) {
    use rand::prelude::*;
    use rand_pcg::Pcg64;
    use std::collections::BTreeMap;

    let mut map = BTreeMap::<[u8; 2], [u8; 2]>::new();
    map.insert([b'a', b'b'], [b'1', b'2']);
    map.insert([b'a', b'c'], [b'1', b'3']);
    map.insert([b'a', b'd'], [b'1', b'4']);

    b.iter(|| {
        let mut rng = Pcg64::seed_from_u64(2);

        let keys = [[b'a', b'b'], [b'a', b'c'], [b'a', b'd']]
            .iter()
            .cloned()
            .cycle();

        for key in keys.take(5_000) {
            map.insert(key, [rng.gen::<u8>(), rng.gen::<u8>()]);
        }
    });
}

benchmark_group!(benches, index_map, btree_map);
benchmark_main!(benches);
