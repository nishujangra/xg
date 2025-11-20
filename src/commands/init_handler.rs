use crate::cli::init::{InitArgs, Languages, RestFramework};

pub fn handle_init(args: InitArgs) {
    match args.lang {
        Languages::Go => {
            println!("🚀 Initializing project...");

            match args.rest_framework {
                RestFramework::Gin => {
                    println!("Language: {:?}", args.lang);
                    println!("Project Name: {}", args.project);
                    println!("REST Framework: {:?}", args.rest_framework);
                },
                RestFramework::Echo => {
                    println!("Language: {:?}", args.lang);
                    println!("Project Name: {}", args.project);
                    println!("REST Framework: {:?}", args.rest_framework);
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
