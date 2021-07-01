use std::ffi::CString;
use std::ptr;

use ash::version::DeviceV1_0;
use ash::vk;

use super::{DescriptorSetLayout, LogicalDevice, RenderPass, SwapChain};

pub struct Pipeline {
    pub value: vk::Pipeline,
    pub layout: vk::PipelineLayout,
}

impl Pipeline {
    pub fn new(
        logical_device: &LogicalDevice,
        swap_chain: &SwapChain,
        render_pass: &RenderPass,
        descriptor_set_layout: &DescriptorSetLayout,
    ) -> Self {
        let vert_shader_source =
            std::fs::read("shaders/spv/voxel.vert.spv").expect("Failed to load vertex shader");
        let frag_shader_source =
            std::fs::read("shaders/spv/voxel.frag.spv").expect("Failed to load fragment shader");

        let vert_shader = Self::create_shader_module(logical_device, vert_shader_source);
        let frag_shader = Self::create_shader_module(logical_device, frag_shader_source);
        let name = CString::new("main").unwrap();

        let vert_shader_create_info = vk::PipelineShaderStageCreateInfo {
            s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineShaderStageCreateFlags::empty(),
            stage: vk::ShaderStageFlags::VERTEX,
            module: vert_shader,
            p_name: name.as_ptr(),
            p_specialization_info: ptr::null(),
        };

        let frag_shader_create_info = vk::PipelineShaderStageCreateInfo {
            s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineShaderStageCreateFlags::empty(),
            stage: vk::ShaderStageFlags::FRAGMENT,
            module: frag_shader,
            p_name: name.as_ptr(),
            p_specialization_info: ptr::null(),
        };

        let shader_stages = [vert_shader_create_info, frag_shader_create_info];

        let vertex_input_create_info = vk::PipelineVertexInputStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineVertexInputStateCreateFlags::empty(),
            vertex_binding_description_count: 0,
            p_vertex_binding_descriptions: ptr::null(),
            vertex_attribute_description_count: 0,
            p_vertex_attribute_descriptions: ptr::null(),
        };

        let input_assembly_create_info = vk::PipelineInputAssemblyStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineInputAssemblyStateCreateFlags::empty(),
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart_enable: vk::FALSE,
        };

        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: swap_chain.extent.width as f32,
            height: swap_chain.extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: swap_chain.extent,
        };

        let viewport_create_info = vk::PipelineViewportStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineViewportStateCreateFlags::empty(),
            viewport_count: 1,
            p_viewports: &viewport,
            scissor_count: 1,
            p_scissors: &scissor,
        };

        let rasterizer_create_info = vk::PipelineRasterizationStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineRasterizationStateCreateFlags::empty(),
            depth_clamp_enable: vk::FALSE,
            rasterizer_discard_enable: vk::FALSE,
            polygon_mode: vk::PolygonMode::FILL,
            cull_mode: vk::CullModeFlags::BACK,
            front_face: vk::FrontFace::COUNTER_CLOCKWISE,
            depth_bias_enable: vk::FALSE,
            depth_bias_constant_factor: 0.0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
            line_width: 1.0,
        };

        let multisample_create_info = vk::PipelineMultisampleStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineMultisampleStateCreateFlags::empty(),
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            sample_shading_enable: vk::FALSE,
            min_sample_shading: 1.0,
            p_sample_mask: ptr::null(),
            alpha_to_coverage_enable: vk::FALSE,
            alpha_to_one_enable: vk::FALSE,
        };

        let color_blend_attachment = vk::PipelineColorBlendAttachmentState {
            blend_enable: vk::TRUE,
            src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::R
                | vk::ColorComponentFlags::G
                | vk::ColorComponentFlags::B
                | vk::ColorComponentFlags::A,
        };

        let color_blend_create_info = vk::PipelineColorBlendStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineColorBlendStateCreateFlags::empty(),
            logic_op_enable: vk::FALSE,
            logic_op: vk::LogicOp::COPY,
            attachment_count: 1,
            p_attachments: &color_blend_attachment,
            blend_constants: [0.0; 4],
        };

        let layout_create_info = vk::PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: 1,
            p_set_layouts: &descriptor_set_layout.value,
            push_constant_range_count: 0,
            p_push_constant_ranges: ptr::null(),
        };

        let layout = unsafe {
            logical_device
                .value
                .create_pipeline_layout(&layout_create_info, None)
        }
        .expect("Failed to create pipeline layout");

        let pipeline_create_info = vk::GraphicsPipelineCreateInfo {
            s_type: vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineCreateFlags::empty(),
            stage_count: 2,
            p_stages: shader_stages.as_ptr(),
            p_vertex_input_state: &vertex_input_create_info,
            p_input_assembly_state: &input_assembly_create_info,
            p_tessellation_state: ptr::null(),
            p_viewport_state: &viewport_create_info,
            p_rasterization_state: &rasterizer_create_info,
            p_multisample_state: &multisample_create_info,
            p_depth_stencil_state: ptr::null(),
            p_color_blend_state: &color_blend_create_info,
            p_dynamic_state: ptr::null(),
            layout,
            render_pass: render_pass.value,
            subpass: 0,
            base_pipeline_handle: vk::Pipeline::null(),
            base_pipeline_index: -1,
        };

        let value = unsafe {
            logical_device.value.create_graphics_pipelines(
                vk::PipelineCache::null(),
                &[pipeline_create_info],
                None,
            )
        }
        .expect("Failed to create pipeline");

        unsafe {
            logical_device
                .value
                .destroy_shader_module(vert_shader, None);
            logical_device
                .value
                .destroy_shader_module(frag_shader, None);
        }

        Self {
            value: value[0],
            layout,
        }
    }

    pub fn destroy(&mut self, logical_device: &LogicalDevice) {
        unsafe {
            logical_device.value.destroy_pipeline(self.value, None);
            logical_device
                .value
                .destroy_pipeline_layout(self.layout, None);
        }
    }

    fn create_shader_module(logical_device: &LogicalDevice, source: Vec<u8>) -> vk::ShaderModule {
        let shader_module_create_info = vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::ShaderModuleCreateFlags::empty(),
            code_size: source.len(),
            p_code: source.as_ptr() as _,
        };

        unsafe {
            logical_device
                .value
                .create_shader_module(&shader_module_create_info, None)
        }
        .expect("Failed to create shader module")
    }
}
