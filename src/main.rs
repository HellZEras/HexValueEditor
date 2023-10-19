mod process;
use std::ptr;
use winapi::shared::minwindef::DWORD;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};
use winapi::um::memoryapi::{ReadProcessMemory, WriteProcessMemory};
use std::io::stdin;

fn main() {
    println!("Give the process name without .exe");
    let mut process_name = String::new();
    stdin().read_line(&mut process_name).expect("Error");
    //Change process id or make a function that retrieves process id through process name
    let process_id_to_modify = process::return_process_pid(&process_name) as DWORD;
    println!("Give the Hex starting with 0x");
    let mut address = String::new();
    stdin().read_line(&mut address).expect("Error");
    let address_to_modify:i64 = address.trim().parse().expect("Not an integer");

    let process_handle = unsafe {
        OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE,
            0,
            process_id_to_modify,
        )
    };

    if process_handle.is_null() {
        eprintln!("Failed to open process. Error code: {}", unsafe {
            winapi::um::errhandlingapi::GetLastError()
        });
        return;
    }

    let mut value_to_read: u32 = 0;
    let read_result = unsafe {
        ReadProcessMemory(
            process_handle,
            address_to_modify as *const _,
            &mut value_to_read as *mut _ as *mut _,
            std::mem::size_of_val(&value_to_read),
            ptr::null_mut(),
        )
    };

    if read_result == 0 {
        eprintln!("Failed to read process memory. Error code: {}", unsafe {
            winapi::um::errhandlingapi::GetLastError()
        });
        unsafe {
            winapi::um::handleapi::CloseHandle(process_handle);
        }
        return;
    }

    println!("Read value at 0x{:x}: {}", address_to_modify, value_to_read);

    let new_value = 400000;

    let write_result = unsafe {
        WriteProcessMemory(
            process_handle,
            address_to_modify as *mut _,
            &new_value as *const _ as *const _,
            std::mem::size_of_val(&new_value),
            ptr::null_mut(),
        )
    };

    if write_result == 0 {
        eprintln!("Failed to write process memory. Error code: {}", unsafe {
            winapi::um::errhandlingapi::GetLastError()
        });
    } else {
        println!("Successfully wrote {} to 0x{:x}", new_value, address_to_modify);
    }

    unsafe {
        winapi::um::handleapi::CloseHandle(process_handle);
    }



}


