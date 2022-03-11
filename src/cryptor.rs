use std::{
    fs::{self, File},
    io::BufWriter,
    path::Path,
};

use png::OutputInfo;

pub struct Cryptor {
    pub filename: String,
    pub info: OutputInfo,
    img_bytes: Vec<u8>,
}

impl Cryptor {
    pub fn new(filename: String) -> Self {
        let path = Path::new(&filename[..]);
        let decoder = png::Decoder::new(File::open(path).unwrap());
        let mut reader = decoder.read_info().unwrap();

        let mut buf = vec![0; reader.output_buffer_size()];
        // Get the PNG header
        let info = reader.next_frame(&mut buf).unwrap();
        // Grab the bytes of the image.
        let bytes = &buf[..info.buffer_size()];
        let img_bytes = Vec::from(bytes);

        Cryptor {
            info,
            filename,
            img_bytes,
        }
    }

    fn write_byte(&mut self, byte: &u8, index: &mut usize) {
        let mut temp_byte = byte.clone();

        // 1 byte per 4 image byte
        for _ in 0..4u32 {
            // get the 2 bits value
            let bits: u8 = temp_byte % 4;
            // shifts to the right and then to the left
            self.img_bytes[*index] >>= 2;
            self.img_bytes[*index] <<= 2;

            self.img_bytes[*index] += bits;
            // shift 2 bits to the right
            temp_byte >>= 2;

            *index += 1;
        }
    }

    pub fn write_message(&mut self, message: String) {
        self.write_data(message.as_bytes());
    }

    /// Write the contents of a file into the picture
    pub fn write_from_file(&mut self, from_file: &str) {
        let data = fs::read(from_file).expect("Something went wrong reading the file");
        self.write_data(&data);
    }

    pub fn write_data(&mut self, data: &[u8]) {
        let mut i_img_byte: usize = 0;
        // Each char is 1 byte (8 bits), the last 2 bits from
        // each byte from the image pixels will be used to save 2 bits from each char
        // foreach char it will be needed 4 image bytes
        for byte in data {
            self.write_byte(byte, &mut i_img_byte);
        }

        // Write final byte
        self.write_byte(&0, &mut i_img_byte);

        self.write_file();
    }

    pub fn read_file_message(&self) -> String {
        let mut ch_byte = 'a' as u8;
        let mut msg_bytes: Vec<u8> = Vec::new();
        let mut i_img_byte: usize = 0;

        while ch_byte != 0u8 {
            let mut tmp_byte = vec![0u8; 4];
            for b in tmp_byte.iter_mut() {
                let img_byte = self.img_bytes[i_img_byte];
                let bits = img_byte % 4;
                *b = bits;

                i_img_byte += 1;
            }

            tmp_byte.reverse();

            let mut byte = tmp_byte[0];
            for i in 1..4 {
                byte <<= 2;
                byte += tmp_byte[i];
            }

            msg_bytes.push(byte);
            ch_byte = byte;
        }
        // remove null char
        msg_bytes.pop();
        String::from_utf8(msg_bytes).unwrap()
    }

    fn write_file(&self) {
        match fs::remove_file(Path::new(&self.filename)) {
            _ => (),
        }

        let file = File::create(&self.filename).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.info.width, self.info.height); // Width is 2 pixels and height is 1.
        encoder.set_color(self.info.color_type);
        encoder.set_depth(self.info.bit_depth);

        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&self.img_bytes[..]).unwrap(); // Save
    }
}
