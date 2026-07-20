//Faça um programa que leia três valores e apresente o maior dos três valores lidos seguido da mensagem “eh o maior”. Utilize a fórmula:

//Obs.: a fórmula apenas calcula o maior entre os dois primeiros (a e b). Um segundo passo, portanto é necessário para chegar no resultado esperado.
//Entrada

//O arquivo de entrada contém três valores inteiros.
//Saída

//Imprima o maior dos três valores seguido por um espaço e a mensagem "eh o maior".


pub fn main() {
    let mut x1 = String::new();
    std::io::stdin().read_line(&mut x1).expect("a função read line não passou do terminal a string");
    let a: Vec<&str> = x1.split_whitespace().collect();
    let numb1: i32 = a[0].parse().expect("não foi");
    let numb2: i32 = a[1].parse().expect("não foi2");
    let numb3: i32 = a[2].parse().expect("não foi3");

    let mut maior = numb1;
    if numb2 > maior {
        maior = numb2
    }
    if numb3 > maior {
        maior = numb3
    }
    println!("{} eh o maior", maior);
}