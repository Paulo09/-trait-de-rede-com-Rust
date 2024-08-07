
                                                                   Trait de Rede - Rust
                                                                       

Crie um novo arquivo chamado "it.rs" e adicione a seguinte estrutura:
pub struct Router {
pub ip: String,
}
 
pub trait Network {
fn ping(&self, host: &str) -> bool;
fn traceroute(&self, host: &str) -> Vec<String>;
fn nslookup(&self, host: &str) -> String;
}
Adicione uma implementação da trait Network para a struct Router.
O método ping deve imprimir uma mensagem simulando uma verificação de ping e retornar "true".
O método traceroute deve imprimir uma mensagem simulando um trace de rota e retornar um vetor de strings com os endereços IP.
O método nslookup deve imprimir uma mensagem simulando uma consulta de DNS e retornar um endereço IP em forma de string.
Crie um novo arquivo chamado "main.rs" e importe o módulo "it" criado anteriormente.
Utilize os métodos da trait Network para realizar operações básicas de rede.
Execute o código e verifique se os resultados estão de acordo com o esperado.
Dicas: Lembre-se de importar o módulo "it" no arquivo main.rs para poder utilizar os métodos da trait Network. Também é importante que os alunos saibam como funcionam os comandos ping, traceroute e nslookup antes de iniciarem essa tarefa.
Perguntas dessa tarefa
Crie o arquivo it.rs


Dica métodos:
*Método ping: O método ping é usado para verificar se um host está online ou não, ele envia um pacote de dados para o endereço IP especificado e espera uma resposta. Se o host responder, ele está online, caso contrário, ele está offline. No exemplo dado, o método ping simplesmente imprime uma mensagem indicando que está verificando a conexão com o host especificado e retorna true, mas na prática, você pode colocar lógica para enviar o pacote de dados e verificar a resposta real.
*Método traceroute: O método traceroute é usado para verificar o caminho que os pacotes de dados estão percorrendo até chegar ao host especificado. Ele envia pacotes de dados com um valor de TTL (Time to Live) incrementando a cada tentativa, registrando o endereço IP de cada dispositivo que os pacotes passam antes de expirar. No exemplo dado, o método traceroute simplesmente imprime uma mensagem indicando que está executando o trace de rota e retorna um vetor com alguns endereços IP fictícios, mas na prática, você pode colocar lógica para enviar os pacotes de dados e verificar os endereços IP de cada dispositivo que os pacotes passam.
*Método nslookup: O método nslookup é usado para realizar uma consulta de DNS (Domain Name System) para obter o endereço IP associado ao nome de host especificado. Ele envia uma consulta para o servidor de DNS configurado no sistema, que retorna o endereço IP correspondente. No exemplo dado, o método nslookup simplesmente imprime uma mensagem indicando que está executando uma consulta de DNS e retorna um endereço IP fictício, mas na prática, você pode colocar lógica para enviar a consulta de DNS e obter o endereço IP real.
crie uma tarefa, com descrição e instruções , para que os alunos desenvolvam o código acima.

Crie o arquivo main.rs


Mostrar rascunhos


Criando o Arquivo it.rs
Rust
pub struct Router {
    pub ip: String,
}

pub trait Network {
    fn ping(&self, host: &str) -> bool {
        println!("Pinging host: {}", host);
        true // Simulando sucesso do ping
    }

    fn traceroute(&self, host: &str) -> Vec<String> {
        println!("Tracing route to: {}", host);
        vec!["192.168.1.1".to_string(), "10.0.0.1".to_string()] // Simulando resultados do traceroute
    }

    fn nslookup(&self, host: &str) -> String {
        println!("Looking up DNS for: {}", host);
        "8.8.8.8".to_string() // Simulando resultado do nslookup
    }
}

impl Network for Router {
    // Implementações dos métodos já estão definidas na trait
}
Use o código com cuidado.

Criando o Arquivo main.rs
Rust
mod it;

fn main() {
    let router = it::Router { ip: "192.168.1.100".to_string() };

    router.ping("google.com");
    let trace = router.traceroute("google.com");
    println!("Trace route: {:?}", trace);
    let ip = router.nslookup("google.com");
    println!("DNS lookup: {}", ip);
}
Use o código com cuidado.

Tarefa para os Alunos
Descrição:
Nesta tarefa, você irá criar uma simulação básica de um roteador e suas funcionalidades de rede. Você irá implementar as operações de ping, traceroute e nslookup utilizando a linguagem Rust.

Instruções:

Crie um novo projeto Rust: Utilize o comando cargo new network_sim para criar um novo projeto.
Crie os arquivos: Crie os arquivos it.rs e main.rs dentro da pasta do seu projeto.
Implemente a estrutura Router: Copie o código do arquivo it.rs fornecido e cole no seu arquivo it.rs.
Implemente a função main: Copie o código do arquivo main.rs fornecido e cole no seu arquivo main.rs.
Execute o código: Utilize o comando cargo run para executar o programa. Você deve ver as mensagens de log simulando as operações de ping, traceroute e nslookup.
Personalize o código: Experimente modificar os valores de retorno dos métodos para simular diferentes resultados. Você pode adicionar mais funcionalidades, como verificar a conectividade com diferentes protocolos (HTTP, FTP, etc.) ou implementar um sistema mais sofisticado de simulação de rede.
Objetivos da Tarefa:

Compreender a estrutura de um programa Rust: Aprender a organizar o código em módulos e como utilizar traits.
Simular operações de rede: Entender o funcionamento básico das operações de ping, traceroute e nslookup.
Utilizar estruturas de dados: Usar a estrutura Vec para armazenar os resultados do traceroute.
Manipular strings: Trabalhar com strings para formatar as mensagens de saída.
Dicas Adicionais:

Bibliotecas externas: Para uma simulação mais realista, você pode utilizar bibliotecas externas que permitem realizar operações de rede de forma mais eficiente e segura.
Testes: Crie testes unitários para garantir a corretude do seu código.
Documentação: Adicione comentários ao seu código para explicar o funcionamento de cada parte.
Possíveis Extensões:

Simular falhas de rede: Introduza falhas aleatórias nas operações de ping, traceroute e nslookup para testar a robustez do seu código.
Implementar um protocolo de rede simples: Crie um protocolo de comunicação simples entre dois ou mais roteadores simulados.
Visualizar a topologia da rede: Utilize uma biblioteca de gráficos para visualizar a topologia da rede simulada.
Observações:

Este código fornece uma simulação básica e simplificada das operações de rede. Na prática, essas operações são mais complexas e envolvem protocolos de comunicação mais sofisticados.
O objetivo desta tarefa é introduzir os conceitos básicos de programação em Rust e simulação de redes.
Ao completar esta tarefa, os alunos terão uma melhor compreensão dos conceitos de programação orientada a objetos em Rust e como aplicar esses conceitos para simular sistemas de rede.
