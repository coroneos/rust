use std::io;
use std::fmt::Debug;

// struct Constitution {
//     name: String,
//     abbr: String,
//     value: u8,
// }
pub mod actors {
    // let Movement = [0; 2];

    enum Direction { N, NE, E, SE, S, SW, W, NW }

    enum Weapon {
		Axe,
		Boomerang,
		Bow,
		Gloves,
		Javelin,
		Spear,
		Sword,
		Whip,
	}

    pub struct Player {
        pub name: String,
        pub weapon: String,
        charisma: u8,
        constitution: u8,
        dexterity: u8,
        intelligence: u8,
        strength: u8,
        wisdom: u8,
        hp: u16,
        mp: u16,
        x: f32,
        y: f32,
    }

    impl Player {
        pub fn name() -> String {
	        println!("What is your name?");

	        let mut name = String::new();

			io::stdin().read_line(&mut name);

			let name: String = match name.trim().parse() {
				Ok(n) => n,
				Err(_) => continue,
			};

	        println!("Hello, {}.", name);
			name
		}

        pub fn r#move(d: Direction) /*-> [f32, f32]*/ {
            match d {
                Direction::N => [0.1, 0.0],
                Direction::NE => [0.1, 0.1],
                Direction::E => [0.0, 0.1],
                Direction::SE => [-0.1, 0.1],
                Direction::S => [-0.1, 0.0],
                Direction::SW => [-0.1, -0.1],
                Direction::W => [0.0, -0.1],
                Direction::NW => [0.1, -0.1],
            };
        }

		
        // fn reposition(c: []) {
		// 	println!("Moved to {1}, {2}", c[0], c[1]);
		// }

		#[derive(Debug)]
		pub fn attack(w: Weapon) {
			/*
			let mut weapon = "";

            match w {
				Weapon::Axe => weapon = "Axe",
				Weapon::Boomerang => weapon = "Boomerang",
				Weapon::Bow => weapon = "Bow",
				Weapon::Gloves => weapon = "Gloves",
				Weapon::Javelin => weapon = "Javelin",
				Weapon::Spear => weapon = "Spear",
				Weapon::Sword => weapon = "Sword",
				Weapon::Whip => weapon = "Whip",
			}

			println!("Attacked with {}", weapon);
			*/
			println!("Attacked with {:?}", w);
		}
    }
}