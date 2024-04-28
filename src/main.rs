use std::io::Result;

use blaze::shell;

fn main() -> Result<()> {
    shell::handling::handle_command_arguments()?;
    Ok(())
}
