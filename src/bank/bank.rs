use std::io::{self, Write};

use crate::bank::account::Account;

pub struct App;

impl App {
    pub fn init(){
        loop{
            let mut input: String = String::new();
            println!("=== BANCO RUST ==="); 
            println!("1 - Criar conta.");
            println!("2 - Entrar na sua conta.");
            println!("0 - Sair do app.");
            print!("Sua escolha: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).expect("Erro ao ler linha.");
            
            let input = input.trim();

            match input {
                "1" => Account::create(),
                "0" => break,
                _ => println!("erro"),
            }
            
        }
    }
}
