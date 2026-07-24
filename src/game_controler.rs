//struct que vai guardar algumas variaveis temporarias que vao servir pra reger os turnos 

use rand::{distr::Iter, seq::SliceRandom};

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
        Estado::Welcome => Welcomefn(players, estado, input),
        Estado::PreparingMatch => PreparingMatchfn(players, estado, input),
    }
}

 fn Welcomefn(players: &mut Vec<Player>, estado: &mut Estado, input: &mut String)
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
        let mut rng = rand::rng();
         players.shuffle(&mut rng);
        *estado = Estado::Playing;
    }
    else {
        let x = jogadores.len();
        println!("voce inseriu {} jogadores, o permitido eh entre 2 e 6\nENCERRANDO JOGO", x);
        
        *estado = Estado::Quitting;
    }
}


fn PreparingMatchfn(players: &mut Vec<Player>, estado: &mut Estado, input: &mut String){


    println!("a ordem de jogadores sera:\n");
    for player in players.iter(){
        let jogador = player.GetNome();
        println!("{}\n", jogador);
    }




    println!("digite <enter> pra continuar\n");
    //vou por println so pra debugar mas depois vo tirar essa bosta
    match input.trim()
    {
        "" => *estado = Estado::Playing,
        _ => {}
    }
}



fn Playingfn(){}
fn Quittingfn(){}
fn Holdingfn(){}
fn Rollingfn(){}
fn Lostfn(){}
fn AddBrainsfn(){}
fn Drawfn(){}
fn Winfn(){}

