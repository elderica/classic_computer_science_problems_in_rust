#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Nucleotide {
    A,
    C,
    G,
    T,
}

impl Nucleotide {
    fn new(c: char) -> Self {
        match c {
            'A' => Nucleotide::A,
            'a' => Nucleotide::A,
            'C' => Nucleotide::C,
            'c' => Nucleotide::C,
            'G' => Nucleotide::G,
            'g' => Nucleotide::G,
            'T' => Nucleotide::T,
            't' => Nucleotide::T,
            _ => panic!("not nucleotide character: {}", c),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Codon(Nucleotide, Nucleotide, Nucleotide);

impl Codon {
    fn new(str: &str) -> Self {
        if str.len() != 3 {
            panic!("string length is not 3");
        }

        let mut it = str.chars().take(3);
        Codon(
            Nucleotide::new(it.next().unwrap()),
            Nucleotide::new(it.next().unwrap()),
            Nucleotide::new(it.next().unwrap()),
        )
    }
}

struct Gene(Vec<Codon>);

impl Gene {
    fn new(s: &str) -> Self {
        let g = s
            .as_bytes()
            .chunks(3)
            .map(std::str::from_utf8)
            .map(|b| match b {
                Ok(c) => Codon::new(c),
                _ => panic!("UTF-8 decode error"),
            })
            .collect();
        Gene(g)
    }

    fn linear_contains(&self, key: &Codon) -> bool {
        self.0.contains(key)
    }

    fn binary_contains(&self, key: &Codon) -> bool {
        let mut sorted: Vec<Codon> = self.0.to_vec();
        sorted.sort_unstable();

        sorted.binary_search(key).is_ok()
    }
}

fn main() {
    let genestr = "ACGTGGCTCTCTAACGTACGTACGTACGGGGTTTATATATACCCTAGGACTCCCTTT";
    let mygene = Gene::new(genestr);
    let acg = Codon::new("ACG");
    let gat = Codon::new("GAT");
    println!("{}", mygene.linear_contains(&acg));
    println!("{}", mygene.linear_contains(&gat));
    println!("{}", mygene.binary_contains(&acg));
    println!("{}", mygene.binary_contains(&gat));
}
