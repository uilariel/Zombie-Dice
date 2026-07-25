
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
    pub fn new(nome: &str) -> Self
    {
        Self {
            nome: nome.to_string(),
            cerebros_comidos: 0,
            turnos_jogados: 0,
        }
    }


    //getter da quantidade de cerebros que o jogador comeu
    pub fn get_cerebros(&self) -> &i32
    {
        &self.cerebros_comidos
    }

    //getter da quantidade de turnos que o jogador jogou
    pub fn get_turnos(&self) -> &i32
    {
        &self.turnos_jogados
    }


    //adciona 1 turno ao turno do jogador
    pub fn add_turno(&mut self)
    {
        self.turnos_jogados+=1;
    }


    //adciona a quantidade inserida ao contador de cerebros comidos do jogador
    pub fn add_cerebro(&mut self, quantidade: i32)
    {
        self.cerebros_comidos += quantidade;
    }
    
    pub fn get_nome(&self) -> &String
    {
        &self.nome
    }


    
}