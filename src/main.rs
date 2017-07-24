extern crate rand;

use std::io;
use std::io::Write;

use rand::Rng;

const GENOME_CHARACTERS: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@\
                                 ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 [\\]^_`\
                                 abcdefghijklmnopqrstuvwxyz\
                                 {|}~";
 

#[derive(Debug)]
struct Population {
    population_size: usize,
    genome_size: usize,

    target: String,
    genomes: Vec<String>
}

impl Population {
    pub fn new(target: String, population_size: usize) -> Self {
        let genome_size = target.len();

        let mut instance = Population {
            population_size: population_size,
            genome_size: genome_size,

            target: target,
            genomes: (0..population_size).map(|_| Self::random_genome(genome_size)).collect()
        };
        instance.sort_genomes();
        instance
    }

    fn random_genome(genome_size: usize) -> String {
        let mut rng = rand::thread_rng();

        (0..genome_size).map(|x| rand::sample(&mut rng, GENOME_CHARACTERS.chars(), 1)[0])
                        .collect()
    }

    // `fitness` here is a static method to avoid having to borrow self both mutably and immutably
    // in next_generation.
    fn fitness(target: &String, genome: &String) -> usize {
        genome.chars()
              .zip(target.chars())
              .map(|(x, y)| if x == y {1} else {0})
              .sum()
    }

    fn crossover(&self, mother: &String, father: &String) -> String {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < 0.35 {
            Self::random_genome(self.genome_size)
        } else {
            mother.chars()
                  .zip(father.chars())
                  .map(|(x, y)| if rng.gen() {x} else {y})
                  .collect()
        }
    }

    pub fn sort_genomes(&mut self) {
        let target = &self.target;
        // We sort by reverse on purpose, so that the most fit are at the start, not at the end.
        self.genomes.sort_by(|x, y| Self::fitness(target, y).cmp(&Self::fitness(target, x)));
    }

    pub fn most_fit(&self) -> String {
        self.genomes[0].clone()
    }

    pub fn next_generation(&mut self) {
        // It is assumed that the genomes are already sorted once this function runs, since the
        // genome list is private.
        let mut new_genomes: Vec<String> = Vec::new();

        for genome in self.genomes.iter().take(self.population_size / 2) {
            new_genomes.push(genome.clone());
        }

        let mut rng = rand::thread_rng();
        let meteor = rng.gen::<f64>() < 0.10;

        while new_genomes.len() < self.population_size {
            let new_genome;

            if meteor {
                let mother = rand::sample(&mut rng, &new_genomes, 1)[0];
                let father = Self::random_genome(self.genome_size);
                new_genome = self.crossover(&mother, &father);
            } else {
                let parents = rand::sample(&mut rng, &new_genomes, 2);
                new_genome = self.crossover(&parents[0], &parents[1]);
            }

            new_genomes.push(new_genome);
        }

        self.genomes = new_genomes;
        self.sort_genomes();
    }
}

fn main() {
    let mut target = String::new();
    loop {
        print!("Enter a target: ");
        io::stdout().flush().ok().expect("Could not flush stdout");

        match io::stdin().read_line(&mut target) {
            Ok(_) => {
                target = String::from(target.trim());
                break;
            },
            _ => continue
        }
    }

    let mut population = Population::new(target, 100);

    let mut generations = 0;
    while !population.genomes.contains(&population.target) {
        println!("Most fit in generation {}: {}", generations, population.most_fit());
        population.next_generation();
        generations += 1;
    }

    println!("Most fit in generation {}: {}", generations, population.most_fit());
    println!("Took {} generations to achieve the target.", generations);
}
