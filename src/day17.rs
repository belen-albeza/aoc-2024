use aoc_runner_derive::aoc;
use regex::Regex;

mod vm;
use vm::VM;

impl From<&str> for VM {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"\d+").unwrap();
        let sections = value.split("\n\n").collect::<Vec<&str>>();

        let registers: Vec<u32> = sections[0]
            .lines()
            .map(|l| re.find(l).unwrap().as_str().parse().unwrap())
            .collect();

        let rom: Vec<u8> = re
            .find_iter(sections[1])
            .map(|x| x.as_str().parse().unwrap())
            .collect();

        Self::new(registers[..3].try_into().unwrap(), rom)
    }
}

#[aoc(day17, part1)]
fn part1(input: &str) -> String {
    let mut vm = VM::from(input);
    vm.run();

    vm.output()
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[aoc(day17, part2)]
fn part2(input: &str) -> u32 {
    let mut res = 0;
    let mut rom: Option<Vec<u32>> = None;

    loop {
        let mut vm = VM::from(input);
        if rom.is_none() {
            rom = Some(vm.rom().into_iter().map(|x| x as u32).collect());
        }

        vm.set_register(0, res);
        vm.run();

        if Some(vm.output()) == rom {
            break;
        }

        res += 1;
    }

    res as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const INPUT2: &str = r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT1), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT2), 117440);
    }
}
