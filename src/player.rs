
// a struct player vai guardar:
//1-> a quantidade de cerebros que o jogador comeu
//2-> a quantidade de turnos que o jogador ja jogou
//3-> seu nome
pub struct Player 
{
    nome: String,
    cerebros_comidos: i32,
    turnos_jogados:i32,
}
/*
    QUAIS FUNCOES PRECISAMOS?
    ->construtor (new)
    ->retornar cerebros comidos
    ->retornar turnos jogados 
    ->adcionar turno
    ->adcionar cerebros 
*/
impl Player
{
    pub fn new(nome: String) -> Self
    {
        Self {
            nome,
            cerebros_comidos: 0,
            turnos_jogados: 0,
        }
    }


    //getter da quantidade de cerebros que o jogador comeu
    pub fn GetCerebros(&self) -> &i32
    {
        &self.cerebros_comidos
    }

    //getter da quantidade de turnos que o jogador jogou
    pub fn GetTurnos(&self) -> &i32
    {
        &self.turnos_jogados
    }


    //adciona 1 turno ao turno do jogador
    pub fn AddTurno(&mut self)
    {
        self.turnos_jogados+=1;
    }


    //adciona a quantidade inserida ao contador de cerebros comidos do jogador
    pub fn AddCerebro(&mut self, quantidade: i32)
    {
        self.cerebros_comidos += quantidade;
    }


    
}