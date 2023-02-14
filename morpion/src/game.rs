#[allow(unused_variables, unused_assignments)]
#[allow(unused_parens)]
#[warn(unused_comparisons)]
pub mod morpion
{
    // Initialise la grille du morpion
    pub fn initialise_grille()->[[char;3];3]{
        [['_';3];3]
    }
    // Affiche la grille du morpion
    pub fn affiche_grille(tab : &mut [[char;3];3]){
        for i in 0..3 {
            println!("{:?} ",&tab[i]);
        }
    }
    // Verifie si il y'a 3 ronds ou 3 croix dans la ligne
    pub fn verif_line(tab : &mut [[char;3];3])->bool{
        let joueur1 =['x';3];
        let joueur2=['0';3];
       
        for i in 0..3 {
               if (tab[i]== joueur1)||(tab[i]==joueur2){
                   return true; 
               }
        }
        false
    }
    // Verifie si il y'a 3 ronds ou 3 croix dans la colonne
    pub fn verif_colonne(valeur_cellule:char,tab : &mut [[char;3];3])->bool{
        for i in 0..3{
            if (tab[0][i]==valeur_cellule)&&(tab[1][i]==tab[2][i])&&(tab[1][i]==tab[0][i]){
                return true;
            }
        }
        false
    }
    // Verifie si il y'a 3 ronds ou 3 croix dans la diagonale ascendante
    pub fn verif_diagonale_ascendante(valeur_cellule:char,tab : &mut [[char;3];3])->bool{
        tab[0][2]==valeur_cellule&&tab[1][1]==tab[2][0]&&tab[1][1]==tab[0][2]
        
    }
    // Verifie si il y'a 3 ronds ou 3 croix dans la descendante
    pub fn verif_diagonale_descendante(valeur_cellule:char,tab : &mut [[char;3];3])->bool{
         tab[0][0]==valeur_cellule && tab[1][1]==tab[2][2]&&tab[1][1]==tab[0][0]
    }
    //Verifie que la cellule est libre
    pub fn verif_cellule(valeur_en_i:usize,valeur_en_j:usize,tab : &mut [[char;3];3])->bool{
         tab[valeur_en_i][valeur_en_j]=='_'
    }
    // pose le symbole 'X' dans tab
    pub fn pose_symbole_x(valeur_en_i:usize,valeur_en_j:usize,tab : &mut [[char;3];3]){
        tab[valeur_en_i][valeur_en_j]='x';
    }
    // pose le symbole 'O' dans tab
    pub fn pose_symbole_o(valeur_en_i:usize,valeur_en_j:usize,tab : &mut [[char;3];3]){
        tab[valeur_en_i][valeur_en_j]='0';
    }
    // Retourne le numero saisie par l'utilisateur
    pub fn saisie_numero_ligne()->usize{
        println!("merci de saisir le numéro de ligne");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let num = input.trim().parse::<usize>().unwrap();
        num
    }
    // Retourne le numero saisie par l'utilisateur
    pub fn saisie_numero_colonne()->usize{
        println!("merci de saisir le numéro de Colonne");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let num = input.trim().parse::<usize>().unwrap();
        num
    }
    // Test si le jeu est fini
    pub fn test_fin_du_jeu(valeur_a_saisir:char,tab : &mut [[char;3];3])->bool{
        if verif_line(tab){return true;}
        if verif_colonne(valeur_a_saisir,tab){return true;}
        if verif_diagonale_ascendante(valeur_a_saisir,tab){return true;}
        if verif_diagonale_descendante(valeur_a_saisir,tab){return true;}
        false
    }
    // Verifie que l'utilisateur saisie bien un entier entre 0 et 2
    pub fn verif_saisie(val:usize)-> bool{
        if !(0..=2).contains(&val) {
            println!("Merci de saisir un Entier en 0 et 2");
            return false;
        }
         true
    }
    
    pub fn jouer () 
    {
        let mut grille= initialise_grille();
        let nombre_de_coup = 0;
        let mut est_fini:bool = false;
        let mut tour = 1;
        let mut valeur_en_i =3;
        let mut valeur_en_j= 3;
        initialise_grille();
        while !est_fini && nombre_de_coup<9 
        {
            affiche_grille(&mut grille);
            if tour == 1 {
                println!("C'est au joueur 1 ");
                valeur_en_i = saisie_numero_ligne();
                valeur_en_j = saisie_numero_colonne();
                while (!verif_saisie(valeur_en_i) || !verif_saisie(valeur_en_j)) || !verif_cellule(valeur_en_i, valeur_en_j, &mut grille) {
                    if !verif_saisie(valeur_en_i) || !verif_cellule(valeur_en_i, valeur_en_j, &mut grille){
                        println!("la cellule est déjà prise, recommencez");
                        valeur_en_i = saisie_numero_colonne();
                    }
                    if !verif_saisie(valeur_en_j) || !verif_cellule(valeur_en_i, valeur_en_j, &mut grille) {
                        println!("la cellule est déjà prise, recommencez");
                        valeur_en_j = saisie_numero_ligne();
                    }
                }
                pose_symbole_o(valeur_en_i,valeur_en_j,&mut grille);
                est_fini=test_fin_du_jeu('0',&mut grille);
            }else if tour == 2{
                println!("C'est au joueur 2 ");
                valeur_en_i = saisie_numero_ligne();
                valeur_en_j = saisie_numero_colonne();
                while (!verif_saisie(valeur_en_i)||!verif_saisie(valeur_en_j))||!verif_cellule(valeur_en_i,valeur_en_j,&mut grille) {
                    if !verif_saisie(valeur_en_i)||!verif_cellule(valeur_en_i,valeur_en_j,&mut grille){
                        println!("la cellule est déjà prise, recommencez");
                        valeur_en_i = saisie_numero_colonne();
                    }
                    if !verif_saisie(valeur_en_j)||!verif_cellule(valeur_en_i,valeur_en_j,&mut grille){
                        println!("la cellule est déjà prise, recommencez");
                        valeur_en_j = saisie_numero_ligne();
                    }
                }   
                
                pose_symbole_x(valeur_en_i,valeur_en_j,&mut grille);
                est_fini=test_fin_du_jeu('x',&mut grille);
            }
            if tour == 1{
                tour = 2;
            }else{
                tour = 1;
            }
            if est_fini{
                if tour == 1{
                    println!("le joueur 1 a gagner")
                }else{ 
                    println!("le joeur 2 a gagner")
                }
                    affiche_grille(&mut grille)
            }
        }
    }
}

