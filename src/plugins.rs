
use eval::{eval, to_value};
use std::str::FromStr;
use std::error::Error;
use math_text_transform::MathTextTransform;
use ::*;
use discord::model::ReactionEmoji;
use discord::model::EmojiId;
use rusqlite::Connection;
use std::path::Path;
use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Clone)]
struct MemoryChunk {
    from: String,
    into: String,
}

impl MemoryChunk {
    fn new(from: String, into: String) -> MemoryChunk {
        MemoryChunk {
            from: from,
            into: into,
        }
    }
}

fn insert_memory(conn: &Connection, chunk: MemoryChunk) {
    conn.execute(
        "INSERT INTO memory (msg_from, msg_into) 
                  VALUES (?1, ?2)",
        &[&chunk.from, &chunk.into],
    ).unwrap();
}

fn whole_memory(conn: &Connection) -> Vec<MemoryChunk> {
    let mut stmt = conn.prepare("SELECT msg_from, msg_into FROM memory")
        .unwrap();
    let chunk_iter = stmt.query_map(&[], |row| {
        MemoryChunk {
            from: row.get(0),
            into: row.get(1),
        }
    }).unwrap();
    let mut chunks: Vec<MemoryChunk> = Vec::new();
    for chunk_res in chunk_iter {
        chunks.push(chunk_res.unwrap());
    }
    chunks
}

fn fetch_memory(conn: &Connection, from: String) -> Option<MemoryChunk> {
    let chunks = whole_memory(conn);
    chunks.iter().find(|&x| x.from == from).cloned()
}

fn delete_memory(conn: &Connection, from: String) {
    conn.execute("DELETE FROM memory WHERE msg_from = (?1)", &[&from])
        .unwrap();
}

fn create_table(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS memory (
        msg_from        TEXT NOT NULL PRIMARY KEY,
        msg_into        TEXT NOT NULL
        )",
        &[],
    ).unwrap();
}

/* Save */
pub struct RememberPlugin;
impl MPlugin for RememberPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["rem", "sa", "@"]
    }
    fn execute(&self, data: PluginData) -> String {
        let conn = Connection::open(Path::new("databases/memory.db")).unwrap();
        let from = data.arguments[0].clone();
        let into = data.arguments
            .into_iter()
            .skip(1)
            .collect::<Vec<String>>()
            .join(" ");
        create_table(&conn);
        insert_memory(&conn, MemoryChunk::new(from, into));
        String::new()
    }
}

pub struct ForgetPlugin;
impl MPlugin for ForgetPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["#"]
    }
    fn execute(&self, data: PluginData) -> String {
        let conn = Connection::open(Path::new("databases/memory.db")).unwrap();
        create_table(&conn);
        delete_memory(&conn, data.arguments[0].clone());
        String::new()
    }
}

pub struct OmnipotencePlugin;
impl MPlugin for OmnipotencePlugin {
    fn id(&self) -> Vec<&str> {
        vec!["!!"]
    }
    fn execute(&self, data: PluginData) -> String {
        let conn = Connection::open(Path::new("databases/memory.db")).unwrap();
        create_table(&conn);
        let memory = whole_memory(&conn);
        let mut data = String::new();
        for chunk in memory {
            data.push_str(&*format!("{} => {}\n", chunk.from, chunk.into));
        }
        format!("```Memory:\n{}```", data)
    }
}

/* Load */
pub struct RecallPlugin;
impl MPlugin for RecallPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["rec", "!"]
    }
    fn execute(&self, data: PluginData) -> String {
        let conn = Connection::open(Path::new("databases/memory.db")).unwrap();
        let name = data.arguments[0].clone();
        create_table(&conn);
        let memory = fetch_memory(&conn, name.clone());
        if memory.is_some() {
            return memory.unwrap().into;
        } else {
            return format!("Memory \"{}\" does not exist.", name);
        }
    }
}

/* For Misconception */
#[derive(Default, Debug)]
struct MarkedString {
    string: String,
    upper_indicies: Vec<u32>,
}

