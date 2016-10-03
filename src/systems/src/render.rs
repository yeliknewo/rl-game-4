use event::{BackChannel};
use graphics::{OutColor, OutDepth, Bundle, Shaders, make_shaders};

pub enum ToRender {

}

pub enum FromRender {

}

pub struct RenderSystem {
    back_channel: BackChannel<ToRender, FromRender>,
    out_color: OutColor,
    out_depth: OutDepth,
    bundles: Vec<Bundle>,
    shaders: Shaders,
}

impl RenderSystem {
    pub fn new(back_channel: BackChannel<ToRender, FromRender>, out_color: OutColor, out_depth: OutDepth) -> RenderSystem {
        RenderSystem {
            back_channel: back_channel,
            out_color: out_color,
            out_depth: out_depth,
            bundles: vec!(),
            shaders: make_shaders(),
        }
    }
}
