fn main() {
    // DuckDB uses Windows Restart Manager APIs (RmStartSession, RmEndSession,
    // RmRegisterResources, RmGetList) which require linking against Rstrtmgr.lib.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        println!("cargo:rustc-link-lib=Rstrtmgr");
    }
}
