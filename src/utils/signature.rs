use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn gen_sign(method: &str, path: &str, date: &str, length: isize, password: &str, operator: &str) -> String {
    let str_for_sign = format!("{}&{}&{}&{}&{}", method, path, date, length, md5(password));
    format!("UpYun {}: {}", operator, md5(&str_for_sign))
}

fn md5(s: &str) -> String {
    let mut h = Md5::new();
    h.input_str(s);
    h.result_str()
}
