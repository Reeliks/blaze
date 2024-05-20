use blaze::shell::command_handler::ShellCommandHandler;

#[tokio::main]
async fn main() {
    ShellCommandHandler::new().handle_command().await.unwrap();
}
