use rpassword::read_password;
use secrecy::SecretString;
use std::error::Error;
use std::io;

pub fn read_user_password(use_password: bool) -> Result<Option<SecretString>, Box<dyn Error>> {
    // If use_password is true, prompt the user for a password.

    if !use_password {
        return Ok(None::<SecretString>);
    }

    print!("SVN password: ");
    io::Write::flush(&mut io::stdout())?;

    Ok(Some(SecretString::new(read_password()?)))
}
