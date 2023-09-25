use std::time::Duration;
use std::thread::sleep;
use psutil::process::Process;

struct Filter {
    blacklist: Vec<&'static str>,
}

impl Filter {
    pub fn new() -> Self {
        Filter { blacklist: vec!["CCleaner"] }
    }

    pub fn add_blacklist(&mut self, process: &'static str) {
        self.blacklist.push(process);
    }

    pub fn start_filtering(&mut self) {
        loop {
            sleep(Duration::new(1, 0));
            let processes = psutil::process::processes().unwrap();

            for process_result in processes {
                if let Ok(process) = process_result {
                    if let Ok(name) = process.name() {
                        if self.blacklist.iter().any(|e| e == &&name) {
                            if let Err(e) = Process::new(process.pid()).unwrap().kill() {
                                println!("Failed to kill process: {}", e);
                            } else {
                                println!("Maybe succeed");
                            }
                        }
                    }
                }
            }
        }
    }
}
