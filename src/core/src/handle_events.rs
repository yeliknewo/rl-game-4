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
    use systems::control::{ToControl, Player};
    use event_clump::{FrontEventClump};

    pub fn handle_events(gfx_window: &mut GfxWindow<Window, Extras>, front_event_clump: &mut FrontEventClump) -> bool {
        if gfx_window.get_extras().1.is_none() {
            let mut controller = gfx_window.get_mut_window().subsystem().sdl().game_controller().unwrap_or_else(|err| panic!(err));
            controller.set_event_state(true);
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
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Up(1.0, Player::One));
                        },
                        Some(Keycode::Down) => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Down(1.0, Player::One));
                        },
                        Some(Keycode::Left) => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Left(1.0, Player::One));
                        },
                        Some(Keycode::Right) => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Right(1.0, Player::One));
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
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Up(0.0, Player::One));
                        },
                        Some(Keycode::Down) => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Down(0.0, Player::One));
                        },
                        Some(Keycode::Left) => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Left(0.0, Player::One));
                        },
                        Some(Keycode::Right) => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Right(0.0, Player::One));
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
                    warn!("Axis Motion");
                    match axis {
                        Axis::LeftX => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(if value >= 0 {
                                ToControl::Right((value / ::std::i16::MAX) as f64, match which {
                                    _ => panic!(which),
                                })
                            } else {
                                ToControl::Left((value / ::std::i16::MIN).abs() as f64, match which {
                                    _ => Player::One,
                                })
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
                    warn!("Button Down");
                    match button {
                        Button::DPadRight => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Right(1.0, match which {
                                _ => Player::One,
                            }));
                        },
                        Button::DPadLeft => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Left(1.0, match which {
                                _ => Player::One,
                            }));
                        },
                        Button::A => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Up(1.0, match which {
                                _ => Player::One,
                            }));
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
                    warn!("Button Up");
                    match button {
                        Button::DPadRight => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Right(0.0, match which {
                                _ => Player::One,
                            }));
                        },
                        Button::DPadLeft => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Left(0.0, match which {
                                _ => Player::One,
                            }));
                        },
                        Button::A => {
                            front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(ToControl::Up(0.0, match which {
                                _ => Player::One,
                            }));
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
