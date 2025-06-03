use std::io::{self, Write};

use crate::bank::save::SaveInfo;
use crate::bank::save;

pub struct Account;

impl Account {
    pub fn create() {
        let mut nome: String = String::new();
        let mut input_cpf: String = String::new();
        let mut input_data_nascimento_dia: String = String::new();
        let mut input_data_nascimento_mes: String = String::new();
        let mut input_data_nascimento_ano: String = String::new();
        println!("=== CRIAR CONTA ===");
        print!("Digite seu nome: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut nome).expect("Erro ao ler linha.");
        print!("Digite sua data de nascimento (DIA): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_data_nascimento_dia).expect("Erro ao ler linha.");
        print!("Digite sua data de nascimento (MES): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_data_nascimento_mes).expect("Erro ao ler linha.");
        print!("Digite sua data de nascimento (ANO): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_data_nascimento_ano).expect("Erro ao ler linha.");
        print!("Digite seu CPF (FAKE): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_cpf).expect("Erro ao ler linha.");
        
    
        let nome = nome.trim();
        let cpf: u64 = match input_cpf.trim().parse() {
            Ok(p) => p,
            Err(e) => {
                println!("Erro ao converter CPF: {}", e);
                return;
            },
        };

        let data_nascimento_dia: usize = match input_data_nascimento_dia.trim().parse() {
            Ok(p) => p,
            Err(e) => {
                println!("Erro ao converter a data de nascimento (DIA): {}", e);
                return;
            },
        };

        let data_nascimento_mes: usize = match input_data_nascimento_mes.trim().parse() {
            Ok(p) => p,
            Err(e) => {
                println!("Erro ao converter a data de nascimento (MES): {}", e);
                return;
            },
        };
        
        let data_nascimento_ano: usize = match input_data_nascimento_ano.trim().parse() {
            Ok(p) => p,
            Err(e) => {
                println!("Erro ao converter a data de nascimento (ANO): {}", e);
                return;
            },
        };

    let json = save::SaveInfo {
        nome: nome.to_string(),
        cpf: cpf,
    };

    SaveInfo::save(json);

    }
    pub fn login() {

    }
}
