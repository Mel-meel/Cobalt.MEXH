/// Sigmoide: functio activationis.
/// Sigmoïde : fonction d’activation.
pub fn sigmoide(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// Derivata sigmoidis.
/// Dérivée de la sigmoïde.
pub fn derivata_sigmoidis(x: f64) -> f64 {
    let s = sigmoide(x);
    s * (1.0 - s)
}

