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
    Rolling,
    Lost,
    AddBrais,
    TurnResult,
    Draw,
    Win,
}

pub fn Update(estado: &mut Estado, players: &mut Vec<Player>,dados: &mut DiceBag, input: &mut String, info_temp:&mut InfoTemp, rodando: &mut bool  )
{
    match estado
    {
        Estado::Welcome => welcome_fn(players, estado, input),
        Estado::PreparingMatch => preparing_match_fn(players, estado, input),
        Estado::Quitting => quitting_fn(estado, rodando),
    } 
}

 fn welcome_fn(players: &mut Vec<Player>, estado: &mut Estado, input: &mut String)
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
    let jogadores :Vec<&str> = input.split(',').map(|s| s.trim()).collect();


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


fn preparing_match_fn(players: &mut Vec<Player>, estado: &mut Estado, input: &mut String){


    println!("a ordem de jogadores sera:\n");
    for player in players.iter(){
        let jogador = player.get_nome();
        println!("{}\n", jogador);
    }




    println!("digite <enter> pra continuar\n"); //pedir pro usuario digitar enter pra continuar 
    //vou por println so pra debugar mas depois vo tirar essa bosta
    match input.trim()
    {
        "" => *estado = Estado::Playing,
        _ => {}
    }
}


//apenas gerencia a transicao do estado, nao tem muito mais logica alem disso
fn playing_fn(estado: &mut Estado, input: &mut String){

    //temos que exibir no render as opcoes que o usuario tem
    match input.trim().to_uppercase().as_str()
    {
        "Q" => *estado = Estado::Quitting,
        "H" => *estado = Estado::Holding,
        "R" => *estado = Estado::Rolling,
         _ => {},
    }
}


//encerra o programa
fn quitting_fn(estado: &mut Estado, rodando: &mut bool, input: &mut String){
    

    if input.trim() == ""{
    *rodando = false;
    }
    else {
        println!("era pra apertar enter mas vo encerrar igual otario");
        *rodando = false;
    }
    //exibe algo tipo obrigado por jogar o zombie dice e pede pro jogador apertar enter pra encerrar
}


fn holding_fn(){}
fn rolling_fn(){}
fn lostfn(){}
fn add_brains_fn(){}
fn draw_fn(){}
fn win_fn(){}

