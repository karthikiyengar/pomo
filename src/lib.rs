use std::collections::HashMap;

pub mod format {
    use std::io::BufReader;

    pub fn get_formatted_duration(value: u32) -> String {
        let seconds = value % 60;
        let minutes = (value / 60) % 60;
        let hours = (value / 60) / 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    pub fn play_audio(file: String) {
        std::thread::spawn(|| {
            let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
            let file = std::fs::File::open(file).unwrap();
            let beep1 = stream_handle.play_once(BufReader::new(file)).unwrap();
            beep1.set_volume(0.2);
            std::thread::sleep(std::time::Duration::from_millis(2000));
        });
    }
}
