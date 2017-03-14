extern crate gcc;

fn main() {
    gcc::compile_library("libtunalloc.a", &["src/tun_alloc.c"]);
}