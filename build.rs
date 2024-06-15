#[cfg(debug_assertions)]
fn lua51_freebsd() {
    println!("cargo:rustc-link-lib=dylib=lua-5.1");
}

#[cfg(debug_assertions)]
fn lua54_freebsd() {
    println!("cargo:rustc-link-lib=dylib=lua-5.4");
}

#[cfg(debug_assertions)]
fn lua51_notfreebsd() {
    println!("cargo:rustc-link-lib=dylib=lua5.1");
}

#[cfg(debug_assertions)]
fn lua54_notfreebsd() {
    println!("cargo:rustc-link-lib=dylib=lua5.4");
}

#[cfg(not(debug_assertions))]
fn main() {
}

#[cfg(debug_assertions)]
fn main() {
    let is_freebsd = cfg!(target_os = "freebsd");
    let is_lua51 = cfg!(feature = "lua51");
    let is_lua54 = cfg!(feature = "lua54");
    if is_freebsd {
        if is_lua51 {
            lua51_freebsd();
        }
        if is_lua54 {
            lua54_freebsd();
        }
    } else {
        if is_lua51 {
            lua51_notfreebsd();
        }
        if is_lua54 {
            lua54_notfreebsd();
        }
    }
}
