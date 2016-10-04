use event::{FrontChannel, BackChannel};
use systems::control::{ToControl, FromControl};
use systems::render::{ToRender, FromRender};

pub struct BackEventClump {
    render: Option<BackChannel<ToRender, FromRender>>,
    control: Option<BackChannel<ToControl, FromControl>>,
}

impl BackEventClump {
    pub fn new(
        render: BackChannel<ToRender, FromRender>,
        control: BackChannel<ToControl, FromControl>
    ) -> BackEventClump {
        BackEventClump {
            render: Some(render),
            control: Some(control),
        }
    }

    pub fn take_render(&mut self) -> Option<BackChannel<ToRender, FromRender>> {
        self.render.take()
    }

    pub fn take_control(&mut self) -> Option<BackChannel<ToControl, FromControl>> {
        self.control.take()
    }
}

pub struct FrontEventClump {
    render: Option<FrontChannel<ToRender, FromRender>>,
    control: Option<FrontChannel<ToControl, FromControl>>,
}

impl FrontEventClump {
    pub fn new(
        render: FrontChannel<ToRender, FromRender>,
        control: FrontChannel<ToControl, FromControl>
    ) -> FrontEventClump {
        FrontEventClump {
            render: Some(render),
            control: Some(control),
        }
    }

    // pub fn take_render(&mut self) -> Option<FrontChannel<ToRender, FromRender>> {
    //     self.render.take()
    // }
    //
    // pub fn take_control(&mut self) -> Option<FrontChannel<ToControl, FromControl>> {
    //     self.control.take()
    // }
    //
    // pub fn give_render(&mut self, render: FrontChannel<ToRender, FromRender>) {
    //     self.render = Some(render);
    // }
    //
    // pub fn give_control(&mut self, control: FrontChannel<ToControl, FromControl>) {
    //     self.control = Some(control);
    // }

    pub fn get_mut_render(&mut self) -> Option<&mut FrontChannel<ToRender, FromRender>> {
        self.render.as_mut()
    }

    pub fn get_mut_control(&mut self) -> Option<&mut FrontChannel<ToControl, FromControl>> {
        self.control.as_mut()
    }
}
