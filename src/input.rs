use std::collections::HashSet;
use winput::message_loop;
use winput::message_loop::EventReceiver;
use winput::Action;
use winput::Vk;
use ABC_Game_Engine::EntitiesAndComponents;
use ABC_Game_Engine::Input;
use ABC_Game_Engine::KeyCode;
use ABC_Game_Engine::Resource;

/// updates the input system
pub(crate) struct ConsoleInput {
    receiver: EventReceiver,
    keys_down: HashSet<KeyCode>,
}

impl ConsoleInput {
    pub(crate) fn new() -> Self {
        let receiver =
            message_loop::start().expect("failed to start message loop for input system");
        let keys_down = HashSet::new();
        Self {
            receiver,
            keys_down,
        }
    }

    /// main update loop for the input system
    pub(crate) fn update(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let input = entities_and_components
            .get_resource_mut::<Input>()
            .expect("failed to get input resource from entities and components");

        input.clear_key_states();
        if let Some(next_event) = self.receiver.try_next_event() {
            match next_event {
                message_loop::Event::Keyboard {
                    vk,
                    action: Action::Press,
                    ..
                } => {
                    let key = vk_to_keycode(vk);
                    if let Some(key) = key {
                        self.keys_down.insert(key);
                    }
                }
                message_loop::Event::Keyboard {
                    vk,
                    action: Action::Release,
                    ..
                } => {
                    let key = vk_to_keycode(vk);
                    if let Some(key) = key {
                        self.keys_down.remove(&key);
                    }
                }
                _ => (),
            }
        }
        for key in self.keys_down.iter() {
            input.set_key_state(*key)
        }

        input.advance_frame();
    }
}

impl Resource for ConsoleInput {
    fn update(&mut self) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn vk_to_keycode(vk: Vk) -> Option<KeyCode> {
    match vk {
        Vk::A => Some(KeyCode::A),
        Vk::B => Some(KeyCode::B),
        Vk::C => Some(KeyCode::C),
        Vk::D => Some(KeyCode::D),
        Vk::E => Some(KeyCode::E),
        Vk::F => Some(KeyCode::F),
        Vk::G => Some(KeyCode::G),
        Vk::H => Some(KeyCode::H),
        Vk::I => Some(KeyCode::I),
        Vk::J => Some(KeyCode::J),
        Vk::K => Some(KeyCode::K),
        Vk::L => Some(KeyCode::L),
        Vk::M => Some(KeyCode::M),
        Vk::N => Some(KeyCode::N),
        Vk::O => Some(KeyCode::O),
        Vk::P => Some(KeyCode::P),
        Vk::Q => Some(KeyCode::Q),
        Vk::R => Some(KeyCode::R),
        Vk::S => Some(KeyCode::S),
        Vk::T => Some(KeyCode::T),
        Vk::U => Some(KeyCode::U),
        Vk::V => Some(KeyCode::V),
        Vk::W => Some(KeyCode::W),
        Vk::X => Some(KeyCode::X),
        Vk::Y => Some(KeyCode::Y),
        Vk::Z => Some(KeyCode::Z),
        Vk::_0 => Some(KeyCode::Key0),
        Vk::_1 => Some(KeyCode::Key1),
        Vk::_2 => Some(KeyCode::Key2),
        Vk::_3 => Some(KeyCode::Key3),
        Vk::_4 => Some(KeyCode::Key4),
        Vk::_5 => Some(KeyCode::Key5),
        Vk::_6 => Some(KeyCode::Key6),
        Vk::_7 => Some(KeyCode::Key7),
        Vk::_8 => Some(KeyCode::Key8),
        Vk::_9 => Some(KeyCode::Key9),
        Vk::Space => Some(KeyCode::Space),
        Vk::Enter => Some(KeyCode::Return),
        Vk::Escape => Some(KeyCode::Escape),
        Vk::Backspace => Some(KeyCode::Backspace),
        Vk::Tab => Some(KeyCode::Tab),
        _ => None, // this list isn't exhaustive, but it's a start it takes a while and I don't think most of the keys will be used anyway...
    }
}
