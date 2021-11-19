use rand::prelude::*;
use std::{
    fs::{self, File},
    io::BufWriter,
    path::Path,
};

use png::OutputInfo;

pub struct Cryptor {
    pub file_name: String,
    pub info: OutputInfo,
}

impl Cryptor {
    pub fn new(file_name: String) -> Self {
        Cryptor {
            info: read_file(&file_name[..]),
            file_name,
        }
    }

    pub fn write_message(&self, message: String) {
        self.write_file(rand_data(self.info.width, self.info.height));
    }

    fn write_file(&self, data: Vec<u8>) {
        fs::remove_file(&self.file_name[..]).unwrap();

        let path = Path::new(&self.file_name[..]);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.info.width, self.info.height); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&data[..]).unwrap(); // Save
    }
}

fn read_file(file_name: &str) -> OutputInfo {
    let decoder = png::Decoder::new(File::open(file_name).unwrap());
    let mut reader = decoder.read_info().unwrap();

    let mut buf = vec![0; reader.output_buffer_size()];
    // Get the PNG header
    let info = reader.next_frame(&mut buf).unwrap();
    // Grab the bytes of the image.
    // let bytes = &buf[..info.buffer_size()];

    println!("{:#?}", info);

    info
}

fn rand_data(width: u32, height: u32) -> Vec<u8> {
    let total_pixels: u32 = (width * height).into();
    let total_bytes: u32 = total_pixels * 3;

    let distr = rand::distributions::Uniform::new_inclusive(0, 255);
    let mut data = vec![0u8; total_bytes.try_into().unwrap()];
    let mut rng = thread_rng();
    for x in &mut data {
        *x = rng.sample(distr);
    }

    data
}
