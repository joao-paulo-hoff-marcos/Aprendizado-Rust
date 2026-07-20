

//Calcule o consumo médio de um automóvel sendo fornecidos a distância total percorrida (em Km) e o total de combustível gasto (em litros).

//Entrada

//O arquivo de entrada contém dois valores: um valor inteiro X representando a distância total percorrida (em Km), e um valor real Y representando o total de combustível gasto, com um dígito após o ponto decimal.

//Saída

//Apresente o valor que representa o consumo médio do automóvel com 3 casas após a vírgula, seguido da mensagem "km/l".


pub fn main() {
let mut a =String::new();
std::io::stdin().read_line(&mut a).expect("num foi");
let mut b = String::new();
std::io::stdin().read_line(&mut b).expect("num foi 2");
let x: f64 = a.trim().parse().expect("nah, foi nao");
let y: f64 = b.trim().parse().expect("nah, foi nao 2");

let bah = x / y;

println!("{:.3} km/l", bah);
}