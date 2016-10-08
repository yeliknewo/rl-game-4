pub mod glutin {
    use graphics::{GfxWindow};
    use graphics::rl_glutin::{Window, Extras};
    use event_clump::{FrontEventClump};

    #[allow(dead_code, unused_variables)]
    pub fn handle_events(gfx_window: &mut GfxWindow<Window, Extras>, front_event_clump: &mut FrontEventClump) -> bool {
        false
    }
}

pub mod sdl2 {
    use std::collections::{HashMap};
    use dependencies::sdl2::event::{Event, WindowEventId};
    use dependencies::sdl2::keyboard::{Keycode};
    use dependencies::sdl2::controller::{GameController, Axis, Button};
    use event_enums::main_x_control::{MainToControl};
    use graphics::{GfxWindow};
    use graphics::rl_sdl2::{Window, Extras};
    use utils::{Player};
    use event_clump::{FrontEventClump};

    #[allow(dead_code, unused_variables)]
    pub fn handle_events(gfx_window: &mut GfxWindow<Window, Extras>, front_event_clump: &mut FrontEventClump) -> bool {
        let mut event_pump = gfx_window.get_mut_extras().1.take().unwrap_or_else(|| panic!("Event Pump was None"));
        let game_controller = gfx_window.get_mut_extras().2.take().unwrap_or_else(|| panic!("Game Controller Subsystem was None"));
        {
            let mut controllers: &mut HashMap<i32, (GameController, Player)> = &mut gfx_window.get_mut_extras().3;

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
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Up(1.0, Player::One));
                            },
                            Some(Keycode::Down) => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Down(1.0, Player::One));
                            },
                            Some(Keycode::Left) => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Left(1.0, Player::One));
                            },
                            Some(Keycode::Right) => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Right(1.0, Player::One));
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
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Up(0.0, Player::One));
                            },
                            Some(Keycode::Down) => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Down(0.0, Player::One));
                            },
                            Some(Keycode::Left) => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Left(0.0, Player::One));
                            },
                            Some(Keycode::Right) => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Right(0.0, Player::One));
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
                        // warn!("Axis Motion");
                        match axis {
                            Axis::LeftX => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(if value >= 0 {
                                    MainToControl::Right((value / ::std::i16::MAX) as f64, match which {
                                        0 => Player::One,
                                        1 => Player::Two,
                                        _ => continue,
                                    })
                                } else {
                                    MainToControl::Left((value / ::std::i16::MIN).abs() as f64, match which {
                                        0 => Player::One,
                                        1 => Player::Two,
                                        _ => continue,
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
                        // warn!("Button Down");
                        match button {
                            Button::DPadRight => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Right(1.0, match which {
                                    0 => Player::One,
                                    1 => Player::Two,
                                    _ => continue,
                                }));
                            },
                            Button::DPadLeft => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Left(1.0, match which {
                                    0 => Player::One,
                                    1 => Player::Two,
                                    _ => continue,
                                }));
                            },
                            Button::DPadUp => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Up(1.0, match which {
                                    0 => Player::One,
                                    1 => Player::Two,
                                    _ => continue,
                                }));
                            },
                            Button::DPadDown => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Down(1.0, match which {
                                    0 => Player::One,
                                    1 => Player::Two,
                                    _ => continue,
                                }));
                            },
                            Button::A => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Up(1.0, match which {
                                    0 => Player::One,
                                    1 => Player::Two,
                                    _ => continue,
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
                        match button {
                            Button::DPadRight => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Right(0.0, match which {
                                    0 => Player::One,
                                    1 => Player::Two,
                                    _ => continue,
                                }));
                            },
                            Button::DPadLeft => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Left(0.0, match which {
                                    0 => Player::One,
                                    1 => Player::Two,
                                    _ => continue,
                                }));
                            },
                            Button::DPadUp => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Up(0.0, match which {
                                    0 => Player::One,
                                    1 => Player::Two,
                                    _ => continue,
                                }));
                            },
                            Button::DPadDown => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Down(0.0, match which {
                                    0 => Player::One,
                                    1 => Player::Two,
                                    _ => continue,
                                }));
                            },
                            Button::A => {
                                front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send_to(MainToControl::Up(0.0, match which {
                                    0 => Player::One,
                                    1 => Player::Two,
                                    _ => continue,
                                }));
                            }
                            _ => {

                            },
                        }
                    },
                    Event::ControllerDeviceAdded {
                        timestamp,
                        which,
                    } => {
                        // warn!("Added, Which: {:?}", which);
                        if let Some(player) = Player::map_number(which) {
                            controllers.insert(which, (game_controller.open(which as u32).unwrap_or_else(|err| panic!(err)), player));
                        }
                    },
                    // Event::ControllerDeviceRemoved {
                    //     timestamp,
                    //     which,
                    // } => {
                    //     warn!("Removed, Which: {:?}", which);
                    //     // controllers.remove(&which).unwrap_or_else(|| panic!("Removed nothing: {:?}", which));
                    // },
                    // Event::ControllerDeviceRemapped {
                    //     timestamp,
                    //     which,
                    // } => {
                    //     warn!("Mapped, Which: {:?}", which);
                    // },
                    _ => {

                    },
                }
            }
        }
        gfx_window.get_mut_extras().1 = Some(event_pump);
        gfx_window.get_mut_extras().2 = Some(game_controller);

        false
    }
}
