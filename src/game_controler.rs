//struct que vai guardar algumas variaveis temporarias que vao servir pra reger os turnos 

use rand::{distr::Iter, seq::SliceRandom};

use crate::{dice_bag::DiceBag, player::Player};
use core::num;
use std::io::stdin;



/*
    A STRUCT INFO TEMPO VAI ARMAZENAR INFORMACOES QUE VAO SER USADAS PRA REGER OS TURNOS MELHOR.
    cerebros -> acumula os cerebros que o jogador for comendo ate a hora que ele decidir contabilizar todos ou morrer
    tiros -> acumula a quantidade de tiros que o jogador tomou na sua rodada
    turnos_completos -> vai ser incrementado sempre que todos os jogadores jogarem a mesma quantidade de vezes.
    turno_atual -> vai servir para reger o vetor circular de 0 ate n-1, com n sendo o numero de jogadores
*/
struct InfoTemp
{
    cerebros: i32,
    tiros: i32,
    turnos_completos: i32,
    turno_atual: usize,
}

impl InfoTemp
{
    pub fn new() -> Self 
    {
        Self { cerebros: 0, tiros: 0, turnos_completos: 0, turno_atual: 0 }
    
    }

    pub fn reset(&mut self)
    {
        self.cerebros = 0;
        self.tiros = 0;
    }

    pub fn prox_turno(&mut self, num_players: usize)
    {
        self.turno_atual = (self.turno_atual + 1)  % num_players;
    
    }

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
        Estado::Quitting => quitting_fn(rodando, input),
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
fn quitting_fn (rodando: &mut bool, input: &mut String){
    

    if input.trim() == ""{
    *rodando = false;
    }
    else {
        println!("era pra apertar enter mas vo encerrar igual otario");
        *rodando = false;
    }
    //exibe algo tipo obrigado por jogar o zombie dice e pede pro jogador apertar enter pra encerrar
}


fn holding_fn(players: &mut Vec<Player>, estado: &mut Estado, info_temp:&mut InfoTemp, input: &mut String){

    players[info_temp.turno_atual].add_cerebro(info_temp.cerebros);


    //chama a render aqui.

    //RESETA AS VARIAVEIS QUE FORAM FEITAS PRA SEREM RESETADAS
    info_temp.reset();

    if input.trim() == ""
    {
        *estado = Estado::TurnResult;
    }
    else {

        //nao sei como nao fazer ele quebrar caso digitem o negocio errado e quero pensar nisso depois que a logica tiver funcionando.
        *estado = Estado::TurnResult
    }

}


fn rolling_fn(){}
fn lostfn(){}
fn add_brains_fn(){}
fn draw_fn(){}
fn win_fn(){}

