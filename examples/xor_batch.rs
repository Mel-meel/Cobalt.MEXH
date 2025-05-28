use mininervus::neuronatus::Neuronatus;
use minitensor::{Tensor1D, Tensor3D};

fn main() {
    let exempla_inputa = vec![
        vec![vec![0.0, 0.0]],
        vec![vec![0.0, 1.0]],
        vec![vec![1.0, 0.0]],
        vec![vec![1.0, 1.0]],
    ];

    let exempla_exitus = vec![
        vec![vec![0.0]],
        vec![vec![1.0]],
        vec![vec![1.0]],
        vec![vec![0.0]],
    ];

    let intratae = Tensor3D::ex_vec(exempla_inputa);
    let exspectatae = Tensor3D::ex_vec(exempla_exitus);

    let mut rete = Neuronatus::novus(2, 2, 1, 0.3);

    for epoch in 0..5000 {
        rete.instruere_batch(&intratae, &exspectatae);

        if epoch % 1000 == 0 {
            println!("Epochon {} completus est.", epoch);
        }
    }

    println!("\nPraedictio finalis:");
    for i in 0..intratae.profunditas {
        let inputum = Tensor1D::ex_vec(intratae.materia[i][0].clone());
        let exitus = rete.praedictio(&inputum);
        println!(
            "Inputum: {:?} → Exitus: {:.4} | Exspectatum: {}",
            inputum.materia,
            exitus.materia[0],
            exspectatae.materia[i][0][0]
        );
    }
}

