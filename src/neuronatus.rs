use minitensor::{Tensor1D, Tensor2D, Tensor3D};
use crate::activatio::*;
use serde::{Serialize, Deserialize};

/// Structura cerebri artificialis simplex.
/// Structure d’un cerveau artificiel simple (réseau de neurones MLP).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Neuronatus {
    pub input: usize,
    pub hidden: usize,
    pub output: usize,

    pub pesi_ih: Tensor2D,     // Poids entrée → cachée
    pub pesi_ho: Tensor2D,     // Poids cachée → sortie
    pub bias_h: Tensor1D,      // Biais couche cachée
    pub bias_o: Tensor1D,      // Biais couche de sortie

    pub celeritas: f64,        // Taux d’apprentissage
}

impl Neuronatus {
    /// Crea novum neuronatum.
/// Crée un nouveau réseau de neurones.
    pub fn novus(input: usize, hidden: usize, output: usize, celeritas: f64) -> Self {
        Self {
            input,
            hidden,
            output,
            pesi_ih: Tensor2D::fortuitus(hidden, input),
            pesi_ho: Tensor2D::fortuitus(output, hidden),
            bias_h: Tensor1D::fortuitus(hidden),
            bias_o: Tensor1D::fortuitus(output),
            celeritas,
        }
    }

    /// Praedictio: dat vectorem inputum, reddit vectorem exitus.
/// Propagation avant : donne une prédiction à partir d’un vecteur d’entrée.
    pub fn praedictio(&self, intrata: &Tensor1D) -> Tensor1D {
        use minitensor::Tensor1D;

        // Convertit inputum in columnam
        let intrata_col = intrata.transpone(); // [n x 1]

        // Calcula activitatem intermediam
        let mut activatio_h = self.pesi_ih.productum_matriciale(&intrata_col); // [hidden x 1]
        activatio_h = activatio_h.adde(&self.bias_h.transpone());
        let activatio_h = Tensor1D::ex_vec(activatio_h.materia.iter().map(|v| sigmoide(v[0])).collect());

        // Calcula exitum
        let mut activatio_o = self.pesi_ho.productum_matriciale(&activatio_h.transpone());
        activatio_o = activatio_o.adde(&self.bias_o.transpone());
        let activatio_o = Tensor1D::ex_vec(activatio_o.materia.iter().map(|v| sigmoide(v[0])).collect());

        activatio_o
    }
    
        /// Instruere rete: unam iterationem discentis perfice.
/// Entraîne le réseau : effectue une itération d’apprentissage.
    pub fn instruere(&mut self, intrata: &Tensor1D, exspectata: &Tensor1D) {
        // 1. Propagatio antea
        let intrata_col = intrata.transpone();

        let mut summa_h = self.pesi_ih.productum_matriciale(&intrata_col);
        summa_h = summa_h.adde(&self.bias_h.transpone());
        let activatio_h_vec: Vec<f64> = summa_h.materia.iter().map(|v| sigmoide(v[0])).collect();
        let activatio_h = Tensor1D::ex_vec(activatio_h_vec.clone());

        let mut summa_o = self.pesi_ho.productum_matriciale(&activatio_h.transpone());
        summa_o = summa_o.adde(&self.bias_o.transpone());
        let activatio_o_vec: Vec<f64> = summa_o.materia.iter().map(|v| sigmoide(v[0])).collect();
        let activatio_o = Tensor1D::ex_vec(activatio_o_vec.clone());

        // 2. Calcula errorem outputi
        let error_o = exspectata.adde(&activatio_o.scale(-1.0));

        // 3. Derivata sigmoidis × error → gradient
        let derivata_o: Vec<f64> = activatio_o_vec.iter().map(|&x| derivata_sigmoidis(x)).collect();
        let gradient_o = Tensor1D::ex_vec(
            derivata_o.iter()
                      .zip(error_o.materia.iter())
                      .map(|(ds, err)| ds * err * self.celeritas)
                      .collect(),
        );

        // 4. Deltas pondus: gradient_o × activatio_hᵗ
        let delta_ho = gradient_o.transpone().productum_matriciale(&activatio_h.transpone().transpone());

        // 5. Update pondus et bias output
        self.pesi_ho = self.pesi_ho.adde(&delta_ho);
        self.bias_o = self.bias_o.adde(&gradient_o);

        // 6. Errorem pro strato occulto
        let pesi_hoᵗ = self.pesi_ho.transpone();
        let error_h = pesi_hoᵗ.productum_matriciale(&error_o.transpone());

        // 7. Derivata sigmoidis × error_h
        let derivata_h: Vec<f64> = activatio_h_vec.iter().map(|&x| derivata_sigmoidis(x)).collect();
        let error_h_vec = error_h.materia.iter().map(|v| v[0]).collect::<Vec<f64>>();
        let gradient_h = Tensor1D::ex_vec(
            derivata_h.iter()
                      .zip(error_h_vec.iter())
                      .map(|(ds, err)| ds * err * self.celeritas)
                      .collect(),
        );

        // 8. Update pondus input → hidden
        let delta_ih = gradient_h.transpone().productum_matriciale(&intrata_col.transpone());
        self.pesi_ih = self.pesi_ih.adde(&delta_ih);
        self.bias_h = self.bias_h.adde(&gradient_h);
    }
    
