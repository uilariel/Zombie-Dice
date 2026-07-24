//struct que vai guardar algumas variaveis temporarias que vao servir pra reger os turnos 

use rand::distr::Iter;

use crate::{dice_bag::DiceBag, player::Player};
use std::io::stdin;
struct InfoTemp
{

}

enum Estado
{
    Welcome,
    PreparingMatch,
    Playing,
    Quitting,
    Holding,
    ROlling,
    Lost,
    AddBrais,
    TurnResult,
    Draw,
    Win,
}

pub fn Update(estado: &mut Estado, players: &mut Vec<Player>,dados: &mut DiceBag, input: &mut String, rodando: &mut bool, info_temp:&mut InfoTemp  )
{
    match estado
    {
        Estado::Welcome => Welcomefn(),
    }
}


pub fn Welcomefn(players: &mut Vec<Player>, estado: &mut Estado, input: &mut String)
{   
    //acho q vou ter q tirar isso depois
    println!("bem vindo ao zombie dice\n por favor, insira o nome dos jogadores");
        stdin()
        .read_line(input)
        .expect("falha ao ler input");


    /*
    SPLIT -> ESTAMOS DIVIDINDO O READ LINE PELA VIRGULA
    MAP -> ESTAMO ENTRANDO EM CADA DIVISAO E APLICANDO TRIM (REMOVE ESPACOS)
    COLLECT -> ESTAMOS JOGADANDO TUDO DENTRO DO VETOR JOGADORES
    QUANDO USAMOS SPLIT O TIPO VAI DE STRING PRA &STR.
     */
    let mut jogadores :Vec<&str> = input.split(',').map(|s| s.trim()).collect();


    if jogadores.len() > 2 && jogadores.len() < 7
    {
        for i in 0..jogadores.len()
        {   

        //definimos jogador como um novo player com o nome q ta armazenado na posicao i do vetor 
        //e jogamos pra dentro do vetor PLayers.
        let jogador = Player::new(jogadores[i]);
        players.push(jogador);

        }

        *estado = Estado::Playing;
    }
    else {
        //ESCREVA AQUI A MENSAGEM PRA INSERIR DE NOVO
    }
}

