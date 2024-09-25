use anyhow::{Error, Result};
use solana_rbpf::{elf::Executable, program::BuiltinProgram, vm::TestContextObject};

use std::fmt::Write;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use crate::utils::check_valid_path;

pub fn cfg(path: Option<String>) -> Result<()> {
    let path = path.ok_or_else(|| Error::msg("Path not provided"))?;
    let path = Path::new(&path);

    check_valid_path(&path)?;

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let program: &'static [u8] = Box::leak(buffer.into_boxed_slice());

    let loader = Arc::new(BuiltinProgram::new_mock());
    let executable = Executable::<TestContextObject>::from_elf(program, loader).unwrap();

    let config = executable.get_config();
    let mut pretty_output = String::new();
    write!(&mut pretty_output, "{:#?}", config)?;

    println!("{}", pretty_output);

    Ok(())
}
