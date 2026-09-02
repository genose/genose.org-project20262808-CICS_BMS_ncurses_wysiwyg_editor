# 📚 Contexte et Memoire du Projet

> **COBOL BMS WYSIWYG Editor** - Documentation du contexte, decisions et memoire technique

Ce document sert de **reference centrale** pour comprendre:
- **Pourquoi** ce projet existe
- **Quelles decisions** ont ete prises et pourquoi
- **Comment** le projet est structure
- **Ou** trouver les informations

---

## 🎯 **Contexte du Projet**

### Problematique

Les developpeurs **COBOL/CICS** sur **mainframe IBM Z** font face a plusieurs defis:

1. **Outils obsolètes** : Les editeurs BMS traditionnels (SPF, ISPF) sont:
   - Limités aux terminaux 3270
   - Sans interface graphique moderne
   - Peu intuitifs pour les nouveaux developpeurs
   - Sans integration avec les IDE modernes

2. **Complexite BMS** : Les maps BMS (Basic Mapping Support) sont:
   - Decrites via des cartes **DFHMSD**, **DFHMDI**, **DFHMND**, etc.
   - Difficiles a visualiser mentalement
   - Sujettes aux erreurs de syntaxe
   - Peu documentées

3. **Generation COBOL** : La creation du code COBOL pour interagir avec les maps est:
   - Repetitive
   - Sujette aux erreurs (noms de maps, mapsets, tailles)
   - Peu maintenable

### Objectifs du Projet

| Objectif | Description | Statut |
|----------|-------------|--------|
| **Edition WYSIWYG** | Creer/editer des maps BMS visuellement | ✅ Implemente |
| **Parsing robuste** | Lire les fichiers BMS existants | ✅ Implemente |
| **Generation COBOL** | Generer le code RECEIVE/SEND MAP | ✅ Implemente |
| **CLI moderne** | Interface terminal avec TUI | ✅ Implemente |
| **Integration VSCode** | Plugin avec preview et syntax highlighting | ✅ Base implementee |
| **Creation depuis zero** | Creer des maps sans fichier existant | ✅ Implemente |
| **Undo/Redo** | Historique des operations | ✅ Implemente |
| **Clipboard** | Copier/Couper/Coller des champs | ✅ Implemente |
| **Export/Import JSON** | Serialization des maps et de l'editeur | ✅ Implemente |
| **Boite de proprietes avancee** | Toutes les proprietes par type d'objet | ✅ Implemente |
| **Support Fieldset/Group** | Avec titre, decoration, bordures | ✅ Implemente |
| **Gestion des couleurs** | Texte, bordure, titre, contenu | ✅ Implemente |
| **ASCII Art & Image to ASCII** | Import et conversion d'images | ✅ Implemente |
| **100% Lua OBJECTS-DEFINITIONS Parity** | Toutes les 65+ proprietes implementees | ✅ COMPLETE |
| **Attributs individuels** | 12 nouveaux champs booleens accessibles | ✅ Implemente |

### Public Cible

1. **Developpeurs COBOL/CICS** : Principal utilisateur, besoin d'outils modernes
2. **Equipes de migration** : Passage du mainframe vers des environnements modernes
3. **Formateurs** : Enseignement du CICS/BMS avec des outils visuels
4. **Architectes** : Conception d'interfaces 3270

---

## 🏗️ **Decisions Architecturales**

### 1. **Choix du Langage Backend: Rust**

| Critere | Evaluation | Decision |
|---------|------------|----------|
| Performance | ⭐⭐⭐⭐⭐ | Parsing rapide, peu de memoire |
| Securite | ⭐⭐⭐⭐⭐ | Pas de segfaults, gestion memoire sure |
| Ecosysteme | ⭐⭐⭐⭐ | `nom` pour parsing, `serde` pour serialization |
| Compilation | ⭐⭐⭐⭐ | Binaires statiques, facile a deployer |
| Apprentissage | ⭐⭐⭐ | Courbe un peu raide mais compensée par les benefits |

**Alternative consideree**: Python (plus simple mais moins performant pour le parsing binaire)

**Justification**:
- Le parsing BMS peut impliquer des fichiers binaires (load modules)
- La generation de code doit etre rapide
- Les binaires Rust sont portables (Linux, macOS, Windows)

