
mod dice_bag;
mod game_controler;
mod player;
mod zdice;

use std::io::stdin;

fn main() {
    let mut input = String::new();
    println!("bem vindo ao zombie dice\n por favor, insira o nome dos jogadores");
        stdin()
        .read_line(&mut input)
        .expect("falha ao ler input");

    let mut jogadores :Vec<String> = input.split(',').map(|s| s.to_string()).collect();

    for i in 0..jogadores.len()
    {
       jogadores[i] = jogadores[i].trim().to_string();
    }
    

    for i in 0..jogadores.len()
    {
        println!("nome:{}", jogadores[i]);
    }
}

