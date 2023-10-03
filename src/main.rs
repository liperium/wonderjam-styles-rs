use rand::seq::IteratorRandom;
use std::io::Write;

fn main() {
    let hard_modifiers = [
        "Un jeu dans le jeu", 
        "Mon..Ami? - Ton allié est aussi ton ennemie", 
        "Maniaque - Fou/Horreur/Hallucinations", 
        "Je veux ma part - système de quête", 
        "Monte le son - Basé sur de la Musique", 
        "Check mon char - Contrôler un véhicule, autre que le personnage principale"
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
        "LÉHCQTLPADNLPPQFLJERECA",
        "Les maitres du jeux",
        "Les Gros Nazes",
        "Monopoly McDo 3/10-6/11",
        "Sleepers",
        "Choccy Milk Studio",
        "S24",
        "Totema Studio",
        "BadTeam",
        "Les CatJams",
        "TFT Addict",
        "Unicorn",
        "Void",
        "Playmakers",
        "OscaroSolo",
        "Les Aventuriers d'Astram",
        "Chicoutifli",
        "Sqaxva",
        "Arisia",
        "Cinq hop"
    ];
    println!("Don't forget to remove old file");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .append(true)
        .open("rolls_wonderjam.csv").expect("Can't open file");
    let mut  rng = rand::thread_rng();

    println!("--------- Starting Generation ----------");
    for team in teams.iter() {
        let rand_easy_mods = easy_modifiers.iter().choose_multiple(&mut rng, 2);
        let rand_hard_mod = hard_modifiers.iter().choose(&mut rng).expect("Yolo");
        writeln!(file,
            "{};{};{};{}",team,rand_easy_mods[0],rand_easy_mods[1],rand_hard_mod)
            .expect("Can't write file");
    }
    println!("Rolls finished, open rolls_wonderjam.csv with Excel and the separator is ';'");

}
