use mininervus::{magister::MagisterNervorum, neuronatus::Neuronatus};
use minitensor::Tensor1D;

fn main() {
    let mut magister = MagisterNervorum::novus();

    let rete1 = Neuronatus::novus(2, 2, 1, 0.3);
    let rete2 = Neuronatus::novus(3, 3, 1, 0.1);

    magister.adde("xor", rete1);
    magister.adde("trinus", rete2);

    let via = "magister.json";

    // Salva
    match magister.salva_in(via) {
        Ok(_) => println!("Magister salvatus in '{}'.", via),
        Err(e) => eprintln!("Error in salvatione: {}", e),
    }

    // Restitue
    match MagisterNervorum::restitue_ex(via) {
        Ok(restitutus) => {
            println!("Magister restitutus cum {} retibus.", restitutus.retia.len());
            for nomen in restitutus.nomina() {
                println!("→ Rete: {}", nomen);
            }
        }
        Err(e) => eprintln!("Error in restitutione: {}", e),
    }
}

