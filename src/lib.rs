use rand::Rng;

pub fn magic_number() -> u8 {
    let mut rng = rand::thread_rng();

    rng.gen_range(0..=1)
}
