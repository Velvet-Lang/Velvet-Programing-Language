use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use serde_json;

#[no_mangle]
pub extern "C" fn parse_velvet_ffi(input: *const c_char) -> *const c_char {
    unsafe {
        let input_str = CStr::from_ptr(input).to_str().unwrap();
        match crate::parse_velvet(input_str) {
            Ok(ast) => CString::new(serde_json::to_string(&ast).unwrap()).unwrap().into_raw(),
            Err(e) => CString::new(format!("Error: {}", e)).unwrap().into_raw(),
        }
    }
}

#[no_mangle]
pub extern "C" fn scan_project_ffi(dir: *const c_char) -> *const c_char {
    unsafe {
        let dir_str = CStr::from_ptr(dir).to_str().unwrap();
        match crate::scan_project(dir_str) {
            Ok((deps, errs)) => CString::new(serde_json::to_string(&(deps, errs)).unwrap()).unwrap().into_raw(),
            Err(e) => CString::new(format!("Error: {}", e)).unwrap().into_raw(),
        }
    }
}

#[no_mangle]
pub extern "C" fn compile_to_rust_ffi(ast_ptr: *const c_char, output: *const c_char, deps_ptr: *const c_char) {
    unsafe {
        let ast_str = CStr::from_ptr(ast_ptr).to_str().unwrap();
        let ast: Vec<crate::VelvetAst> = serde_json::from_str(ast_str).unwrap();
        let deps_str = CStr::from_ptr(deps_ptr).to_str().unwrap();
        let deps: Vec<String> = serde_json::from_str(deps_str).unwrap();
        let out = CStr::from_ptr(output).to_str().unwrap();
        let _ = crate::compile_to_rust(&ast, out, &deps);
    }
}

#[no_mangle]
pub extern "C" fn run_velvet_ffi(input: *const c_char) {
    unsafe {
        let input_str = CStr::from_ptr(input).to_str().unwrap();
        let _ = crate::run_velvet(input_str);
    }
}

// Nowa: Klonuj git repo (użyj git2)
#[no_mangle]
pub extern "C" fn clone_git_repo_ffi(repo: *const c_char, dest: *const c_char) -> *const c_char {
    unsafe {
        let repo_str = CStr::from_ptr(repo).to_str().unwrap();
        let dest_str = CStr::from_ptr(dest).to_str().unwrap();
        match git2::Repository::clone(repo_str, dest_str) {
            Ok(_) => CString::new("OK").unwrap().into_raw(),
            Err(e) => CString::new(format!("Error: {}", e)).unwrap().into_raw(),
        }
    }
}
