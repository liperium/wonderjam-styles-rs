use rand::seq::IteratorRandom;
use std::fs;
use std::io::Write;

struct Roll {
    team: String,
    rolls: Vec<String>,
}

const HARD_MODIFIERS: [&str; 6] = [
    "Mon..Ami? - Ton allié est aussi ton ennemie",
    "Check mon char - Contrôler un véhicule, autre que le personnage principale",
    "Monte le son - Basé sur de la Musique",
    "Je veux ma part - système de quête",
    "Maniaque - Fou/Horreur/Hallucinations",
    "Un jeu dans le jeu",
];

const EASY_MODIFIERS: [&str; 17] = [
    "Chacun pour soi",
    "Survivre un autre jour - Survival element",
    "LIBERTÉ  & DÉMOCRATIE - Guns, Guns, and Eagles",
    "Fin inattendue",
    "Hasard - Aléatoire/Pari",
    "Qui a éteint la lumière? - Moment de noirceur",
    "Inhumain - Personnage principale pas humain",
    "Versus",
    "Enigme",
    "Collectionneur - objets ramassables",
    "Un point pour moi - Genre Arcade",
    "Combine des choses - Fabrication/Chimie/Cuisine",
    "Dans les étoiles - Univers dans l’espace",
    "Aide moi mon pote! - Coop élément",
    "Recommence…Encore… - élément Rogue Lite",
    "Progression exponentielle - Les chiffres finaux doivent être énorme",
    "T’as vu ma nouvelle tenue? - Personnalisation du joueur",
];

const ROLL_FILES: [&str; 2] = ["rolls.csv", "re-rolls.csv"];
const TEAMS_INPUT_FILE: &str = "teams.txt";

fn main() {
    println!("Reading {}", TEAMS_INPUT_FILE);
    let all_teams = fs::read_to_string(TEAMS_INPUT_FILE).expect("Cannot read file");
    let teams = all_teams.lines();
    let output_folder = "output/";
    fs::create_dir(output_folder); //Creates dir if not existant

    let mut teams_first_rolls: Vec<Roll> = vec![];
    let mut first_run = true;

    println!("--------- Starting Generation ----------");

    for file in ROLL_FILES.iter() {
        let file_path = output_folder.to_owned() + file.to_owned();
        std::fs::remove_file(file_path.clone()).unwrap_or(());
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .append(true)
            .open(file_path)
            .expect("Can't open file");
        let mut rng = rand::thread_rng();
        writeln!(
            file,
            "{};{};{};{}",
            "Équipe", "Style 1", "Style 2", "Style 3 - Difficile"
        )
        .expect("Can't write file");
        for (i, team) in teams.clone().enumerate() {
            let mut rand_easy_mods = EASY_MODIFIERS.iter().choose_multiple(&mut rng, 2);
            let mut rand_hard_mod = HARD_MODIFIERS.iter().choose(&mut rng).expect("Yolo");

            if first_run {
                let roll = Roll {
                    team: team.to_string(),
                    rolls: vec![
                        rand_easy_mods[0].to_string(),
                        rand_easy_mods[1].to_string(),
                        rand_hard_mod.to_string(),
                    ],
                };
                teams_first_rolls.push(roll);
            } else {
                while contains_xsame_rolls(
                    &teams_first_rolls[i],
                    vec![
                        rand_easy_mods[0].to_string(),
                        rand_easy_mods[1].to_string(),
                        rand_hard_mod.to_string(),
                    ],
                    2,
                ) {
                    rand_easy_mods = EASY_MODIFIERS.iter().choose_multiple(&mut rng, 2);
                    rand_hard_mod = HARD_MODIFIERS.iter().choose(&mut rng).expect("Yolo");
                }
            }
            writeln!(
                file,
                "{};{};{};{}",
                team, rand_easy_mods[0], rand_easy_mods[1], rand_hard_mod
            )
            .expect("Can't write file");
        }
        first_run = false;
    }

    println!("--------- Finished Generation ----------");
    println!("Rolls finished, open the .csv files with Excel and the separator is ';' - semicolon");
}
// Checks if it contains the same rolls as the previous roll, and returns a bool if is over that number.
// This allows us to roll over bad rerolls, that can happen ( having the same re-roll as your original is not desirable )
fn contains_xsame_rolls(original_rolls: &Roll, mods: Vec<String>, number_of: usize) -> bool {
    let mut count = 0;
    for roll in original_rolls.rolls.iter() {
        if mods.contains(roll) {
            count += 1;
        }
    }
    count >= number_of
}
