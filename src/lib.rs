use colored::*;
use dwarrs::{get_struct, open_dwar, DWRecord};
use indicatif::{ProgressBar, ProgressStyle};
use open;
use oxychem::{get_cas, get_cid, get_properties};
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

//******************************************************************/
#[cfg(target_os = "macos")]
fn open_datawarrior() -> Result<(), Box<dyn Error>> {
    // * Open DataWarrior
    let script = "activate application \"DataWarrior\"
    tell application \"System Events\"
    
        repeat until exists window \"About OSIRIS DataWarrior\" of process \"DataWarrior\"
        end repeat
    
        repeat while exists window \"About OSIRIS DataWarrior\" of process \"DataWarrior\" 
            delay 1
        end repeat
    
        -- display dialog \"DataWarrior is ready\"
    end tell
    return";

    Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("Launch and wait applescript -> command failed to start");

    // Open a background table (if not DataWarrior closes)
    open::with(
        "/Applications/DataWarrior.app/reference/ApprovedDrugs2015.dwar",
        "DataWarrior",
    )?;

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn open_datawarrior() -> Result<(), Box<dyn Error>> {
    open::with("", "DataWarrior")?;
    let delay = time::Duration::from_secs(8);
    thread::sleep(delay);

    // TODO: Open a background table, unknown if DataWarrior closes in windows.

    Ok(())
}

//******************************************************************/
pub fn run(path: PathBuf, output: PathBuf) -> Result<(), Box<dyn Error>> {
    open_datawarrior()?;

    let data = open_dwar(path)?;

    let pb = ProgressBar::new(data.len() as u64);
    pb.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.green/blue}] {pos}/{len} {msg} ({eta})",
    ));

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_path(output)?;

    for (idx, record) in data.iter().enumerate() {
        pb.set_position(idx as u64);
        // TODO: Start from a custom index
        //println!("{:?}", record.compound);
        //if idx <= 832 {
        //if idx >= 832 {
        //    continue;
        //}

        //if idx <= 458 {
        //    continue;
        //}

        let cid = get_cid(record.compound.clone())?;

        if let -1 = cid {
            wtr.serialize(DWRecord {
                structure: "".to_string(),
                smiles: "NA".to_string(),
                inchikey: "NA".to_string(),
                cas: "NA".to_string(),
                compound: record.compound.clone(),
            })?;

            pb.println(format!(
                "{} {}",
                "[❓] Compound not found:".red().bold(),
                //"[𐄂] Compound not found".red().bold(),
                //"=>".red(),
                record.compound.blue().bold()
            ));
        } else {
            let cas_value = get_cas(cid)?;
            let (smiles_value, inchikey_value) = get_properties(cid)?;
            let struct_value = get_struct(cid)?;
            // let struct_value = "".to_string(); // get_struct(cid)?;

            wtr.serialize(DWRecord {
                structure: struct_value,
                smiles: smiles_value,
                inchikey: inchikey_value,
                cas: cas_value,
                compound: record.compound.clone(),
            })?;

            pb.println(format!(
                "{} {}",
                "[✅] Compound found:".green().bold(),
                //"[🗸] Compound found".green().bold(),
                //"=>".green(),
                record.compound.blue().bold()
            ));
        }

        wtr.flush()?;
    }

    pb.finish_with_message("Done!");

    Ok(())
}
