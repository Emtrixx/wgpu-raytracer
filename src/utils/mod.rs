use gif::{Encoder, Frame, Repeat};

pub(crate) fn save_gif(
    path: &str,
    frames: &mut Vec<Vec<u8>>,
    speed: i32,
    size: u16,
) -> Result<(), failure::Error> {
    let mut image = std::fs::File::create(path)?;
    let mut encoder = Encoder::new(&mut image, size, size, &[])?;
    encoder.set_repeat(Repeat::Infinite)?;

    for mut frame in frames {
        encoder.write_frame(&Frame::from_rgba_speed(size, size, &mut frame, speed))?;
    }

    Ok(())
}
