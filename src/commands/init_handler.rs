use crate::cli::init::InitArgs;

pub fn handle_init(args: InitArgs) {
    println!("🚀 Initializing project...");
    println!("Language: {}", args.lang);
    println!("Project Name: {}", args.project);
    println!("REST Framework: {:?}", args.rest_framework);

    // Here you will call your generator logic later
    println!("(Generator logic will go here)");
}
