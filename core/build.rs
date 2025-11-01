//! Build script for StrataRouter core

fn main() {
    // Set environment variables for PyO3
    if cfg!(feature = "python") {
        println!("cargo:rerun-if-changed=src/");
    }
    
    // Enable AVX2 detection (runtime check, not compile-time)
    #[cfg(target_arch = "x86_64")]
    {
        println!("cargo:rustc-cfg=target_feature_checked");
    }
    
    // Set build timestamp
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", 
             std::time::SystemTime::now()
                 .duration_since(std::time::UNIX_EPOCH)
                 .unwrap()
                 .as_secs());
    
    println!("cargo:rerun-if-changed=build.rs");
}
