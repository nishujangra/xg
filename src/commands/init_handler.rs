use crate::{cli::create_go_app::{GoArgs, RestFramework}};

use crate::{generators::go as go_lang};

pub fn handle_init(args: GoArgs) {
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
