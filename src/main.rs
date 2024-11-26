use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe um número! \n favor, insira um número");
    let secrect_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Ler a entrada de número falhou!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você chutou: {}", guess);

        match guess.cmp(&secrect_number) {
            Ordering::Less => println!("É mais!"),
            Ordering::Greater => println!("É menos!"),
            Ordering::Equal => {
                println!("Accertou! Miserávi!");
                break;
            }
        }
    }
}