fn load_words() -> HashMap<Vec<String>, String> {
    let mut map = HashMap::new();
    let mut words_str: String = String::new();
    let mut file = File::open("res/mistaken.txt").unwrap();
    file.read_to_string(&mut words_str);
    for excerpt in words_str.lines() {
        let mut data = excerpt.split("->");
        let left = data.nth(0).unwrap().to_lowercase();
        let right = data.nth(0).unwrap();
        let a_right = right.split(", ").map(|x| x.to_string()).collect::<Vec<String>>();
        map.insert(a_right, left);
    }
    map
} 

pub struct MisconceptionPlugin;
impl MPlugin for MisconceptionPlugin {
    fn id(&self) -> Vec<&str> { vec!["misconcept", "uhm"] }
    fn execute(&self, data: PluginData) -> String {

        fn to_misconception(text: String) -> String {
            let misconceptions = load_words();
            let words = text.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
            words.iter().map(|x| {
                let mut key: Vec<String> = Vec::new();
                for key in misconceptions.keys() {
                    if key.contains(&x.to_string()) {
                        return misconceptions.get(key).unwrap().clone() 
                    }
                }
                x.clone()
            }).collect::<Vec<String>>().join(" ")
        }

        let ref msg = data.message;
        let ref d = data.discord;
        
        let last_msg_if = data.discord.get_messages(
            data.message.channel_id,
            discord::GetMessages::MostRecent,
            Some(1),
        );

        if last_msg_if.is_ok() {
            let last_msg = &last_msg_if.unwrap()[0];
            let last_msg_content = last_msg.content.clone();
            if last_msg.author.id == msg.author.id {
                d.edit_message(last_msg.channel_id, last_msg.id, &*to_misconception(last_msg_content));
            } else {
                return to_misconception(last_msg_content)
            }
        }
        String::new()
    }
}


pub struct PurgePlugin;
impl MPlugin for PurgePlugin {
    fn id(&self) -> Vec<&str> {
        vec!["purge", "clear"]
    }
    fn execute(&self, data: PluginData) -> String {
        if data.arguments.len() == 1 {
            let ref num_up = data.arguments[0];
            let num = FromStr::from_str(&*num_up).unwrap();
            let mut deleted: u64 = 0;
            let mut attemps: u64 = 1;
            while deleted < num {
                let last_msg_if = data.discord.get_messages(
                    data.message.channel_id,
                    discord::GetMessages::MostRecent,
                    Some(1),
                );
                if last_msg_if.is_ok() {
                    let last_msgs = &last_msg_if.unwrap();
                    if last_msgs.len() > 0 {
                        let ref last_msg = last_msgs[0];
                        if last_msg.author.id == data.message.author.id {
                            data.discord
                                .delete_message(last_msg.channel_id, last_msg.id)
                                .ok();
                            deleted += 1;
                        } else {
                            if attemps > 100 { // Can't get most recent messages more than 100 times..
                                break;
                            } else {
                                let messages = data.discord
                                    .get_messages(
                                        data.message.channel_id,
                                        discord::GetMessages::MostRecent,
                                        Some(attemps),
                                    )
                                    .expect("Failed to get recent messages.");
                                for message in messages {
                                    if message.author.id == data.message.author.id {
                                        data.discord
                                            .delete_message(message.channel_id, message.id)
                                            .ok();
                                        deleted += 1;
                                    }
                                }
                                attemps += 1;
                            }
                        }
                    }
                }
            }
            /* 
         while (msgs.len() as u64) < num {
            let mut others = data.discord.get_messages(data.message.channel_id, discord::GetMessages::MostRecent, Some(index_buffer))
               .unwrap_or(Vec::new());
            println!("adding {:?}", others);
            msgs.append( &mut others );
         }
         for msg in msgs.iter().filter(|x| x.author.id == data.message.author.id) {
            data.discord.delete_message(msg.channel_id, msg.id).ok();
         }*/
        }

        String::new()
    }
}



