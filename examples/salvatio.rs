use mininervus::neuronatus::Neuronatus;

fn main() {
    let rete = Neuronatus::novus(2, 4, 1, 0.1);
    let via = "rete_salvatum.json";

    match rete.salva_in(via) {
        Ok(_) => println!("Rete salvatus in '{}'.", via),
        Err(e) => eprintln!("Error in salvatione: {}", e),
    }

    match Neuronatus::restitue_ex(via) {
        Ok(restitutus) => println!("Rete restitutum: {:?}", restitutus),
        Err(e) => eprintln!("Error in restitutione: {}", e),
    }
}

