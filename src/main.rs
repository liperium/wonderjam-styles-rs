use rand::seq::IteratorRandom;
use std::io::Write;

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
        "Dragon Squad",
        "TechNet",
        "ChockBar de Team",
        "Eirb GANG",
        "Résoskour",
        "LambdaPeanutButter",
        "Les Gros Nazes",
        "Monopoly McDo 3/10-6/11",
        "Sleepers",
        "S24",
        "BadTeam",
        "Les CatJams",
        "TFT Addict",
        "Unicorn",
        "Void",
        "Playmakers",
        "OscaroSolo",
        "Les Aventuriers d'Astram",
        "NoName",
        "Chicoutifli",
        "Sqaxva",
        "Arisia",
        "Cinq hop",
        "Les Trois Mousquetaires**",
        "LÉHCQTLPADNLPPQFLJERECA",
        "Les maitres du jeux",
        "Choccy Milk Studio",
        "Totema Studio",
        "Pain à l'ail",
    ];
    let output_folder = "output/";

    let files = ["rolls_wonderjam.csv", "re-rolls_wonderjam.csv"];
    println!("Don't forget to remove old file");

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
        for team in teams.iter() {
            let rand_easy_mods = easy_modifiers.iter().choose_multiple(&mut rng, 2);
            let rand_hard_mod = hard_modifiers.iter().choose(&mut rng).expect("Yolo");
            writeln!(file,
                "{};{};{};{}",team,rand_easy_mods[0],rand_easy_mods[1],rand_hard_mod)
                .expect("Can't write file");
        }
    }
    
    println!("Rolls finished, open the .csv files with Excel and the separator is ';'");
}
