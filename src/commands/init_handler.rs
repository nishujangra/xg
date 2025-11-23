use crate::{cli::init::{InitArgs, Languages, RestFramework}};

use crate::{generators::go as go_lang};

pub fn handle_init(args: InitArgs) {
    match args.lang {
        Languages::Go => {
            println!("Initializing project...");

            match args.rest_framework {
                RestFramework::Gin => {
                    go_lang::gin::init(&args.project);
                },
                RestFramework::Echo => {
                    go_lang::echo::init(&args.project);
                }
            }
        }
        _ => {
            println!(
                "Wrong languages selected, {:?} is not currently supported",
                args.lang
            );
        }
    }
}
