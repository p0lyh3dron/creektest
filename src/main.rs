fn main() {
    let mut audio = creek::api::Audio::new("rodio");

    let mut script = creek::api::Scripts::new("lua");
    let _ = script.exec("require('main'); init();");

    let sound = audio.load_sound_file("../creek/assets/sound.wav").unwrap();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(350));
        audio.play_sound(sound);
        loop {
            _ = script.exec("update();");
        }
    });

    let mut io = creek::api::IO::new("vulkan");
    io.init();
}