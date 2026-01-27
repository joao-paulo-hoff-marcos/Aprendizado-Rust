// Leia dois valores inteiros, no caso para variáveis A e B. A seguir, calcule a soma entre elas e atribua à variável SOMA. A seguir escrever o valor desta variável.
// Entrada

// O arquivo de entrada contém 2 valores inteiros.
// Saída

// Imprima a mensagem "SOMA" com todas as letras maiúsculas, com um espaço em branco antes e depois da igualdade seguido pelo valor correspondente à soma de A e B. Como todos os problemas, não esqueça de imprimir o fim de linha após o resultado, caso contrário, você receberá "Presentation Error".


pub fn main() {

    // criar uma string vazia para guardar o valor do terminal
    let mut x1 = String::new();
    // lendo o valor do terminal, x1
    std::io::stdin().read_line(&mut x1).expect("falha ao ler");
    // criar a segunda string vazia para guardar o valor do terminal
    let mut x2 = String::new();
    //lendo o segundo valor do terminal, x2
    std::io::stdin().read_line(&mut x2).expect("falha 2");
    // fazendo a string (x1) virar um número, x3
    let a: i32 = x1.trim().parse().expect("numero invalido");
    // fazendo a string (x2) virar um numero, x4
    let b: i32 = x2.trim().parse().expect("sem numero");
    // somar os números lidos da entrada
    let robeto = a+b;
    // imprimir o valor no terminal
    println!("SOMA = {}", robeto);
}