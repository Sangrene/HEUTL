use crate::shared::errors::Error;
use serde_json::Value;
use std::time::SystemTime;
use std::{
    process::{Command, Stdio},
    time::Duration,
};
use sysinfo::{Pid, System};

fn limit_process_memory_and_time(pid: Pid, memory_limit: u64, time_limit: u64) {
    let mut sys = System::new_all();
    sys.refresh_all();
    let start_time = SystemTime::now();

    while sys.process(pid).is_some() {
        if sys.process(pid).unwrap().memory() > memory_limit {
            println!("Process memory limit exceeded");
            sys.process(pid).unwrap().kill();
        }
        if start_time.elapsed().unwrap() > Duration::from_secs(time_limit) {
            println!("Process time limit exceeded");
            sys.process(pid).unwrap().kill();
        }
        sys.refresh_all();
    }
}

// TODO: Get result using memmap file / memory buffer instead of stdout
pub fn run_python_script(script: &String, input: &Value) -> Result<String, Error> {
    let handle = Command::new("python/.venv/bin/python")
        .arg("python/container.py")
        .arg(format!("--script={}", script))
        .arg(format!("--input={}", input.to_string()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut sys = System::new_all();
    sys.refresh_all();

    let pid = Pid::from_u32(handle.id());

    limit_process_memory_and_time(pid, 1024 * 1024 * 1024, 30);

    let output = handle.wait_with_output().unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout)
        .to_string()
        .trim()
        .to_string();
    Ok(stdout)
}

pub fn run_python_script_output_json(script: &String, input: &Value) -> Result<Value, Error> {
    let result = run_python_script(script, input)?;
    let result = serde_json::from_str::<Value>(&result)?;
    Ok(result)
}
