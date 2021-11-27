# To Run

```
rustup run nightly cargo bench
```

## Results

Results on my MacBook Air (M1, 2020) running Big Sur 11.0.1:
```
running 2 tests
test btree_map ... bench:      24,106 ns/iter (+/- 949)
test index_map ... bench:      93,411 ns/iter (+/- 2,936)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured
```
