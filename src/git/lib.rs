use std::fs;
use std::io::Error;

pub fn cat_file(pretty: bool, object: String) { 
}

pub fn init() -> Result<(), Error> {
    fs::create_dir(".tinygit")?;
    fs::create_dir(".tinygit/objects")?;
    fs::create_dir(".tinygit/refs")?;
    fs::write(".tinygit/HEAD", "ref: refs/heads/main\n")?;
    println!("Initialized git directory");
    Ok(())
}
