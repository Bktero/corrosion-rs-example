# Example of corrosion-rs to build and install a C library written in Rust

From your terminal:

```shell
cmake -S . -B cmake-build -DCMAKE_INSTALL_PREFIX=install
cd cmake-build/
make install
```

Result:

```shell
$ cd ..
$ tree install
install
├── include
│   └── bakasable.h
└── lib
    └── libbakasable_rs.a

3 directories, 2 files
```
