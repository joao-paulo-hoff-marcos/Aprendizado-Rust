// Leia dois valores inteiros. A seguir, calcule o produto entre estes dois valores e atribua esta operação à variável PROD. A seguir mostre a variável PROD com mensagem correspondente.
// Entrada

// O arquivo de entrada contém 2 valores inteiros.
// Saída

// Imprima a mensagem "PROD" e a variável PROD conforme exemplo abaixo, com um espaço em branco antes e depois da igualdade. Não esqueça de imprimir o fim de linha após o produto, caso contrário seu programa apresentará a mensagem: “Presentation Error”.

use std::io;

pub fn main() {
    // preparando a leitura dos valores pelo terminal
    let mut x1 = String::new();
    let mut x2 = String::new();
    io::stdin().read_line(&mut x1).expect("não foi");
    io::stdin().read_line(&mut x2).expect("não foi 2");
    // converter strings para os números
    let a: i32 = x1.trim().parse().expect("errou");
    let b: i32 = x2.trim().parse().expect("errou");
    // obter o resultado da multiplicação dos valores de entrada
    let c: i32 = a * b;
    // printar o resultado do produto
    println!("PROD = {}", c);
}