    /// Instruere rete per batch.
/// Entraîne le réseau sur un batch d'exemples.
pub fn instruere_batch(&mut self, intratae: &Tensor3D, exspectatae: &Tensor3D) {
    // Verifica dimensiones
    assert_eq!(intratae.profunditas, exspectatae.profunditas, "Batch amplitudo debet convenire.");

    for index in 0..intratae.profunditas {
        let input_slice = &intratae.materia[index][0]; // Tensor1D attendu sous forme [1][input]
        let target_slice = &exspectatae.materia[index][0];

        let inputum = Tensor1D::ex_vec(input_slice.clone());
        let exspectatum = Tensor1D::ex_vec(target_slice.clone());

        self.instruere(&inputum, &exspectatum);
    }
}

use std::fmt;

/// Disciplina multi-epochalis.
/// Effectue plusieurs itérations d’apprentissage.
pub fn disciplina(
    &mut self,
    intratae: &Tensor3D,
    exspectatae: &Tensor3D,
    epochs: usize,
    silentium: bool, // true = pas d'affichage
) {
    assert_eq!(intratae.profunditas, exspectatae.profunditas, "Batch amplitudo debet convenire.");

    for epochon in 0..epochs {
        self.instruere_batch(intratae, exspectatae);

        // Indicium progressus si non silentium
        if !silentium && (epochon % 100 == 0 || epochon == epochs - 1) {
            let perditio = self.perditio_batch(intratae, exspectatae);
            println!(
                "Epochon {:>5} / {} completus est. Perditio: {:.6}",
                epochon + 1,
                epochs,
                perditio
            );
        }

    }
}

/// Perditio MSE inter duas Tensor1D.
/// Erreur quadratique moyenne entre deux vecteurs.
pub fn perditio_mse(a: &Tensor1D, b: &Tensor1D) -> f64 {
    assert_eq!(a.magnitudo, b.magnitudo);
    let n = a.magnitudo as f64;
    a.materia.iter()
        .zip(&b.materia)
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>() / n
}

/// Perditio MSE per totum batch.
/// MSE sur un lot d’exemples.
pub fn perditio_batch(&self, intratae: &Tensor3D, exspectatae: &Tensor3D) -> f64 {
    let mut totalis = 0.0;
    let n = intratae.profunditas as f64;

    for i in 0..intratae.profunditas {
        let inputum = Tensor1D::ex_vec(intratae.materia[i][0].clone());
        let exspectatum = Tensor1D::ex_vec(exspectatae.materia[i][0].clone());
        let praedictum = self.praedictio(&inputum);
        totalis += Self::perditio_mse(&praedictum, &exspectatum);
    }

    totalis / n
}


}

