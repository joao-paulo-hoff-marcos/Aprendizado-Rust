

//Escreva um programa que leia três valores com ponto flutuante de dupla precisão: A, B e C. Em seguida, calcule e mostre:
//a) a área do triângulo retângulo que tem A por base e C por altura.
//b) a área do círculo de raio C. (pi = 3.14159)
//c) a área do trapézio que tem A e B por bases e C por altura.
//d) a área do quadrado que tem lado B.
//e) a área do retângulo que tem lados A e B.
//Entrada

//O arquivo de entrada contém três valores com um dígito após o ponto decimal.
//Saída

//O arquivo de saída deverá conter 5 linhas de dados. Cada linha corresponde a uma das áreas descritas acima, sempre com mensagem correspondente e um espaço entre os dois pontos e o valor. O valor calculado deve ser apresentado com 3 dígitos após o ponto decimal.



pub fn main() {
    let mut x1 = String::new();
    std::io::stdin().read_line(&mut x1).expect("a função read line não passou do terminal a string");
    let a: Vec<&str> = x1.split_whitespace().collect();
    let numb1: f64 = a[0].parse().expect("o codigo não virou uma variavel");
    let numb2: f64 = a[1].parse().expect("o codigo não virou uma variavel2");
    let numb3: f64 = a[2].parse().expect("o codigo não virou uma variavel3");

    let tr1: f64 = numb1 * numb3 / 2.0;
    let bol1: f64 = numb3.powf(2.0)*3.14159;
    let tr2: f64 = (numb1 + numb2) * numb3 / 2.0;
    let qua1: f64 = numb2*numb2;
    let ret1: f64 = numb1*numb2;


    println!("TRIANGULO: {:.3}", tr1);
    println!("CIRCULO: {:.3}", bol1);
    println!("TRAPEZIO: {:.3}", tr2);
    println!("QUADRADO: {:.3}", qua1);
    println!("RETANGULO: {:.3}", ret1);
}