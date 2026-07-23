
mod dice_bag;
mod game_controler;
mod player;
mod zdice;
fn main() {
    let s = String::from("hello");
    let s = takes_ownership(s);
    println!("{}", s);
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string); //depois desse escopo ser finalizado, essa variavel eh
    //jogada no lixo!
    some_string
}
