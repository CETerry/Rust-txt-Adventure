use std::collections::HashMap;

pub struct Response {
	pub response: String,
	pub status: Status,
}

#[derive(PartialEq)]
pub enum Status {
	Success,
	Failure,
	Exit,
	Terminate,
}

impl Response {
	pub fn new(response: String, status: Status) -> Self {
		Self {
			response: response,
			status: status,
		}
	}
	pub fn ok(response: String) -> Self {
		Self {
			response: response,
			status: Status::Success,
		}
	}

}

#[derive(Debug)]
pub struct Backend {
	locations: HashMap<String, Vec<String>>,
	player_location: String,
	inventory: Vec<String>,
	item_map: HashMap<String, Vec<String>>,
	move_to_desc: HashMap<String, String>,
    examine_desc: HashMap<String, String>,
	snek: bool,
}

impl Backend{
	pub fn new() -> Self {
		let locations = HashMap::from([
			(String::from("Path"), Vec::from([String::from("Rundown Path"), String::from("South Woods"), String::from("North Woods")])),
			(String::from("Rundown Path"), Vec::from([String::from("Path"), String::from("Beaten Path"), String::from("North Woods"), String::from("Rusty Pickup"), String::from("The Hole")])),
			(String::from("Beaten Path"), Vec::from([String::from("Rundown Path"), String::from("Cabin"), String::from("Rusty Pickup"), String::from("Tall Grass")])),
			(String::from("Rusty Pickup"), Vec::from([String::from("Rundown Path"), String::from("Beaten Path"), String::from("Backyard"), String::from("North Woods")])),
			(String::from("Tall Grass"), Vec::from([String::from("Beaten Path"), String::from("The Hole"), String::from("Backyard")])),
			(String::from("The Hole"), Vec::from([String::from("Rundown Path"), String::from("Tall Grass"), String::from("South Woods")])),
			(String::from("Backyard"), Vec::from([String::from("Rusty Pickup"), String::from("Tall Grass")])),
			(String::from("North Woods"), Vec::from([String::from("Rusty Pickup"), String::from("Rundown Path"), String::from("Path")])),
			(String::from("South Woods"), Vec::from([String::from("The Hole"), String::from("Path")])),
			(String::from("Cabin"), Vec::from([String::from("Beaten Path")])),
		]);
		let move_to_desc = HashMap::from([
			(String::from("Path"), (String::from("You find yourself on a path surrounded by woods to your north and south, to the east the path continues to the cabin."))),
			(String::from("Rundown Path"), (String::from("You move along to find a run-down path. To the north you can see the woods make way to a clearing containing a rusty pickup truck, to the south you see a trange clearing with a hole, to the west the path continues, to the east the path continues to the contemptible cabin."))),
			(String::from("Beaten Path"), (String::from("You are on a beaten shoddy path. To the north you see a rusty pickup truck, to the south you see a large swath of tall grass,to the west the path continues, to the east the vile cabin."))),
			(String::from("Rusty Pickup"), (String::from("You approach a pickup too old and worndown for use. to the east the backyard of the detestable cabin, to the west the woods, to the south a rundown path and a beaten path."))),
			(String::from("Tall Grass"), (String::from("You move into a patch of tall grass that goes up to your chest. To the north is a beaten path, to the west the grass gives way to a clearing with a hole, to the east the backyard of the repulsive cabin,."))),
			(String::from("The Hole"), (String::from("You approach a small clearing with the only noteworthy feature being a hole in the ground. To the north is a run-down path, to the east a patch of tall grass, to the west the woods."))),
			(String::from("Backyard"), (String::from("You slink around the cabin to find a large clearing behind it, To the east there is a clearing with a rusty pickup and an area od tall grass."))),
			(String::from("North Woods"), (String::from("You enter a fairly lit area of the woods. to the south a path and a run-down path, to the east the woods give way to a clearing with a rusty pickup."))),
			(String::from("South Woods"), (String::from("You enter a dimly lit area of the woods. You are barely able to see a path to the north, and a clearing with a hole to the east."))),
			(String::from("Cabin"), (String::from("You enter the horrid building with every fiber of your being screaming for you to leave as soon as possible.")))
        ]);
        let examine_desc = HashMap::from([
            (String::from("Path"), (String::from("There is nothing more to examine here."))),
			(String::from("Run-down Path"), (String::from("There is nothing more to examine here."))),
			(String::from("Beaten Path"), (String::from("The air here feels thick and heavyfrom the cabins malign influence. On the door to the cabin you notice cryptic glyphs etched into the wood as well as talismans made from sticks and small animal bones hanging in front of the door."))),
			(String::from("Rusty Pickup"), (String::from("After investigating the truck you notice that in the glovebox is a vial of antivenom."))),
			(String::from("Tall Grass"), (String::from("With how tall the grass is you can't help but think this would make a good hiding place."))),
			(String::from("The Hole"), (String::from("You get closer to the hole. Peering into it you see nothing but an inky blackness all the way down. Curiosity getting the better of you drop a rock from the surrounding area into it. You stand there waiting and waiting and waiting for you to hear it hit the bottom but it never does."))),
			(String::from("Backyard"), (String::from("Despite how large and spacious the cabin's backyard is there isn't much of note except a single tree with a knife stuck in it."))),
			(String::from("North Woods"), (String::from("After some exploring you come across a tiny hut. In that hut you see a snake gaurding a single well made charm."))),
			(String::from("South Woods"), (String::from("With how dark these woods are you think that you could easily get lost here, but then again so could something chasing you."))),
			(String::from("Cabin"), (String::from("You can feel the very air here is thick with evil influence from the book. You find the book on a desk with its page open inviting anyone to read what's inscribed within, but you know better than to do that.")))
        ]);
		let player_location = String::from("Path");
		let item_map = HashMap::from([
			(String::from("North Woods"), Vec::from([String::from("Charm")])),
			(String::from("Rusty Pickup"), Vec::from([String::from("Antivenom")])),
			(String::from("Backyard"), Vec::from([String::from("Knife")])),
			(String::from("Cabin"), Vec::from([String::from("Book")])),
		]);
		Self {
			locations: locations,
			player_location: player_location,
			inventory: Vec::from([String::from("Flashlight"), String::from("Phone"), String::from("Wallet")]),
			item_map: item_map,
			move_to_desc: move_to_desc,
             examine_desc: examine_desc,
			snek: true,
		}
	}

