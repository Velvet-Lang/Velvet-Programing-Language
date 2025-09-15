// velvet-core/src/ffi.rs
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn parse_velvet_ffi(input: *const c_char) -> *const c_char {
    unsafe {
        let input_str = CStr::from_ptr(input).to_str().unwrap();
        match crate::parse_velvet(input_str) {
            Ok(ast) => {
                let ast_str = format!("{:?}", ast);
                CString::new(ast_str).unwrap().into_raw()
            }
            Err(e) => CString::new(format!("Error: {}", e)).unwrap().into_raw(),
        }
    }
}

#[no_mangle]
pub extern "C" fn compile_to_rust_ffi(ast_ptr: *const c_char, output: *const c_char) {
    unsafe {
        let ast_str = CStr::from_ptr(ast_ptr).to_str().unwrap();
        // Parse z string (uproszczone)
        let ast: Vec<crate::VelvetAst> = serde_json::from_str(ast_str).unwrap();  // Zakładamy JSON serialize
        let out = CStr::from_ptr(output).to_str().unwrap();
        let _ = crate::compile_to_rust(&ast, out);
    }
}
