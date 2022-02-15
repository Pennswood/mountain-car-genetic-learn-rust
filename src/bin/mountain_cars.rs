mod car_sim;
mod settings;

use genevo::{
    operator::prelude::*, population::ValueEncodedGenomeBuilder, prelude::*, types::fmt::Display,
};

// const TARGET_TEXT: &str = "See how a genius creates a legend";
//const TARGET_TEXT: &str = "Hello how are you";
// const TARGET_TEXT: &str = "All the world's a stage, and all the men and women merely players: \
//                           they have their exits and their entrances; and one man in his time \
//                           plays many parts, his acts being seven ages.";

#[derive(Debug)]
struct Parameter {
    population_size: usize,
    generation_limit: u64,
    num_individuals_per_parents: usize,
    selection_ratio: f64,
    num_crossover_points: usize,
    mutation_rate: f64,
    reinsertion_ratio: f64,
}



impl Default for Parameter {
    fn default() -> Self {
        Parameter {
            population_size: (((settings::WIDTH/8) as f64).ln()) as usize,
            generation_limit: 2000,
            num_individuals_per_parents: 2,
            selection_ratio: 0.7,
            num_crossover_points: (((((settings::WIDTH/8) as f64)) as i64) / 6)as usize,
            mutation_rate: 0.05 / (((settings::WIDTH/8) as f64)).ln(),
            reinsertion_ratio: 0.7,
        }
    }
}

/// The phenotype
type Text = String;

/// The genotype
type TextGenome = Vec<u8>;

/// How do the genes of the genotype show up in the phenotype
trait AsPhenotype {
    fn as_text(&self) -> Text;
}

impl AsPhenotype for TextGenome {
    fn as_text(&self) -> Text {
        String::from_utf8(self.to_vec()).unwrap()
    }
}

/// The fitness function for `TextGenome`s.
#[derive(Clone, Debug)]
struct FitnessCalc;

impl FitnessFunction<TextGenome, usize> for FitnessCalc {
    fn fitness_of(&self, genome: &TextGenome) -> usize {
        let mut enumGenome = Vec::new();
        for i in genome {
            let data = settings::u8_to_enum_arr(*i);
            enumGenome.extend(data.iter().copied());
            
        }
        car_sim::simulate(enumGenome) as usize
    }

    fn average(&self, fitness_values: &[usize]) -> usize {
        fitness_values.iter().sum::<usize>() / fitness_values.len()
    }

    fn highest_possible_fitness(&self) -> usize {
        settings::WIDTH
    }

    fn lowest_possible_fitness(&self) -> usize {
        0
    }
}

fn main() {
    let params = Parameter::default();

    let initial_population: Population<TextGenome> = build_population()
        .with_genome_builder(ValueEncodedGenomeBuilder::new(settings::MAX_ITER/4, 32, 126))
        .of_size(params.population_size)
        .uniform_at_random();

    let mut monkeys_sim = simulate(
        genetic_algorithm()
            .with_evaluation(FitnessCalc)
            .with_selection(MaximizeSelector::new(
                params.selection_ratio,
                params.num_individuals_per_parents,
            ))
            .with_crossover(MultiPointCrossBreeder::new(params.num_crossover_points))
            .with_mutation(RandomValueMutator::new(params.mutation_rate, 32, 126))
            .with_reinsertion(ElitistReinserter::new(
                FitnessCalc,
                true,
                params.reinsertion_ratio,
            ))
            .with_initial_population(initial_population)
            .build(),
    )
    .until(or(
        FitnessLimit::new(FitnessCalc.highest_possible_fitness()),
        GenerationLimit::new(params.generation_limit),
    ))
    .build();

    println!("Starting Shakespeare's Monkeys with: {:?}", params);

    loop {
        let result = monkeys_sim.step();
        match result {
            Ok(SimResult::Intermediate(step)) => {
                let evaluated_population = step.result.evaluated_population;
                let best_solution = step.result.best_solution;
                println!(
                    "Step: generation: {}, average_fitness: {}, \
                     best fitness: {}, duration: {}, processing_time: {}",
                    step.iteration,
                    evaluated_population.average_fitness(),
                    best_solution.solution.fitness,
                    step.duration.fmt(),
                    step.processing_time.fmt()
                );
                //println!("      {}", best_solution.solution.genome.as_text());
                //                println!("| population: [{}]", result.population.iter().map(|g| g.as_text())
                //                    .collect::<Vec<String>>().join("], ["));
            },
            Ok(SimResult::Final(step, processing_time, duration, stop_reason)) => {
                let best_solution = step.result.best_solution;
                println!("{}", stop_reason);
                println!(
                    "Final result after {}: generation: {}, \
                     best solution with fitness {} found in generation {}, processing_time: {}",
                    duration.fmt(),
                    step.iteration,
                    best_solution.solution.fitness,
                    best_solution.generation,
                    processing_time.fmt()
                );
                //println!("      {}", best_solution.solution.genome.as_text());
                break;
            },
            Err(error) => {
                println!("{}", error);
                break;
            },
        }
    }
}