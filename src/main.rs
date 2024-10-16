use utils::ask_question;
pub mod utils;
use zk_rust_io;

fn main() {
    let eth_address: String = zk_rust_io::read();
    zk_rust_io::commit(&eth_address);
    let difficiulity :u128 = 2;

    println!("ajson ddress: {}", eth_address);
    let clean_address = if eth_address.starts_with("0x") {
        &eth_address[40..]
    } else {
        &eth_address[38..]
    };

    println!("clean ddress is here: {}", clean_address);

    let decimal_value = u128::from_str_radix(clean_address, 16).expect("Invalid eth address.");
    let is_winner: bool = decimal_value % difficiulity == 0;
    println!("iswinner stat: {}", is_winner);

    zk_rust_io::commit(&decimal_value);
    zk_rust_io::commit(&is_winner);
}


fn input() {
    let mut eth_address = String::new();
    let question1 = "fill eth addr";
    eth_address.push_str(&ask_question(question1)); // Adresi string'e ekle
    println!("addr is here: {}",  eth_address);
    zk_rust_io::write(&eth_address);
}


fn output() {
    let (_fval, decimal_value, is_winner): (String, u128, bool) = zk_rust_io::out();
    println!("dec val.: {}",  decimal_value);
    println!("Is winner: {}", is_winner);
}