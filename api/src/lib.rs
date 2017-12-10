#![allow(dead_code)]
extern crate discord;

pub mod misaki {

    use std::rc::Rc;
    use discord::Discord;
    use discord::Connection;
    use discord::model::Message;

    #[derive(Default)]
    pub struct MisakiSettings {
        pub embed_mode: bool,
        pub should_mark: bool,
		pub react_custom: bool,
        pub latex_color: String,
        pub latex_size: u32,
        pub hyper_shill: bool,
        pub uzi_mode: bool,
    }

    impl MisakiSettings {
        pub fn is_number(&self, name: &str) -> bool {
            match &*String::from(name).to_lowercase() {
                "latex_size" => true,
                _ => false
            }
        }

        pub fn is_string(&self, name: &str) -> bool {
            match &*String::from(name).to_lowercase() {
                "latex_col" => true,
                _ => false
            }
        }

        pub fn is_bool(&self, name: &str) -> bool {
            match &*String::from(name).to_lowercase() {
                "embed" => true,
                "mark" => true,
                "react" => true,
                "hypershill" => true,
                "uzi" => true,
                _ => false
            }
        }

        pub fn set_num(&mut self, name: &str, to: u32) -> Option<u32> {
            match &*String::from(name).to_lowercase() {
                "latex_size" => {
                    self.latex_size = to;
                    Some(self.latex_size)
                }
                _ => None,
            }
        }

        pub fn set_str(&mut self, name: &str, to: String) -> Option<String> {
            return match &*String::from(name).to_lowercase() {
                "latex_col" => {
                    self.latex_color = to.clone();
                    Some(self.latex_color.clone())
                }
                _ => None,
            };
        }

        pub fn set(&mut self, name: &str, to: bool, flip: bool) -> Option<bool> {
            return match &*String::from(name).to_lowercase() {
                "embed" => {
                    self.embed_mode = if flip { !self.embed_mode } else { to };
                    return Some(self.embed_mode);
                }
                "mark" => {
                    self.should_mark = if flip { !self.should_mark } else { to };
                    return Some(self.should_mark);
                } 
				"react" => {
					self.react_custom = if flip { !self.react_custom } else { to };
					return Some(self.react_custom);
				}
                "hypershill" => {
                    self.hyper_shill = if flip { !self.hyper_shill } else { to };
                    return Some(self.hyper_shill)
                }
                "uzi" => {
                    self.uzi_mode = if flip { !self.uzi_mode } else { to };
                    return Some(self.uzi_mode)
                }
                _ => None,
            };
        }
    }


    // used for taking up less space when passing information around to plugins
    pub struct PluginData<'a> {
        pub connection: &'a Connection,
        pub discord: &'a Discord,
        pub message: &'a Message,
        pub arguments: Vec<String>,
        pub plugins: Rc<Vec<Box<MPlugin>>>,
        pub settings: &'a mut MisakiSettings,
    }

    pub trait MPlugin {
        fn id(&self) -> Vec<&str>;
        fn execute(&self, data: PluginData) -> String;
    }

}