	pub fn send_command(&mut self, command: &str) -> Response {
		let command = command.to_lowercase();
		let words: Vec<&str> = command.split_whitespace().collect();
		match words[0] {
			"help" => {
				return Response::ok(self.help());
			},
			"quit" => {
				return Response::new(String::from("You have ended the game."), Status::Terminate);
			},
			"end" => {
				return Response::new(String::from("You have ended the game."), Status::Exit);
			}
			"go" => {
				if words.len() < 2 {
					return Response::ok(String::from("Go where?"));
				}
				match words[1]{
					"to" => {
						if words.len() < 3 {
							return Response::ok(String::from("Go where?"));
						}
						let mut destination = String::new();
						for i in 2..words.len() {
							destination.push_str(words[i]);
							destination.push_str(" ");
						}
						destination = destination.trim().to_string();
						if words[2] == "cabin" && !self.inventory.contains(&String::from("Charm")) {
							return Response::ok(String::from("As you approach you feel a wave of malaise hit you, and you seem to be unable to muster the energy to keep moving forward."));
						}
						return Response::ok(self.move_to(destination.to_string()));
					},
					_ => {
						return Response::ok(String::from("I don't understand."));
					},
				}
			},
			"inventory" => {
				if self.inventory.len() == 0 {
					return Response::ok(String::from("You have nothing."));
				}
				return Response::ok(format!("You have {}.", self.inventory.join(", ")));
			},
			"take" => {
				if words.len() < 2 {
					return Response::ok(String::from("Take what?"));
				}
				if self.player_location == String::from("North Woods") && words[1] == "charm" && self.snek {
					self.snek = false;
					if self.inventory.contains(&String::from("Antivenom")) {
						self.take(String::from(words[1]));
						self.inventory.remove(self.inventory.iter().position(|i| i.to_lowercase() == "antivenom").unwrap());
						return Response::ok(String::from("You try to take the charm, but the snake bites you. Fearing the effects you quickly consume the antivenom and feel better. No longer fearing the items dillegent gaurd you take the charm."));
					}
					return Response::new(String::from("You try to take the charm, but the snake bites. It only hurts for a moment until you feel dizzy, and start seeing the ground come up to greet you before everything goes black. Don't worry though it only hurt for a moment. GAME OVER"), Status::Exit);
				}
				if self.take(String::from(words[1])) {
					return Response::ok(format!("You take the {}.", words[1]));
				}
				return Response::ok(format!("You don't see a {}.", words[1]));
			},
			"drop" => {
				if words.len() < 2 {
					return Response::ok(String::from("Drop what?"));
				}
				if self.drop(String::from(words[1])) && self.player_location == String::from("The Hole"){
					if words[1] == "book" {
						return Response::new(String::from("You drop the book into the hole. Watching as it gets ever so smaller, and even waiting a few moments after it disappeared. On your way back home you feel a sense of relief knowing the world is free from its toxic influence. Well, free from it for now at least. YOU WIN"), Status::Exit);
					}
					return Response::ok(format!("You drop the {} into the hole and never hear it hit the bottom.", words[1]));
				}
				if self.drop(String::from(words[1])) {
					return Response::ok(format!("You drop the {}.", words[1]));
				}
				return Response::ok(format!("You don't have a {}.", words[1]));
			},
            "examine" => {
                if words.len() > 1 {
                    return Response::ok(String::from("I don't understand."));
                }
                else {
                    return Response::ok(self.examine().to_string());
                }
            }
            "hide" => {
                if words.len() > 1 {
                    return Response::ok(String::from("I don't understand."));
                }
                else {
                    return Response::ok(self.hide().to_string());
                }
            }
			_ => {
				return Response::ok(String::from("I don't understand."));
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
				if self.player_location != String::from("The Hole") {
					self.item_map.get_mut(&self.player_location).unwrap().push(found_item.to_string());
				}
				let index = self.inventory.iter().position(|i| i.to_lowercase() == lower_item).unwrap();
				self.inventory.remove(index);
				return true;
			},
			None => {
				return false;
			}
		}
	}

