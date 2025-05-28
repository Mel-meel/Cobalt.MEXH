# Neurocrates

> *Rete Neuronale Minimalisticum in lingua Rustica*

---

**Neurocrates** est instrumentum ad creandum, exercendum et administrandum retia neuronalia simplicia.
Scriptum est in Rust, cum typis `Tensor1D`, `Tensor2D`, et `Tensor3D`, necnon CLI ad usum manualem vel automatum.

### Characteristicae

- Crates modularis cum `Tensoribus` definitis (1D, 2D, 3D)
- Retiaria neuronalia simplicia cum functionibus activationis
- Disciplina per `epochas`, cum optione logicae externa
- Serializationis subsidium (per `serde`)
- Interfacia CLI ad experimenta celeria

### Exemplum

```bash
cargo run --example xor_batch

