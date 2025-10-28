# Rust-main

A simple rust project to determine minimimum of the binary.
This mimics the code in [rust-main-no-std](github.com/winksaville/rust-main-no-std)
that used `#[no_std]` and `#[no_main]` that code generated
a binary of about 1040 bytes. The release build is > 319,000 bytes
and the development build is 3.9 MB.


## Run

Type CTRL-C to exit the running program.

```
wink@3900x 25-10-28T17:35:06.495Z:~/data/prgs/rust/rust-main (main)
$ cargo run --release
    Finished `release` profile [optimized] target(s) in 0.03s
     Running `target/release/rust-main`
argc = 1
hello
hello
^C
wink@3900x 25-10-28T17:38:38.352Z:~/data/prgs/rust/rust-main (main)
```

## Build

This series of builds show changes to the [profile.release]
in Cargo.toml and the effect of various options on the binary size.

The smallest binary size with all optimizations is 319,336 bytes.


```
wink@3900x 25-10-28T17:42:07.951Z:~/data/prgs/rust/rust-main (main)
$ rg profile\.release -A 4 Cargo.toml ; cargo clean ; cargo build --release ; ls -l target/release/rust-main
9:[profile.release]
10-#strip = true
11-#lto = true
12-#opt-level = "z"
13-#codegen-units = 1
     Removed 11 files, 904.3KiB total
   Compiling rust-main v0.1.0 (/home/wink/data/prgs/rust/rust-main)
    Finished `release` profile [optimized] target(s) in 0.13s
-rwxr-xr-x 2 wink users 461992 Oct 28 10:42 target/release/rust-main
wink@3900x 25-10-28T17:42:29.070Z:~/data/prgs/rust/rust-main (main)
$ rg profile\.release -A 4 Cargo.toml ; cargo clean ; cargo build --release ; ls -l target/release/rust-main
9:[profile.release]
10-strip = true
11-#lto = true
12-#opt-level = "z"
13-#codegen-units = 1
     Removed 34 files, 1.1MiB total
   Compiling rust-main v0.1.0 (/home/wink/data/prgs/rust/rust-main)
    Finished `release` profile [optimized] target(s) in 0.13s
-rwxr-xr-x 2 wink users 373656 Oct 28 10:42 target/release/rust-main
wink@3900x 25-10-28T17:42:45.581Z:~/data/prgs/rust/rust-main (main)
$ rg profile\.release -A 4 Cargo.toml ; cargo clean ; cargo build --release ; ls -l target/release/rust-main
9:[profile.release]
10-strip = true
11-lto = true
12-#opt-level = "z"
13-#codegen-units = 1
     Removed 34 files, 909.6KiB total
   Compiling rust-main v0.1.0 (/home/wink/data/prgs/rust/rust-main)
    Finished `release` profile [optimized] target(s) in 2.32s
-rwxr-xr-x 2 wink users 336184 Oct 28 10:42 target/release/rust-main
wink@3900x 25-10-28T17:42:57.418Z:~/data/prgs/rust/rust-main (main)
$ rg profile\.release -A 4 Cargo.toml ; cargo clean ; cargo build --release ; ls -l target/release/rust-main
9:[profile.release]
10-strip = true
11-lto = true
12-opt-level = "z"
13-#codegen-units = 1
     Removed 34 files, 836.5KiB total
   Compiling rust-main v0.1.0 (/home/wink/data/prgs/rust/rust-main)
    Finished `release` profile [optimized] target(s) in 2.12s
-rwxr-xr-x 2 wink users 319336 Oct 28 10:43 target/release/rust-main
wink@3900x 25-10-28T17:43:08.469Z:~/data/prgs/rust/rust-main (main)
$ rg profile\.release -A 4 Cargo.toml ; cargo clean ; cargo build --release ; ls -l target/release/rust-main
9:[profile.release]
10-strip = true
11-lto = true
12-opt-level = "z"
13-codegen-units = 1
     Removed 34 files, 803.5KiB total
   Compiling rust-main v0.1.0 (/home/wink/data/prgs/rust/rust-main)
    Finished `release` profile [optimized] target(s) in 2.14s
-rwxr-xr-x 2 wink users 319336 Oct 28 10:43 target/release/rust-main
wink@3900x 25-10-28T17:43:24.241Z:~/data/prgs/rust/rust-main (main)
```

For completeness is the debug/development build size:

3980848 bytes, about 3.9 MB.

```
wink@3900x 25-10-28T17:45:34.477Z:~/data/prgs/rust/rust-main (main)
$ cargo clean ; cargo build ; ls -l target/debug/rust-main
     Removed 36 files, 8.6MiB total
   Compiling rust-main v0.1.0 (/home/wink/data/prgs/rust/rust-main)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
-rwxr-xr-x 2 wink users 3980848 Oct 28 10:45 target/debug/rust-main
wink@3900x 25-10-28T17:45:58.761Z:~/data/prgs/rust/rust-main (main)
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
