use oxywar::run;
use std::env;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[1]);

    //let path =
    //    Path::new("/Users/louis/Documents/DataWarrior-Tables/Tablas-4/33__Scotch_Pine_Oil.dwar");

    if let Err(e) = run(path.to_path_buf(), path.with_extension("csv")) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
