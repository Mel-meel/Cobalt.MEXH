use std::collections::HashMap;
use crate::neuronatus::Neuronatus;
use minitensor::Tensor1D;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, BufReader};

/// Magister Nervorum.
/// Maître des neurones : gère plusieurs réseaux de neurones.
pub struct MagisterNervorum {
    pub retia: HashMap<String, Neuronatus>,
}

#[derive(Serialize, Deserialize)]
impl MagisterNervorum {
    /// Crea novum magistrum sine rete.
/// Crée un gestionnaire vide.
    pub fn novus() -> Self {
        Self {
            retia: HashMap::new(),
        }
    }

    /// Adde novum rete sub nomine.
/// Ajoute un nouveau réseau sous un nom donné.
    pub fn adde(&mut self, nomen: &str, rete: Neuronatus) {
        self.retia.insert(nomen.to_string(), rete);
    }

    /// Praedictio per nomen.
/// Fait une prédiction avec un réseau identifié par son nom.
    pub fn praedictio(&self, nomen: &str, inputum: &Tensor1D) -> Option<Tensor1D> {
        self.retia.get(nomen).map(|r| r.praedictio(inputum))
    }

    /// Instruere rete per nomen.
/// Entraîne le réseau identifié.
    pub fn instruere(&mut self, nomen: &str, inputum: &Tensor1D, exspectatum: &Tensor1D) -> bool {
        if let Some(rete) = self.retia.get_mut(nomen) {
            rete.instruere(inputum, exspectatum);
            true
        } else {
            false
        }
    }

    /// Redde omnia nomina retium.
/// Renvoie la liste des réseaux enregistrés.
    pub fn nomina(&self) -> Vec<String> {
        self.retia.keys().cloned().collect()
    }
    
        /// Salva omnes retia in archivo.
/// Sauvegarde tous les réseaux dans un fichier JSON.
    pub fn salva_in(&self, via: &str) -> std::io::Result<()> {
        let mut file = File::create(via)?;
        let json = serde_json::to_string_pretty(self)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Restitue magistrum e archivo.
/// Restaure un gestionnaire entier depuis un fichier.
    pub fn restitue_ex(via: &str) -> std::io::Result<Self> {
        let file = File::open(via)?;
        let reader = BufReader::new(file);
        let magister: MagisterNervorum = serde_json::from_reader(reader)?;
        Ok(magister)
    }
}

