pub mod tensor;
pub mod neuronatus;
pub mod activatio;
pub mod magister;

#[cfg(test)]
mod probationes {
    // Importa structuras e modulo tensorum.
    // Importe les structures depuis le module des tenseurs.
    use super::tensor::{Tensor1D, Tensor2D};

    /// Probat creationem Tensor2D e materia data.
    /// Teste la création d’un Tensor2D depuis une matrice.
    #[test]
    fn probatio_tensor2d_ex_vec() {
        let materia = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let tensor = Tensor2D::ex_vec(materia.clone());

        assert_eq!(tensor.ordines, 2); // Expectatur duo ordines.
        assert_eq!(tensor.columnae, 2); // Expectatur duae columnae.
        assert_eq!(tensor.materia, materia);
    }

    /// Probat creationem tensoris nulli.
    /// Teste la création d’un tenseur rempli de zéros.
    #[test]
    fn probatio_tensor2d_nullus() {
        let tensor = Tensor2D::nullus(3, 2);

        assert_eq!(tensor.ordines, 3);
        assert_eq!(tensor.columnae, 2);

        // Omnis valor debet esse nullus.
        // Chaque valeur doit être égale à zéro.
        for ordo in &tensor.materia {
            assert_eq!(ordo, &vec![0.0, 0.0]);
        }
    }

    /// Probat generatorem fortuitum.
    /// Teste la génération aléatoire.
    #[test]
    fn probatio_tensor2d_fortuitus() {
        let tensor = Tensor2D::fortuitus(3, 2);
        assert_eq!(tensor.ordines, 3);
        assert_eq!(tensor.columnae, 2);

        for ordo in &tensor.materia {
            assert_eq!(ordo.len(), 2);
        }
    }

    /// Probat summam duorum Tensor2D.
    /// Teste l’addition de deux matrices.
    #[test]
    fn probatio_tensor2d_adde() {
        let a = Tensor2D::ex_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let b = Tensor2D::ex_vec(vec![vec![0.5, 1.0], vec![1.5, 2.0]]);
        let summa = a.adde(&b);

        let expectatur = Tensor2D::ex_vec(vec![vec![1.5, 3.0], vec![4.5, 6.0]]);
        assert_eq!(summa, expectatur);use serde::{Serialize, Deserialize};
    }

    /// Probat multiplicationem scalaris.
    /// Teste la multiplication par un scalaire.
    #[test]
    fn probatio_tensor2d_multiplica() {
        let a = Tensor2D::ex_vec(vec![vec![1.0, -2.0], vec![0.5, 4.0]]);
        let multiplicatus = a.multiplica_per_scalar(2.0);
        let expectatur = Tensor2D::ex_vec(vec![vec![2.0, -4.0], vec![1.0, 8.0]]);
        assert_eq!(multiplicatus, expectatur);
    }

    /// Probat productum matriciale.
    /// Teste le produit matriciel.
    #[test]
    fn probatio_tensor2d_productum_matriciale() {
        let a = Tensor2D::ex_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let b = Tensor2D::ex_vec(vec![vec![2.0, 0.0], vec![1.0, 2.0]]);
        let productum = a.productum_matriciale(&b);
        let expectatur = Tensor2D::ex_vec(vec![vec![4.0, 4.0], vec![10.0, 8.0]]);
        assert_eq!(productum, expectatur);
    }

    /// Probat Tensor1D e vectore simplici.
    /// Teste la création d’un vecteur simple.
    #[test]
    fn probatio_tensor1d_ex_vec() {
        let materia = vec![1.0, 2.0, 3.0];
        let vector = Tensor1D::ex_vec(materia.clone());
        assert_eq!(vector.magnitudo, 3);
        assert_eq!(vector.materia, materia);
    }

    /// Probat vectorem nullum.
    /// Teste un vecteur rempli de zéros.
    #[test]
    fn probatio_tensor1d_nullus() {
        let vector = Tensor1D::nullus(4);
        assert_eq!(vector.magnitudo, 4);
        assert!(vector.materia.iter().all(|&x| x == 0.0));
    }

    /// Probat additionem vectorum.
    /// Teste l’addition de deux vecteurs.
    #[test]
    fn probatio_tensor1d_adde() {
        let a = Tensor1D::ex_vec(vec![1.0, 2.0]);
        let b = Tensor1D::ex_vec(vec![0.5, 0.5]);
        let summa = a.adde(&b);
        assert_eq!(summa.materia, vec![1.5, 2.5]);
    }

    /// Probat multiplicationem vectoris per scalar.
    /// Teste la multiplication d’un vecteur par un scalaire.
    #[test]
    fn probatio_tensor1d_multiplica() {
        let vector = Tensor1D::ex_vec(vec![1.0, -2.0]);
        let multiplicatus = vector.multiplica_per_scalar(3.0);
        assert_eq!(multiplicatus.materia, vec![3.0, -6.0]);
    }

    /// Probat productum scalaris duorum vectorum.
    /// Teste le produit scalaire de deux vecteurs.
    #[test]
    fn probatio_tensor1d_productum_scalaris() {
        let a = Tensor1D::ex_vec(vec![1.0, 2.0, 3.0]);
        let b = Tensor1D::ex_vec(vec![4.0, 0.0, -1.0]);
        let productum = a.productum_scalaris(&b);
        assert_eq!(productum, 1.0 * 4.0 + 2.0 * 0.0 + 3.0 * -1.0);
    }

    /// Probat conversionem vectoris in columnam.
    /// Teste la transformation d’un vecteur en colonne.
    #[test]
    fn probatio_tensor1d_transpone() {
        let vector = Tensor1D::ex_vec(vec![1.0, 2.0, 3.0]);
        let columna = vector.transpone();
        assert_eq!(columna.ordines, 3);
        assert_eq!(columna.columnae, 1);
        assert_eq!(columna.materia, vec![vec![1.0], vec![2.0], vec![3.0]]);
    }
    
    use super::tensor::Tensor3D;

/// Probat creationem tensoris 3D ex vectoribus.
/// Teste la création d’un tenseur 3D depuis des vecteurs.
#[test]
fn probatio_tensor3d_ex_vec() {
    let materia = vec![
        vec![vec![1.0, 2.0], vec![3.0, 4.0]],
        vec![vec![5.0, 6.0], vec![7.0, 8.0]],
    ];

    let tensor = Tensor3D::ex_vec(materia.clone());
    assert_eq!(tensor.profunditas, 2);
    assert_eq!(tensor.altitudo, 2);
    assert_eq!(tensor.latitudo, 2);
    assert_eq!(tensor.materia, materia);
}

/// Probat creationem tensoris nulli 3D.
/// Teste un tenseur 3D rempli de zéros.
#[test]
fn probatio_tensor3d_nullus() {
    let tensor = Tensor3D::nullus(3, 2, 4);
    assert_eq!(tensor.profunditas, 3);
    assert_eq!(tensor.altitudo, 2);
    assert_eq!(tensor.latitudo, 4);

    for planum in &tensor.materia {
        for ordo in planum {
            for &val in ordo {
                assert_eq!(val, 0.0);
            }
        }
    }
}

/// Probat creationem tensoris fortuiti 3D.
/// Teste la création d’un tenseur 3D aléatoire.
#[test]
fn probatio_tensor3d_fortuitus() {
    let tensor = Tensor3D::fortuitus(2, 3, 2);
    assert_eq!(tensor.profunditas, 2);
    assert_eq!(tensor.altitudo, 3);
    assert_eq!(tensor.latitudo, 2);
}

}

