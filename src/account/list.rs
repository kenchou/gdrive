use crate::config;
use crate::config::Config;

pub fn list() -> Result<(), config::Error> {
    let accounts = Config::list_accounts()?;

    if accounts.is_empty() {
        println!("No accounts found");
        println!("Use `gdrive account add` to add an account.");
    } else {
        for account in accounts {
            println!("{}", account);
        }
    }

    Ok(())
}