fn main() {
    cc::Build::new()
        .pic(false)
        .warnings(false)
        .flag("-mmcu=atmega32u4")
        .compiler("avr-gcc")
        .archiver("avr-ar")
        .file("src/usb_keyboard.c")
        .compile("usb_keyboard");
}
