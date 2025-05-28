use clap::{Parser, Subcommand};
use mininervus::{magister::MagisterNervorum, neuronatus::Neuronatus};
use minitensor::Tensor1D;

#[derive(Parser)]
#[command(name = "mininervus")]
#[command(about = "Magister artificialium neuronorum", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commande,
}

#[derive(Subcommand)]
enum Commande {
    /// Crea novum rete neuronale
    Crea {
        #[arg(short, long)]
        nom: String,
        #[arg(short, long)]
        input: usize,
        #[arg(short, long)]
        hidden: usize,
        #[arg(short, long)]
        output: usize,
        #[arg(short, long, default_value_t = 0.3)]
        rate: f64,
    },

    /// Praedictio
    Praedictio {
        #[arg(short, long)]
        nom: String,
        #[arg()]
        input: Vec<f64>,
    },

    /// Salva rete
    Salva {
        #[arg(short, long)]
        nom: String,
        #[arg(short, long)]
        versus: String,
    },

    /// Onera rete ex archivo
    Onera {
        #[arg(short, long)]
        nom: String,
        #[arg(short, long)]
        ex: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let mut magister = MagisterNervorum::novus();

    match cli.command {
        Commande::Crea { nom, input, hidden, output, rate } => {
            let rete = Neuronatus::novus(input, hidden, output, rate);
            magister.adde(&nom, rete);
            println!("Rete '{}' creatum est.", nom);
        }

        Commande::Praedictio { nom, input } => {
            let inputum = Tensor1D::ex_vec(input);
            if let Some(exitus) = magister.praedictio(&nom, &inputum) {
                println!("Exitus: {:?}", exitus.materia);
            } else {
                println!("Rete '{}' non est inventum.", nom);
            }
        }

        Commande::Salva { nom, versus } => {
            if let Some(rete) = magister.retia.get(&nom) {
                match rete.salva_in(&versus) {
                    Ok(_) => println!("Rete '{}' salvatus est in '{}'", nom, versus),
                    Err(e) => eprintln!("Error in salvatione: {}", e),
                }
            } else {
                eprintln!("Rete '{}' non est inventum.", nom);
            }
        }

        Commande::Onera { nom, ex } => {
            match Neuronatus::restitue_ex(&ex) {
                Ok(rete) => {
                    magister.adde(&nom, rete);
                    println!("Rete '{}' oneratim est ex '{}'", nom, ex);
                }
                Err(e) => eprintln!("Error in restitutione: {}", e),
            }
        }
    }
}

