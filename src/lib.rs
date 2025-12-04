use regex::Regex;

pub fn day_1(input: &str) -> u32 {
    let mut landings_on_zero_count = 0;
    let mut position: i32 = 50;
    let matcher = Regex::new(r"^([R|L])(\d+)").unwrap();

    for instruction in input.lines() {
        let caps = matcher.captures(instruction).unwrap();
        let direction = &caps[1];
        let mut steps_count: i32 = caps[2].parse().unwrap();

        if direction == 'R'.to_string() {
            steps_count = -steps_count
        }

        let unrefined_position = position + steps_count;

        if unrefined_position < 0 {
            let divided_position: f64 = unrefined_position as f64 / 100.0;
            let traversals_of_zero_count = divided_position.abs() as i32;

            position = unrefined_position
                + (100
                    * (if traversals_of_zero_count > 0 {
                        traversals_of_zero_count
                    } else {
                        1
                    }));
        } else {
            position = unrefined_position % 100;
        }

        if position == 0 {
            landings_on_zero_count += 1;
        }
    }

    landings_on_zero_count
}
