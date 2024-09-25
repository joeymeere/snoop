use anyhow::{Error, Result};
use solana_rbpf::{
    elf::Executable, program::BuiltinProgram, static_analysis::Analysis, vm::TestContextObject,
};

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use crate::utils::check_valid_path;

pub fn instructions(path: Option<String>) -> Result<()> {
    let path = path.ok_or_else(|| Error::msg("Path not provided"))?;
    let path = Path::new(&path);

    check_valid_path(&path)?;

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let program: &'static [u8] = Box::leak(buffer.into_boxed_slice());

    let loader = Arc::new(BuiltinProgram::new_mock());
    let executable = Executable::<TestContextObject>::from_elf(program, loader).unwrap();
    let analysis = Analysis::from_executable(&executable).unwrap();

    for ix in analysis.instructions.iter() {
        for (i, instr) in ix.to_vec().iter().enumerate() {
            println!("{:?}: {:?}", i, instr);
        }
    }

    Ok(())
}
