extern crate cmake;
use cmake::Config;

fn main()
{
    let dst = Config::new("vendor/realm-core").build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
//    println!("cargo:rustc-link-search=static={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=realm-dbg");
}
