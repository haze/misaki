use misaki_api::misaki::{MPlugin, PluginData};
use std::collections::HashMap;
use discord::model::{ReactionEmoji, EmojiId};
use discord;

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
                                ).unwrap();
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
                            ).unwrap();
                        }
                        None => (), // skip
                    }
                }
            }
        }
        String::new()
    }
}