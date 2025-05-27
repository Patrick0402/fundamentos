fn criar_nome() -> String {
    String::from("Lucas")
}

fn gerar_novissimo_nome(nome: &str) -> String {
    let mut novissimo_nome = nome.to_string();
    novissimo_nome.push_str(" Souza");
    novissimo_nome
}
fn main() {
    // Criação da variável nome
    let nome = criar_nome();
    println!("Meu nome é {}", nome);

    // Criação da variável MUTÁVEL novo_nome que passa a ser dona do valor de nome
    let mut novo_nome = nome;
    println!("Meu novo nome é {}", novo_nome);
    // A variável nome não pode mais ser usada, pois o valor foi movido para novo_nome

    // Modificação da variável novo_nome
    novo_nome = gerar_novissimo_nome(&novo_nome);
    // Borrowing da variável novo_nome dentro da função
    println!("Meu novíssimo nome é {}", novo_nome);

    // Criação da variável novo_nome usando shadowing
    let novo_nome = String::from("Lucas Shadow");
    println!("Meu novo nome sombrio é {}", novo_nome);

    // Trabalhando com arrays
    let numeros_original: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Números: {:?}", numeros_original);

    // Clonando o array
    let mut numeros_clone = numeros_original;
    /*
      Em Rust, arrays de tamanho fixo (até 32 elementos) cujos elementos implementam o trait Copy
      também implementam Copy. Nesse caso, atribuir o array a uma nova variável faz uma cópia automática.

      Portanto, não é necessário usar o método clone() para copiar arrays simples como [i32; N].

      No entanto, se o array contiver tipos que não implementam Copy (como String), a atribuição moverá os dados,
      e será necessário usar o método clone() para criar uma cópia real.

    */
    println!("Números clonados: {:?}", numeros_clone);

    // Modificando o array clonado
    numeros_clone[0] = 10;
    println!("Números clonados após modificação: {:?}", numeros_clone);

    // O valor do array original permanece inalterado pois trabalhamos com uma cópia
    println!(
        "Números originais ainda são acessíveis: {:?}",
        numeros_original
    );

    /*
    Isso também se aplica a variáveis de tamanho fixo, como i32, que implementam Copy.
    */

    let x: i32 = 5;
    let y: i32 = x; // Aqui, x é copiado para y, pois i32 implementa Copy

    println!("Valor de x: {}, Valor de y: {}", x, y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nome() {
        let nome = criar_nome();
        assert_eq!(nome, "Lucas");
    }

    #[test]
    fn test_novo_nome() {
        let nome = criar_nome();
        let novo_nome = nome.clone();
        assert_eq!(novo_nome, "Lucas");
    }

    #[test]
    fn test_novissimo_nome() {
        let nome = criar_nome();
        let novissimo_nome = gerar_novissimo_nome(&nome);
        assert_eq!(novissimo_nome, "Lucas Souza");
    }
}
