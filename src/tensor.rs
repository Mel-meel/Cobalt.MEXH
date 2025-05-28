use rand::Rng;
use std::fmt;
use serde::{Serialize, Deserialize};

//
// TENSOR BIDIMENSIONALIS
// ----------------------
// Tensor du type matrice (2D) avec lignes et colonnes.
//

/// Tensor bidimensionalis, cum ordinibus et columnis.
/// Tenseur à deux dimensions, avec lignes et colonnes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tensor2D {
    /// Materia interna: rursus vectorum.
/// Données internes : vecteur de vecteurs (matrice).
    pub materia: Vec<Vec<f64>>,

    /// Numerus ordinum (rangs horizontalium).
/// Nombre de lignes.
    pub ordines: usize,

    /// Numerus columnarum (columnae verticales).
/// Nombre de colonnes.
    pub columnae: usize,
}

impl Tensor2D {
    /// Crea tensorem e vectore duorum dimensionum.
/// Crée un tenseur à partir d’un vecteur 2D.
    pub fn ex_vec(materia: Vec<Vec<f64>>) -> Self {
        let ordines = materia.len();

        // Saltem unus ordo necessarius est.
        // Il doit y avoir au moins une ligne.
        assert!(ordines > 0, "Saltem unus ordo esse debet.");

        let columnae = materia[0].len();

        // Omnes ordines aequae longitudinis esse debent.
        // Toutes les lignes doivent avoir la même longueur.
        for ordo in &materia {
            assert_eq!(ordo.len(), columnae, "Omnes ordines eandem longitudinem habere debent.");
        }

        Self { materia, ordines, columnae }
    }

    /// Tensor plene nullus.
/// Crée une matrice remplie de zéros.
    pub fn nullus(ordines: usize, columnae: usize) -> Self {
        let materia = vec![vec![0.0; columnae]; ordines];
        Self { materia, ordines, columnae }
    }

    /// Tensor fortuitus inter 0 et 1.
/// Crée une matrice avec des valeurs aléatoires uniformes.
    pub fn fortuitus(ordines: usize, columnae: usize) -> Self {
        let mut aleator = rand::thread_rng();

        // Genera matricem aleatoriam.
        // Génère une matrice avec des flottants aléatoires.
        let materia = (0..ordines)
            .map(|_| (0..columnae).map(|_| aleator.alea::<f64>()).collect())
            .collect();

        Self { materia, ordines, columnae }
    }

    /// Adde alium tensorem (dimensiones aequae).
/// Additionne deux tenseurs de même dimension.
    pub fn adde(&self, alius: &Self) -> Self {
        assert_eq!(self.ordines, alius.ordines);
        assert_eq!(self.columnae, alius.columnae);

        // Summatio elementa ad elementum.
        // Addition élément par élément.
        let materia = self.materia.iter().zip(&alius.materia)
            .map(|(ordo_a, ordo_b)| {
                ordo_a.iter().zip(ordo_b).map(|(a, b)| a + b).collect()
            })
            .collect();

        Self::ex_vec(materia)
    }

    /// Multiplica omnem elementum per scalar.
/// Multiplie tous les éléments par un scalaire.
    pub fn multiplica_per_scalar(&self, scala: f64) -> Self {
        let materia = self.materia.iter()
            .map(|ordo| ordo.iter().map(|x| x * scala).collect())
            .collect();

        Self::ex_vec(materia)
    }

    /// Productum matriciale duorum tensorum.
/// Produit matriciel entre deux matrices (dot product).
    pub fn productum_matriciale(&self, alius: &Self) -> Self {
        // Verifica compatibilitatem dimensionum.
        // Vérifie la compatibilité des dimensions.
        assert_eq!(self.columnae, alius.ordines);

        let mut resultatum = vec![vec![0.0; alius.columnae]; self.ordines];

        // Computatio elementorum via summatio multiplicatorum.
        // Calcul classique par triple boucle.
        for i in 0..self.ordines {
            for j in 0..alius.columnae {
                for k in 0..self.columnae {
                    resultatum[i][j] += self.materia[i][k] * alius.materia[k][j];
                }
            }
        }

        Self::ex_vec(resultatum)
    }
}

/// Forma humanis legibilis tensoris.
/// Affichage lisible pour l'œil humain.
impl fmt::Display for Tensor2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for ordo in &self.materia {
            writeln!(
                f,
                "{}",
                ordo.iter()
                    .map(|v| format!("{:.4}", v))
                    .collect::<Vec<_>>()
                    .join("\t")
            )?;
        }
        Ok(())
    }
}

//
// TENSOR UNIDIMENSIONALIS (VECTOR)
// --------------------------------
// Vecteur simple, utile pour les biais, entrées, etc.
//

/// Tensor unidimensionalis, sive vector.
/// Vecteur simple, une seule dimension.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tensor1D {
    /// Elementa vectoris.
/// Les valeurs du vecteur.
    pub materia: Vec<f64>,

    /// Longitudo vectoris.
/// Taille du vecteur.
    pub magnitudo: usize,
}

