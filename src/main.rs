use rand::seq::IteratorRandom;
use std::io::Write;

struct Roll{
    team:String,
    rolls:Vec<String>,
}

fn containsXSameRolls(originalRolls:&Roll,mods:Vec<String>,numberOf:usize)->bool{
    let mut count = 0;
    for roll in originalRolls.rolls.iter(){
        if mods.contains(roll){
            count += 1;
        }
    }
    count >= numberOf
}

fn main() {
    let hard_modifiers = [
        "Mon..Ami? - Ton allié est aussi ton ennemie", 
        "Check mon char - Contrôler un véhicule, autre que le personnage principale",
        "Monte le son - Basé sur de la Musique", 
        "Je veux ma part - système de quête", 
        "Maniaque - Fou/Horreur/Hallucinations", 
        "Un jeu dans le jeu", 
    ];
    let easy_modifiers = [
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
    let teams = [
        "Maître du jeux",
        "Ornithogwak",
        "GameDev4Fun",
        "SoloQ",
        "S24",
        "Unicorn",
        "Scala Team",
        "Diamant noir",
        "Monkey Squad",
        "5269636b204173746c6579a",
        "Coalition Anti-Gandhi",
        "Les Légendeux",
        "TBD",
        "lesPoutitsChampignons",
        "Jam Tonic",
        "TechNet",
        "Toaster Cat",
        "Nom nom",
        "Gaz a lighter",
        "Chicoutifli",
        "Bruncheux",
        "Sleepers",
        "Palaref Games",
        "Les gros nazes",
        "[insert name here]",
        "Totema Studio",
        "Propulsion-Saké",
        "Ubilly"
    ];
    let output_folder = "output/";

    let files = ["rolls_wonderjam.csv", "re-rolls_wonderjam.csv"];
    println!("Don't forget to remove old file");

    let mut teams_first_rolls:Vec<Roll> = vec![];
    let mut first_run = true;

    println!("--------- Starting Generation ----------");
    for file in files.iter() {
        let file_path = output_folder.to_owned()+file.to_owned();
        std::fs::remove_file(file_path.clone()).unwrap_or(());
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .append(true)
            .open(file_path).expect("Can't open file");
        let mut  rng = rand::thread_rng();
        writeln!(file,
            "{};{};{};{}","Équipe","Style 1", "Style 2", "Style 3")
            .expect("Can't write file");
        for (i,team) in teams.iter().enumerate() {
            
            let mut rand_easy_mods = easy_modifiers.iter().choose_multiple(&mut rng, 2);
            let mut rand_hard_mod = hard_modifiers.iter().choose(&mut rng).expect("Yolo");

            if first_run{
                let roll = Roll{team:team.to_string(),rolls:vec!(rand_easy_mods[0].to_string(),rand_easy_mods[1].to_string(),rand_hard_mod.to_string())};
                teams_first_rolls.push(roll);
            }else {
                while containsXSameRolls(&teams_first_rolls[i],vec!(rand_easy_mods[0].to_string(),rand_easy_mods[1].to_string(),rand_hard_mod.to_string()),2){
                    rand_easy_mods = easy_modifiers.iter().choose_multiple(&mut rng, 2);
                    rand_hard_mod = hard_modifiers.iter().choose(&mut rng).expect("Yolo");
                }
            }
            writeln!(file,
                "{};{};{};{}",team,rand_easy_mods[0],rand_easy_mods[1],rand_hard_mod)
                .expect("Can't write file");
        }
        first_run = false;
    }
    
    println!("Rolls finished, open the .csv files with Excel and the separator is ';'");
}
