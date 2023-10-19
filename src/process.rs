use sysinfo::{ProcessExt, System, SystemExt};



pub fn return_process_pid(name:&str) -> usize{
    let mut pid_retrieved = 0;
    let s = System::new_all();
    for process in s.processes_by_name(name) {
        pid_retrieved =  process.pid().into();
    }
    pid_retrieved
}