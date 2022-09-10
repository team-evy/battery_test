/// Copyright 2022 team-evy
/// 
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
/// 
///     http://www.apache.org/licenses/LICENSE-2.0
/// 
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

fn main() -> Result<(), battery::Error> {
    let battery_manager = battery::Manager::new()?;

    for (i, maybe_battery) in battery_manager.batteries()?.enumerate() {
        let bat = maybe_battery?;
        println!("battery number {}", i);
        println!("state of charge: {:?}", bat.state_of_charge());
        println!("state of health: {:?}", bat.state_of_health());
        println!("current state: {:?}", bat.state());
        println!("temperature: {:?}", bat.temperature());
        println!("time to full (applies when charging): {:?}", bat.time_to_full());

        println!("\nFULL DETAILS:\n{:#?}", bat);
    }

    Ok(())
}