pub struct ShillPlugin;
impl MPlugin for ShillPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["shill", "box"]
    }
    fn execute(&self, data: PluginData) -> String {
        if data.arguments[0].len() > 0 {
            let ref text = data.arguments.join(" ");
            let mut base = text.chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            for rest in text.chars().skip(1) {
                base.push_str(&*format!("\n{}", rest));
            }
            return format!("```{}```", base);
        }
        String::from("Blank Message")
    }
}

pub struct SettingsPlugin;
impl MPlugin for SettingsPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["settings", "edit"]
    }
    fn execute(&self, data: PluginData) -> String {
        let ref args = data.arguments;
        let ref setting = *args[0].to_lowercase();
        let value = if args.len() > 1 {
            Some(args[1].clone())
        } else {
            None
        };

        let mut z = None;
        if value.is_some() {
            let n_val = value.unwrap();
            let string_res: Result<bool, _> = FromStr::from_str(&*n_val);
            if string_res.is_ok() {
                z = data.settings.set(setting, string_res.unwrap(), false);
            }
        } else {
            z = data.settings.set(setting, false, true)
        }
        if z.is_some() {
            format!("{} := `{:?}`", setting, z.unwrap())
        } else {
            String::new()
        }
    }
}

pub struct EvalPlugin;
impl MPlugin for EvalPlugin {
    fn id(&self) -> Vec<&str> {
        vec![";", "e", "eval"]
    }
    fn execute(&self, data: PluginData) -> String {
        let text = data.arguments.join(" ");
        match eval(&*text) {
            Ok(val) => format!("{} = {}", text, to_value(val)),
            Err(why) => String::from(why.description()),
        }
    }
}

pub struct UserInfoPlugin;
impl MPlugin for UserInfoPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["dox", "whois", "usr"]
    }
    fn execute(&self, data: PluginData) -> String {
        let ref msg = data.message;
        let ref d = data.discord;
        if msg.mentions.len() > 0 {
            let ref mem = msg.mentions[0];
            let avatar_url = d.get_user_avatar_url(mem.id, mem.avatar.as_ref().unwrap());
            d.send_embed(msg.channel_id, "", |b| {
                b.color(0xFFFFFF)
                    .title(&*mem.name)
                    .thumbnail(&avatar_url)
                    .fields(|f| {
                        f.field("Discriminator", &*mem.discriminator.to_string(), true)
                            .field("Bot?", &*mem.bot.to_string(), true)
                            .field("Id", &*mem.id.to_string(), true)
                    })
            }).expect("Failed to send embed.");
        }
        String::new()
    }
}

pub struct MockPlugin;
impl MPlugin for MockPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["mock", "mo"]
    }

    fn execute(&self, data: PluginData) -> String {
        fn mock(text: String) -> String {
            let mut rng = rand::thread_rng();
            text.chars().map(|x| {
                if rng.gen() { x.to_uppercase().to_string() } else { x.to_lowercase().to_string() }
            }).collect::<String>()
        }

        let ref args = data.arguments;
        let ref msg = data.message;
        let ref d = data.discord;
        
        let last_msg_if = data.discord.get_messages(
            data.message.channel_id,
            discord::GetMessages::MostRecent,
            Some(1),
        );
        if last_msg_if.is_ok() {
            let last_msg = &last_msg_if.unwrap()[0];
            let last_msg_content = last_msg.content.clone();
            if last_msg.author.id == msg.author.id {
                d.edit_message(last_msg.channel_id, last_msg.id, &*mock(last_msg_content));
            } else {
                return mock(last_msg_content)
            }
        }
        String::new()
    }
}

