
use std::process::Command;

pub fn exec_linux_shell(s:String) -> Option<String> {
    let output = Command::new("/bin/bash")
        .arg("-c")
        .arg(s)
        .output()
        .expect("shell error");

    // let res:String = "".to_string();
    // let result = BufReader::new(res);
    // cmd.stdout(result.into());
    // cmd.output();
    let res = String::from_utf8(output.stdout).unwrap();
    Some(res)
}