---

### 2. **Structure Modulaire**

```
┌─────────────────────────┐
│      core/ (Rust)       │  ← Backend pur (pas d'UI)
│  - model.rs             │  ← Structures de donnees
│  - parser.rs            │  ← Parsing BMS
│  - generator.rs         │  ← Generation COBOL/HTML
│  - editor.rs            │  ← Logique d'edition
└─────────────┬───────────┘
              │
              ├─────────────────────────┐
              │      cli/ (Rust)        │  ← Interface CLI
              │  - main.rs              │  ← TUI avec ratatui
              └─────────────┬───────────┘
                            │
              ┌─────────────┴─────────────────────────┐
              │    vscode-extension/ (TypeScript)        │  ← Plugin VSCode
              │  - extension.ts                         │
              │  - syntaxes/bms.tmLanguage.json         │
              └─────────────────────────────────────────┘
```

**Avantages**:
- Separation claire des responsabilites
- `core/` peut etre utilise par d'autres frontends (web, mobile, etc.)
- Tests unitaires facilites
- Maintenance simplifiee

---

### 3. **Choix des Bibliothèques Rust**

| Besoin | Bibliotheque | Justification |
|--------|--------------|---------------|
| Parsing | [`nom`](https://github.com/rust-bakery/nom) | Parseur combinatoire, performant, bien documenté |
| TUI | [`ratatui`](https://github.com/ratatui-org/ratatui) + [`crossterm`](https://github.com/crossterm-rs/crossterm) | Moderne, cross-platform, actif |
| Erreurs | [`thiserror`](https://github.com/dtolnay/thiserror) | Typage fort des erreurs |
| CLI | [`clap`](https://github.com/clap-rs/clap) | Standard de l'ecosysteme Rust |
| Serialization | [`serde`](https://github.com/serde-rs/serde) + [`serde_json`](https://github.com/serde-rs/json) | Pour le JSON ✅ Implemente |

**Note**: `ratatui` est le fork actif de `tui-rs` (qui est en maintenance seulement).

---

### 4. **Gestion des Erreurs**

**Strategie**: Utilisation de `thiserror` + `anyhow`

```rust
// Dans core/
#[derive(thiserror::Error, Debug)]
pub enum BmsParseError {
    #[error("Failed to parse BMS: {0}")]
    ParseError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

// Dans cli/
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let map = parse_bms_file("test.bms")
        .context("Failed to parse BMS file")?;
    Ok(())
}
```

**Avantages**:
- Erreurs types dans le core
- Contextes riches dans la CLI
- Conversion automatique entre les deux

---

### 5. **Undo/Redo**

**Implementation**: Pile doperations (`EditHistory`)

```rust
pub struct EditHistory {
    pub undo_stack: Vec<EditOperation>,
    pub redo_stack: Vec<EditOperation>,
    pub max_size: usize,  // Limite a 100 operations
}

pub enum EditOperation {
    AddField { field: BmsField, index: usize },
    RemoveField { field: BmsField, index: usize },
    ModifyField { old_field: BmsField, new_field: BmsField, index: usize },
    MoveField { field_index: usize, old_pos: (u16, u16), new_pos: (u16, u16) },
    ResizeField { field_index: usize, old_length: u16, new_length: u16 },
    ChangeColor { field_index: usize, old_color: Option<Color>, new_color: Option<Color> },
    ChangeAttributes { field_index: usize, old_attrs: Vec<FieldAttribute>, new_attrs: Vec<FieldAttribute> },
}
```

**Avantages**:
- Undo/Redo illimité (dans la limite de `max_size`)
- Chaque operation est atomique
- Facile a etendre

**Inconvénients**:
- Consommation memoire pour les grosses maps
- Pas de compression des operations (ex: deplacer un champ pixel par pixel = 10 operations)

**Ameliorations futures**:
- Compression des operations (batch)
- Serialization pour sauvegarder l'historique ✅ Implemente via JSON

---

### 6. **Clipboard**

**Implementation**: `Option<BmsField>` dans `BmsEditor`

```rust
pub struct BmsEditor {
    pub clipboard: Option<BmsField>,
    // ...
}

impl BmsEditor {
    pub fn copy_selected(&mut self) {
        if let Some(index) = self.selected_field {
            self.clipboard = Some(self.map.fields[index].clone());
        }
    }
    
    pub fn paste_at_cursor(&mut self) -> Option<usize> {
        if let Some(field) = self.clipboard.clone() {
            let mut new_field = field;
            new_field.pos = self.cursor_pos;
            Some(self.add_field(new_field))
        } else {
            None
        }
    }
}
```

**Limitation actuelle**:
- Un seul champ dans le clipboard

**Ameliorations futures**:
- Multiple fields dans le clipboard
- Serialization pour echanger entre instances ✅ Implemente via JSON

---

### 7. **Plugin VSCode**

**Architecture**:

```
┌─────────────────────────────────────────────────────────┐
│                    VSCode Extension                       │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌─────────────────┐    ┌─────────────────────────────┐  │
│  │  extension.ts   │    │        Webview               │  │
│  │  - Commandes    │    │  - Preview HTML/CSS          │  │
│  │  - LSP*         │    │  - Interaction JavaScript    │  │
│  └─────────────────┘    └─────────────────────────────┘  │
│                           │                              │
│                           ▼                              │
│  ┌─────────────────────────────────────────────────────┐  │
│  │               Rust Core (child_process)              │  │
│  │  - Parsing BMS                                   │  │
│  │  - Generation HTML                               │  │
│  └─────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘

* LSP = Language Server Protocol (optionnel)
```

**Decisions**:
- **Pas de LSP pour l'instant**: Trop complexe pour la V1
- **Appel direct au binaire Rust**: Simple et efficace
- **Fallback JavaScript**: Si le binaire Rust n'est pas disponible

**Ameliorations futures**:
- Implementation du LSP pour:
  - Validation en temps reel
  - Autocompletion
  - Diagnostics

---

## 🔄 **Historique des Decisions**

### Decision 1: Format des Fichiers BMS

**Probleme**: Les fichiers BMS peuvent etre:
- Source texte (`.bms`) avec des cartes DFHMSD, DFHMDI, etc.
- Load modules binaires

**Decision**:
- **V1**: Support uniquement des fichiers source texte
- **Futur**: Ajouter le support des load modules binaires

**Justification**:
- 90% des cas d'usage sont des fichiers texte
- Le parsing texte est plus simple
- Les load modules necessitent des connaissances spécifiques sur le format binaire

---

### Decision 2: Generation des Noms de Champs

**Probleme**: Les champs BMS n'ont pas de nom dans la specification (DFHMND n'a pas de parametre NAME)

**Decision**:
- Utiliser `FIELD1`, `FIELD2`, etc. pour les champs crees via l'editeur
- Stocker le nom dans un commentaire BMS: `DFHMND ... * FIELD1`

**Alternative consideree**:
- Ajouter un champ NAME non-standard (mais incompatible avec les parseurs existants)
- Utiliser la position comme identifiant (peu lisible)

**Justification**:
- Compatible avec les parseurs BMS standard
- Les noms sont utiles pour la documentation
- Facile a ignorer par les outils qui ne les supportent pas

---

### Decision 3: Gestion des Couleurs

**Probleme**: Les couleurs BMS sont limitees a un ensemble predefini

**Decision**:
```rust
pub enum Color {
    Black, Blue, Green, Cyan, Red, Magenta, Yellow, White,
    Turquoise, Pink, Orange, Purple, Gray, LightGreen,
    Custom(u16),  // Pour les codes personnalises
}
```

**Justification**:
- Support de toutes les couleurs standard
- Extensible pour les couleurs personnalisees
- Conversion facile depuis/to string

---

### Decision 4: Taille de la Grille

**Probleme**: Les ecrans 3270 standard sont 24x80, mais d'autres tailles existent

**Decision**:
- Taille par defaut: 24x80
- Configurable via DFHMDI SIZE=(n,m)
- Support des tailles personnalisees dans l'editeur

**Justification**:
- Compatible avec la majorite des ecrans
- Flexible pour les cas specifiques

---

### Decision 5: Mode de Selection

**Probleme**: Comment selectionner les champs dans la TUI?

**Decision**:
- **Selection par curseur**: Le curseur positionne la selection automatique
- **Selection par Tab**: Navigation entre les champs
- **Selection manuelle**: Cliquer sur un champ (futur)

**Justification**:
- Intuitif pour les utilisateurs de VI/emacs
- Rapide pour les power users
- Extensible pour la souris

---

### Decision 6: Boite de Proprietes Dynamique

**Probleme**: Comment afficher et editer toutes les proprietes des different types d'objets BMS (Field, Group/Fieldset, ASCII Art, Image)?

**Decision**:
- Creer un enum `PropertyType` listant toutes les proprietes possibles
- Implemente `get_properties_for_field()` pour generer dynamiquement la liste des proprietes selon le type d'objet
- Regrouper les proprietes par categories (Commun, Couleurs, Fieldset, ASCII Art, etc.)
- Navigation avec Up/Down, modification avec +/- dans la boite de proprietes

**Avantages**:
- Approche type-safe avec Rust enums
- Extensible pour de nouveaux types d'objets
- Interface utilisateur coherente
- Gestion dynamique des proprietes selon le contexte

---

## 💡 **Bonnes Pratiques**

### Rust

1. **Documentation**:
   ```rust
   /// Creer une nouvelle map avec des champs par defaut
   /// 
   /// # Examples
   /// ```
   /// let map = create_default_map("MENU", "APPSET");
   /// assert_eq!(map.size, (24, 80));
   /// ```
   pub fn create_default_map(name: &str, mapset: &str) -> BmsMap {
       // ...
   }
   ```

2. **Tests**:
   - Un test par fonction publique
   - Tests des edge cases (maps vides, champs invalides, etc.)
   - Utilisation de `rstest` pour les tests parametres

3. **Gestion des erreurs**:
   - Utiliser `thiserror` pour les erreurs types
   - Utiliser `anyhow` pour les erreurs dans la CLI
   - Toujours fournir un contexte (`with_context`)

4. **Performance**:
   - Eviter les allocations inutiles
   - Utiliser `&str` au lieu de `String` quand possible
   - Preferer les iterateurs aux boucles for

---

### TypeScript (VSCode)

1. **Typage fort**:
   ```typescript
   interface BmsField {
       name: string;
       pos: [number, number];
       length: number;
       attrb: string[];
       color?: string;
   }
   ```

2. **Communication avec Rust**:
   - Utiliser `child_process.execSync` pour les operations simples
   - Passer par des fichiers temporaires pour les grosses donnees
   - Envisager WebAssembly pour le futur

3. **Webviews**:
   - Toujours verifier que `enableScripts: true`
   - Utiliser des URI locales pour les ressources
   - Gérer les erreurs de communication

---

## ⚠️ **Problemes Connus et Solutions**

| Probleme | Solution actuelle | Solution future |
|----------|-------------------|-----------------|
| Pas de nom pour les champs BMS | Commentaires dans le fichier | Champ NAME non-standard |
| Undo/Redo consomme beaucoup de memoire | Limite a 100 operations | Compression des operations |
| Pas de support des load modules binaires | Non implemente | Ajouter le parsing binaire |
| Selection multiple non implementee | Un seul champ a la fois | Implemente la selection multiple |
| Pas de drag & drop | Navigation au clavier | Ajouter la souris |
| Clipboard limite a un champ | `Option<BmsField>` | `Vec<BmsField>` |
| Pas de validation en temps reel | Validation au sauvegarde | LSP pour VSCode |
| Conversion SVG vers ASCII Art directe | Conversion externe requise (SVG→PNG) | Integration native de SVG |

---

## 📖 **Glossaire**

| Terme | Definition |
|-------|------------|
| **BMS** | Basic Mapping Support - Systeme de definition d'ecrans 3270 |
| **CICS** | Customer Information Control System - Moniteur transactionnel IBM |
| **DFHMSD** | Carte de definition d'une map BMS |
| **DFHMDI** | Carte de dimensionnement d'une map |
| **DFHMND** | Carte de definition d'un champ |
| **DFHMDF** | Carte de definition d'un champ formate |
| **Mapset** | Ensemble de maps BMS |
| **WYSIWYG** | What You See Is What You Get - Edition visuelle |
| **TUI** | Text-based User Interface - Interface utilisateur texte |
| **LSP** | Language Server Protocol - Protocole pour les serveurs de langage |

---

## 🔗 **Ressources Externes**

### Documentation IBM
- [BMS Programming Guide](https://www.ibm.com/docs/en/cics-ts/6.1?topic=reference-bms-programming)
- [DFHMSD Macro](https://www.ibm.com/docs/en/cics-ts/6.1?topic=reference-dfhmsd-macro)
- [DFHMDI Macro](https://www.ibm.com/docs/en/cics-ts/6.1?topic=reference-dfhmdi-macro)
- [DFHMND Macro](https://www.ibm.com/docs/en/cics-ts/6.1?topic=reference-dfhmnd-macro)

### Outils existants
- [IBM CICS Explorer](https://www.ibm.com/products/cics-explorer) - Outil officiel IBM
- [Micro Focus Enterprise Developer](https://www.microfocus.com/products/enterprise-developer/) - IDE COBOL
- [GnuCOBOL](https://gnucobol.sourceforge.io/) - COBOL open source

### Bibliothèques utiles
- [nom](https://github.com/rust-bakery/nom) - Parsing en Rust
- [ratatui](https://github.com/ratatui-org/ratatui) - TUI en Rust
- [tower-lsp](https://github.com/ebkalderon/tower-lsp) - LSP en Rust
- [vscode-extensions](https://code.visualstudio.com/api) - Documentation VSCode

---

## 📝 **Journal des Changes (Changelog)**

### Version 0.1.0 (Initial)
- Creation du projet
- Implementation du parseur BMS
- Generation de code COBOL
- Editeur WYSIWYG basique
- CLI avec TUI (ratatui)
- Plugin VSCode de base

### Version 0.2.0 (2026-08-29)
- **Boite de proprietes avancee**: Toutes les proprietes par type d'objet
- **Support Fieldset/Group**: Titre, decoration, bordures, couleurs
- **Gestion des couleurs**: Texte, bordure, titre, contenu
- **ASCII Art & Image to ASCII**: Import et conversion d'images
- **Raccourcis clavier**: Ctrl+A, Ctrl+M, Ctrl+H, Ctrl+Space
- **Ameliorations UI**: Messages d'erreur, coordonnees curseur
- **Fixes**: Correction des erreurs de compilation (out of bounds)

### Version 0.3.0 (2026-09-02) - Lua OBJECTS-DEFINITIONS Parity Complete ✅
- **100% Lua OBJECTS-DEFINITIONS Parity**: Toutes les 65+ proprietes implementees
- **Attributs individuels**: field_enabled, field_visible, field_required, field_readonly, field_protected, field_numeric, field_has_error, field_selected, field_focused, field_highlighted, field_hidden, field_in_edit_mode
- **Proprietes dynamiques**: field_avail_color, field_avail_font_family, field_avail_pos, field_avail_text_align, field_avail_vertical_align, field_avail_border_style, field_font_family, field_footer_align, field_footer_title
- **Optimisation UI**: Toggle essential/toutes categories (A) pour eviter le scroll excessif
- **Amelioration de la boite de proprietes**: Navigation et edition optimisees
- **Correction du scroll**: Affichage par defaut des categories essentielles seulement

---

## 🤝 **Contributeurs**

| Role | Contribution | Period |
|------|--------------|--------|
| Auteur | Architecture, implementation complete | 2024 |
| Mistral Vibe | Generation du code initial | 2024-08-28 |
| Mistral Vibe | Boite de proprietes, Fieldset, couleurs, fixes | 2026-08-29 |

---

## 📞 **Support**

Pour les questions, suggestions ou rapports de bugs:
- **Repository**: [genose.org-project20262808-CICS_BMS_ncurses_wysiwyg_editor](https://github.com/genose.org/genose.org-project20262808-CICS_BMS_ncurses_wysiwyg_editor)
- **Issues**: Ouvrir une issue sur GitHub
- **Discussions**: Utiliser les discussions GitHub

---

> **Derniere mise a jour**: 2026-09-02
> **Version**: 0.3.0
> **Auteur**: Genose.org (Cotillard Sebastien)
> **Concept**: Genose.org (Cotillard Sebastien)
