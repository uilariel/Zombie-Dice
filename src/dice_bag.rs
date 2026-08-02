use rand::{prelude::SliceRandom};
/* ENTIDADE RESPONSAVEL POR GUARDAR OS DADOS 

METODOS QUE ELE DEVE TER:
->construtor que inicializa 13 dados no saco FEITO
-> informar quantos dados tem no saco FEITO
-> puxar 3 dados NAO
-> devolver todos os dados
-> devolver apenas os dados de pegada FEITO 
-> devolver apenas os de cerebro FEITO
*/

use crate::zdice::{Face, Zdice};
pub struct DiceBag
{
    dados_mesa: Vec<Zdice>,
    dados_saco: Vec<Zdice>,
}

impl DiceBag
{
    pub fn new() -> Self 
    {   
        let mut vetor = Vec::new();
        let amarelo = String::from("amarelo");
        let verde = String::from("verde");
        let vermelho = String::from("vermelho");

        for _ in 1 ..=6 {

            vetor.push(Zdice::new(&verde));
        }

        for _ in 1..=4{
            vetor.push(Zdice::new(&amarelo));
        }

        for _ in 1..3{
            vetor.push(Zdice::new(&vermelho));
        }

        Self {
            dados_mesa: Vec::new(),
            dados_saco: vetor,
        }    
    }   

    //retorna a quantidade de dados que tem no saco
    pub fn get_quantidade_saco(&self) -> usize
    {
        self.dados_saco.len()
    }

    /*   OBJETIVO DESSA FUNCAO -> tirar 3 dados do vetor do saco e por no vetor da mesa.

     */

    /*
        ETAPAS DA FUNCAO
        1-> DEVOLVER OS DADOS DE PEGADA SEMPRE
        2->CHECAR SE TEMOS PELO MENOS 3 NO SACO
        3->DEVOLVER OS DADOS DE CEREBRO SE NECESSARIO
        4->EMBARALHAR O ARRAY
        5->MOVER OS ULTIMOS 3 DO VETOR DO SACO PRA MESA
     */
    pub fn puxar(&mut self) 
    {   

        //devolve os dados que forem necessarios devolver
        self.devolver_pegada();
        if self.dados_saco.len() < 3{
            self.devolver_cerebro();
        }

        //embaralha o vetor dos dados
        let mut rng = rand::rng();
        self.dados_saco.shuffle(&mut rng);

        for _ in 1..=3{
            let dado = self.dados_saco.pop().unwrap();
            self.dados_mesa.push(dado);
        }

    }

     fn devolver_pegada(&mut self)
    {   
        let mut tracker = 0;
        let tamanho_mesa =  self.dados_mesa.len();

        //esse loop for move os dados de pegada pro final do vetor da mesa
        //a variavel tracker conta quantos dados foram movidos 
        for i in 0..self.dados_mesa.len(){
            if self.dados_mesa[i].get_fqc() == &Face::Pegada
            {
                self.dados_mesa.swap(i, tamanho_mesa - i);
                tracker+=1;
            }
        }

        //remove o ultimo elemento do vetor de dados da mesa e da o valor dele ao dado
        //joga esse dado dentro do saco
        for _ in 0..tracker{
            let dado = self.dados_mesa.pop().unwrap();
            self.dados_saco.push(dado);
        }

    }

     fn devolver_cerebro(&mut self)
    {
        let mut tracker = 0;
        let tamanho_mesa = self.dados_mesa.len();

        //esse loop vai mover os dados de cerebro pro final do vetor da mesa
        //a variavel tracker conta quantos dados foram movidos 
        for i in 0..self.dados_mesa.len(){
            if self.dados_mesa[i].get_fqc() == &Face::Pegada
            {
                self.dados_mesa.swap(i, tamanho_mesa - i);
                tracker+=1;
            }
        }

        //remove o ultimo elemento do vetor de dados da mesa e da o valor dele ao dado
        //joga esse dado dentro do saco
        for _ in 0..tracker{
            let dado = self.dados_mesa.pop().unwrap();
            self.dados_saco.push(dado);
        }

    }
    //devolve os dados que estao na mesa de volta pro saco
    pub fn devolver_dados(&mut self)
    {
        let tamanho_mesa = self.dados_mesa.len();
        
        for _ in 0..tamanho_mesa{
            let dado = self.dados_mesa.pop().unwrap();
            self.dados_saco.push(dado);
        }
    }


    pub fn get_quantidade_mesa(&self) -> usize
    {
        self.dados_mesa.len()
    }

     pub fn get_fqc_mesa(&self, index: usize) -> &Face
    {
        self.dados_mesa[index].get_fqc()
    }
    

}