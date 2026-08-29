# COBOL BMS WYSIWYG Editor

> Editeur visuel pour la creation et l'edition d'ecrans CICS/BMS depuis zero

![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![VSCode](https://img.shields.io/badge/vscode-1.75+-blue.svg)

**Fonctionnalite principale**: Creation et edition visuelle (WYSIWYG) de maps BMS pour COBOL/CICS.

## 🎯 Capacites

### 📐 Edition WYSIWYG
- Creation de **nouvelles maps BMS depuis zero**
- **Ajout/suppression** de champs (DFHMND)
- **Deplacement** de champs avec le clavier
- **Redimensionnement** interactif
- **Modification des proprietes**: couleur, attributs, type, PIC, INITIAL
- **Boite de proprietes avancee**: Toutes les proprietes par type d'objet (Field, Group/Fieldset, ASCII Art, Image)
  - Proprietes communes: Nom, Type, Position, Longueur, Attributs, Couleurs
  - Fieldset: Titre, Hauteur, Decoration, Alignement titre, Couleurs (titre, bordure, contenu)
  - ASCII Art: Import et conversion d'images
- **Undo/Redo** illimite
- **Clipboard**: Copier/Couper/Coller

### 🎨 Previsualisation
- Rendu visuel des champs avec leurs couleurs
- Distinction des champs PROT, NUM, ALPH
- Scroll pour les grandes maps
- Selection visuelle

### 💻 Generation de code
- Generation automatique de code **COBOL/CICS** (RECEIVE MAP, SEND MAP)
- Export au format **BMS** standard
- Import depuis fichiers BMS existants

### 🖥️ Interface
- **CLI** avec TUI (ncurses-like via ratatui)
- **Plugin VSCode** avec preview webview et syntax highlighting

---

## 📦 Structure du projet

```
.
├── Cargo.toml                          # Workspace Rust
├── core/                               # Backend (parsing + edition)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   └── bms/
│   │       ├── model.rs       # BmsMap, BmsField, Color, etc.
│   │       ├── parser.rs      # Parseur DFHMSD/DFHMDI/DFHMND
│   │       ├── generator.rs   # Generation COBOL + HTML
│   │       └── editor.rs      # ✨ Edition WYSIWYG
│   └── tests/
│       └── parser_test.rs
│       └── editor_test.rs
├── cli/                                # Interface CLI
│   ├── Cargo.toml
│   └── src/
│       └── main.rs            # ✨ TUI WYSIWYG complete
├── vscode-extension/                   # Plugin VSCode
│   ├── package.json
│   ├── tsconfig.json
│   ├── src/
│   │   └── extension.ts
│   ├── syntaxes/
│   │   ├── bms.tmLanguage.json
│   │   └── language-configuration.json
│   └── webviews/
│       ├── styles.css
│       └── preview.js
├── examples/                           # Exemples
│   ├── menu01.bms
│   └── dataentry.bms
├── .gitignore
├── LICENSE
└── README.md
```

---

## 🚀 Installation

### Prerequis
- **Rust** 1.70+ : [https://www.rust-lang.org](https://www.rust-lang.org)
- **Node.js** 16+ : Pour le plugin VSCode

### Backend + CLI
```bash
git clone https://github.com/genose.org/genose.org-project20262808-CICS_BMS_ncurses_wysiwyg_editor.git
cd genose.org-project20262808-CICS_BMS_ncurses_wysiwyg_editor
cargo build --release
```

Le binaire sera dans `target/release/cobol-bms`

### Plugin VSCode
```bash
cd vscode-extension
npm install
npm run compile
# Puis dans VSCode: F5 pour tester
```

---

## 💡 Utilisation

### Creation d'une map depuis zero

```bash
# Creer une nouvelle map vide (24x80)
cobol-bms new --name MONMENU --mapset APPSET1 --width 80 --height 24 --edit

# Ou sans arguments (valeurs par defaut)
cobol-bms new -e
```

### Edition d'une map existante

```bash
cobol-bms edit examples/menu01.bms
```

### Mode interactif (TUI)

Lance l'editeur WYSIWYG avec la commande `edit` ou `new -e`.

#### 🗝️ Raccourcis clavier

| Touche | Action |
|--------|--------|
| `j`/`k` / `↓`/`↑` | Deplacer le curseur |
| `h`/`l` / `←`/`→` | Deplacer le curseur |
| `Tab` / `Shift+Tab` | Champ suivant/precedent |
| `Ctrl+A` | Ajouter un champ/object avec boite de selection |
| `a` | Ajouter un champ (longueur 10) |
| `A` | Ajouter un champ (longueur 20) |
| `d` | Supprimer le champ selectionne |
| `m` | Deplacer le champ (fleches pour deplacer, Enter pour valider) |
| `r` | Redimensionner le champ |
| `e` | Editer les proprietes |
| `C` | Changer la couleur |
| `t` | Changer les attributs |
| `c` | Copier le champ |
| `x` | Couper le champ |
| `v` | Coller a la position du curseur |
| `n` | Nouvelle map vide |
| `N` | Charger un template de map |
| `g` | Generer le code COBOL |
| `Ctrl+S` | Sauvegarder |
| `Ctrl+Z` | Undo |
| `Ctrl+Y` | Redo |
| `Ctrl+C` | Copier dans le presse-papier |
| `Ctrl+Q` | Quitter (avec confirmation si modifie) |
| `Ctrl+M` | Basculer Preview Canvas/Code |
| `Ctrl+H` | Basculer ecran Aide/Normal |
| `Ctrl+Space` | Basculer Preview/Edit |
| `?` | Aide |
| ` ` (espace) | Basculer mode Preview/Edit |

#### 🎨 Selecteurs

- **Couleurs** (`C`): B (Blue), G (Green), R (Red), Y (Yellow), W (White), C (Cyan), M (Magenta), K (Black), O (Orange), P (Pink), Espace (None)
- **Attributs** (`t`): P (PROT), N (NORM), U (NUM), A (ALPH), L (ALNUM), I (INTENS), B (BLINK), V (REVERSE), D (DARK)

---

## 📋 Exemples

### 1. Creation d'un menu simple

```bash
# Demarrer l'editeur
cobol-bms edit

# Dans l'editeur:
# 1. Appuyez sur 'N' pour charger un template
# 2. Utilisez Tab pour naviguer entre les champs
# 3. Modifiez les textes avec 'e' puis +/-
# 4. Ajoutez de nouveaux champs avec 'a'
# 5. Sauvegardez avec Ctrl+S
```

### 2. Generation de code COBOL

Apres avoir cree votre map:
```bash
# Depuis l'editeur: appuyez sur 'g'
# Ou en CLI:
cobol-bms generate ma_map.bms --output ma_map.cbl
```

Exemple de code genere:
```cobol
IDENTIFICATION DIVISION.
PROGRAM-ID. MONMENU.

DATA DIVISION.
WORKING-STORAGE SECTION.
01  WS-EIBRESP      PIC S9(8) COMP VALUE 0.
01  MONMENU.
    05  TITLE       PIC X(30).
    05  OPTION1     PIC X(2).

PROCEDURE DIVISION.
    EXEC CICS
        RECEIVE MAP('MONMENU')
        INTO(MONMENU)
    END-EXEC.
    
    EXEC CICS
        SEND MAP('MONMENU')
        FROM(MONMENU)
    END-EXEC.
    
    EXEC CICS RETURN END-EXEC.
    GOBACK.
```

### 3. Format BMS supporte

```bms
DFHMSD TYPE=MONMENU,MAPSET=APPSET1,LANG=COBOL,PHYSICAL=YES
DFHMDI SIZE=(24,80)
DFHMND TYPE=MAP
DFHMND POS=(1,25),LENGTH=30,ATTRB=(PROT,INTENS),COLOR=WHITE,INITIAL='MON MENU'
DFHMND POS=(3,10),LENGTH=20,ATTRB=(NORM,ALPH),COLOR=YELLOW
DFHMND POS=(5,10),LENGTH=10,ATTRB=(NUM),COLOR=GREEN
DFHMND POS=(23,1),LENGTH=80,ATTRB=(PROT,REVERSE),COLOR=BLUE
```

---

## 🏗️ Architecture Technique

```
┌─────────────────────────────────────────────────────────────────┐
│                    EDITEUR WYSIWYG                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────┐ │
│  │   core (Rust)   │    │   cli (Rust)    │    │  VSCode     │ │
│  │                 │    │                 │    │ (TypeScript) │ │
│  │ • model.rs     │    │ • TUI mode      │    │ • extension │ │
│  │ • parser.rs    │◄───►│ • editor.rs     │    │ • webview   │ │
│  │ • generator.rs │    │ • main.rs       │    │ • LSP*      │ │
│  │ • editor.rs    │    │                 │    │             │ │
│  └─────────────────┘    └─────────────────┘    └─────────────┘ │
│                          │                                    │
│                          ▼                                    ▼
│              ┌───────────────────┐              ┌─────────────┐ │
│              │   BMS Files       │              │ COBOL Files │ │
│              │   (.bms)          │              │ (.cbl)      │ │
│              └───────────────────┘              └─────────────┘ │
└─────────────────────────────────────────────────────────────────┘

* LSP (Language Server Protocol) optionnel pour validation/autocompletion
```

### Modules Rust (core/)

| Module | Responsabilite |
|--------|----------------|
| `model.rs` | Structures: BmsMap, BmsField, Color, FieldType, FieldAttribute |
| `parser.rs` | Parsing des cartes BMS (DFHMSD, DFHMDI, DFHMND, etc.) |
| `generator.rs` | Generation de code COBOL et rendu HTML |
| `editor.rs` | ✨ **Edition WYSIWYG**: BmsEditor, EditHistory, operations d'edition |

### Fonctionnalites de l'editeur (editor.rs)

- `BmsEditor`: Etat complet de l'editeur avec map, selection, curseur, historique
- `EditHistory`: Gestion undo/redo avec pile doperations
- `EditOperation`: Ajout, suppression, deplacement, redimensionnement, modification
- `CursorDirection`: Navigation dans la grille
- `create_default_map()`: Generation de template pre-rempli
- `export_to_bms()`: Export au format BMS standard

---

## 🔌 Integration VSCode

### Commandes disponibles
- `Ctrl+Shift+P` > **Preview BMS Map**: Affiche la previsualisation graphique
- `Ctrl+Shift+P` > **Generate COBOL from BMS**: Genere le code COBOL

### Fonctionnalites
- Syntax highlighting pour les fichiers `.bms`
- Autocompletion des mots-cles BMS
- Colorisation selon le type de carte
- Preview webview avec rendu visuel
- Integration avec le backend Rust pour le parsing

---

## 📊 Roadmap

- [x] Parsing BMS complet
- [x] Generation COBOL
- [x] CLI basique (preview, generate)
- [x] **Editeur WYSIWYG en TUI**
- [x] Creation de maps depuis zero
- [x] Ajout/suppression de champs
- [x] Deplacement/redimensionnement
- [x] Undo/Redo
- [x] Clipboard
- [x] Plugin VSCode de base
- [x] **Boite de proprietes avancee** avec toutes les proprietes par type d'objet
- [x] **Support Fieldset/Group** avec titre, decoration, bordures
- [x] **Gestion des couleurs** (texte, bordure, titre, contenu)
- [x] **ASCII Art** import et conversion d'images
- [x] **Image to ASCII** avec file chooser et assistant
- [x] **Gestion des erreurs** avec messages en bas de l'ecran
- [x] **Coordonnees du curseur** affichees en bas de l'ecran
- [ ] **Selection multiple**
- [ ] **Alignement automatique**
- [ ] **Grille magnetique** (snap to grid)
- [ ] **Groupement de champs**
- [ ] **Import/Export JSON**
- [ ] **LSP complet** (validation, autocompletion)
- [ ] **Preview en temps reel** dans VSCode
- [ ] **Edition collaborative**

---

## 🤝 Contribution

1. Forker le projet
2. Creer une branche (`git checkout -b feature/ma-fonctionnalite`)
3. Committer vos changements
4. Pousser vers la branche
5. Ouvrir une Pull Request

---

## 📜 License

MIT License - voir [LICENSE](LICENSE)

---

## 📞 Contact

- **Auteur**: genose.org
- **Repository**: [genose.org-project20262808-CICS_BMS_ncurses_wysiwyg_editor](https://github.com/genose.org/genose.org-project20262808-CICS_BMS_ncurses_wysiwyg_editor)
