use {
    num_traits::*,
    std::{io, mem, ptr},
    winapi::{
        shared::minwindef::*,
        um::{
            errhandlingapi::*, handleapi::*, memoryapi::*, processthreadsapi::*, psapi::*,
            tlhelp32::*, winbase::*, winnt::*,
        },
    },
};

fn main() {
    let mut procname = String::new();

    println!("[+] Process Name : ");
    io::stdin()
        .read_line(&mut procname)
        .expect("Enter The Process Name!");
    let procname = procname.trim();

    let hprocess: HANDLE = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, getpid(procname)) };
    if hprocess == INVALID_HANDLE_VALUE {
        unsafe { println!("OpenProcess error : {}", GetLastError()) };
        return;
    }

    let mut option = String::new();
    println!("=> Options : \n\t[1] Commit\n\t[2] Reserve\n\t[3] change process priority\n\t[4] list the processes");

    io::stdin().read_line(&mut option).expect("Enter a number");
    let option: u32 = option.trim().parse().expect("Enter a number");

    if option == 1 {
        let mut capacity = String::new();
        println!("=> Options : \n\t[1] KB\n\t[2] MB\n\t[3] GB");

        io::stdin()
            .read_line(&mut capacity)
            .expect("Enter a number");

        let capacity: usize = capacity.trim().parse().expect("Enter a number");
        if capacity == 1 {
            let mut size = String::new();
            println!("Enter the size that you want to add : ");
            io::stdin().read_line(&mut size).expect("Enter a number");

            let size: usize = size.trim().parse().expect("Enter a number!");

            let alloc = unsafe {
                VirtualAllocEx(
                    hprocess,
                    ptr::null_mut(),
                    1024 * size,
                    MEM_COMMIT,
                    PAGE_EXECUTE_READWRITE,
                )
            };
            if alloc == INVALID_HANDLE_VALUE {
                unsafe {
                    println!("VirtualAllocEx error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }

            let mut pmc: PROCESS_MEMORY_COUNTERS = unsafe { mem::zeroed() };
            pmc.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS> as DWORD;

            let gpm = unsafe {
                GetProcessMemoryInfo(hprocess, &mut pmc as *mut PROCESS_MEMORY_COUNTERS, pmc.cb)
            };
            if gpm == 0 {
                unsafe {
                    println!("GetProcessMemoryInfo error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }
            let committed_memory = pmc.PagefileUsage;
            println!("[+] Committed memory : {}", committed_memory);
        } else if capacity == 2 {
            let mut size = String::new();
            println!("Enter the size that you want to add : ");
            io::stdin().read_line(&mut size).expect("Enter a number");

            let size: usize = size.trim().parse().expect("Enter a number!");

            let alloc = unsafe {
                VirtualAllocEx(
                    hprocess,
                    ptr::null_mut(),
                    pow(1024, 2) * size,
                    MEM_COMMIT,
                    PAGE_EXECUTE_READWRITE,
                )
            };
            if alloc == INVALID_HANDLE_VALUE {
                unsafe {
                    println!("VirtualAllocEx error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }

            let mut pmc: PROCESS_MEMORY_COUNTERS = unsafe { mem::zeroed() };
            pmc.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS> as DWORD;

            let gpm = unsafe {
                GetProcessMemoryInfo(hprocess, &mut pmc as *mut PROCESS_MEMORY_COUNTERS, pmc.cb)
            };
            if gpm == 0 {
                unsafe {
                    println!("GetProcessMemoryInfo error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }
            let committed_memory = pmc.PagefileUsage;
            println!("[+] Committed memory : {}", committed_memory);
        } else if capacity == 3 {
            let mut size = String::new();
            println!("Enter the size that you want to add : ");
            io::stdin().read_line(&mut size).expect("Enter a number");

            let size: usize = size.trim().parse().expect("Enter a number!");

            let alloc = unsafe {
                VirtualAllocEx(
                    hprocess,
                    ptr::null_mut(),
                    pow(1024, 3) * size,
                    MEM_COMMIT,
                    PAGE_EXECUTE_READWRITE,
                )
            };
            if alloc == INVALID_HANDLE_VALUE {
                unsafe {
                    println!("VirtualAllocEx error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }

            let mut pmc: PROCESS_MEMORY_COUNTERS = unsafe { mem::zeroed() };
            pmc.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS> as DWORD;

            let gpm = unsafe {
                GetProcessMemoryInfo(hprocess, &mut pmc as *mut PROCESS_MEMORY_COUNTERS, pmc.cb)
            };
            if gpm == 0 {
                unsafe {
                    println!("GetProcessMemoryInfo error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }
            let committed_memory = pmc.PagefileUsage;
            println!("[+] Committed memory : {}", committed_memory);
        }
    } else if option == 2 {
        let mut capacity = String::new();
        println!("=> Options : \n\t[1] KB\n\t[2] MB\n\t[3] GB");

        io::stdin()
            .read_line(&mut capacity)
            .expect("Enter a number");

        let capacity: usize = capacity.trim().parse().expect("Enter a number");
        if capacity == 1 {
            let mut size = String::new();
            println!("Enter the size that you want to add : ");
            io::stdin().read_line(&mut size).expect("Enter a number");

            let size: usize = size.trim().parse().expect("Enter a number!");

            let alloc = unsafe {
                VirtualAllocEx(
                    hprocess,
                    ptr::null_mut(),
                    1024 * size,
                    MEM_RESERVE,
                    PAGE_EXECUTE_READWRITE,
                )
            };
            if alloc == INVALID_HANDLE_VALUE {
                unsafe {
                    println!("VirtualAllocEx error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }

            let mut pmc: PROCESS_MEMORY_COUNTERS = unsafe { mem::zeroed() };
            pmc.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS> as DWORD;

            let gpm = unsafe {
                GetProcessMemoryInfo(hprocess, &mut pmc as *mut PROCESS_MEMORY_COUNTERS, pmc.cb)
            };
            if gpm == 0 {
                unsafe {
                    println!("GetProcessMemoryInfo error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }
            let reserved_memory = pmc.WorkingSetSize;
            println!("[+] Reserved memory : {}", reserved_memory);
        } else if capacity == 2 {
            let mut size = String::new();
            println!("Enter the size that you want to add : ");
            io::stdin().read_line(&mut size).expect("Enter a number");

            let size: usize = size.trim().parse().expect("Enter a number!");

            let alloc = unsafe {
                VirtualAllocEx(
                    hprocess,
                    ptr::null_mut(),
                    pow(1024, 2) * size,
                    MEM_RESERVE,
                    PAGE_EXECUTE_READWRITE,
                )
            };
            if alloc == INVALID_HANDLE_VALUE {
                unsafe {
                    println!("VirtualAllocEx error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }

            let mut pmc: PROCESS_MEMORY_COUNTERS = unsafe { mem::zeroed() };
            pmc.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS> as DWORD;

            let gpm = unsafe {
                GetProcessMemoryInfo(hprocess, &mut pmc as *mut PROCESS_MEMORY_COUNTERS, pmc.cb)
            };
            if gpm == 0 {
                unsafe {
                    println!("GetProcessMemoryInfo error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }
            let reserved_memoryx = pmc.WorkingSetSize;
            println!("[+] Reserved memory : {}", reserved_memoryx);
        } else if capacity == 3 {
            let mut size = String::new();
            println!("Enter the size that you want to add : ");
            io::stdin().read_line(&mut size).expect("Enter a number");

            let size: usize = size.trim().parse().expect("Enter a number!");

            let alloc = unsafe {
                VirtualAllocEx(
                    hprocess,
                    ptr::null_mut(),
                    pow(1024, 3) * size,
                    MEM_RESERVE,
                    PAGE_EXECUTE_READWRITE,
                )
            };
            if alloc == INVALID_HANDLE_VALUE {
                unsafe {
                    println!("VirtualAllocEx error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }

            let mut pmc: PROCESS_MEMORY_COUNTERS = unsafe { mem::zeroed() };
            pmc.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS> as DWORD;

            let gpm = unsafe {
                GetProcessMemoryInfo(hprocess, &mut pmc as *mut PROCESS_MEMORY_COUNTERS, pmc.cb)
            };
            if gpm == 0 {
                unsafe {
                    println!("GetProcessMemoryInfo error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }
            let reserved_memory = pmc.WorkingSetSize;
            println!("[+] Reserved memory : {}", reserved_memory);
        }
    } else if option == 3 {
        let mut ppt = String::new();
        println!("=> Options : \n\t[1] critical\n\t[2] idle\n\t[3] normal\n\t[4] above normal\n\t[5] below normal\n\t[6] real time");

        io::stdin().read_line(&mut ppt).expect("enter a number");
        let ppt: u32 = ppt.trim().parse().expect("enter a number!");
        let pagepriority: i32;

        let gpp = unsafe { GetPriorityClass(hprocess) };
        println!("The actual page priority is {}", gpp);

        if ppt == 1 {
            pagepriority = unsafe { SetPriorityClass(hprocess, HIGH_PRIORITY_CLASS) };
            if pagepriority == 0 {
                unsafe {
                    println!("SetPriorityClass error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }

            let gpp = unsafe { GetPriorityClass(hprocess) };
            println!("The page priority is {}", gpp);
        } else if ppt == 2 {
            pagepriority = unsafe { SetPriorityClass(hprocess, IDLE_PRIORITY_CLASS) };
            if pagepriority == 0 {
                unsafe {
                    println!("SetPriorityClass error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }

            let gpp = unsafe { GetPriorityClass(hprocess) };
            println!("The page priority is {}", gpp);
        } else if ppt == 3 {
            pagepriority = unsafe { SetPriorityClass(hprocess, NORMAL_PRIORITY_CLASS) };
            if pagepriority == 0 {
                unsafe {
                    println!("SetPriorityClass error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }

            let gpp = unsafe { GetPriorityClass(hprocess) };
            println!("The page priority is {}", gpp);
        } else if ppt == 4 {
            pagepriority = unsafe { SetPriorityClass(hprocess, ABOVE_NORMAL_PRIORITY_CLASS) };
            if pagepriority == 0 {
                unsafe {
                    println!("SetPriorityClass error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }

            let gpp = unsafe { GetPriorityClass(hprocess) };
            println!("The page priority is {}", gpp);
        } else if ppt == 5 {
            pagepriority = unsafe { SetPriorityClass(hprocess, BELOW_NORMAL_PRIORITY_CLASS) };
            if pagepriority == 0 {
                unsafe {
                    println!("SetPriorityClass error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }

            let gpp = unsafe { GetPriorityClass(hprocess) };
            println!("The page priority is {}", gpp);
        } else if ppt == 6 {
            pagepriority = unsafe { SetPriorityClass(hprocess, REALTIME_PRIORITY_CLASS) };
            if pagepriority == 0 {
                unsafe {
                    println!("SetPriorityClass error : {}", GetLastError());
                    CloseHandle(hprocess)
                };
                return;
            }

            let gpp = unsafe { GetPriorityClass(hprocess) };
            println!("The page priority is {}", gpp);
        }
    } else if option == 4 {
        let hsnapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
        if hsnapshot == ptr::null_mut() {
            unsafe { println!("CreateToolhelp32Snapshot error : {}", GetLastError()) };
        };

        let mut pee: PROCESSENTRY32 = unsafe { mem::zeroed() };
        pee.dwSize = mem::size_of::<PROCESSENTRY32>() as DWORD;

        if unsafe { Process32First(hsnapshot, &mut pee) } == 0 {
            unsafe {
                println!("Process32First error : {}", GetLastError());
                CloseHandle(hsnapshot)
            };
        }

        println!("{}:{}", "ProcessName", "PID");
        println!("----------------------------------");

        loop {
            let pid = pee.th32ProcessID;
            let processname = unsafe { std::ffi::CStr::from_ptr(pee.szExeFile.as_ptr() as _) }
                .to_str()
                .unwrap();

            println!("{}:{}", processname, pid);

            if unsafe { Process32Next(hsnapshot, &mut pee) } == 0 {
                break;
            }
        }

        unsafe { CloseHandle(hsnapshot) };
        return;
    }
}

fn getpid(procname: &str) -> DWORD {
    let mut pid: DWORD = 0;

    let hsnapshot: HANDLE = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if hsnapshot == INVALID_HANDLE_VALUE {
        unsafe {
            println!("Createtoolhelp32Snapshot error : {}", GetLastError());
            return 1;
        };
    }

    let mut pe: PROCESSENTRY32 = unsafe { std::mem::zeroed() };
    pe.dwSize = std::mem::size_of::<PROCESSENTRY32>() as DWORD;

    unsafe { Process32First(hsnapshot, &mut pe) };

    if unsafe { Process32First(hsnapshot, &mut pe) } != 0 {
        if pe.th32ProcessID == 0 {
            loop {
                unsafe { Process32Next(hsnapshot, &mut pe) };
                let exefile = unsafe { std::ffi::CStr::from_ptr(pe.szExeFile.as_ptr() as _) }
                    .to_str()
                    .unwrap();
                if exefile == procname {
                    pid = pe.th32ProcessID;
                    println!("[+] Process Name : {}\n[+] Process ID : {}", procname, pid);
                    break;
                }
                if unsafe { Process32Next(hsnapshot, &mut pe) } == 0 {
                    break;
                }
            }
        }
    }

    unsafe { CloseHandle(hsnapshot) };
    pid
}
