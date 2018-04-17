use std::env;
use std::process::Command;


fn main() {
   println!("cargo:rustc-link-search=./clips_core_source_630/core");
   println!("cargo:rustc-link-lib=static=clips");
   println!("cargo:rerun-if-changed=build.rs");
   let clips_core_dir = env::current_dir().unwrap().join("clips_core_source_630/core");
   Command::new("make").current_dir(&clips_core_dir).args(&["-f","../makefiles/makefile.lib"]).status().unwrap();
   
}
