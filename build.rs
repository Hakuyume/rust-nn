extern crate nvcc;

use std::fs;

use std::io::Write;

fn main() {
    nvcc::compile_library("libcustom_kernel.a", &["src/kernel.cu"]);

    {
        let mut file = fs::File::create("src/generic_value/values.rs").unwrap();
        for i in 0..1024 {
            file.write_fmt(format_args!("pub struct U{};\n", i))
                .unwrap();
        }
    }

    {
        let mut file = fs::File::create("src/generic_value/impls.rs").unwrap();
        file.write_all(b"use super::*;\n").unwrap();
        file.write_all(b"use super::values::*;\n").unwrap();

        for i in 0..1024 {
            file.write_fmt(format_args!("impl USize for U{} {{ const VALUE: usize = {}; }}\n",
                                        i,
                                        i))
                .unwrap();
        }

        for i in 0..32 {
            for j in i..32 {
                file.write_fmt(format_args!("impl Sub<U{}> for U{} {{ type Output = U{}; }}\n",
                                            i,
                                            j,
                                            j - i))
                    .unwrap();
            }
        }
    }
}
