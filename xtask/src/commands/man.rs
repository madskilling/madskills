use std::fs;
use std::path::PathBuf;

use clap::Args;

#[derive(Args, Debug)]
pub struct ManArgs {
    /// Output directory (default: target/man)
    #[arg(long = "out-dir", default_value = "target/man")]
    pub out_dir: PathBuf,
}

pub fn cmd_man(args: ManArgs) -> Result<(), String> {
    let out_dir = crate::workspace_root().join(args.out_dir);
    fs::create_dir_all(&out_dir).map_err(|e| format!("{}: {e}", out_dir.display()))?;

    // Generate main command manpage
    let cmd = madskills::command();
    let man = clap_mangen::Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Vec::new();
    man.render(&mut buffer)
        .map_err(|e| format!("render manpage: {e}"))?;

    let man_path = out_dir.join("madskills.1");
    fs::write(&man_path, buffer).map_err(|e| format!("{}: {e}", man_path.display()))?;
    println!("wrote {}", man_path.display());

    // Generate subcommand manpages
    for subcommand in cmd.get_subcommands() {
        let name = subcommand.get_name();
        let man = clap_mangen::Man::new(subcommand.clone());
        let mut buffer: Vec<u8> = Vec::new();
        man.render(&mut buffer)
            .map_err(|e| format!("render manpage for {name}: {e}"))?;

        let man_path = out_dir.join(format!("madskills-{name}.1"));
        fs::write(&man_path, buffer).map_err(|e| format!("{}: {e}", man_path.display()))?;
        println!("wrote {}", man_path.display());
    }

    Ok(())
}
