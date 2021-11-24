use rand::prelude::*;

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
