use mininervus::neuronatus::Neuronatus;
use minitensor::Tensor1D;

/// Exemplum XOR cum rete Neuronatus.
/// Exemple de résolution du XOR avec un réseau de neurones.

fn main() {
    // Inputa et exspectata
    // Les entrées et les sorties attendues
    let exempla = vec![
        (vec![0.0, 0.0], vec![0.0]),
        (vec![0.0, 1.0], vec![1.0]),
        (vec![1.0, 0.0], vec![1.0]),
        (vec![1.0, 1.0], vec![0.0]),
    ];

    // Crea Neuronatum cum 2 inputis, 2 occultis, 1 outputo
    let mut rete = Neuronatus::novus(2, 2, 1, 0.5);

    // Itera mille vicibus
    for epochon in 0..10000 {
        for (inputum, exspectatum) in &exempla {
            let in_vec = Tensor1D::ex_vec(inputum.clone());
            let ex_vec = Tensor1D::ex_vec(exspectatum.clone());
            rete.instruere(&in_vec, &ex_vec);
        }

        if epochon % 1000 == 0 {
            println!("Epochon {} perfectum est.", epochon);
        }
    }

    println!("\nPraedictio finalis:\n");

    // Praedicatio post disciplinam
    for (inputum, exspectatum) in exempla {
        let in_vec = Tensor1D::ex_vec(inputum.clone());
        let exitus = rete.praedictio(&in_vec);
        println!(
            "Inputum: {:?} → Praedictum: {:.4} | Exspectatum: {}",
            inputum,
            exitus.materia[0],
            exspectatum[0]
        );
    }
}