impl Tensor1D {
    /// Ex vectore floatium.
/// Crée un vecteur à partir d'un `Vec<f64>`.
    pub fn ex_vec(materia: Vec<f64>) -> Self {
        let magnitudo = materia.len();
        Self { materia, magnitudo }
    }

    /// Vector nullus (omnes elementa sunt 0).
/// Vecteur rempli de zéros.
    pub fn nullus(magnitudo: usize) -> Self {
        let materia = vec![0.0; magnitudo];
        Self { materia, magnitudo }
    }

    /// Vector fortuitus.
/// Vecteur de valeurs aléatoires entre 0 et 1.
    pub fn fortuitus(magnitudo: usize) -> Self {
        let mut aleator = rand::thread_rng();
        let materia = (0..magnitudo).map(|_| aleator.alea::<f64>()).collect();
        Self { materia, magnitudo }
    }

    /// Adde alium vectorem.
/// Additionne deux vecteurs.
    pub fn adde(&self, alius: &Self) -> Self {
        assert_eq!(self.magnitudo, alius.magnitudo);
        let materia = self.materia.iter().zip(&alius.materia)
            .map(|(a, b)| a + b).collect();
        Self::ex_vec(materia)
    }

    /// Multiplica elementa per scalar.
/// Multiplie chaque valeur par un scalaire.
    pub fn multiplica_per_scalar(&self, scala: f64) -> Self {
        let materia = self.materia.iter().map(|x| x * scala).collect();
        Self::ex_vec(materia)
    }

    /// Productum scalaris (dot product).
/// Produit scalaire entre deux vecteurs.
    pub fn productum_scalaris(&self, alius: &Self) -> f64 {
        assert_eq!(self.magnitudo, alius.magnitudo);
        self.materia.iter().zip(&alius.materia).map(|(a, b)| a * b).sum()
    }

    /// Transpone in columnam (Tensor2D).
/// Transforme ce vecteur en matrice colonne.
    pub fn transpone(&self) -> Tensor2D {
        let materia = self.materia.iter().map(|x| vec![*x]).collect();
        Tensor2D::ex_vec(materia)
    }
}

//
// TENSOR TRIDIMENSIONALIS (3D)
// ----------------------------
// Utilisatur in batchis, imaginibus, aut temporibus.
//

/// Tensor tridimensionalis: profunditas × altitudo × latitudo.
/// Tenseur à trois dimensions : profondeur × hauteur × largeur.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tensor3D {
    /// Materia interna: series matricum.
/// Données internes : série de matrices (vecteur de vecteurs de vecteurs).
    pub materia: Vec<Vec<Vec<f64>>>,

    /// Numerus planorum seu stratorum.
/// Nombre de plans/profondeur.
    pub profunditas: usize,

    /// Numerus ordinum per planum.
/// Nombre de lignes par plan.
    pub altitudo: usize,

    /// Numerus columnarum per ordinem.
/// Nombre de colonnes par ligne.
    pub latitudo: usize,
}

impl Tensor3D {
    /// Crea tensorem e materia explicita.
/// Crée un tenseur 3D à partir de données explicites.
    pub fn ex_vec(materia: Vec<Vec<Vec<f64>>>) -> Self {
        let profunditas = materia.len();
        assert!(profunditas > 0, "Saltem unum stratum esse debet."); // Il doit y avoir au moins un plan

        let altitudo = materia[0].len();
        let latitudo = materia[0][0].len();

        for planum in &materia {
            assert_eq!(planum.len(), altitudo, "Omnia strata eandem altitudinem habere debent.");
            for ordo in planum {
                assert_eq!(ordo.len(), latitudo, "Omnes ordines eandem latitudinem habere debent.");
            }
        }

        Self { materia, profunditas, altitudo, latitudo }
    }

    /// Tensor nullus tridimensionalis.
/// Crée un tenseur 3D rempli de zéros.
    pub fn nullus(profunditas: usize, altitudo: usize, latitudo: usize) -> Self {
        let materia = vec![vec![vec![0.0; latitudo]; altitudo]; profunditas];
        Self { materia, profunditas, altitudo, latitudo }
    }

    /// Tensor fortuitus tridimensionalis.
/// Crée un tenseur 3D avec valeurs aléatoires entre 0 et 1.
    pub fn fortuitus(profunditas: usize, altitudo: usize, latitudo: usize) -> Self {
        let mut aleator = rand::thread_rng();
        let materia = (0..profunditas).map(|_| {
            (0..altitudo).map(|_| {
                (0..latitudo).map(|_| aleator.alea::<f64>()).collect()
            }).collect()
        }).collect();

        Self { materia, profunditas, altitudo, latitudo }
    }
}

/// Forma legibilis tensoris tridimensionalis.
/// Affichage lisible d’un tenseur 3D.
impl fmt::Display for Tensor3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, planum) in self.materia.iter().enumerate() {
            writeln!(f, "Stratum {}:", i)?;
            for ordo in planum {
                writeln!(
                    f,
                    "{}",
                    ordo.iter()
                        .map(|v| format!("{:.3}", v))
                        .collect::<Vec<_>>()
                        .join("\t")
                )?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

