use mininervus::{neuronatus::Neuronatus, magister::MagisterNervorum};
use minitensor::Tensor1D;

fn main() {
    // Crea magistrum
    let mut magister = MagisterNervorum::novus();

    // Adde duos retia
    let rete_a = Neuronatus::novus(2, 2, 1, 0.3);
    let rete_b = Neuronatus::novus(2, 4, 1, 0.5);

    magister.adde("xorus", rete_a);
    magister.adde("altus", rete_b);

    // Instruere retia cum exemplis XOR
    let exempla = vec![
        (vec![0.0, 0.0], vec![0.0]),
        (vec![0.0, 1.0], vec![1.0]),
        (vec![1.0, 0.0], vec![1.0]),
        (vec![1.0, 1.0], vec![0.0]),
    ];

    for _ in 0..5000 {
        for (inp, out) in &exempla {
            let x = Tensor1D::ex_vec(inp.clone());
            let y = Tensor1D::ex_vec(out.clone());
            magister.instruere("xorus", &x, &y);
        }
    }

    // Praedictio finalis
    println!("Retia in magistro: {:?}", magister.nomina());

    for (inp, out) in exempla {
        let inputum = Tensor1D::ex_vec(inp.clone());
        if let Some(exitus) = magister.praedictio("xorus", &inputum) {
            println!(
                "Inputum: {:?} → Exitus: {:.4} | Exspectatum: {}",
                inp,
                exitus.materia[0],
                out[0]
            );
        }
    }
}

