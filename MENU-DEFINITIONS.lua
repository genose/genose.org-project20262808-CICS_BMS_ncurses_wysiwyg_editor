-- -----------------------------
-- Editor Window Menu
-- -----------------------------
-- GUI utilise .initial et .edited selon le mode en cours, GUI utilise .default pour se renseigner et initialiser les champs de l'interface,
-- le constructeur objet prend TYPE pour initialiser le default de field_*.default = field_avail_*.default[ (TYPE) ],
-- le rendu utilise des template visuel en combinant les propriété chainé selon les conditions
-- ------------------------------

-- preview code / canvas render => ctrl+p  (swap view)
-- ================= separator =================
-- add field => ctrl+a ( list of available fields Objects, then select one to add , then open edit properties window(hangle pagging&scrolling), then to validate ctrl+enter, to cancel ctrl+esc)
-- edit selected => ctrl+e (open edit properties window(hangle pagging&scrolling), then to validate ctrl+enter, to cancel ctrl+esc)
-- delete selected => ctrl+d
-- ================ separator =================
-- open => ctrl+o, boite de dialogue avec avec FS complet, naiviger dans le FS, filtrer par extension, selectionner un fichier, valider ctrl+enter, annuler ctrl+esc
-- save => ctrl+s, boite de dialogue avec avec FS complet, ne peut sauvegardrer un fichier vide, naiviger dans le FS, choix de l'extension par type du fichier, nommer le fichier, valider ctrl+enter, annuler ctrl+esc
-- ================= separator =================
-- quit => ctrl+q



