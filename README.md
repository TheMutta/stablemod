# Stablemod
## A race worthy meta-operating system

Stablemod (previously MicroKosm) is an security-oriented experimental meta-operating system first created in C and then evolved in the Rust programming language. It received second place in EUCYS 2023 for its security oriented platform and ideals for what at the time was only a twinkle in my eye.

Today, stablemod is being constantly developing and is racing towards its first alpha release. It is **NOT** ready for use, not nearly so. Expect frequent updates being pushed and code being modified until the API stabilizes.

### Building and Testing
In the repo are provided scripts to handly compile all the subrepos and to test them with the obviously named script.

A new config utility couple with a comprehensive expandable build system is in the works for the near future.

#### Requirements
 - Qemu System x86_64, Aarch64, Riscv64
 - Rust Nightly toolchain with cargo
 - wget

#### Building
To build the entire system:
 ```
./build.sh <arch>
 ```

 To test it:
 ```
 ./test.sh <arch>
 ```

 And to clean up for a clean build:
```
 ./clean.sh
```

