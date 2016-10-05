pub mod glutin {
    use graphics::{GfxWindow};
    use graphics::rl_glutin::{Window, Extras};
    use event_clump::{FrontEventClump};

    pub fn handle_events(gfx_window: &mut GfxWindow<Window, Extras>, front_event_clump: &mut FrontEventClump) -> bool {
        false
    }
}

pub mod sdl2 {
    use dependencies::sdl2::event::{EventType, Event, WindowEventId};
    use dependencies::sdl2::keyboard::{Keycode};
    use dependencies::sdl2::controller::{Axis, Button};
    use graphics::{GfxWindow};
    use graphics::rl_sdl2::{Window, Extras};
    use systems::control::{ToControl};
    use event_clump::{FrontEventClump};

    pub fn handle_events(gfx_window: &mut GfxWindow<Window, Extras>, front_event_clump: &mut FrontEventClump) -> bool {
        if gfx_window.get_extras().1.is_none() {
            let mut event_pump = gfx_window.get_mut_window().subsystem().sdl().event_pump().unwrap_or_else(|err| panic!(err));
            event_pump.enable_event(EventType::KeyDown);
            event_pump.enable_event(EventType::KeyUp);
            event_pump.enable_event(EventType::Window);
            event_pump.enable_event(EventType::ControllerAxisMotion);
            event_pump.enable_event(EventType::ControllerButtonDown);
            event_pump.enable_event(EventType::ControllerButtonUp);
            gfx_window.get_mut_extras().1 = Some(event_pump);
        }

        let mut event_pump = gfx_window.get_mut_extras().1.as_mut().unwrap_or_else(|| panic!("Event Pump was None"));

        while let Some(event) = event_pump.poll_event() {
            match event {
                Event::Window {
                    timestamp,
                    window_id,
                    win_event_id,
                    data1,
                    data2,
                } => {
                    match win_event_id {
                        WindowEventId::Close => {
                            return true;
                        },
                        _ => {

                        },
                    }
                },
                Event::KeyDown {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                } => {
                    match keycode {
                        Some(Keycode::Escape) => {
                            return true;
                        },
                        Some(Keycode::Up) => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Up(1.0));
                        },
                        Some(Keycode::Down) => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Down(1.0));
                        },
                        Some(Keycode::Left) => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Left(1.0));
                        },
                        Some(Keycode::Right) => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Right(1.0));
                        },
                        _ => {

                        },
                    }
                },
                Event::KeyUp {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                } => {
                    match keycode {
                        Some(Keycode::Escape) => {
                            return true;
                        },
                        Some(Keycode::Up) => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Up(0.0));
                        },
                        Some(Keycode::Down) => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Down(0.0));
                        },
                        Some(Keycode::Left) => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Left(0.0));
                        },
                        Some(Keycode::Right) => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Right(0.0));
                        },
                        _ => {

                        },
                    }
                },
                Event::ControllerAxisMotion {
                    timestamp,
                    which,
                    axis,
                    value,
                } => {
                    match axis {
                        Axis::LeftX => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(if value >= 0 {
                                ToControl::Right((value / ::std::i16::MAX) as f64)
                            } else {
                                ToControl::Left((value / ::std::i16::MIN).abs() as f64)
                            });
                        },
                        _ => {

                        },
                    }
                },
                Event::ControllerButtonDown {
                    timestamp,
                    which,
                    button,
                } => {
                    match button {
                        Button::DPadRight => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Right(1.0));
                        },
                        Button::DPadLeft => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Left(1.0));
                        },
                        Button::A => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Up(1.0));
                        }
                        _ => {

                        },
                    }
                },
                Event::ControllerButtonUp {
                    timestamp,
                    which,
                    button,
                } => {
                    match button {
                        Button::DPadRight => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Right(0.0));
                        },
                        Button::DPadLeft => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Left(0.0));
                        },
                        Button::A => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Up(0.0));
                        }
                        _ => {

                        },
                    }
                },
                _ => {

                },
            }
        }

        false
    }
}
