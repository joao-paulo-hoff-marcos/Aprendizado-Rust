// Leia 2 valores inteiros e armazene-os nas variáveis A e B.
// Efetue a soma de A e B atribuindo o seu resultado na variável X.
// Imprima X conforme exemplo apresentado abaixo.
// Não apresente mensagem alguma além daquilo que está sendo especificado e não esqueça de imprimir o fim de linha após o resultado, caso contrário, você receberá "Presentation Error".

// Entrada
// A entrada contém 2 valores inteiros.

// Saída
// Imprima a mensagem "X = " (letra X maiúscula) seguido pelo valor da variável X e pelo final de linha. Cuide para que tenha um espaço antes e depois do sinal de igualdade, conforme o exemplo abaixo.

pub fn main() {
    let mut x1 = String::new();
    std::io::stdin().read_line(&mut x1).expect("falha ao ler entrada");
    let mut x2 = String::new();
    std::io::stdin().read_line(&mut x2).expect("Falha 2");
    let x3: i32 = x1.trim().parse().expect("Número Inválido");
    let x4: i32 = x2.trim().parse().expect("sem numero");
    let x = x3 + x4;
    // println!("{x}");
    println!("X = {}", x);
}