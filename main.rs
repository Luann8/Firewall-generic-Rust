use std::net::{IpAddr, SocketAddr};

// Definição de uma enumeração para representar o protocolo de rede
enum Protocol {
    TCP,
    UDP,
    ICMP,
    Other,
}

// Definição de uma estrutura para representar uma regra de firewall
struct FirewallRule {
    source_ip: IpAddr,
    destination_ip: IpAddr,
    protocol: Protocol,
    port: Option<u16>, // Porta opcional para regras específicas de porta
    allow: bool, // Se verdadeiro, permite o tráfego, senão bloqueia
}

// Função para verificar se um pacote deve ser permitido ou bloqueado com base nas regras do firewall
fn check_packet(rules: &[FirewallRule], source: IpAddr, destination: IpAddr, protocol: Protocol, port: Option<u16>) -> bool {
    for rule in rules {
        if (rule.source_ip == source || rule.source_ip == IpAddr::from([0, 0, 0, 0])) &&
           (rule.destination_ip == destination || rule.destination_ip == IpAddr::from([0, 0, 0, 0])) &&
           (rule.protocol == protocol || rule.protocol == Protocol::Other) &&
           (rule.port == port || rule.port.is_none()) {
            return rule.allow;
        }
    }
    false // Se nenhuma regra corresponder, bloquear por padrão
}

// Função de exemplo para simular o tráfego de entrada
fn simulate_incoming_traffic(rules: &[FirewallRule], source: IpAddr, destination: IpAddr, protocol: Protocol, port: Option<u16>) {
    if check_packet(rules, source, destination, protocol, port) {
        println!("Pacote de {} para {} via {:?} na porta {:?} permitido", source, destination, protocol, port);
    } else {
        println!("Pacote de {} para {} via {:?} na porta {:?} bloqueado", source, destination, protocol, port);
    }
}

fn main() {
    // Definição de algumas regras de exemplo
    let rules = vec![
        FirewallRule {
            source_ip: IpAddr::from([192, 168, 1, 100]),
            destination_ip: IpAddr::from([8, 8, 8, 8]),
            protocol: Protocol::TCP,
            port: Some(80),
            allow: true,
        },
        FirewallRule {
            source_ip: IpAddr::from([0, 0, 0, 0]), // Qualquer IP
            destination_ip: IpAddr::from([8, 8, 8, 8]),
            protocol: Protocol::Other,
            port: None,
            allow: false, // Bloquear todas as conexões com 8.8.8.8
        },
    ];

    // Simulando tráfego de entrada
    simulate_incoming_traffic(&rules, IpAddr::from([192, 168, 1, 100]), IpAddr::from([8, 8, 8, 8]), Protocol::TCP, Some(80));
    simulate_incoming_traffic(&rules, IpAddr::from([192, 168, 1, 101]), IpAddr::from([8, 8, 8, 8]), Protocol::UDP, Some(53));
    simulate_incoming_traffic(&rules, IpAddr::from([10, 0, 0, 1]), IpAddr::from([8, 8, 8, 8]), Protocol::ICMP, None);
}
