//! Real-time log watcher for BLT and crash logs

use colored::Colorize;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Spawn a background thread to watch game logs
pub fn spawn_log_watcher(game_path: &Path) {
    let game_path = game_path.to_path_buf();

    // Get crash log path (LocalAppData)
    let crash_log = dirs::data_local_dir().map(|p| p.join("PAYDAY 2/crashlog.txt"));

    std::thread::spawn(move || {
        watch_logs_loop(game_path, crash_log);
    });
}

fn watch_logs_loop(game_path: PathBuf, crash_log: Option<PathBuf>) {
    let mut blt_file: Option<BufReader<File>> = None;
    let mut crash_file: Option<BufReader<File>> = None;
    let mut blt_pos = 0;
    let mut crash_pos = 0;

    println!("👀 Watching logs...");

    loop {
        // 1. Scan for latest BLT log if not open
        if blt_file.is_none() {
            if let Some(reader) = try_open_latest_blt_log(&game_path) {
                blt_pos = reader.get_ref().stream_position().unwrap_or(0);
                blt_file = Some(reader);
            }
        }

        // 2. Read new BLT log lines
        if let Some(reader) = &mut blt_file {
            blt_pos = read_new_lines(reader, blt_pos, "[BLT]", true);
        }

        // 3. Watch crash log
        if let Some(path) = &crash_log {
            if crash_file.is_none() {
                if let Ok(f) = File::open(path) {
                    let mut f = f;
                    let _ = f.seek(SeekFrom::End(0));
                    crash_pos = f.stream_position().unwrap_or(0);
                    crash_file = Some(BufReader::new(f));
                }
            }

            if let Some(reader) = &mut crash_file {
                crash_pos = read_new_lines(reader, crash_pos, "[CRASH]", false);
            }
        }

        std::thread::sleep(Duration::from_millis(500));
    }
}

fn try_open_latest_blt_log(game_path: &Path) -> Option<BufReader<File>> {
    let logs_dir = game_path.join("mods/logs");
    let entries = std::fs::read_dir(&logs_dir).ok()?;

    let mut latest_log: Option<(PathBuf, std::time::SystemTime)> = None;

    for entry in entries.flatten() {
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) == Some("txt") {
            if let Ok(meta) = path.metadata() {
                if let Ok(modified) = meta.modified() {
                    if let Some((_, last_mod)) = latest_log {
                        if modified > last_mod {
                            latest_log = Some((path, modified));
                        }
                    } else {
                        latest_log = Some((path, modified));
                    }
                }
            }
        }
    }

    if let Some((path, _)) = latest_log {
        if let Ok(mut f) = File::open(&path) {
            let _ = f.seek(SeekFrom::End(0));

            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            println!("👀 Watching BLT log: {}", file_name);

            return Some(BufReader::new(f));
        }
    }

    None
}

fn read_new_lines(reader: &mut BufReader<File>, mut pos: u64, prefix: &str, is_blt: bool) -> u64 {
    let current_len = reader.get_ref().metadata().map(|m| m.len()).unwrap_or(0);

    if current_len < pos {
        // File truncated - reset
        pos = 0;
        let _ = reader.seek(SeekFrom::Start(0));
    }

    let mut line = String::new();
    while let Ok(n) = reader.read_line(&mut line) {
        if n == 0 {
            break;
        }

        let colored_prefix = if is_blt {
            prefix.blue()
        } else {
            prefix.red().bold()
        };

        print!("{} {}", colored_prefix, line);
        pos += n as u64;
        line.clear();
    }

    pos
}
