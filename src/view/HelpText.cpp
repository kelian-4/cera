#include "view/HelpText.hpp"

namespace cera::view {

std::string shortHelpLine() {
    return "^S sauver  ^Q quitter  ^F chercher  ^R remplacer  ^Z annuler  "
           "^Y retablir  ^H aide  ^D doc";
}

const std::vector<HelpCategory>& fullDocumentation() {
    static const std::vector<HelpCategory> kDoc = {
        {
            "Fichier",
            {
                {"Ctrl-S", "Sauvegarder le fichier courant"},
                {"Ctrl-Q", "Quitter Cera (demande confirmation si modifications non sauvegardees)"},
            },
        },
        {
            "Edition",
            {
                {"Ctrl-Z", "Annuler la derniere action"},
                {"Ctrl-Y", "Retablir l'action annulee"},
                {"Ctrl-X", "Couper la selection vers le presse-papier"},
                {"Ctrl-C", "Copier la selection vers le presse-papier"},
                {"Ctrl-V", "Coller le contenu du presse-papier"},
            },
        },
        {
            "Recherche",
            {
                {"Ctrl-F", "Rechercher dans le fichier (fleches pour naviguer, Echap pour quitter)"},
                {"Ctrl-R", "Rechercher-remplacer, avec confirmation ligne par ligne"},
            },
        },
        {
            "Navigation",
            {
                {"Ctrl-L", "Aller a une ligne precise"},
                {"Ctrl-N", "Passer au tampon (fichier ouvert) suivant"},
                {"Ctrl-P", "Passer au tampon (fichier ouvert) precedent"},
                {"Fleches", "Deplacer le curseur"},
                {"Ctrl-Fleche", "Etendre la selection"},
                {"Page haut/bas", "Defiler d'un ecran"},
                {"Origine/Fin", "Aller au debut/a la fin de la ligne"},
            },
        },
        {
            "Aide",
            {
                {"Ctrl-H", "Afficher/masquer le bandeau d'aide en bas de l'ecran"},
                {"Ctrl-D", "Afficher cette page de documentation complete"},
                {"Echap", "Revenir a l'edition du fichier"},
            },
        },
    };
    return kDoc;
}

}
