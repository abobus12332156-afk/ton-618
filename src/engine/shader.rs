use ash::util;
use ash::vk;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;

pub struct ShaderModule {
    pub module: vk::ShaderModule,
}

impl ShaderModule {
    pub fn new(device: &ash::Device, path: &str) -> Self {
        let mut file = File::open(path).expect("Failed to open shader");

        let mut bytes = Vec::new();

        file.read_to_end(&mut bytes).expect("Failed to read shader");

        let code = util::read_spv(&mut Cursor::new(bytes)).expect("Invalid SPIR-V");

        let create_info = vk::ShaderModuleCreateInfo::default().code(&code);

        let module = unsafe {
            device
                .create_shader_module(&create_info, None)
                .expect("Failed to create shader module")
        };

        println!("Shader loaded: {}", path);

        Self { module }
    }
}
