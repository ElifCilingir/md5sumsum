use std::env;
use walkdir::WalkDir;
use std::process::Command;


fn main() {
    let mut paths: Vec<String> = vec![];
    let mut hashcat = String::from("");
    for _argument in env::args().skip(1) {

        for entry in WalkDir::new(_argument)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| !e.file_type().is_dir())
        {
            paths.push(String::from(entry.path().to_string_lossy()))
        }
    }
    paths.sort();
    for (i, _x) in paths.iter().enumerate() {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(format!("md5sum \"{}\"", paths[i]));
        let output = String::from_utf8_lossy(&cmd.output().unwrap().stdout).to_string();
        
        let vec: Vec<&str> = output.split(" ").collect();
        hashcat += vec[0];
    }
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(format!("echo \"{}\" | md5sum", hashcat));
    let output = String::from_utf8_lossy(&cmd.output().unwrap().stdout).to_string();
    print!("{}", output);
}