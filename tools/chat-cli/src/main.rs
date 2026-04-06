#[tokio::main]
async fn main() {
    match craw_chat_cli::parse_cli_args(std::env::args()) {
        Ok(command) => {
            if craw_chat_cli::is_interactive_command(&command) {
                if let Err(error) = craw_chat_cli::execute_interactive_command(command).await {
                    eprintln!("{error}");
                    std::process::exit(error.exit_code());
                }
            } else {
                match craw_chat_cli::execute_command(command).await {
                    Ok(output) => match craw_chat_cli::render_output(&output) {
                        Ok(rendered) => {
                            if !rendered.is_empty() {
                                println!("{rendered}");
                            }
                        }
                        Err(error) => {
                            eprintln!("{error}");
                            std::process::exit(error.exit_code());
                        }
                    },
                    Err(error) => {
                        eprintln!("{error}");
                        std::process::exit(error.exit_code());
                    }
                }
            }
        }
        Err(error) => {
            if error.exit_code() == 0 {
                println!("{}", error.message());
            } else {
                eprintln!("{error}");
            }
            std::process::exit(error.exit_code());
        }
    }
}
