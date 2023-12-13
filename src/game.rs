use std::collections::HashMap;

#[derive(Debug)]
pub struct Backend {
	locations: HashMap<String, Vec<String>>,
	player_location: String,
	inventory: Vec<String>,
	item_map: HashMap<String, Vec<String>>,
}

impl Backend{
	pub fn new() -> Self {
		let locations = HashMap::from([
			(String::from("Path1"), Vec::from([String::from("Path2"), String::from("WoodsS"), String::from("WoodsN")])),
			(String::from("Path2"), Vec::from([String::from("Path1"), String::from("Path3"), String::from("WoodsN"), String::from("RustyPickup"), String::from("TheHole")])),
			(String::from("Path3"), Vec::from([String::from("Path2"), String::from("Cabin"), String::from("RustyPickup"), String::from("TallGrass")])),
			(String::from("RustyPickup"), Vec::from([String::from("Path2"), String::from("Path3"), String::from("Backyard"), String::from("WoodsN")])),
			(String::from("TallGrass"), Vec::from([String::from("Path3"), String::from("TheHole"), String::from("Backyard")])),
			(String::from("TheHole"), Vec::from([String::from("Path2"), String::from("TallGrass"), String::from("WoodsS")])),
			(String::from("Backyard"), Vec::from([String::from("RustyPickup"), String::from("TallGrass")])),
			(String::from("WoodsN"), Vec::from([String::from("RustyPickup"), String::from("Path2"), String::from("Path1")])),
			(String::from("WoodsS"), Vec::from([String::from("TheHole"), String::from("Path1")])),
			(String::from("Cabin"), Vec::from([String::from("Path3")])),
		]);
		let player_location = String::from("Path1");
		let item_map = HashMap::from([
			(String::from("WoodsN"), Vec::from([String::from("Charm")])),
			(String::from("RustyPickup"), Vec::from([String::from("Antivenom")])),
			(String::from("Backyard"), Vec::from([String::from("Knife"), String::from("TruckKey"), String::from("Eggs")])),
			(String::from("Cabin"), Vec::from([String::from("Mcguffin")])),
		]);
		Self {
			locations: locations,
			player_location: player_location,
			inventory: Vec::from([String::from("Flashlight"), String::from("Phone"), String::from("Wallet")]),
			item_map: item_map,
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
						if !self.move_to(String::from(destination)) {
							return String::from("You can't go there.");
						}
						return format!("You go to {}.", destination);
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

	fn move_to(&mut self, destination:String) -> bool {
		if !self.locations.contains_key(&self.player_location) {
			return false;
		}
		let locations = self.locations.get(&self.player_location).unwrap();
		let lower_destination = destination.to_lowercase();
		if !locations.iter().any(|loc| loc.to_lowercase() == lower_destination) {
			return false;
		}
		let new_destination = locations.iter().find(|&loc| loc.to_lowercase() == lower_destination);
		match new_destination {
			Some(found_destination) => {
				self.player_location = found_destination.to_string();
			},
			None => {
				return false;
			}
		}
		return true;
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