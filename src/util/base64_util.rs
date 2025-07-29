use base64::{engine::general_purpose, Engine as _};
use rand::thread_rng;
use rand::Rng;

// 生成一个随机的base64
#[allow(dead_code)]
pub fn generate_random_base64_value() -> String {
    let rng = &mut thread_rng();
    let bytes = rng.r#gen::<[u8; 32]>();
    general_purpose::STANDARD.encode(&bytes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen_random() {
        let secret = generate_random_base64_value();
        println!("secret: {}", secret);
    }
}
