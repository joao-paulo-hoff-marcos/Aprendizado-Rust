// Leia 2 valores de ponto flutuante de dupla precisão A e B, que correspondem a 2 notas de um aluno. A seguir, calcule a média do aluno, sabendo que a nota A tem peso 3.5 e a nota B tem peso 7.5 (A soma dos pesos portanto é 11). Assuma que cada nota pode ir de 0 até 10.0, sempre com uma casa decimal.
// Entrada

// O arquivo de entrada contém 2 valores com uma casa decimal cada um.
// Saída

// Imprima a mensagem "MEDIA" e a média do aluno conforme exemplo abaixo, com 5 dígitos após o ponto decimal e com um espaço em branco antes e depois da igualdade. Utilize variáveis de dupla precisão (double) e como todos os problemas, não esqueça de imprimir o fim de linha após o resultado, caso contrário, você receberá "Presentation Error".


fn get_num() -> f64 {
    use std::io;
    let mut x1 = String::new();
    io::stdin().read_line(&mut x1).expect("não foi");
    let a: f64 = x1.trim().parse().expect("não foi 2");
    return a;
}


pub fn main() {
    let p1 = 3.5;
    let p2 = 7.5;
    let den = p1 + p2;
    // preparando leitura dos valores para o terminal.
    let a = get_num();
    let b = get_num();
    //converter as String para os numeros decimais
    //obter o resultado da soma e divisão dos valores de entrada
    let c: f64 = (a*p1 + b*p2) / den;
    // printar o resultado da soma e divisão
    println!("MEDIA = {:.5}", c);
}