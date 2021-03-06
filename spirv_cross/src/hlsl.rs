use super::ErrorCode;
use spirv;
use bindings::root::*;
use std::ptr;
use std::ffi::CStr;
use std::os::raw::c_void;

#[allow(non_snake_case, non_camel_case_types)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ShaderModel {
    V3_0,
    V4_0,
    V4_0L9_0,
    V4_0L9_1,
    V4_0L9_3,
    V4_1,
    V5_0,
    V5_1,
    V6_0,
}

#[allow(non_snake_case, non_camel_case_types)]
impl ShaderModel {
    fn as_raw(&self) -> i32 {
        match *self {
            ShaderModel::V3_0 => 30,
            ShaderModel::V4_0 => 40,
            ShaderModel::V4_0L9_0 => 40,
            ShaderModel::V4_0L9_1 => 40,
            ShaderModel::V4_0L9_3 => 40,
            ShaderModel::V4_1 => 41,
            ShaderModel::V5_0 => 50,
            ShaderModel::V5_1 => 51,
            ShaderModel::V6_0 => 60,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompilerVertexOptions {
    pub invert_y: bool,
    pub transform_clip_space: bool,
}

impl Default for CompilerVertexOptions {
    fn default() -> CompilerVertexOptions {
        CompilerVertexOptions {
            invert_y: false,
            transform_clip_space: false,
        }
    }
}


#[derive(Debug, Clone)]
pub struct CompilerOptions {
    pub shader_model: ShaderModel,
    pub vertex: CompilerVertexOptions,
}

impl CompilerOptions {
    fn as_raw(&self) -> ScHlslCompilerOptions {
        ScHlslCompilerOptions {
            shader_model: self.shader_model.as_raw(),
            vertex_invert_y: self.vertex.invert_y,
            vertex_transform_clip_space: self.vertex.transform_clip_space,
        }
    }
}

impl Default for CompilerOptions {
    fn default() -> CompilerOptions {
        CompilerOptions {
            shader_model: ShaderModel::V3_0,
            vertex: CompilerVertexOptions::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Compiler {
    _unconstructable: (),
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler { _unconstructable: () }
    }

    pub fn compile(
        &self,
        parsed_module: &spirv::ParsedModule,
        options: &CompilerOptions,
    ) -> Result<String, ErrorCode> {
        unsafe {
            let mut hlsl_ptr = ptr::null();
            check!(sc_internal_compiler_hlsl_compile(
                parsed_module.ir.as_ptr() as *const u32,
                parsed_module.ir.len() as usize,
                &mut hlsl_ptr,
                &options.as_raw(),
            ));
            let hlsl = match CStr::from_ptr(hlsl_ptr).to_owned().into_string() {
                Err(_) => return Err(ErrorCode::Unhandled),
                Ok(v) => v,
            };
            check!(sc_internal_free_pointer(hlsl_ptr as *mut c_void));
            Ok(hlsl)
        }
    }
}
