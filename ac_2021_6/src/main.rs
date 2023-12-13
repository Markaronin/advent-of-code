use advent_of_code_util::parse::read_lines;

struct LanternfishPopulation {
    pops: [usize; 9],
}
impl LanternfishPopulation {
    fn from_str(string: &str) -> Self {
        let mut new_population = LanternfishPopulation { pops: [0; 9] };
        string
            .split(',')
            .map(|fish| fish.parse::<usize>().unwrap())
            .for_each(|fish| new_population.pops[fish] += 1);
        new_population
    }
    fn forward_1_day(&mut self) {
        let new_fish = self.pops[0];
        for i in 1..=8 {
            self.pops[i - 1] = self.pops[i];
        }
        self.pops[6] += new_fish;
        self.pops[8] = new_fish;
    }
    fn forward_n_days(&mut self, n: usize) {
        for _ in 0..n {
            self.forward_1_day();
        }
    }
    fn total_population(&self) -> usize {
        self.pops.iter().sum()
    }
}

fn main() {
    let mut pop = LanternfishPopulation::from_str(&read_lines("ac_2021_6/input")[0]);
    pop.forward_n_days(18);
    println!(
        "After 18 days, there are {} lanternfish",
        pop.total_population()
    );
    pop.forward_n_days(80 - 18);
    println!(
        "After 80 days, there are {} lanternfish",
        pop.total_population()
    );
    pop.forward_n_days(256 - 80);
    println!(
        "After 256 days, there are {} lanternfish",
        pop.total_population()
    );
}
