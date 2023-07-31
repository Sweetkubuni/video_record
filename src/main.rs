#![allow(unused)]
fn main() {
use std::process::Command;
use log::{info, error, LevelFilter};
use systemd_journal_logger::JournalLog;

JournalLog::default()
    .with_syslog_identifier("video_record".to_string())
    .install().unwrap();
log::set_max_level(LevelFilter::Info);

let mut libcamera_process = Command::new("libcamera-vid")
	.args(["-t", "10000"])
        .args(["-o", "/media/ssd-drive/test.h264"])
        .spawn();

match libcamera.try_wait() {
   Ok(mut child) => {
     match child.stdout {
       None => error!("stdout not defined"),
       Some(_) =>{ let stdout = child.stdout.take().unwrap(); info!("{:?}", stdout) },
     }
   },
   Err(error) => error!("problem spawning command: {:?}", error),
};

match libcamera_process.try_wait() {
    Ok(Some(status)) => warn!("exited with: {status}"),
    Ok(None) => {
      println!("status not ready yet, lets wait");
      
    }
}

    
}
