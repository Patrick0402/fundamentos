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
