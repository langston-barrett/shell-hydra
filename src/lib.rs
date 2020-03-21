// --------------------------------------------------------------
// * shell-hydra

extern crate console;

#[macro_use]
extern crate serde;

mod charin;
use charin::CharIn;

use std::process::Command;

// --------------------------------------------------------------
// ** types

#[derive(Clone, Eq, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Element {
    #[serde(default)]
    pub key: char,
    pub desc: String,
    #[serde(default)]
    pub exit: bool,
    #[serde(alias = "cmd")]
    #[serde(alias = "keys")]
    #[serde(default)]
    pub rec: Option<Rec>,
}

#[derive(Clone, Eq, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Rec {
    Cmd(Vec<String>),
    Rec(Vec<Element>),
}

fn print_elements(elts: &Vec<Element>) {
    for elt in elts {
        println!("{} → {}", elt.key, elt.desc);
    }
}

pub fn go(mut elts: Vec<Element>) -> std::io::Result<()> {
    let mut stream = CharIn::new();
    loop {
        println!("\x1B[2J"); // clear the terminal
        print_elements(&elts);
        let c = stream.next().unwrap();
        if c == 'q' {
            return Ok(());
        }
        let melt = elts.iter().find(|elt| elt.key == c);
        match melt {
            None => {
                return Ok(());
            }
            Some(elt) => {
                match &elt.rec {
                    Some(Rec::Cmd(cmd)) => {
                        match cmd.split_first() {
                            Some((first, args)) => {
                                let mut child = Command::new(first)
                                    .args(args)
                                    .spawn()
                                    .expect("failed to run");
                                child.wait();
                                return Ok(());
                            }
                            None => {
                                println!("User error: invalid command {:?}", cmd);
                            }
                        }
                    }
                    Some(Rec::Rec(new_elts)) => {
                        elts = new_elts.to_vec();
                    }
                    _ => {
                        println!("none!")
                    }
                }

            }
        }
    }
}
