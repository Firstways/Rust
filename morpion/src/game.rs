#[allow(unused_variables, unused_assignments)]
#[allow(unused_parens)]
pub mod morpion{
    // Initialise la grille du morpion
    pub fn InitialiseGrille()->[[char;3];3]{
        let mut tab = [['_';3];3];
        
        return tab;
    }
    // Affiche la grille du morpion
    pub fn AfficheGrille(tab : &mut [[char;3];3]){
        for i in 0..3 {
            println!("{:?} ",&tab[i]);
        }
    }
    // Verifie si il y'a 3 ronds ou 3 croix dans la ligne
    pub fn VerifLine(tab : &mut [[char;3];3])->bool{
        for i in 0..3 {
            let joueur1 =['x';3];
            let joueur2=['o';3];
               if (tab[i]== joueur1)||(tab[i]==joueur2){
                   return true; 
               }
        }
        return false
    }
    // Verifie si il y'a 3 ronds ou 3 croix dans la colonne
    pub fn VerifColonne(valeurCellule:char,tab : &mut [[char;3];3])->bool{
        if (tab[0][0]==valeurCellule)&&(tab[0][1]==tab[0][2])&&(tab[0][1]==tab[0][0]){
            return true;
        }
        if(tab[1][0]==valeurCellule)&&(tab[1][1]==tab[1][2])&&(tab[1][1]==tab[1][0]){
            return true;
        }
        if(tab[2][0]==valeurCellule)&&(tab[2][1]==tab[1][2])&&(tab[2][1]==tab[1][0]){
            return true;
        }
        return false;
    }
    // Verifie si il y'a 3 ronds ou 3 croix dans la diagonale ascendante
    pub fn VerifDiagonaleAscendante(valeurCellule:char,tab : &mut [[char;3];3])->bool{
        if(tab[0][2]==valeurCellule)&&(tab[1][1]==tab[2][0])&&(tab[1][1]==tab[0][2]){
            return true;
        }
        false
    }
    // Verifie si il y'a 3 ronds ou 3 croix dans la descendante
    pub fn VerifDiagonaleDescendante(valeurCellule:char,tab : &mut [[char;3];3])->bool{
        if(tab[0][0]==valeurCellule)&&(tab[1][1]==tab[2][2])&&(tab[1][1]==tab[0][0]){
            return true;
        }
        false
    }
    //Verifie que la cellule est libre
    pub fn VerifCellule(ValeurEnI:usize,ValeurEnJ:usize,tab : &mut [[char;3];3]){
        // to do
    }
    // pose le symbole 'X' dans tab
    pub fn PoseSymboleX(ValeurEnI:usize,ValeurEnJ:usize,tab : &mut [[char;3];3]){
        tab[ValeurEnI][ValeurEnJ]='x';
    }
    // pose le symbole 'O' dans tab
    pub fn PoseSymboleO(ValeurEnI:usize,ValeurEnJ:usize,tab : &mut [[char;3];3]){
        tab[ValeurEnI][ValeurEnJ]='0';
    }
    pub fn SaisieNumeroLigne()->usize{
        println!("merci de saisir le numéro de ligne");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let num = input.trim().parse::<usize>().unwrap();
        return num;
    }
    pub fn SaisieNumeroColonne()->usize{
        println!("merci de saisir le numéro de Colonne");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let num = input.trim().parse::<usize>().unwrap();
        return num;
    }
    pub fn TestFinDuJeu(valeurAsaisir:char,tab : &mut [[char;3];3])->bool{
        if (VerifLine(tab)==true){return true;}
        if (VerifColonne(valeurAsaisir,tab)==true){return true;}
        if (VerifDiagonaleAscendante(valeurAsaisir,tab)==true){return true;}
        if (VerifDiagonaleDescendante(valeurAsaisir,tab)==true){return true;}
        return false;
    }
    pub fn VerifSaisie(val:usize)-> bool{
        if (val < 0 || val > 2) {
            return false
        }
        return true;
    }
    pub fn jouer () {
        let mut grille= InitialiseGrille();
        let mut nombreDeCoup = 0;
        let mut estFinie:bool = false;
        let mut tour = 1;
        let mut valeurEnI =3;
        let mut valeurEnJ= 3;
        InitialiseGrille();
        let mut pos = ' ';
        while (estFinie || nombreDeCoup<9 ){
            AfficheGrille(&mut grille);
            if (tour == 1){
               println!("C'est au joueur 1 ");
                valeurEnI = SaisieNumeroColonne();
                valeurEnJ = SaisieNumeroLigne();
                while (!VerifSaisie(valeurEnI)||!VerifSaisie(valeurEnJ)) {
                    if (!VerifSaisie(valeurEnI)){valeurEnI = SaisieNumeroColonne();}
                    if (!VerifSaisie(valeurEnJ)){valeurEnI = SaisieNumeroLigne();}
                }
                PoseSymboleO(valeurEnI,valeurEnJ,&mut grille);
                estFinie=TestFinDuJeu('0',&mut grille);
            }else if (tour == 2){
                println!("C'est au joueur 2 ");
                valeurEnI = SaisieNumeroColonne();
                valeurEnJ = SaisieNumeroLigne();
                while (!VerifSaisie(valeurEnI)||!VerifSaisie(valeurEnJ)) {
                    if (!VerifSaisie(valeurEnI)){valeurEnI = SaisieNumeroColonne();}
                    if (!VerifSaisie(valeurEnJ)){valeurEnI = SaisieNumeroLigne();}
                }
                PoseSymboleX(valeurEnI,valeurEnJ,&mut grille);
                estFinie=TestFinDuJeu('x',&mut grille);
            }
            if (tour == 1){
                tour = 2;
            }else{
                tour = 1;
            }
        }
    }
}
