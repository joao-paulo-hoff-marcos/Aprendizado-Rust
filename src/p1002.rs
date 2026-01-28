// A fórmula para calcular a área de uma circunferência é: area = π . raio2. Considerando para este problema que π = 3.14159:

// - Efetue o cálculo da área, elevando o valor de raio ao quadrado e multiplicando por π.
// Entrada

// A entrada contém um valor de ponto flutuante (dupla precisão), no caso, a variável raio.
// Saída

// Apresentar a mensagem "A=" seguido pelo valor da variável area, conforme exemplo abaixo, com 4 casas após o ponto decimal. Utilize variáveis de dupla precisão (double). Como todos os problemas, não esqueça de imprimir o fim de linha após o resultado, caso contrário, você receberá "Presentation Error".




pub fn main() {
    //preparando leitura para o terminal
    let mut x1 = String::new();
    std::io::stdin().read_line(&mut x1).expect("não foi");
    // converter a string para
    let a: f64 = x1.trim().parse().expect("não tem um número existente");
    let x: f64 = a * a * 3.14159;
    println!("A={:.4}", x);
}