pub struct ReactPlugin;
impl MPlugin for ReactPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["react", "re"]
    }
    fn execute(&self, data: PluginData) -> String {
        let ref args = data.arguments;
        let ref msg = data.message;
        let ref d = data.discord;


        let mut unicode: HashMap<String, String> = HashMap::new();

        let ref word = args[0];
        let last_message_if =
            d.get_messages(msg.channel_id, discord::GetMessages::MostRecent, Some(1));
        if last_message_if.is_ok() {
            let last_msgs = last_message_if.unwrap();
            let last_msg = &last_msgs[0];

            if data.settings.react_custom {
                let mut occurance_map: HashMap<char, i32> = HashMap::new();
                // we're smart, use the whole thing.
                unicode.insert(
                    String::from(" "),
                    String::from("<:space_blank:350213580651757579>"),
                );

                unicode.insert(
                    String::from(" 2"),
                    String::from("<:space_blank_1:350213580827656202>"),
                );

                unicode.insert(
                    String::from(" 3"),
                    String::from("<:space_blank_2:350213580643237889>"),
                );

                unicode.insert(
                    String::from(" 3"),
                    String::from("<:space_blank_3:350213580857147392>"),
                );

                unicode.insert(
                    String::from(" 3"),
                    String::from("<:space_blank_4:350213580970262528>"),
                );

                unicode.insert(
                    String::from("a"),
                    String::from("<:cap_A:350164697095340032>"),
                );
                unicode.insert(
                    String::from("b"),
                    String::from("<:cap_B:350164699595145216>"),
                );
                unicode.insert(
                    String::from("c"),
                    String::from("<:cap_C:350164703391121408>"),
                );
                unicode.insert(
                    String::from("d"),
                    String::from("<:cap_D:350164704234045442>"),
                );
                unicode.insert(
                    String::from("e"),
                    String::from("<:cap_E:350164704569851905>"),
                );
                unicode.insert(
                    String::from("f"),
                    String::from("<:cap_F:350164704855064577>"),
                );
                unicode.insert(
                    String::from("g"),
                    String::from("<:cap_G:350164705341341706>"),
                );
                unicode.insert(
                    String::from("h"),
                    String::from("<:cap_H:350164705555251200>"),
                );
                unicode.insert(
                    String::from("i"),
                    String::from("<:cap_I:350164705534541825>"),
                );
                unicode.insert(
                    String::from("j"),
                    String::from("<:cap_J:350164705974812672>"),
                );
                unicode.insert(
                    String::from("k"),
                    String::from("<:cap_K:350164705626685441>"),
                );
                unicode.insert(
                    String::from("l"),
                    String::from("<:cap_L:350164705966555136>"),
                );
                unicode.insert(
                    String::from("m"),
                    String::from("<:cap_m:350164705907703809>"),
                );
                unicode.insert(
                    String::from("n"),
                    String::from("<:cap_N:350164705794457601>"),
                );
                unicode.insert(
                    String::from("o"),
                    String::from("<:cap_O:350164706142453760>"),
                );
                unicode.insert(
                    String::from("p"),
                    String::from("<:cap_P:350164706075344896>"),
                );
                unicode.insert(
                    String::from("q"),
                    String::from("<:cap_Q:350164705895120906>"),
                );
                unicode.insert(
                    String::from("r"),
                    String::from("<:cap_R:350164706197110784>"),
                );
                unicode.insert(
                    String::from("s"),
                    String::from("<:cap_S:350164706033664000>"),
                );
                unicode.insert(
                    String::from("t"),
                    String::from("<:cap_T:350164706075344906>"),
                );
                unicode.insert(
                    String::from("u"),
                    String::from("<:cap_U:350164706071150593>"),
                );
                unicode.insert(
                    String::from("v"),
                    String::from("<:cap_V:350164705811234817>"),
                );
                unicode.insert(
                    String::from("w"),
                    String::from("<:cap_W:350164705723023363>"),
                );
                unicode.insert(
                    String::from("x"),
                    String::from("<:cap_X:350164706092122122>"),
                );
                unicode.insert(
                    String::from("y"),
                    String::from("<:cap_Y:350164705979138051>"),
                );
                unicode.insert(
                    String::from("z"),
                    String::from("<:cap_Z:350164706259894272>"),
                );
                unicode.insert(
                    String::from("a2"),
                    String::from("<:cap_A_2:350172373414051840>"),
                );
                unicode.insert(
                    String::from("b2"),
                    String::from("<:cap_B_2:350172375154819073>"),
                );
                unicode.insert(
                    String::from("c2"),
                    String::from("<:cap_C_2:350172376429887488>"),
                );
                unicode.insert(
                    String::from("d2"),
                    String::from("<:cap_D_2:350172377193119746>"),
                );
                unicode.insert(
                    String::from("e2"),
                    String::from("<:cap_E_2:350172377700761600>"),
                );
                unicode.insert(
                    String::from("f2"),
                    String::from("<:cap_F_2:350172377684115456>"),
                );
                unicode.insert(
                    String::from("g2"),
                    String::from("<:cap_G_2:350172377713475594>"),
                );
                unicode.insert(
                    String::from("h2"),
                    String::from("<:cap_H_2:350172377843367936>"),
                );
                unicode.insert(
                    String::from("i2"),
                    String::from("<:cap_I_2:350172377637715969>"),
                );
                unicode.insert(
                    String::from("j2"),
                    String::from("<:cap_J_2:350172377709150210>"),
                );
                unicode.insert(
                    String::from("k2"),
                    String::from("<:cap_K_2:350172377990299659>"),
                );
                unicode.insert(
                    String::from("l2"),
                    String::from("<:cap_L_2:350172377893830658>"),
                );
                unicode.insert(
                    String::from("m2"),
                    String::from("<:cap_M_2:350172377793036290>"),
                );
                unicode.insert(
                    String::from("n2"),
                    String::from("<:cap_N_2:350172377998688258>"),
                );
                unicode.insert(
                    String::from("o2"),
                    String::from("<:cap_O_2:350172378145488916>"),
                );
                unicode.insert(
                    String::from("p2"),
                    String::from("<:cap_P_2:350172378212335616>"),
                );
                unicode.insert(
                    String::from("q2"),
                    String::from("<:cap_Q_2:350172378237763585>"),
                );
                unicode.insert(
                    String::from("r2"),
                    String::from("<:cap_R_2:350172378749206538>"),
                );
                unicode.insert(
                    String::from("s2"),
                    String::from("<:cap_S_2:350172378472382466>"),
                );
                unicode.insert(
                    String::from("t2"),
                    String::from("<:cap_T_2:350172378518519812>"),
                );
                unicode.insert(
                    String::from("u2"),
                    String::from("<:cap_U_2:350172378921304064>"),
                );
                unicode.insert(
                    String::from("v2"),
                    String::from("<:cap_V_2:350172378913046538>"),
                );
                unicode.insert(
                    String::from("w2"),
                    String::from("<:cap_W_2:350172379055521792>"),
                );
                unicode.insert(
                    String::from("x2"),
                    String::from("<:cap_X_2:350172379038613514>"),
                );
                unicode.insert(
                    String::from("y2"),
                    String::from("<:cap_Y_2:350172378963378176>"),
                );
                unicode.insert(
                    String::from("z2"),
                    String::from("<:cap_Z_2:350172378799669251>"),
                );

                for ch in args.join(" ").chars() {
                    *occurance_map.entry(ch).or_insert(0) += 1;
                    match occurance_map.get(&ch) {
                        Some(n) => {
                            let mut x_name: String = ch.to_string();
                            let emoji = unicode.get(&format!(
                                "{}{}",
                                x_name,
                                if *n > 1 {
                                    n.to_string()
                                } else {
                                    "".to_string()
                                }
                            ));
                            if emoji.is_some() {
                                let a_emoji = emoji.unwrap().chars().skip(2).collect::<String>();
                                let real = a_emoji
                                    .chars()
                                    .rev()
                                    .collect::<String>()
                                    .chars()
                                    .skip(1)
                                    .collect::<String>()
                                    .chars()
                                    .rev()
                                    .collect::<String>();
                                let mut data = real.split(":");
                                let name = data.nth(0).unwrap().to_string();
                                let id = data.nth(0).unwrap().parse::<u64>().unwrap();
                                d.add_reaction(
                                    msg.channel_id,
                                    last_msg.id,
                                    ReactionEmoji::Custom {
                                        name: name,
                                        id: EmojiId(id),
                                    },
                                );
                            }
                        }
                        None => (), // skip for now..
                    }
                }
            } else {
                unicode.insert(String::from("a"), String::from("\u{1F1E6}"));
                unicode.insert(String::from("b"), String::from("\u{1F1E7}"));
                unicode.insert(String::from("c"), String::from("\u{1F1E8}"));
                unicode.insert(String::from("d"), String::from("\u{1F1E9}"));
                unicode.insert(String::from("e"), String::from("\u{1F1EA}"));
                unicode.insert(String::from("f"), String::from("\u{1F1EB}"));
                unicode.insert(String::from("g"), String::from("\u{1F1EC}"));
                unicode.insert(String::from("h"), String::from("\u{1F1ED}"));
                unicode.insert(String::from("i"), String::from("\u{1F1EE}"));
                unicode.insert(String::from("j"), String::from("\u{1F1EF}"));
                unicode.insert(String::from("k"), String::from("\u{1F1F0}"));
                unicode.insert(String::from("l"), String::from("\u{1F1F1}"));
                unicode.insert(String::from("m"), String::from("\u{1F1F2}"));
                unicode.insert(String::from("n"), String::from("\u{1F1F3}"));
                unicode.insert(String::from("o"), String::from("\u{1F1F4}"));
                unicode.insert(String::from("p"), String::from("\u{1F1F5}"));
                unicode.insert(String::from("q"), String::from("\u{1F1F6}"));
                unicode.insert(String::from("r"), String::from("\u{1F1F7}"));
                unicode.insert(String::from("s"), String::from("\u{1F1F8}"));
                unicode.insert(String::from("t"), String::from("\u{1F1F9}"));
                unicode.insert(String::from("u"), String::from("\u{1F1FA}"));
                unicode.insert(String::from("v"), String::from("\u{1F1FB}"));
                unicode.insert(String::from("w"), String::from("\u{1F1FC}"));
                unicode.insert(String::from("x"), String::from("\u{1F1FD}"));
                unicode.insert(String::from("y"), String::from("\u{1F1FE}"));
                unicode.insert(String::from("z"), String::from("\u{1F1FF}"));

                for ch in word.chars() {
                    let emoj = unicode.get(&ch.to_string());
                    match emoj {
                        Some(_) => {
                            d.add_reaction(
                                msg.channel_id,
                                last_msg.id,
                                ReactionEmoji::Unicode(emoj.unwrap().to_string()),
                            );
                        }
                        None => (), // skip
                    }
                }
            }
        }
        String::new()
    }
}

pub struct TextTransformPlugin;
impl MPlugin for TextTransformPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["transf", "mt"]
    }
    fn execute(&self, data: PluginData) -> String {
        let ref args = data.arguments;
        let text = args[1..].join(" ");
        match &*args[0] {
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
            _ => return String::new(),
        }
    }
}

pub struct AboutPlugin;
impl MPlugin for AboutPlugin {
    fn id(&self) -> Vec<&str> {
        vec!["about", "bot"]
    }
    fn execute(&self, data: PluginData) -> String {
        let ref d = data.discord;
        let ref msg = data.message;
        let conn = Connection::open(Path::new("databases/memory.db")).unwrap();
        create_table(&conn);
        d.send_embed(msg.channel_id, "", |b| {
            b.color(0xFF3333)
                .url("https://github.com/hvze/misaki")
                .title("Misaki (Modular Selfbot)")
                .thumbnail("https://ill.fi/ncdt.png")
                .fields(|f| {
                    f.field(
                        "Memory Chunks",
                        &*whole_memory(&conn).len().to_string(),
                        true,
                    ).field("Modules", "12" /* lol */, true)
                })
        }).expect("Failed to send embed.");
        String::new()
    }
}