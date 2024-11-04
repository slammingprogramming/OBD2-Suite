use std::fs::File;
use std::io::{self, Write};

pub fn generate_report(data: &str) -> io::Result<()> {
    let mut file = File::create("diagnostic_report.txt")?;
    writeln!(file, "Diagnostic Report")?;
    writeln!(file, "{}", data)?;
    Ok(())
}