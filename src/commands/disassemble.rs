use anyhow::{Error, Result};
use solana_rbpf::{
    elf::Executable, program::BuiltinProgram, static_analysis::Analysis, vm::TestContextObject,
};

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use crate::utils::{assert_asm_extension, check_valid_path};

pub fn disassemble(path: Option<String>, outfile: Option<String>) -> Result<()> {
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
    let stdout = std::io::stdout();

    match outfile {
        Some(outpath) => {
            let outpath = assert_asm_extension(outpath);
            let mut file = File::create(outpath)?;
            analysis.disassemble(&mut stdout.lock())?;
            analysis.disassemble(&mut file)?;
        }
        None => {
            let stdout = std::io::stdout();
            analysis.disassemble(&mut stdout.lock())?;
        }
    }

    Ok(())
}
