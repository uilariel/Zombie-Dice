
mod dice_bag;
mod game_controler;
mod player;
mod zdice;
use crate::{dice_bag::DiceBag, game_controler::{Estado, InfoTemp, Update}, player::Player};
use std::io::stdin;

fn main() {
 
    let mut info_temp = InfoTemp::new(); 
    let mut estado = Estado::Welcome;
    let mut players: Vec<Player> = Vec::new();
    let mut rodando = true;
    let mut dados = DiceBag::new();

    while rodando
    {
        Update(&mut estado, &mut players, &mut dados, &mut info_temp, &mut rodando);
    }
}

