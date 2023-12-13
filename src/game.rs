use std::collections::HashMap;

#[derive(Debug)]
pub struct Backend {
	locations: HashMap<String, Vec<String>>,
	player_location: String,
	inventory: Vec<String>,
	item_map: HashMap<String, Vec<String>>,
	move_to_desc: HashMap<String, String>,
}

impl Backend{
	pub fn new() -> Self {
		let locations = HashMap::from([
			(String::from("Path"), Vec::from([String::from("Run-downPath"), String::from("SouthWoods"), String::from("NorthWoods")])),
			(String::from("Run-downPath"), Vec::from([String::from("Path"), String::from("BeatenPath"), String::from("NorthWoods"), String::from("RustyPickup"), String::from("TheHole")])),
			(String::from("BeatenPath"), Vec::from([String::from("Run-downPath"), String::from("Cabin"), String::from("RustyPickup"), String::from("TallGrass")])),
			(String::from("RustyPickup"), Vec::from([String::from("Run-downPath"), String::from("BeatenPath"), String::from("Backyard"), String::from("NorthWoods")])),
			(String::from("TallGrass"), Vec::from([String::from("BeatenPath"), String::from("TheHole"), String::from("Backyard")])),
			(String::from("TheHole"), Vec::from([String::from("Run-downPath"), String::from("TallGrass"), String::from("SouthWoods")])),
			(String::from("Backyard"), Vec::from([String::from("RustyPickup"), String::from("TallGrass")])),
			(String::from("NorthWoods"), Vec::from([String::from("RustyPickup"), String::from("Run-downPath"), String::from("Path")])),
			(String::from("SouthWoods"), Vec::from([String::from("TheHole"), String::from("Path")])),
			(String::from("Cabin"), Vec::from([String::from("BeatenPath")])),
		]);
		let move_to_desc = HashMap::from([
			(String::from("Path"), (String::from("You find yourself on a path surrounded by woods to your north and south, to the east the path continues to the cabin, to the west your escape from this wretched place."))),
			(String::from("Run-downPath"), (String::from("You move along to find a run-down path. To the north you can see the woods make way to a clearing containing a rusty pickup truck, to the south you see a trange clearing with a hole, to the west the path continues, to the east the path continues to the contemptible cabin."))),
			(String::from("BeatenPath"), (String::from("You are on a beaten shoddy path. To the north you see a rusty pickup truck, to the south you see a large swath of tall grass,to the west the path continues, to the east the vile cabin."))),
			(String::from("RustyPickup"), (String::from("You approach a pickup too old and worndown for use. to the east the backyard of the detestable cabin, to the west the woods, to the south a rundown path and a beaten path."))),
			(String::from("TallGrass"), (String::from("You move into a patch of tall grass that goes up to your chest. To the north is a beaten path, to the west the grass gives way to a clearing with a hole, to the east the backyard of the repulsive cabin,."))),
			(String::from("TheHole"), (String::from("You approach a small clearing with the only noteworthy feature being a hole in the ground. To the north is a run-down path, to the east a patch of tall grass, to the west the woods."))),
			(String::from("Backyard"), (String::from("You slink around the cabin to find a large clearing behind it, To the east there is a clearing with a rusty pickup and an area od tall grass."))),
			(String::from("NorthWoods"), (String::from("You enter a fairly lit area of the woods. to the south a path and a run-down path, to the east the woods give way to a clearing with a rusty pickup."))),
			(String::from("SouthWoods"), (String::from("You enter a dimly lit area of the woods. You are barely able to see a path to the north, and a clearing with a hole to the east."))),
			(String::from("Cabin"), (String::from("You enter the horrid building with every fiber of your being screaming for you to leave as soon as possible.")))
        ]);
		let player_location = String::from("Path");
		let item_map = HashMap::from([
			(String::from("NorthWoods"), Vec::from([String::from("Charm")])),
			(String::from("RustyPickup"), Vec::from([String::from("Antivenom")])),
			(String::from("Backyard"), Vec::from([String::from("Knife"), String::from("TruckKey"), String::from("Eggs")])),
			(String::from("Cabin"), Vec::from([String::from("Mcguffin")])),
		]);
		Self {
			locations: locations,
			player_location: player_location,
			inventory: Vec::from([String::from("Flashlight"), String::from("Phone"), String::from("Wallet")]),
			item_map: item_map,
			move_to_desc: move_to_desc,
		}
	}