    fn examine(&mut self) -> String {
        if !self.item_map.contains_key(&self.player_location) {
			self.item_map.insert(self.player_location.clone(), Vec::new());
		}
        if !self.item_map.get(&self.player_location).unwrap().contains(&String::from("Book")) && self.player_location == String::from("Cabin"){
            return String::from("You feel as if the very essence of the world around you screams for you to put the book back.");
        }
        else if !self.item_map.get(&self.player_location).unwrap().contains(&String::from("Antivenom")) && self.player_location == String::from("Rusty Pickup"){
            return String::from("There is nothing else to examine here.");
        }
        else if !self.item_map.get(&self.player_location).unwrap().contains(&String::from("Knife")) && self.player_location == String::from("Backyard"){
            return String::from("There is nothing else to examine here.");
        }
        else if !self.item_map.get(&self.player_location).unwrap().contains(&String::from("Charm")) && self.player_location == String::from("North Woods"){
            return String::from("There is nothing else to examine here.");
        }
        else {
            return self.examine_desc.get(&self.player_location).unwrap().to_string(); 
        }
    }

    fn hide(&self) -> String {
        return String::from("You attempt to hide.");
    }
	pub fn get_location(&self) -> &str {
		return self.player_location.as_str();
	}

	pub fn get_inventory(&self) -> &Vec<String> {
		return &self.inventory;
	}

	pub fn get_exits(&self) -> Vec<String> {
		if !self.locations.contains_key(&self.player_location) {
			return Vec::new();
		}
		let locations = self.locations.get(&self.player_location).unwrap();
		let mut exits = Vec::new();
		for location in locations {
			exits.push(location.to_string());
		}
		return exits;
	}

	pub fn help(&self) -> String {
		return String::from("Commands:\n\tgo to <location>\n\tinventory\n\ttake <item>\n\tdrop <item>\n\t examine\n\t hide\n\tquit");
	}

}
