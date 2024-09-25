use anyhow::{Error, Result};
use graphviz_rust::cmd::Format;
use graphviz_rust::exec_dot;
use solana_rbpf::{
    elf::Executable, program::BuiltinProgram, static_analysis::Analysis, vm::TestContextObject,
};

use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use crate::utils::{check_valid_path, find_project_root};

pub fn graph(path: Option<String>) -> Result<()> {
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

    let mut graph_buffer = Vec::new();
    analysis
        .visualize_graphically(&mut graph_buffer, None)
        .unwrap();
    let graph_string = String::from_utf8(graph_buffer)?;

    let graph_pdf = exec_dot(graph_string, vec![Format::Pdf.into()])?;
    let project_root = find_project_root()?;
    let pdf_filename = project_root.join("graph_output.pdf");

    if let Some(parent) = pdf_filename.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&pdf_filename, graph_pdf).map_err(|e| {
        Error::new(e).context(format!("Failed to write PDF to {}", pdf_filename.display()))
    })?;

    Ok(())
}
