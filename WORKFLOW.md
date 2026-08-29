# 🎯 Workflow: Creation d'un ecran BMS depuis zero

Ce guide montre comment utiliser l'editeur WYSIWYG pour creer un ecran BMS complet depuis zero.

---

## 📋 Scenario: Creation d'un formulaire de saisie client

Nous allons creer un ecran de saisie avec:
- Un titre
- Des labels et champs de saisie
- Des boutons d'action
- Une ligne de statut

---

## 🚀 Etape par etape

### Etape 1: Demarrer l'editeur

```bash
cobol-bms edit
```

L'editeur s'ouvre avec une map vide (24x80).

### Etape 2: Charger un template (optionnel)

Appuyez sur **`N`** (majuscule) pour charger un template pre-rempli.

Ou créez une nouvelle map vide avec **`n`** (minuscule).

### Etape 3: Ajouter un titre

1. Deplacez le curseur en ligne 1, colonne 25 avec les fleches
2. Appuyez sur **`A`** pour ajouter un champ de 20 caracteres
3. Le champ est selectionne

### Etape 4: Modifier les proprietes du titre

1. Appuyez sur **`e`** pour editer les proprietes
2. Avec **Up/Down** naviguez entre les proprietes
3. Avec **+** et **-** modifiez les valeurs:
   - Row: 1
   - Col: 25
   - Length: 30
4. Appuyez sur **`Enter`** pour sauvegarder ou **`Esc`** pour annuler
5. Appuyez sur **`C`** pour changer la couleur rapidement
6. Selectionnez **White** avec la touche **`w`**
7. Validez avec **Enter**
8. Appuyez sur **`t`** pour changer les attributs rapidement
9. Selectionnez **PROT** (p) et **INTENS** (i)
10. Validez avec **Enter**

### Etape 5: Ajouter un label "Customer:"

1. Deplacez le curseur en ligne 3, colonne 1
2. Appuyez sur **`a`** pour ajouter un champ de 10 caracteres
3. Modifiez les proprietes:
   - Col: 1
   - Length: 9
4. Changez la couleur en **Green** (`g`)
5. Ajoutez l'attribut **PROT** (`p`)
6. Pour l'initialiser avec "Customer:", il faudra modifier le fichier BMS manuellement pour l'instant

### Etape 6: Ajouter un champ de saisie

1. Deplacez le curseur en ligne 3, colonne 11
2. Appuyez sur **`A`** pour ajouter un champ de 20 caracteres
3. Modifiez la couleur en **Yellow** (`y`)
4. Ajoutez les attributs **NORM** (`n`) et **ALPH** (`a`)

### Etape 7: Ajouter d'autres champs

Repetez les etapes 5-6 pour ajouter:
- Ligne 5: Label "Order #:" (col 1) + Champ de saisie (col 11, length 10, NUM)
- Ligne 7: Label "Amount:" (col 1) + Champ de saisie (col 11, length 15, NUM, PIC='9(10)V99')

### Etape 8: Ajouter une ligne de statut

1. Deplacez le curseur en ligne 23, colonne 1
2. Appuyez sur **`A`** pour ajouter un champ de 80 caracteres
3. Changez la couleur en **Blue** (`b`)
4. Ajoutez les attributs **PROT** (`p`) et **REVERSE** (`v`)
5. Initialisez avec "F1=Help F3=Exit F12=Save" (a modifier manuellement dans le fichier BMS)

### Utilisation de la Boite de Proprietes Avancee

La boite de proprietes ( accessible via **`e`**) permet de modifier toutes les proprietes d'un champ:

- **Navigation**: Utilisez **Up/Down** pour vous deplacer entre les proprietes
- **Modification**: Utilisez **+** et **-** pour modifier les valeurs
- **Groupes de proprietes**: Les proprietes sont regroupees par categorie:
  - **Commun**: Nom, Type, Position (Row/Col), Longueur, Attributs
  - **Couleurs**: Couleur du texte, Couleur de bordure
  - **Valeurs**: Initial, PIC, Nom de groupe
  - **Multi-ligne**: Hauteur (pour les Fieldsets et ASCII Art)
  - **Fieldset**: Titre, Hauteur, Decoration, Bordure, Alignement du titre, Couleurs personnalisees
  - **ASCII Art**: Donnees ASCII Art
  - **BMS Avance**: Justification, AutoSkip, FieldExit, BlankZero, Repeat, etc.

Pour les **Fieldsets** (Group):
- La premiere ligne contient: (decoration) (titre) (decoration)
- La derniere ligne contient: (decoration)
- Minimum 3 lignes requis
- Le titre peut etre aligne a gauche, centre ou droite

### Etape 9: Sauvegarder

1. Appuyez sur **`Ctrl+S`**
2. Entrez le nom du fichier: `customer_form.bms`
3. Validez avec **Enter**

### Etape 10: Generer le code COBOL

Appuyez sur **`g`** pour generer `customer_form.cbl`

---

## 📝 Resultat: Fichier BMS genere

