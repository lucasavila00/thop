use makers;
use parser;
use std::collections::HashMap;

pub struct Instance<'a> {
    instance: &'a parser::THOPFile,
    distance_matrix: Vec<Vec<u32>>,
    items_per_city: HashMap<u32, Vec<&'a parser::ItemSection>>,
}
impl<'a> Instance<'a> {
    // pub fn parse(contents: &str) -> Instance<'a> {
    //     let instance = &parser::instance::parse(&contents);

    //     Instance {
    //         instance,
    //         distance_matrix: makers::make_distance_matrix(&instance.node_coord_section),
    //         items_per_city: makers::make_items_per_city(&instance.items_section),
    //     }
    // }

    pub fn new(instance: &'a parser::THOPFile) -> Instance {
        Instance {
            instance,
            distance_matrix: makers::make_distance_matrix(&instance.node_coord_section),
            items_per_city: makers::make_items_per_city(&instance.items_section),
        }
    }

    pub fn get_max_speed(&self) -> f64 {
        self.instance.max_speed.unwrap()
    }

    pub fn visit_city(&self, city: u32, asked_items_hash: &HashMap<u32, bool>) -> (u32, u32, u32) {
        // (u32: weight, u32: profit, u32: n items catched)
        let mut weight = 0;
        let mut profit = 0;
        let mut caught = 0;
        match self.items_per_city.get(&city) {
            Some(items) => {
                for item in items {
                    if asked_items_hash.contains_key(&item.index) {
                        caught += 1;
                        weight += item.weight;
                        profit += item.profit;
                    }
                }
            }
            None => {}
        }
        (weight, profit, caught)
    }

    pub fn speed_descresc_per_weight(&self) -> f64 {
        (self.instance.max_speed.unwrap() - self.instance.min_speed.unwrap())
            / (self.instance.capacity_of_knapsack.unwrap() as f64)
    }

    pub fn get_capacity_of_knapsack(&self) -> u32 {
        self.instance.capacity_of_knapsack.unwrap()
    }

    pub fn get_max_time(&self) -> u32 {
        self.instance.max_time.unwrap()
    }

    pub fn get_distance(&self, a: &u32, b: &u32) -> u32 {
        if a == b {
            return 0;
        };
        // o menor valor é a linha
        // menos um por causa do index
        // --
        // a coluna é o maior valor menos a linha
        // menos um por causa do index
        //      1    2   3   4
        //      -    -   -   -
        // 1         5   6   8
        // 2             8   6
        // 3                 5
        // 4

        let min = a.min(b);
        let max = a.max(b);

        let line = (*min - 1) as usize;
        let row = (*max - min - 1) as usize;
        *self
            .distance_matrix
            .get(line)
            .expect("unable to get line")
            .get(row)
            .expect("unable to get row")
    }
}

#[cfg(test)]
mod test_full {

    use std::fs::File;
    use std::io::prelude::*;

    use super::*;

    #[test]
    fn integration() {
        let mut f = File::open("./input-a/instances/ex4-n5_1.thop").expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let instance = parser::instance::parse(&contents);
        let super_file = Instance::new(&instance);

        assert_eq!(super_file.get_distance(&1, &1), 0);
        assert_eq!(super_file.get_distance(&1, &2), 5);
        assert_eq!(super_file.get_distance(&2, &1), 5);
        assert_eq!(super_file.get_distance(&1, &3), 6);
        assert_eq!(super_file.get_distance(&3, &1), 6);
        assert_eq!(super_file.get_distance(&1, &4), 8);
        assert_eq!(super_file.get_distance(&4, &1), 8);
        assert_eq!(super_file.get_distance(&2, &3), 8);
        assert_eq!(super_file.get_distance(&3, &2), 8);
        assert_eq!(super_file.get_distance(&2, &4), 6);
        assert_eq!(super_file.get_distance(&4, &2), 6);
        assert_eq!(super_file.get_distance(&3, &4), 5);
        assert_eq!(super_file.get_distance(&4, &3), 5);

        let mut h1 = HashMap::new();
        h1.insert(1, true);
        h1.insert(2, true);

        let mut h2 = HashMap::new();
        h2.insert(3, true);
        h2.insert(4, true);
        h2.insert(5, true);
        assert_eq!(super_file.visit_city(2, &h1), (5, 50, 2));
        assert_eq!(super_file.visit_city(3, &h2), (5, 180, 3));

        assert_eq!(super_file.speed_descresc_per_weight(), 0.3);
    }
}
