use beatboxer_tui::app::app::App;
use std::io::{BufRead, BufReader};
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};
use std::{env, fs};

const BACKEND_ZIP: &[u8] =
    include_bytes!("../../BeatBoxer-Engien/build/bundle/beat-boxer_engien.zip");

fn main() {
    let temp_dir = "/tmp/supp";
    let zip_path = format!("{}/backend.zip", temp_dir);
    let extract_dir = format!("{}/backend_extracted", temp_dir);

    let _ = fs::remove_dir_all(&extract_dir);
    fs::create_dir_all(&extract_dir).unwrap();
    fs::write(&zip_path, BACKEND_ZIP).expect("Can't write Zip");

    Command::new("unzip")
        .arg("-o")
        .arg(&zip_path)
        .arg("-d")
        .arg(&extract_dir)
        .output()
        .expect("Error while unzip BeatBoxer-Engien");

    let java_bin = format!("{}/jre/bin/java", extract_dir);
    let mut perms = fs::metadata(&java_bin).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&java_bin, perms).unwrap();

    let mut child = Command::new(&java_bin)
        .arg("-cp")
        .arg(format!("{}/app/lib/*", extract_dir))
        .arg("com.FerdiStro.Main")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Can't run BeatBoxer-Engien");

    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        if let Ok(text) = line {
            println!("[JAVA]: {}", text);
            if text.contains("BACKEND_READY") {
                break;
            }
        }
    }
    println!("--------------------------------------------");
    println!("Start TUI:");
    println!("--------------------------------------------");

    //Set Envs:
    unsafe {
        env::set_var("BEATBOXER_READ_PATH", "fromEngien_shm.bin");
        env::set_var("BEATBOXER_WRITE_PATH", "toEngien_shm.bin");
        env::set_var("BEATBOXER_READ_CDJ_PATH", "x_player_wave_form.bin");
        env::set_var("BEATBOXER_FILE_EXPLORER_PATH", "/");
    }

    App::new().unwrap();

    child.kill().unwrap();
    let _ = fs::remove_dir_all(temp_dir);
}
