#![allow(unused)]
fn main() {
    use log::{error, info, LevelFilter};
    use std::process::Command;
    use std::{thread, time};
    use systemd_journal_logger::JournalLog;

    JournalLog::default()
        .with_syslog_identifier("video_record".to_string())
        .install()
        .unwrap();
    log::set_max_level(LevelFilter::Info);

    let mut libcamera_process = Command::new("libcamera-vid")
        .args(["-t", "10000"])
        .args(["-o", "/media/ssd-drive/test.h264"])
        .spawn();

    match libcamera_process {
        Ok(mut handle) => {
            info!("spawn libcamera process...");

            loop {
                match handle.try_wait() {
                    Ok(Some(status)) => {
                        info!("exited with: {status}");
                        break;
                    }
                    Ok(None) => {
                        println!("status not ready yet, lets wait");
                        let ten_millis = time::Duration::from_millis(10);
                        thread::sleep(ten_millis);
                    }
                    Err(e) => error!("error attempting to wait: {e}"),
                }
            }
        }
        Err(error) => error!("problem spawning command: {:?}", error),
    };
}
