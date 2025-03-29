use nix::unistd::{ForkResult, fork};
use rs_libreoffice_bindings::Office;
use std::process::exit;
use std::thread;
use std::time::Duration;

struct Input {
    pub file_path: String,
    pub output_path: String,
    pub filter: String,
    pub out_file_type: String,
}

pub fn main() -> Result<(), String> {
    let office_path = std::env::var("LOK_PATH").expect("LOK_PATH not set");
    let items: Vec<Input> = vec![
        Input {
            file_path: "./fixtures/example.docx".to_string(),
            output_path: "./output/example.pdf".to_string(),
            filter: "writer_pdf_Export".to_string(),
            out_file_type: "pdf".to_string(),
        },
        Input {
            file_path: "./fixtures/example.pptx".to_string(),
            output_path: "./output/example_ppt.pdf".to_string(),
            filter: "impress_pdf_Export".to_string(),
            out_file_type: "pdf".to_string(),
        },
        Input {
            file_path: "./fixtures/example.xlsx".to_string(),
            output_path: "./output/example.html".to_string(),
            filter: "calc_HTML_WebQuery".to_string(),
            out_file_type: "html".to_string(),
        },
    ];

    let mut child_pids = Vec::new();

    for (idx, item) in items.iter().enumerate() {
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => {
                println!("parent process: spawned child {} with PID: {}", idx, child);
                child_pids.push(child);

                // Brief pause between forks to prevent potential race conditions
                thread::sleep(Duration::from_millis(100));
            }
            Ok(ForkResult::Child) => {
                // Each child process runs its own LibreOffice instance
                println!("child process {}: initializing LibreOffice", idx);
                let office = Office::new(&office_path)?;

                println!("child process {}: loading document {}", idx, item.file_path);
                let document = office.load_document(&item.file_path)?;

                println!(
                    "child process {}: saving to {} as {}",
                    idx, item.output_path, item.out_file_type
                );
                document.save_as(&item.output_path, &item.out_file_type, &item.filter)?;

                println!("child process {}: saved document successfully", idx);

                drop(document);
                drop(office);
                exit(0);
            }
            Err(err) => {
                eprintln!("fork failed for item {}: {}", idx, err);
            }
        }
    }

    println!(
        "parent process: waiting for {} child processes to complete",
        child_pids.len()
    );

    loop {
        if child_pids.is_empty() {
            break;
        }

        thread::sleep(Duration::from_millis(500));

        // check for completed children
        child_pids.retain(|&pid| {
            match nix::sys::wait::waitpid(
                nix::unistd::Pid::from_raw(pid.into()),
                Some(nix::sys::wait::WaitPidFlag::WNOHANG),
            ) {
                Ok(nix::sys::wait::WaitStatus::StillAlive) => true,
                _ => {
                    println!("child with PID {} has completed", pid);
                    false
                }
            }
        });
    }

    println!("Complete");
    Ok(())
}
