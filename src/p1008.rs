//Escreva um programa que leia o número de um funcionário, seu número de horas trabalhadas, o valor que recebe por hora e calcula o salário desse funcionário. A seguir, mostre o número e o salário do funcionário, com duas casas decimais.

//Entrada

//O arquivo de entrada contém 2 números inteiros e 1 número com duas casas decimais, representando o número, quantidade de horas trabalhadas e o valor que o funcionário recebe por hora trabalhada, respectivamente.

//Saída

//Imprima o número e o salário do funcionário, conforme exemplo fornecido, com um espaço em branco antes e depois da igualdade. No caso do salário, também deve haver um espaço em branco após o $.


// criar a funçao para chamar o numero
fn get_num() -> i32 {
    let mut x1 = String::new();
    std::io::stdin().read_line(&mut x1).expect("a read_line não passou as infomaçoes do terminal para a string");
    let a: i32 = x1.trim().parse().expect("a string não passou a ser um numero");
    return a;
}

fn get_num2() -> f32 {
    let mut x2 = String::new();
    std::io::stdin().read_line(&mut x2).expect("a read_line não passou as infomaçoes do terminal para a string");
    let c: f32 = x2.trim().parse().expect("a string não passou a ser um numero");
    return c;
}



pub fn main() {
    let a = get_num();
    let b = get_num2();
    let c: f32 = get_num2();

    let x3: f32 = b * c;

    println!("NUMBER = {}", a);
    println!("SALARY = U$ {:.2}", x3);
}