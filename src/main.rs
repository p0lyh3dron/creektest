fn main() {
    let mut script = creek::api::Scripts::new("lua");
    let _ = script.exec("require('main'); init();");

    let mut graphics = creek::api::Graphics::new("vulkan");
    graphics.init();

    let mut governor = creek::api::Governor::new("winit");
    governor.init();

    governor.submit();

    loop {
        _ = script.exec("update();");

        graphics.update();
    }
}