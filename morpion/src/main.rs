#[allow(unused_variables, unused_assignments)]

mod game;
fn main() {
    game::morpion::jouer();
}
#[cfg(test)] 
mod tests {
    use crate::game::morpion::initialise_grille;
    use crate::game::morpion::verif_line;
    use crate::game::morpion::verif_colonne;
    use crate::game::morpion::verif_diagonale_ascendante;
    use crate::game::morpion::verif_diagonale_descendante;
    use crate::game::morpion::verif_cellule;
    use crate::game::morpion::pose_symbole_x;
    use crate::game::morpion::pose_symbole_o;
    use crate::game::morpion::verif_saisie;


    #[test]
    // test l'initialisation de la grille
    fn test_initialise_grille() {
        let tableau_vide = initialise_grille();
        let tableau_a_tester = [['_';3];3];
        assert_eq!(tableau_vide,tableau_a_tester);
    }
    #[test]
    // Teste une condition de victoire
    fn test_verif_premiere_ligne(){
        let mut array_2d: [[char; 3]; 3] = [['x', 'x', 'x'], ['_', '_', '_'], ['_', '_', '_']];
        assert_eq!(true,verif_line(&mut array_2d));
    }
    // Teste une condition de victoire
    #[test]
    fn test_verif_deuxieme_ligne() {
        let mut array_2d: [[char; 3]; 3] = [['_', '_', '_'],['x', 'x', 'x'], ['_', '_', '_']];
        assert_eq!(true,verif_line(&mut array_2d));
    }
    // Teste une condition de victoire
    #[test]
    fn test_verif_troisieme_ligne() {
        let mut array_2d: [[char; 3]; 3] = [['_', '_', '_'],['_', '_', '_'], ['x', 'x', 'x']];
        assert_eq!(true,verif_line(&mut array_2d));
    }
    // Teste une condition de victoire
    #[test]
    fn test_verif_premiere_colonne_x() {
        let symbole_joueur = 'x';
        let mut array_2d: [[char; 3]; 3] = [['x', '_', '_'],['x', '_', '_'], ['x', '_', '_']];
        assert_eq!(true,verif_colonne(symbole_joueur,&mut array_2d));
    }
    // Teste une condition de victoire
    fn test_verif_deuxieme_colonne_x() {
        let symbole_joueur = 'x';
        let mut array_2d: [[char; 3]; 3] = [['_', 'x', '_'],['_', 'x', '_'], ['_', 'x', '_']];
        assert_eq!(true,verif_colonne(symbole_joueur,&mut array_2d));
    }
    // Teste une condition de victoire
    fn test_verif_troisieme_colonne_x() {
        let symbole_joueur = 'x';
        let mut array_2d: [[char; 3]; 3] = [['_', 'x', '_'],['_', 'x', '_'], ['_', 'x', '_']];
        assert_eq!(true,verif_colonne(symbole_joueur,&mut array_2d));
    }
    // Teste une condition de victoire
    #[test]
    fn test_verif_diagonale_ascendante() {
        let symbole_joueur = 'x';
        let mut array_2d: [[char; 3]; 3] = [['_', '_', 'x'],['_', 'x', '_'], ['x', '_', '_']];
        assert_eq!(true,verif_diagonale_ascendante(symbole_joueur, &mut array_2d));
}
    // Teste une condition de victoire
    #[test]
    fn test_verif_diagonale_descendante() {
        let symbole_joueur = 'x';
        let mut array_2d: [[char; 3]; 3] = [['x', '_', '_'],['_', 'x', '_'], ['_', '_', 'x']];
        assert_eq!(true,verif_diagonale_descendante(symbole_joueur,&mut array_2d));
    }
    // Teste si la cellule selectionner est libre
    #[test]
    fn test_verif_cellule() {
        let ligne_a_tester=1;
        let colonne_a_tester =1;
        let mut array_2d: [[char; 3]; 3] = [['_', '_', '_'],['_', 'x', '_'], ['_', '_', '_']];
        assert_eq!(false,verif_cellule(ligne_a_tester,colonne_a_tester,&mut array_2d))
    }
    #[test]
    // Teste si la pose d'un symbole est correct
    fn test_pose_symbole_x() {
        let ligne_a_tester=1;
        let colonne_a_tester =1;
        let mut array_2d: [[char; 3]; 3] = [['_', '_', '_'],['_', 'x', '_'], ['_', '_', '_']];
        let mut tableau_a_tester: [[char; 3]; 3] = [['_', '_', '_'],['_', '_', '_'], ['_', '_', '_']];
        pose_symbole_x(ligne_a_tester,colonne_a_tester,&mut tableau_a_tester);
        assert!(array_2d==tableau_a_tester);
    }
    // Teste si la pose d'un symbole est correct

    #[test]
    fn test_pose_symbole_o() {
        let ligne_a_tester=1;
        let colonne_a_tester =1;
        let mut array_2d: [[char; 3]; 3] = [['_', '_', '_'],['_', '0', '_'], ['_', '_', '_']];
        let mut tableau_a_tester: [[char; 3]; 3] = [['_', '_', '_'],['_', '_', '_'], ['_', '_', '_']];

        pose_symbole_o(ligne_a_tester,colonne_a_tester,&mut tableau_a_tester);
        assert!(array_2d==tableau_a_tester);
    }
    // Teste si le nombre est en dehors des cases du tableau
    #[test]
    fn test_verif_saisie_plus_grand_que_deux() {
        let valeur_a_tester = 3;
        assert_eq!(false,verif_saisie(valeur_a_tester));
    }
}

