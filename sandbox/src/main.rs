extern crate edocore;

use edocore::debug;
use edocore::{
    math::{
        vector::Vector3,
    },
};

extern crate edorenderer;
use edorenderer::{
    backend,
    states::{
        halstate::HalState,
        types::{mesh::Mesh, image::Image, rendertext::RenderText},
        pipelineconfig::PipelineConfig,
        meshbuffer::MeshBuffer,
    },
};

use gfx_hal::{
    window,
    pso,
};
use log::{debug, error, info, trace, warn, Level, LevelFilter};

pub mod utils;

pub mod voxel_data;
use voxel_data::voxelgrid::VoxelGrid;
use std::mem;

use edocore::math::vector::{UVector3};

fn main() {
    // debug::log(&*format!("What's good lads, 512x512x512 u8's would take up {} memory", mem::size_of::<VoxelGrid>() ));
    let mut builder = pretty_env_logger::formatted_builder();
    builder.filter(Some("sandbox"), LevelFilter::max()); //Replace None with Some("library_name") to make it only output that stuff. Example: Some("edorenderer")
    builder.init();

    debug::log("hello world!");

    let mut pc = PipelineConfig::new();
    pc.add_descriptor_layouts(vec!(
        pso::DescriptorSetLayoutBinding {
            binding: 0,
            ty: pso::DescriptorType::SampledImage,
            count: 1,
            stage_flags: pso::ShaderStageFlags::FRAGMENT,
            immutable_samplers: false,
        },
        //Yeet
        pso::DescriptorSetLayoutBinding {
            binding: 1,
            ty: pso::DescriptorType::Sampler,
            count: 1,
            stage_flags: pso::ShaderStageFlags::FRAGMENT,
            immutable_samplers: false,
        }
    ));
    pc.add_descriptor_range(pso::DescriptorRangeDesc {
        ty: pso::DescriptorType::SampledImage,
        count: 1,
    });
    pc.add_descriptor_range(pso::DescriptorRangeDesc {
        ty: pso::DescriptorType::Sampler,
        count: 1,
    });

    let event_loop = winit::event_loop::EventLoop::new();

    let mut halstate = HalState::new(&event_loop, pc);

    let text = RenderText::new(&mut halstate.renderer, String::from("u gay"));

    let mut vg = VoxelGrid::new(UVector3::new(512, 512, 512));

    let mut mesh1 = Mesh::<backend::Backend>::new_quad(&mut halstate.renderer, Vector3::new( 0.5,-0.5, 0.0), Image::default());
    let mesh1_idx = halstate.add_mesh(mesh1);

    let mut mesh2 = Mesh::<backend::Backend>::new_quad(&mut halstate.renderer, Vector3::new(-0.5,-0.5, 0.0), Image::default());
    let mesh2_idx = halstate.add_mesh(mesh2);

    // let mut mesh3 = Mesh::<backend::Backend>::new_quad(&mut halstate.renderer, Vector3::new( 0.0, 0.5, 0.0), Image::default());
    // let mesh3_idx = halstate.add_mesh(mesh3);
    // let mesh3_idx = halstate.create_text(String::from("u gay"));
    let mesh3_idx = halstate.add_mesh(&*text.meshes[0]);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;

        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit
                }
                winit::event::WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = winit::event_loop::ControlFlow::Exit,
                winit::event::WindowEvent::Resized(dims) => {
                    halstate.resize(dims);
                }
                _ => {}
            },
            winit::event::Event::EventsCleared => {
                halstate.render_mesh(mesh1_idx);
                halstate.render_mesh(mesh2_idx);
                halstate.render_mesh(mesh3_idx);
                halstate.render();
            }
            _ => {}
        }
    });

    //This should never happen, as the above function will handle program exit and everything.
    println!("EdoRenderer ded :)");
}
