use regex::Regex;

pub fn day_1(input: &str) -> u32 {
  let mut landings_on_zero_count = 0;
  //let mut position: i8 = 50;
  let mut position: i32 = 50;
  let matcher = Regex::new(r"^([R|L])(\d+)").unwrap();

  for instruction in input.lines() {
      let caps = matcher.captures(instruction).unwrap();
      let direction = &caps[1];
      let mut steps_count: i32 = caps[2].parse().unwrap();

      if direction == 'R'.to_string() {
          steps_count = steps_count * -1
      }

      let unrefined_position = position as i32 + steps_count;

      if unrefined_position < 0 {
          let divided_position: f64 = unrefined_position as f64 / 100.0;
          let traversals_of_zero_count = divided_position.abs() as i32;

          position = unrefined_position + (100 * (if traversals_of_zero_count > 0 {traversals_of_zero_count} else {1}));
      } else {
          position = unrefined_position % 100;
      }

      if position == 0 {
        landings_on_zero_count += 1;
      }


      println!("hello {instruction} {position} | {steps_count} | {unrefined_position}");

      //if total_position < 0 {
      //  adjusted_position = total_position + (100 * if traversals_of_zero_count > 0 {traversals_of_zero_count} else {1})
      //} else {
      //  let mut divided_position: f64 = (total_position as f64 / 100.0);
      //  let remainder = ((divided_position - traversals_of_zero_count as f64) * 100.0).abs();
      //  adjusted_position = remainder.floor();
      //}

        //println!("hello {instruction} {position} | {total_position} | {traversals_of_zero_count} | {adjusted_position}");


      /*
      //TODO: it breaks when a left direction goes below zero, this logic is no good...
      let mut unrounded_position: i32 = position as i32 + steps_count;
      //if unrounded_position < 0 {
      //  unrounded_position += 100
      //}
      let mut divided_position: f64 = (unrounded_position as f64 / 100.0);

      //if divided_position == 0.0 {
      //    landings_on_zero_count += 1;
      //}

      let traversals_in_this_instruction = divided_position.floor() as u32;
      if divided_position < 0.0 {
        unrounded_position += (100 * traversals_in_this_instruction as i32)
      }
      let remainder = ((divided_position - traversals_in_this_instruction as f64) * 100.0).abs() as i8;

        //println!("hello {instruction} {position} | {unrounded_position} | {divided_position} | {traversals_in_this_instruction} | {remainder}");

      //traversals_of_zero_count += traversals_in_this_instruction;
      if remainder == 0 {
        landings_on_zero_count += 1;
      }
      position = remainder;
      */
  }

  return landings_on_zero_count;

}