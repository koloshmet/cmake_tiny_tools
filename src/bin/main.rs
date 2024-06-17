use cmake_tiny_tools::init;
use cmake_tiny_tools::run;

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    let Some(cmd) = args.nth(1) else {
        println!("Usage: cmake-tiny-tools <command>");
        std::process::exit(1);
    };
    let mut cmd_args = args.skip(2);
    match cmd.as_str() {
        "init" => {
            let project_type = cmd_args.find(|arg| arg.as_str() == "--bin")
                .and_then(|_| Some(init::ProjectType::Executable))
                .unwrap_or(init::ProjectType::Library);
            init(project_type).await.unwrap_or_else(|e| {
                println!("Error: {e}");
                std::process::exit(1);
            });
        }
        "run" => {
            let source_file = std::path::PathBuf::from(cmd_args.nth(0).unwrap_or("main.cpp".to_string()));
            run(&source_file).await.unwrap_or_else(|e| {
                println!("Error: {e}");
                std::process::exit(1);
            });
        }
        _ => {
            println!("Command {cmd} is not supported");
            std::process::exit(1);
        }
    }
}
