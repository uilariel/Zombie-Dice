use core::panic;

use rand::{random_range};

/*
s -> shot/tiro
f -> foot/pegada
b -> brain/cerebro
*/
#[derive(PartialEq, Eq )]
pub enum Face {

    Cerebro,
    Tiro,
    Pegada,
    None,
}

enum Cor {
    Amarelo,
    Verde,
    Vermelho,
}

pub struct Zdice {
    cor: Cor,
    faces: [char; 6],
    face_que_caiu: Face,
}

impl Zdice
{   

    /*
    ESSA FUNCAO PRECISA FAZER O SEGUINTE:
    ->SORTEAR UM NUMERO DE 0 A 5
    ->DEFINIIR FACE COMO ESSE INDEX DO ARRAY
    ->FAZER O MATCH DA FACE E MUDAR O CAMPO face_que_caiu DEPENDENDO DO Q TIVER NO INDEX
    */
    pub fn rolar(&mut self)
    {   
        //random num between 0 and 5
        let rng = random_range(0..=5);
        let face: char = self.faces[rng];
        match face
        {
            's' => self.face_que_caiu = Face::Tiro,
            'b' => self.face_que_caiu = Face::Cerebro,
            'f' => self.face_que_caiu = Face::Pegada,
            _ => panic!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"),
        }

    }


    //cria um dado com faces diferentes dependendo da cor escolhida
    pub fn new(cor: &String) -> Self
    {   
        let yellow = ['b','b','f','f','s','s'];
        let green = ['b','b','b','f','f','s'];
        let red = ['b','f','f','s','s','s'];
        match cor.as_str()
        {
            "amarelo" => Self {
                cor: Cor::Amarelo,
                faces: yellow,
                face_que_caiu: Face::None,
            },

            "verde" => Self {
                cor: Cor::Verde,
                faces: green,
                face_que_caiu: Face::None,
            },

            "vermelho" => Self {
                cor: Cor::Vermelho,
                faces: red,
                face_que_caiu: Face::None,
            },
            _ => panic!("Cor invalida!"),
        }
    }

    //getter da face que caiu no dado
    pub fn get_fqc(&self) -> &Face
    {
        &self.face_que_caiu
    }

    //getter da cor do dado
    pub fn get_cor(&self) -> &Cor
    {
        &self.cor
    }
}
