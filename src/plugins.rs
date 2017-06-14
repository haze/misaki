extern crate discord;		
extern crate math_text_transform;

use math_text_transform::MathTextTransform;
use MPlugin;
use discord::model::Message;

pub struct TextTransformPlugin;
impl MPlugin for TextTransformPlugin {
    fn id(&self) -> String { String::from("trans") }
    fn execute(&self, msg: &Message, args: Vec<String>) -> String {
    	let form = &*args[0];
    	let text = args[1..].iter().fold(String::new(), |acc, s| acc + " " + s);
    	match form {
    	    "b" => return text.to_math_bold(),
    	    "i" => return text.to_math_italic(),
    	    "bi" => return text.to_math_bold_italic(),
    	    "ss" => return text.to_math_sans_serif(),
    	    "ssb" => return text.to_math_sans_serif_bold(),
    	    "ssi" => return text.to_math_sans_serif_italic(),
    	    "ssbi" => return text.to_math_sans_serif_bold_italic(),
    	    "s" => return text.to_math_script(),
    	    "bs" => return text.to_math_bold_script(),
    	    "f" => return text.to_math_fraktur(),
    	    "m" => return text.to_math_monospace(),
    	    "ds" => return text.to_math_double_struck(),
    	    _ => return format!("Text Transform {} not found.", form)
    	}
    }
}