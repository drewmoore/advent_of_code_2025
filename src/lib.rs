use regex::Regex;

pub fn day_1(input: &str) -> u32 {
  let mut traversals_of_zero_count = 0;
  let mut position: i8 = 50;
  let matcher = Regex::new(r"^([R|L])(\d+)").unwrap();

  for instruction in input.lines() {
      let caps = matcher.captures(instruction).unwrap();
      let direction = &caps[1];
      let mut steps_count: i32 = caps[2].parse().unwrap();

      if direction == 'L'.to_string() {
          steps_count = steps_count * -1
      }

      let unrounded_position: i32 = position as i32 + steps_count;
      let divided_position: f64 = (unrounded_position as f64 / 100.0).abs();
      let traversals_in_this_instruction = divided_position.floor() as u32;
      let remainder = ((divided_position - traversals_in_this_instruction as f64) * 100.0) as i8;

      traversals_of_zero_count += traversals_in_this_instruction;
      if remainder == 0 {
        traversals_of_zero_count += 1;
      }
      position = remainder;
  }

  return traversals_of_zero_count;

}