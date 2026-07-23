use rand::{Rng, random, random_range,prelude::SliceRandom};
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
    DadosMesa: Vec<Zdice>,
    DadosSaco: Vec<Zdice>,
}

impl DiceBag
{
    pub fn new() -> Self 
    {   
        let mut vetor = Vec::new();
        let amarelo = String::from("amarelo");
        let verde = String::from("verde");
        let vermelho = String::from("vermelho");

        for i in 1 ..=6 {

            vetor.push(Zdice::new(&verde));
        }

        for i in 1..=4{
            vetor.push(Zdice::new(&amarelo));
        }

        for i in 1..3{
            vetor.push(Zdice::new(&vermelho));
        }

        Self {
            DadosMesa: Vec::new(),
            DadosSaco: vetor,
        }    
    }   

    //retorna a quantidade de dados que tem no saco
    pub fn getQuantidadeSaco(&self) -> usize
    {
        self.DadosSaco.len()
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
        self.devolverPegada();
        if self.DadosSaco.len() < 3{
            self.devolverCerebro();
        }

        //embaralha o vetor dos dados
        let mut rng = rand::rng();
        self.DadosSaco.shuffle(&mut rng);

        for _ in 1..=3{
            let dado = self.DadosSaco.pop().unwrap();
            self.DadosMesa.push(dado);
        }

    }

     fn devolverPegada(&mut self)
    {   
        let mut tracker = 0;
        let tamanho_mesa =  self.DadosMesa.len();

        //esse loop for move os dados de pegada pro final do vetor da mesa
        //a variavel tracker conta quantos dados foram movidos 
        for i in 0..self.DadosMesa.len(){
            if(self.DadosMesa[i].getFQC() == &Face::Pegada)
            {
                self.DadosMesa.swap(i, tamanho_mesa - i);
                tracker+=1;
            }
        }

        //remove o ultimo elemento do vetor de dados da mesa e da o valor dele ao dado
        //joga esse dado dentro do saco
        for _ in 0..tracker{
            let mut dado = self.DadosMesa.pop().unwrap();
            self.DadosSaco.push(dado);
        }

    }

     fn devolverCerebro(&mut self)
    {
        let mut tracker = 0;
        let tamanho_mesa = self.DadosMesa.len();

        //esse loop vai mover os dados de cerebro pro final do vetor da mesa
        //a variavel tracker conta quantos dados foram movidos 
        for i in 0..self.DadosMesa.len(){
            if(self.DadosMesa[i].getFQC() == &Face::Pegada)
            {
                self.DadosMesa.swap(i, tamanho_mesa - i);
                tracker+=1;
            }
        }

        //remove o ultimo elemento do vetor de dados da mesa e da o valor dele ao dado
        //joga esse dado dentro do saco
        for i in 0..tracker{
            let dado = self.DadosMesa.pop().unwrap();
            self.DadosSaco.push(dado);
        }

    }
    //devolve os dados que estao na mesa de volta pro saco
    pub fn devolverDados(&mut self)
    {
        let tamanho_mesa = self.DadosMesa.len();
        
        for i in 0..tamanho_mesa{
            let dado = self.DadosMesa.pop().unwrap();
            self.DadosSaco.push(dado);
        }
    }

    

}