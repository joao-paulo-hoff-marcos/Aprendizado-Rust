

//Leia quatro valores inteiros A, B, C e D. A seguir, calcule e mostre a diferença do produto de A e B pelo produto de C e D segundo a fórmula: DIFERENCA = (A * B - C * D).
//Entrada

//O arquivo de entrada contém 4 valores inteiros.
//Saída

//Imprima a mensagem DIFERENCA com todas as letras maiúsculas, conforme exemplo abaixo, com um espaço em branco antes e depois da igualdade.


fn get_num() -> i32 {
    let mut x = String::new();
    std::io::stdin().read_line(&mut x).expect("não conseguiu ler a entrada");
    let a: i32 = x.trim().parse().expect("o numero não foi feito");
    return a;
}


pub fn main() {
    let a = get_num();
    let b = get_num();
    let c = get_num();
    let d = get_num();

    let j: i32 = a * b - c * d;
    println!("DIFERENCA = {}", j);
}