use std::io::{self,Write};
use bs58;

fn base58_to_wallet(base58:&str)-> Vec<u8>{
    bs58::decode(base58).into_vec().unwrap()
}

fn wallet_to_base58(wallet:Vec<u8>)->String{
    bs58::encode(wallet).into_string()
}

fn main() {
    println!("Welcome to the Solana Wallet Converter CLI! 😃");

    loop {
        println!("\nChoose an option: 🤔");
        println!("1. Convert Base58 Secret Key to Wallet Format (u8 array)");
        println!("2. Convert Wallet Format (u8 array) to Base58 Secret Key");
        println!("3. Exit");

        println!("Enter your choice (Type the no. 1,2 or 3): 🙂");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1"=>{
                print!("Enter the Base58 string: ");
                io::stdout().flush().unwrap();
                let mut base58= String::new();
                io::stdin().read_line(&mut base58).unwrap();
                let wallet= base58_to_wallet(base58.trim());
                println!("Wallet in u8 array: {:?}",wallet);
                print!("---------------------------------------------------------------------------------------------------");
            }
            "2"=>{
                print!("Enter the Wallet (comma-seperated u8 array):  ");
                io::stdout().flush().unwrap();
                let mut wallet_input = String::new();
                io::stdin().read_line(&mut wallet_input).unwrap();
                
                let wallet: Vec<u8>= wallet_input.trim().split(',').map(|num| num.parse().unwrap()).collect();
                
                let base58= wallet_to_base58(wallet);
                println!("Base58 string: {}",base58);
                print!("---------------------------------------------------------------------------------------------------");
            }
            "3"=>{
                println!("Goodbye! 🥹");
                print!("---------------------------------------------------------------------------------------------------");
                break;
            }
            _=> println!("Invalid option. Please try again with the no.s 1,2 or 3"),
        }
    }    
}