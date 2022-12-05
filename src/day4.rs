

#[derive(Copy, Clone, Debug)]
struct Elf {
    min: u32,
    max: u32
}

impl Elf {
    fn new(line: &str) -> Self {
        let mut fields = line.split("-");
        let min = fields.next().unwrap().parse().unwrap();
        let max = fields.next().unwrap().parse().unwrap();
        Elf {
            min,
            max
        }
    }

    fn is_included(&self, other: &Elf) -> bool {
        let result = self.min >= other.min && self.max <= other.max;


        result
    }
    fn have_verlap(&self, other: &Elf) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}

pub fn day4() {

    let mut counter_step1 = 0;
    let mut counter_step2 = 0;

    for line in include_str!("./day4.txt")
        .split("\n") {
        let mut elves_str = line.split(",");
        let elf1 = Elf::new(elves_str.next().unwrap());
        let elf2: Elf = Elf::new(elves_str.next().unwrap());
        if elf1.is_included(&elf2) || elf2.is_included(&elf1) {
            counter_step1 = counter_step1 + 1;
        }

        if elf1.have_verlap(&elf2) || elf2.have_verlap(&elf1) {
            counter_step2 = counter_step2 + 1;
        }

    }


    println!("Step 1 : {}", counter_step1);
    println!("Step 1 : {}", counter_step2);

}