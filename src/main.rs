#![windows_subsystem = "windows"]
extern crate imagesize;
extern crate winapi;

mod janela;
use imagesize::size;
use std::cmp::Ordering::*;
use std::env;

const RESPOSTA: [&str; 4] = [
    "10x15 ou 20x30.",
    "3x4 ou 30x40.",
    "20x25.",
    "5x7 ou 15x21.",
];

struct VerTam {
    foto: String,
    largura: usize,
    altura: usize,
    resposta: &'static str
}

impl VerTam {
    fn obter_dados() -> Self {
        if env::args().len() != 2 {
            janela::rodar_janela("Favor entrar com um arquivo.");
            panic!();
        };

        let foto = env::args()
            .nth(1)
            .unwrap()
            .split('\\')
            .last()
            .unwrap()
            .into();

        let dim = size(&foto).unwrap();

        Self {
            foto,
            largura: dim.width,
            altura: dim.height,
            resposta: "",
        }
    }

    fn comparar(&mut self) -> &Self {
        match self.largura.cmp (&self.altura) {
            Greater => {
                let temp = self.largura;
                self.largura = self.altura;
                self.altura = temp;
                self.calcular();
            }
            Less => self.resposta = self.calcular(),
            Equal => self.resposta = "quadrado.",
        }
        self
    }

    fn calcular(&self) -> &'static str {
        [2, 3, 4, 5, 7]
            .windows(2) // Forma os PARES: [[2, 3],[3, 4],[4, 5],[5, 7]]
            .enumerate()
            .find(|f| self.largura / f.1[0] - self.altura / f.1[1] == 0)
            .map_or("desconhecido.", |r| RESPOSTA[r.0])
    }

    fn responder(&self) {
        let resposta = format!("O tamanho da foto < {} > é {}", self.foto, self.resposta);
        janela::rodar_janela(&resposta);
    }
}

fn main() {
    VerTam::obter_dados().comparar().responder();
}

#[cfg(test)]
mod teste {
    use VerTam;
    #[test]
    fn teste() {
        let mut vertam = VerTam {
            foto: "Foto_A308.jpg".into(),
            largura: 1200,
            altura: 1680,
            resposta: "",
        };
        vertam.comparar().responder();
    }
}
