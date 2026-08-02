//struct que vai guardar algumas variaveis temporarias que vao servir pra reger os turnos 

use rand::{distr::Iter, seq::SliceRandom};

use crate::{dice_bag::DiceBag, player::Player, zdice::Face};
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

    pub fn add_cerebro(&mut self)
    {
        self.cerebros+=1;
    }

    pub fn add_tiro(&mut self)
    {
        self.tiros+=1;
    }

    pub fn get_cerebros_temp(&self) -> &i32
    {
        &self.cerebros
    }

    pub fn get_tiros_temp(&self) -> &i32
    {
        &self.tiros
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
        Estado::Welcome => welcome_fn(players, estado),
        Estado::PreparingMatch => preparing_match_fn(players, estado),
        Estado::Quitting => quitting_fn(rodando),
    } 
}

 fn welcome_fn(players: &mut Vec<Player>, estado: &mut Estado)
{   

    let mut input = String::new();

    stdin()
        .read_line(&mut input);

    //acho q vou ter q tirar isso depois
    println!("bem vindo ao zombie dice\n por favor, insira o nome dos jogadores");
        stdin()
        .read_line(&mut input)
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
        println!("jogadores registrados com sucesso!/nDigite <enter> para continuar\n")
    }
    else {
        let x = jogadores.len();
        println!("voce inseriu {} jogadores, o permitido eh entre 2 e 6\nENCERRANDO JOGO", x);
        
        *estado = Estado::Quitting;
    }

    let mut input2 = String::new();

    stdin()
    .read_line(&mut input2);

}


fn preparing_match_fn(players: &mut Vec<Player>, estado: &mut Estado){


    println!("a ordem de jogadores sera:\n");
    for player in players.iter(){
        let jogador = player.get_nome();
        println!("{}\n", jogador);
    }




    println!("digite <enter> pra continuar\n"); //pedir pro usuario digitar enter pra continuar 
    //vou por println so pra debugar mas depois vo tirar essa bosta
    let mut input = String::new();
    stdin()
    .read_line(&mut input);
    *estado = Estado::Playing;
     
    
}


//apenas gerencia a transicao do estado, nao tem muito mais logica alem disso
fn playing_fn(estado: &mut Estado){


    let mut input = String::new();
    stdin()
    .read_line(&mut input);
    //temos que exibir no render as opcoes que o usuario tem
    match input.trim().to_uppercase().as_str()
    {
        "Q" =>{ 
            *estado = Estado::Quitting;
            println!("voce selecionou Q!, saindo do jogo!\n")
        }
        "H" => {
            *estado = Estado::Holding;
            println!("voce selecionou H, holdando!\n");

        }
        "R" => {
            *estado = Estado::Rolling;
            println!("voce selecionou R, vamos rolar os dados!\n")
        }
         _ => {},
    }

   let mut input2 = String::new();
    stdin()
    .read_line(&mut input2);

    println!("digite <enter> para continuar>")
}


//encerra o programa
fn quitting_fn (rodando: &mut bool){
    

   *rodando = false;
   println!("obrigado por jogar o zombie dice!\ndigite <enter> para encerrar o programa!")
}


fn holding_fn(players: &mut Vec<Player>, estado: &mut Estado, info_temp:&mut InfoTemp){

    players[info_temp.turno_atual].add_cerebro(info_temp.cerebros);


    //chama a render aqui.

    //RESETA AS VARIAVEIS QUE FORAM FEITAS PRA SEREM RESETADAS
    info_temp.reset();

    
    println!("foi esperto! holdou e comeu {} cerebros\ndigite <enter> para continuar", players[info_temp.turno_atual].get_cerebros());
    *estado = Estado::TurnResult;
    
    let mut input = String::new();
    stdin()
        .read_line(&mut input);
  



}

/*====================================================================================
    OQUE A FUNCAO ROILLING PRECISA FAZER:
    1-> rolar os dados
    2-> processar o resultado dos dados
    3-> decidir o proximo estado da maquina
    [VAMOS PRECISAR DE VARIAVEIS LOCAIS, UMA PRA OS TIROS DESSA ROLAGEM 
    E OUTRA OS CEREBROS]
    
    =>etapa 1:


*/
fn rolling_fn(estado: &mut Estado, info_temp:&mut InfoTemp, dados: &mut DiceBag){
    
    //apos chamar essa funcao vamos ter 3 dados rolados nos ultimos 3 vetores da mesa
    dados.puxar();

    //ler os 3 ultimos dados do vetor da mesa

    for i in 0..3
    {   
        //ultimo dado do vetor da mesa
        let index = dados.get_quantidade_mesa() - 1;
        let face = dados.get_fqc_mesa(index - i);

        //adcionando aos contadores temporarios asa faces que os dados rolados puxados cairam
        match face 
        {
            Face::Cerebro => info_temp.add_cerebro(),
            Face::Tiro => info_temp.add_tiro(),
            _ => {},
        }
    }
        //contando o total acumulado pra ver pra qual estado mudar
        let  total_brains =  info_temp.get_cerebros_temp();
        let  total_shots = info_temp.get_tiros_temp();


    
        //mandando o jogador pro canto
        if *total_shots >= 3
        {
        println!("infelizmente voce foi abatido!\ntotal de cerebros que voce poderia ter comido: {}\ntotal de tiros recebidos: {}\nDigite<enter> para continuar",total_brains, total_shots);
        let mut input = String::new();
        stdin()
            .read_line(&mut input);
        
            if input.trim() == ""{
            *estado = Estado::Lost;
            }
        }
        else
        {   
            println!("boa campanha!\n cerebros comidos ate agora: {}\ntiros totais recebidos: {}\nDigite<enter> para continuar", total_brains, total_shots);
            let mut input = String::new();
            stdin()
            .read_line(&mut input);
        
            if input.trim() == ""
            {
            *estado = Estado::Playing;
            }
        }
}

fn lostfn(){

}
fn draw_fn(){}
fn win_fn(){}

