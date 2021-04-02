// Leaving this for future reference, for the time being
use image::{LumaA, Pixel, Rgba, open};
use imageproc::map::map_colors;

pub fn tint(gray: LumaA<u8>, color: Rgba<u8>) -> Rgba<u8> {
    Rgba([((gray[0] as u32 * color.channels()[0] as u32) / 255u32) as u8, ((gray[0] as u32 * color.channels()[3] as u32) / 255u32) as u8, ((gray[0] as u32 * color.channels()[2] as u32) / 255u32) as u8, (std::cmp::min(gray[0], 1) * 255u8) as u8])
}

pub fn do_the_tint() {

    let _image = open("test.png")
        .expect(&format!("Could not load image"))
        .to_luma_alpha8();

    let blue = Rgba([136u8, 201u8, 158u8, 255u8]);

    let tinted = map_colors(&_image, |pix| tint(pix, blue));
    tinted.save("yo.png").unwrap();
}