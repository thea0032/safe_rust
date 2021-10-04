use std::{env::args, fs::{create_dir, remove_dir_all}, path::PathBuf, process::Command};

fn main() {
    let mut args = args();
    args.next();
    if let Some(val) = args.next() {
        let mut buffer = PathBuf::from(val);
        buffer.push("target");
        let mut command = Command::new("cargo").arg("build").spawn().expect("Cargo is not installed!");
        let code = command.wait().expect("Command wasn't running!");
        println!("FILE LOCATION: {:?}", buffer);
        remove_dir_all(&buffer).expect("Could not delete directory!");
        create_dir(&buffer).expect("Could not create directory!");
        let target = buffer.to_str().unwrap().to_string();
        buffer.pop();
        println!("{:?}", target);
        buffer.push("src");
        let src = buffer.to_str().unwrap().to_string();
        println!("{:?}", src);
        #[cfg(target_os = "windows")]
        if code.success() {
            let mut command = Command::new("Xcopy");
            command.args(vec![src + "\\*", target, "/E".to_string(), "/H".to_string(), "/C".to_string(), "/I".to_string()]);
            command.spawn().expect("Command is not running!").wait().expect("Could not copy!");
        }
        #[cfg(not(target_os = "windows"))]
        if code.success() {
            let mut command = Command::new("cp");
            command.args(vec![src, target, "-r".to_string()]);
            command.spawn().expect("Command is not running!").wait().expect("Could not copy!");
        }
    } else {
        println!("Please include the path of your rust project's directory when you run this!");
    }
}