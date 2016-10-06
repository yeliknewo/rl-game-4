use event::{FrontChannel, BackChannel};
use event_enums::main_x_control::{MainToControl, MainFromControl};
use event_enums::main_x_render::{MainToRender, MainFromRender};

pub struct BackEventClump {
    render: Option<BackChannel<MainToRender, MainFromRender>>,
    control: Option<BackChannel<MainToControl, MainFromControl>>,
}

impl BackEventClump {
    pub fn new(
        render: BackChannel<MainToRender, MainFromRender>,
        control: BackChannel<MainToControl, MainFromControl>
    ) -> BackEventClump {
        BackEventClump {
            render: Some(render),
            control: Some(control),
        }
    }

    pub fn take_render(&mut self) -> Option<BackChannel<MainToRender, MainFromRender>> {
        self.render.take()
    }

    pub fn take_control(&mut self) -> Option<BackChannel<MainToControl, MainFromControl>> {
        self.control.take()
    }
}

pub struct FrontEventClump {
    render: Option<FrontChannel<MainToRender, MainFromRender>>,
    control: Option<FrontChannel<MainToControl, MainFromControl>>,
}

impl FrontEventClump {
    pub fn new(
        render: FrontChannel<MainToRender, MainFromRender>,
        control: FrontChannel<MainToControl, MainFromControl>
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

    pub fn get_mut_render(&mut self) -> Option<&mut FrontChannel<MainToRender, MainFromRender>> {
        self.render.as_mut()
    }

    pub fn get_mut_control(&mut self) -> Option<&mut FrontChannel<MainToControl, MainFromControl>> {
        self.control.as_mut()
    }
}
