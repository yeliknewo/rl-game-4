use std::sync::{Arc};

use components::{RenderId, Transform, Camera, RenderData};
use dependencies::specs::{RunArg, System};
use dependencies::gfx::traits::{Factory, FactoryExt};
use dependencies::gfx::{Primitive};
use dependencies::gfx::tex::{FilterMethod, SamplerInfo, WrapMode};
use event::{BackChannel};
use graphics::{OutColor, OutDepth, Bundle, Shaders, make_shaders, GlEncoder, GlFactory, Packet, GlTexture, pipe};
use utils::{Delta};

pub enum ToRender {
    Encoder(GlEncoder),
}

pub enum FromRender {
    Encoder(GlEncoder),
}

pub struct RenderSystem {
    back_channel: BackChannel<ToRender, FromRender>,
    out_color: OutColor,
    out_depth: OutDepth,
    bundles: Arc<Vec<Bundle>>,
    shaders: Shaders,
}

impl RenderSystem {
    pub fn new(back_channel: BackChannel<ToRender, FromRender>, out_color: OutColor, out_depth: OutDepth) -> RenderSystem {
        RenderSystem {
            back_channel: back_channel,
            out_color: out_color,
            out_depth: out_depth,
            bundles: Arc::new(vec!()),
            shaders: make_shaders(),
        }
    }

    pub fn add_render(&mut self,
        factory: &mut GlFactory,
        packet: &Packet,
        texture: GlTexture
    ) -> RenderId {
        warn!("Creating Shader Set");
        let shader_set = factory.create_shader_set(self.shaders.get_vertex_shader(), self.shaders.get_fragment_shader()).unwrap_or_else(|err| panic!("Create Shader Set Error: {:?}", err));

        warn!("Creating Program");
        let program = factory.create_program(&shader_set).unwrap_or_else(|err| panic!("Create Program Error: {:?}", err));

        warn!("Creating Pipeline from Program");
        let pso = factory.create_pipeline_from_program(
            &program,
            Primitive::TriangleList,
            packet.get_rasterizer(),
            pipe::new()
        ).unwrap_or_else(|err| panic!("Create Pipeline from Program Error: {:?}", err));

        warn!("Creating Sampler Info");
        let sampler_info = SamplerInfo::new(
            FilterMethod::Scale,
            WrapMode::Mirror,
        );

        warn!("Creating Vertex Buffer");
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(packet.get_vertices(), packet.get_indices());

        warn!("Creating Pipe Data");
        let data = pipe::Data {
            vbuf: vbuf,
            spritesheet: (texture, factory.create_sampler(sampler_info)),
            texture_data: factory.create_constant_buffer(1),
            projection_data: factory.create_constant_buffer(1),
            out_color: self.out_color.clone(),
            out_depth: self.out_depth.clone(),
        };

        warn!("Creating Id");


        warn!("Getting Bundles as Mutable");
        let mut bundles = Arc::get_mut(&mut self.bundles).unwrap_or_else(|| panic!("Arc Shit"));

        let id = bundles.len();

        warn!("Adding new bundle to Bundles");
        bundles.push(Bundle::new(slice, pso, data));

        warn!("Returning Render Id");
        RenderId::new(id)
    }

    fn render(&mut self, arg: &RunArg, mut encoder: GlEncoder) {
        use dependencies::specs::Join;

        let (render_ids, mut transforms, mut cameras, mut render_datas) = arg.fetch(|w|
            (
                w.read::<RenderId>(),
                w.write::<Transform>(),
                w.write::<Camera>(),
                w.write::<RenderData>()
            )
        );
    }

    fn process_event(&mut self, arg: &RunArg, event: ToRender) -> bool {
        match event {
            ToRender::Encoder(encoder) => {
                self.render(arg, encoder);
                false
            },
        }
    }
}

impl System<Delta> for RenderSystem {
    fn run(&mut self, arg: RunArg, _: Delta) {
        let mut event = self.back_channel.recv_to();
        while self.process_event(&arg, event) {
            event = self.back_channel.recv_to();
        }
    }
}
