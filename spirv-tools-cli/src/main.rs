use libc::free;
use std::{ffi::c_void, ptr::null_mut};

fn main() {
    println!("Hello, world!");
    unsafe {
        let path_in = std::env::args().nth(1).unwrap();
        let path_out = std::env::args().nth(2).unwrap();
        let optim =
            spirv_tools_sys::spvOptimizerCreate(spirv_tools_sys::spv_target_env_SPV_ENV_VULKAN_1_3);
        let options = spirv_tools_sys::spvOptimizerOptionsCreate();
        let data = std::fs::read(path_in).unwrap();
        let mut optimized = null_mut();
        spirv_tools_sys::spvOptimizerRun(
            optim,
            data.as_ptr() as *const u32,
            data.len() / 4,
            &mut optimized,
            options,
        );
        let optimized = &mut *optimized;
        std::fs::write(
            path_out,
            std::slice::from_raw_parts(optimized.code as *const u8, optimized.wordCount * 4),
        )
        .unwrap();
        free(optimized.code as *mut c_void);
        free(optimized as *mut spirv_tools_sys::spv_binary_t as *mut c_void);
        spirv_tools_sys::spvOptimizerOptionsDestroy(options);
        spirv_tools_sys::spvOptimizerDestroy(optim);
    }
}