	pub fn send_command(&mut self, command: &str) -> String {
		let command = command.to_lowercase();
		let words: Vec<&str> = command.split_whitespace().collect();
		match words[0] {
			"long" => {
				return String::from("You are in a dark forest. You can go north, south, east, or west. You are in a dark forest. You can go north, south, east, or west. You are in a dark forest. You can go north, south, east, or west. You are in a dark forest. You can go north, south, east, or west. You are in a dark forest. You can go north, south, east, or west. You are in a dark forest. You can go north, south, east, or west. ");
			},
			"go" => {
				if words.len() < 2 {
					return String::from("Go where?");
				}
				match words[1]{
					"to" => {
						if words.len() < 3 {
							return String::from("Go where?");
						}
						let destination = words[2];
						return self.move_to(destination.to_string());
					},
					_ => {
						return String::from("I don't understand.");
					},
				}
			},
			"inventory" => {
				if self.inventory.len() == 0 {
					return String::from("You have nothing.");
				}
				return format!("You have {}.", self.inventory.join(", "));
			},
			"take" => {
				if words.len() < 2 {
					return String::from("Take what?");
				}
				if self.take(String::from(words[1])) {
					return format!("You take the {}.", words[1]);
				}
				return format!("You don't see a {}.", words[1]);
			},
			"drop" => {
				if words.len() < 2 {
					return String::from("Drop what?");
				}
				if self.drop(String::from(words[1])) {
					return format!("You drop the {}.", words[1]);
				}
				return format!("You don't have a {}.", words[1]);
			},
			_ => {
				return String::from("I don't understand.");
			},
		}
	}

	fn move_to(&mut self, destination:String) -> String {
		if !self.locations.contains_key(&self.player_location) {
			return "You can't go there.".to_string();
		}
		let locations = self.locations.get(&self.player_location).unwrap();
		let lower_destination = destination.to_lowercase();
		if !locations.iter().any(|loc| loc.to_lowercase() == lower_destination) {
			return "You can't go there.".to_string();
		}
		let new_destination = locations.iter().find(|&loc| loc.to_lowercase() == lower_destination);
		match new_destination {
			Some(found_destination) => {
				self.player_location = found_destination.to_string();
			},
			None => {
				return "You can't go there.".to_string();
			}
		}
		return self.move_to_desc.get(&self.player_location).unwrap().to_string();
	}

	fn take(&mut self, item:String) -> bool {
		let lower_item = item.to_lowercase();
		if !self.item_map.contains_key(&self.player_location) {
			return false;
		}
		let items = self.item_map.get(&self.player_location).unwrap();
		if !items.iter().any(|i| i.to_lowercase() == lower_item) {
			return false;
		}
		let new_item = items.iter().find(|&i| i.to_lowercase() == lower_item);
		match new_item {
			Some(found_item) => {
				self.inventory.push(found_item.to_string());
				let index = items.iter().position(|i| i.to_lowercase() == lower_item).unwrap();
				self.item_map.get_mut(&self.player_location).unwrap().remove(index);
			},
			None => {
				return false;
			}
		}
		return true;
	}

	fn drop(&mut self, item:String) -> bool {
		let lower_item = item.to_lowercase();
		if !self.inventory.iter().any(|i| i.to_lowercase() == lower_item) {
			return false;
		}
		let new_item = self.inventory.iter().find(|&i| i.to_lowercase() == lower_item);
		match new_item {
			Some(found_item) => {
				if !self.item_map.contains_key(&self.player_location) {
					self.item_map.insert(self.player_location.clone(), Vec::new());
				}
				self.item_map.get_mut(&self.player_location).unwrap().push(found_item.to_string());
				let index = self.inventory.iter().position(|i| i.to_lowercase() == lower_item).unwrap();
				self.inventory.remove(index);
				return true;
			},
			None => {
				return false;
			}
		}
	}

	pub fn get_location(&self) -> &str {
		return self.player_location.as_str();
	}

	pub fn get_inventory(&self) -> &Vec<String> {
		return &self.inventory;
	}

}
