use crate::cli::init::{InitArgs, Languages};

pub fn handle_init(args: InitArgs) {
    match args.lang {
        Languages::Go => {
            println!("🚀 Initializing project...");
            println!("Language: {:?}", args.lang);
            println!("Project Name: {}", args.project);
            println!("REST Framework: {:?}", args.rest_framework);
            // Here you will call your generator logic later
            println!("(Generator logic will go here)");
        }
        _ => {
            println!("Wrong languages selected, {:?} is not currently supported", args.lang);
        }
    }

}