```bms
DFHMSD TYPE=CUSTOMER,MAPSET=APPSET1,LANG=COBOL,PHYSICAL=YES
DFHMDI SIZE=(24,80)
DFHMND TYPE=MAP
*
* Titre
DFHMND POS=(1,25),LENGTH=30,ATTRB=(PROT,INTENS),COLOR=WHITE,INITIAL='CUSTOMER FORM'
*
* Customer field
DFHMND POS=(3,1),LENGTH=9,ATTRB=(PROT),COLOR=GREEN,INITIAL='Customer:'
DFHMND POS=(3,11),LENGTH=20,ATTRB=(NORM,ALPH),COLOR=YELLOW
*
* Order field
DFHMND POS=(5,1),LENGTH=8,ATTRB=(PROT),COLOR=GREEN,INITIAL='Order #:'
DFHMND POS=(5,11),LENGTH=10,ATTRB=(NUM),COLOR=YELLOW
*
* Amount field
DFHMND POS=(7,1),LENGTH=7,ATTRB=(PROT),COLOR=GREEN,INITIAL='Amount:'
DFHMND POS=(7,11),LENGTH=15,ATTRB=(NUM),COLOR=YELLOW,PIC='9(10)V99'
*
* Status line
DFHMND POS=(23,1),LENGTH=80,ATTRB=(PROT,REVERSE),COLOR=BLUE,INITIAL='F1=Help F3=Exit F12=Save'
```

---

## 💻 Code COBOL genere

```cobol
IDENTIFICATION DIVISION.
PROGRAM-ID. CUSTOMER.
AUTHOR. BMS-GENERATOR.

ENVIRONMENT DIVISION.
CONFIGURATION SECTION.
SOURCE-COMPUTER. IBM-Z.
OBJECT-COMPUTER. IBM-Z.

DATA DIVISION.
WORKING-STORAGE SECTION.
01  WS-EIBRESP      PIC S9(8) COMP VALUE 0.
01  WS-EIBRESP2     PIC S9(8) COMP VALUE 0.
01  WS-EIBFN       PIC X(4) VALUE SPACES.

01  CUSTOMER.
    05  FILLER       PIC X(30).
    05  FILLER       PIC X(9).
    05  FILLER       PIC X(20).
    05  FILLER       PIC X(8).
    05  FILLER       PIC 9(10).
    05  FILLER       PIC X(7).
    05  FILLER       PIC 9(15).
    05  FILLER       PIC X(80).

PROCEDURE DIVISION.
MAIN-PARAGRAPH.
    EXEC CICS
        RECEIVE MAP('CUSTOMER')
              MAPSET('APPSET1')
        INTO(CUSTOMER)
        RESP(WS-EIBRESP)
        RESP2(WS-EIBRESP2)
    END-EXEC.

    IF WS-EIBRESP = DFHRESP(NORMAL)
        CONTINUE
    ELSE
        EXEC CICS
            ABEND
        END-EXEC
    END-IF.

    * Your business logic here
    * Process customer data...

    EXEC CICS
        SEND MAP('CUSTOMER')
              MAPSET('APPSET1')
              FROM(CUSTOMER)
        RESP(WS-EIBRESP)
        RESP2(WS-EIBRESP2)
    END-EXEC.

    EXEC CICS
        RETURN
    END-EXEC.
    GOBACK.
```

---

## 🎨 Astuces

### Ajouter plusieurs champs rapidement

1. Selectionnez un champ
2. Appuyez sur **`c`** pour copier
3. Deplacez le curseur
4. Appuyez sur **`v`** pour coller
5. Modifiez les proprietes si necessaire

### Aligner des champs

1. Ajoutez un premier champ
2. Copiez-le (**`c`**)
3. Utilisez les fleches pour deplacer le curseur
4. Collez (**`v`**) et ajustez la position

### Modifier plusieurs champs

1. Selectionnez un champ avec **Tab**
2. Modifiez ses proprietes (**`C`**, **`t`**, **`e`**)
3. Passez au champ suivant avec **Tab**
4. Repetez les modifications

### Annuler une erreur

Appuyez sur **`Ctrl+Z`** pour annuler la derniere operation.
Appuyez sur **`Ctrl+Y`** pour refaire.

---

## 🔧 Resolution de problemes

### Le champ n'apparait pas
- Verifiez que la position (POS) est dans les limites de la map (1-24 lignes, 1-80 colonnes)
- Verifiez que la longueur (LENGTH) est > 0

### La couleur ne change pas
- Assurez-vous d'avoir selectionne un champ avant d'ouvrir le selecteur de couleurs
- Validez avec **Enter** apres avoir selectionne la couleur

### Le fichier n'est pas sauvegarde
- Verifiez que vous avez les permissions d'ecriture dans le dossier
- Essayez un chemin absolu comme `/tmp/test.bms`

---

## 📚 Reference rapide

| Action | Raccourci |
|--------|-----------|
| Nouveau champ (menu) | Ctrl+A |
| Nouveau champ | `a` (10) ou `A` (20) |
| Supprimer champ | `d` |
| Deplacer champ | `m` + fleches + Enter |
| Redimensionner | `r` + fleches + Enter |
| Copier | `c` ou Ctrl+C |
| Couper | `x` |
| Coller | `v` |
| Couleur | `C` + lettre couleur + Enter |
| Attributs | `t` + lettre attribut + Enter |
| Editer proprietes | `e` + Up/Down nav + +/- modify |
| Sauvegarder | Ctrl+S |
| Generer COBOL | `g` |
| Undo | Ctrl+Z |
| Redo | Ctrl+Y |
| Preview Canvas/Code | Ctrl+M |
| Aide | Ctrl+H ou `?` |
| Preview/Edit | Ctrl+Space ou ` ` (espace) |
| Quitter | Ctrl+Q |

---

## 🎯 Prochaines etapes

- [ ] Ajouter la possibilite de renommer les champs
- [ ] Ajouter la saisie de valeurs INITIAL et PIC directement dans l'editeur
- [ ] Implémenter le deplacement par glisser-deposer (drag & drop)
- [ ] Ajouter un mode grille pour aligner les champs automatiquement
