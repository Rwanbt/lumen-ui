# Roadmap v2 — Catalogue complet (lumen-ui)

> **Statut** : proposition (Draft). Ne remplace pas [ROADMAP.md](../ROADMAP.md) (plan v0.1→v1.0,
> réalisé). Ce document planifie l'expansion **v1.1 → v2.0** : passer d'un design system
> « app-shell + widgets fondamentaux » à un **catalogue complet** comparable à shadcn/Radix/MUI,
> plus un différenciateur audio.
> **Cible inchangée** : egui 0.34.x, MSRV Rust 1.92, licence MIT OR Apache-2.0.
> **Référence d'exhaustivité** : WAI-ARIA Authoring Practices Guide (APG) ∪ Radix Primitives ∪
> shadcn/ui ∪ MUI ∪ Material 3.

---

## Avancement (mis à jour 2026-06-16) — branche `feat/v2-catalogue`

> Slice horizontale de widgets à faible risque réalisée en autonomie, plus la fondation et le PoC
> icônes. La progression est transverse (v1.1–v1.5, v1.8), pas jalon-par-jalon.
> Légende : ✅ fait · 🟡 partiel · ⬜ à faire.

**Pré-requis**
- [x] **ADR-0008** — icônes SVG→painter par codegen (tranché + PoC `tools/lumen-icon-gen` prouvé)
- [x] **ADR-0009** — trait `Theme` : `mode()` additif + recettes en fonctions pures (non-breaking)

**Jalons**
- ✅ **v1.1 Frozen-core evolution** — [x] `Theme::mode()` ; [x] variants Danger/Warning/Success
  (Button/Badge/Alert) ; [x] `IconButton` helper (générique `impl Widget`, premier consommateur
  de `mode().emphasis()` via ADR-0009) — **JALON CLÔTURÉ**
- ✅ **v1.2 Overlays & feedback** — [x] Alert · Progress · CircularProgress · Spinner · Skeleton ·
  EmptyState ; [x] Tooltip/Popover/ContextMenu (préexistants) ; [x] DropdownMenu · HoverCard
  — **JALON CLÔTURÉ**
- 🟡 **v1.3 Data display** — [x] Table · Avatar · Chip · Stat · Code · Kbd · Divider · Badge ;
  [x] DataGrid (egui_extras `TableBuilder` — tri émis/virtualisation, feature `datagrid`, ADR-0010) ;
  [ ] DescriptionList · Timeline · Calendar · Carousel
- 🟡 **v1.4 Inputs & forms** — [x] SegmentedControl · Rating ; [ ] Textarea · NumberInput ·
  RangeSlider · Combobox · MultiSelect · DatePicker · TimePicker · ColorPicker · FileUpload · Form
- ✅ **v1.5 Icônes à l'échelle** — [x] PoC pipeline codegen (ADR-0008) ; [x] set Lucide curé
  **généré et intégré** dans `lumen-ui-icons` (arrow ×4, chevron-left/up depuis `svg/` →
  `generated_icons.rs` via `include!`) — **JALON CLÔTURÉ**. Extensible : ajouter un SVG +
  régénérer. Les icônes à `<circle>` (search…) attendent l'upgrade `usvg` (ADR-0008 §6.3).
- 🟡 **v1.6 Motion v2** — [x] `prefers-reduced-motion` (`set_reduced_motion`/`reduced_motion` →
  `ease`/`fade` instantanés) ; [ ] slide/scale, enter/exit, stagger, FLIP (bloqués : egui stable
  n'expose pas ergonomiquement les transforms de sous-arbre — cf. note `transitions.rs`)
- ⬜ **v1.7 Layout v2** — grille fractionnaire, template-areas, sticky, container, resizable
- 🟡 **v1.8 Navigation & patterns** — [x] Breadcrumb · Pagination · Stepper ; [x] Form (layout) ·
  AuthCard · MasterDetail · Wizard (patterns) · TreeView (widget) · DataTable pattern
  (search + sortable DataGrid + pagination, feature `datagrid`) ; [ ] Drawer
- ⬜ **v1.9 Thèmes v2 & outillage** — theme builder, rampes sémantiques, auto color-scheme, `.ron`
- ⬜ **v1.10 Crate `lumen-ui-audio`** — Knob · Fader · VU meter · Waveform · XY pad
- ⬜ **v2.0 Consolidation** — galerie WASM, mdBook, snapshots, audit WCAG, gel SemVer

**Widgets livrés cette session (19)** : Spinner, Progress, Divider, Alert, Skeleton, Avatar, Chip,
Kbd, Stat, Breadcrumb, SegmentedControl, Pagination, EmptyState, Link, CircularProgress, Rating,
Stepper, Code, Table. (Chacun : recette pure `XxxRecipe::resolve` + couverture test multi-thèmes.)

**Décisions restantes (hors autonomie)** : DataGrid → ADR `egui_extras` ; pickers/combobox →
conception d'état ; crate audio → validation sur app réelle.

---

## 0. Pré-requis architecturaux (à trancher AVANT le catalogue)

Deux décisions bloquent l'expansion. Chacune mérite un ADR.

### ADR-0009 — Évolution du trait `Theme` sans breaking change (LE verrou)

**Problème** : `Theme` ([theme.rs](../crates/lumen-ui-core/src/theme.rs)) expose une méthode par
recette (`button_recipe`, `card_recipe`, …) **sans default**. Ajouter un widget thémé = nouvelle
méthode = casse les 4 thèmes internes + tout thème tiers. Le catalogue x3 est impossible ainsi.

**Options** :
1. **Méthodes par défaut dérivées des tokens** — chaque nouvelle `*_recipe` a un `default`
   raisonnable calculé depuis `Tokens`. Un thème ne surcharge que ce qu'il veut. Additif, non
   breaking. **Recommandé.**
2. **Registre générique** : `fn recipe(&self, kind: ComponentKind, state, ctx) -> Recipe` +
   enum extensible. Plus souple mais perd le typage par recette (régression d'ergonomie IA).
3. Statu quo + ADR par ajout (coûteux, ralentit tout).

**Détail des deux options principales** (à discuter avant de trancher) :

*Option 1 — méthodes par défaut*
```rust
fn tooltip_recipe(&self, ctx: &UiContext) -> TooltipRecipe {
    TooltipRecipe::from_tokens(self.tokens(), ctx)   // défaut dérivé des tokens
}
```
- ✅ Additif (thèmes existants compilent sans changement) · ✅ typage fort conservé (découvrable
  par l'IA et l'autocomplete) · ✅ surcharge sélective.
- ⚠️ Le trait grossit (1 méthode/widget) ; il faut un défaut `from_tokens` de qualité par recette.

*Option 2 — registre générique*
```rust
fn recipe(&self, kind: ComponentKind, state: WidgetState, ctx: &UiContext) -> ComponentRecipe;
```
- ✅ Le trait ne grossit jamais (ajout = variante d'enum).
- ❌ Perte du typage par recette (type union large / `Box<dyn Any>`) · ❌ gros `match` verbeux ·
  ❌ moins découvrable pour l'IA · ❌ casse la cohérence des 7 recettes typées actuelles.

**Décision recommandée** : option 1 (le typage explicite empêche l'invention d'API — cœur de la
proposition de valeur IA). Seul point réellement ouvert : la qualité des défauts `from_tokens`.
Pré-requis dur de la v1.1.

### ADR-0008 — Pipeline d'icônes SVG → painter (codegen build-time) · **TRANCHÉ (2026-06-16)**

Voir §6 et [docs/adr/0008-svg-icon-codegen.md](adr/0008-svg-icon-codegen.md).
**Décision actée : codegen `xtask` (`usvg` + `lyon`) émettant du painter**, pour préserver
« no asset / no runtime dep / vectoriel / tokenisé ». Runtime `resvg` et police d'icônes rejetés.

---

## 1. Inventaire vérifié (v1.0, juin 2026)

| Crate | Contenu réel |
|-------|--------------|
| `core` | tokens, density/context, trait `Theme` (7 recettes), `install`, anim, a11y, builder, **DarkTheme + LightTheme**, PaletteTheme |
| `widgets` (16) | button, card, checkbox, radio, switch, slider, select, text_field, tabs, accordion, modal, overlay, toast, badge, text, focus |
| `patterns` (6) | dashboard, sidebar, command_palette, bars (toolbar/status), rows, logpanel |
| `layout` | Flex (row/col, gap, justify, align), Grid (cols fixes), Breakpoint/responsive |
| `motion` | Easing, Spring (animate/animate_color), transitions::fade |
| `themes` | audio_dark, high_contrast (PaletteTheme) → **+ Dark/Light du core = 4 thèmes** |
| `icons` | 8 icônes dessinées au painter (Check, Close, ChevronDown/Right, Plus, Minus, Search, Menu) — **pas de SVG** |

---

## 2. Taxonomie exhaustive de référence (✅ présent · 🟡 partiel/composable · ❌ manquant)

### 2.1 Widgets — Inputs & formulaires
- ✅ TextField, Checkbox, Radio (group), Switch, Slider, Select
- 🟡 PasswordField (obscured sur TextField ?), SearchField (TextField + icône)
- ❌ Textarea (multi-ligne), NumberInput (steppers), RangeSlider (2 poignées),
  Slider à crans/marks, Combobox/Autocomplete (select recherchable), MultiSelect,
  DatePicker, TimePicker, DateRangePicker, ColorPicker, FileUpload/Dropzone,
  Rating, SegmentedControl/ToggleGroup, OTP/PIN input, InlineEdit (editable text),
  **Form** (label + field + erreur + aide + validation), FormField, Fieldset

### 2.2 Widgets — Boutons & actions
- ✅ Button (Primary/Secondary/Ghost)
- 🟡 IconButton (Button + Icon), Danger (variant prévu, à exposer)
- ❌ ButtonGroup, SplitButton, ToggleButton, FAB, Link, DropdownMenu (menu d'actions ≠ select),
  Menubar, Toolbar riche

### 2.3 Widgets — Navigation
- ✅ Tabs ; (Sidebar/CommandPalette en patterns)
- ❌ Breadcrumb, Pagination, Stepper/Wizard, NavigationRail, BottomNav, Drawer,
  TreeView, Anchor/ScrollSpy, Disclosure/NavGroup

### 2.4 Widgets — Affichage de données
- ✅ Card, Badge, Accordion, Text/Heading ; (List via rows pattern)
- ❌ **Table / DataGrid** (tri, pagination, virtualisation, sélection) — le plus gros manque,
  Avatar (+ AvatarGroup), Chip/Tag (removable), Stat/Metric, Timeline, TreeView,
  Calendar, Code block, Kbd, Carousel, DescriptionList, Markdown render, RichText editor

### 2.5 Widgets — Overlays & surfaces
- ✅ Modal, Overlay, Toast
- ❌ Tooltip, Popover, HoverCard, DropdownMenu, ContextMenu, Sheet/Drawer, AlertDialog

### 2.6 Widgets — Feedback & statut
- ✅ Toast, Modal
- ❌ Alert/Banner inline, Progress (linéaire), Spinner/Progress circulaire, Skeleton loader,
  EmptyState, LoadingOverlay, Notification/NotificationCenter

### 2.7 Widgets — Typographie & média
- ✅ Text, Heading, Icon
- 🟡 Divider/Separator, Spacer (tokens ?)
- ❌ Link stylé, Code/Kbd, Blockquote, List (ul/ol), AspectRatio, Image (loading/fallback),
  ScrollArea stylée

### 2.8 Widgets — Audio (différenciateur vs design systems web)
> lumen-ui est né du contexte DAW (Seno). Aucun système web n'offre ça → **avantage unique**.
- ❌ Knob (rotatif), Fader/VerticalSlider, VU/Peak meter, Waveform display, Spectrum analyzer,
  XY pad, Step-sequencer grid, Piano keyboard, Transport controls, LevelBar, AB switch

### 2.9 Patterns
- ✅ DashboardLayout, Sidebar, CommandPalette, Toolbar/StatusBar (bars), LogPanel, Rows
- ❌ **Form layout**, SettingsPage, AuthCard/Login, MasterDetail, **DataTable pattern**,
  Wizard/Onboarding, NotificationCenter, Kanban, SplitView/ResizablePanels, InspectorPanel
  (audio), FilterBar, SearchResults, ErrorPage/EmptyState pattern

### 2.10 Layout
- ✅ Flex, Grid (cols fixes), Breakpoint/responsive
- ❌ Grille fractionnaire (`fr`/`auto`/`minmax`), grid-template-areas, Stack (+ divider/spacing),
  Sticky, Container (max-width), AspectRatio, Center, ResizableSplit/SplitPane, Masonry,
  z-index/layering tokens, ScrollArea wrapper tokenisé

### 2.11 Motion
- ✅ Easing, Spring, Fade
- ❌ Slide, Scale, Rotate, **enter/exit (mount/unmount)**, stagger (cascade de listes),
  layout animation (FLIP), keyframes/timeline, gesture/drag spring, AnimatePresence,
  presets de transition par composant, **respect `prefers-reduced-motion` (a11y)**

### 2.12 Thèmes
- ✅ Dark, Light, AudioDark, HighContrast
- ❌ Light « par défaut » poli, rampes sémantiques complètes (info/success/warning/danger),
  **theme builder / dérivation** documentée, auto `prefers-color-scheme`, transition de thème
  animée, overrides par composant, accent/brand customisable, sérialisation `.ron`
  (`tools/lumen-theme-gen` prévu jamais fait), contrast checker, presets (Nord, Solarized…)

### 2.13 Icônes
- ✅ 8 icônes painter
- ❌ tout le reste + **mécanisme d'import SVG** (cf. §6)

---

## 3. Roadmap versionnée v1.1 → v2.0

> Chaque version est publiable, ≤ scope raisonnable, et respecte « pas de vN+1 sans vN validée ».
> Ordre pensé pour lever les verrous d'abord, puis empiler le volume.

### v1.1 — Évolution du frozen core (l'activateur) · 1 sem
- **ADR-0009** : méthodes par défaut sur `Theme` → ajout de widgets non breaking.
- Exposer `Danger`/`Warning`/`Success` comme variantes complètes (boutons, badges, alerts).
- Helper `IconButton` (compose Button + Icon).
- **Sortie** : ajouter un widget thémé ne casse plus aucun thème.

### v1.2 — Overlays & feedback · 2 sem
- Tooltip, Popover, DropdownMenu, ContextMenu, HoverCard.
- Alert/Banner inline, Progress (linéaire + circulaire/Spinner), Skeleton, EmptyState.
- **Sortie** : feedback & menus contextuels complets, 1 ligne chacun.

### v1.3 — Data display · 2–3 sem
- **Table/DataGrid** (colonnes typées, tri, pagination, sélection ; virtualisation en option).
- Avatar (+ group), Chip/Tag removable, Stat/Metric, DescriptionList, Code/Kbd, Divider.
- **Sortie** : afficher un tableau de données = configuration déclarative.

### v1.4 — Inputs avancés & formulaires · 2–3 sem
- Textarea, NumberInput, RangeSlider, SegmentedControl, Combobox/Autocomplete, MultiSelect.
- DatePicker, TimePicker, ColorPicker, FileUpload.
- **Form** (label/field/erreur/aide/validation) — brique la plus demandée.
- **Sortie** : un formulaire validé complet sans état manuel.

### v1.5 — Icônes à l'échelle (pipeline SVG) · 1–2 sem
- **ADR-0008** + `xtask` codegen `usvg`+`lyon` → painter.
- Import d'un sous-ensemble curé de **Lucide** (~150–250 icônes), feature-gated par tailles.
- API inchangée (`Icon::name()`), couleur tokenisée préservée.
- **Sortie** : poser une icône courante ≈ aussi simple qu'en HTML.

### v1.6 — Motion v2 · 2 sem
- Slide/Scale/Rotate, **enter/exit (AnimatePresence)**, stagger, layout animation (FLIP),
  keyframes/timeline, respect `prefers-reduced-motion`.
- **Sortie** : montage/démontage et listes animés, 60 fps, a11y motion respectée.

### v1.7 — Layout v2 · 2 sem
- Grille fractionnaire + template-areas, Stack, Container, Sticky, AspectRatio,
  ResizableSplit/SplitPane, ScrollArea tokenisée.
- **Sortie** : parité CSS Grid/Flexbox pratique.

### v1.8 — Navigation & patterns métier · 2–3 sem
- Breadcrumb, Pagination, Stepper/Wizard, Drawer/Sheet, TreeView.
- Patterns : Form layout, SettingsPage, AuthCard, MasterDetail, DataTable pattern, FilterBar.
- **Sortie** : assembler une app CRUD complète en < 50 lignes.

### v1.9 — Thèmes v2 & outillage · 2 sem
- Theme builder / dérivation, rampes sémantiques, auto `prefers-color-scheme`,
  transition de thème animée, `tools/lumen-theme-gen` (`.ron` ↔ code, preview), presets.
- **Sortie** : créer/éditer un thème complet sans Rust.

### v1.10 — Crate audio `lumen-ui-audio` (différenciateur) · 3 sem
- Knob, Fader, VU/Peak meter, Waveform, Spectrum, XY pad, Transport, LevelBar.
- Validé sur une vraie app (Seno / un plugin CLAP).
- **Sortie** : construire un GUI de plugin audio sans réinventer les contrôles DSP.

### v2.0 — Consolidation & release · 2 sem
- Galerie WASM exhaustive, mdBook complet, snapshots `egui_kittest` sur tout le catalogue,
  audit WCAG 2.1 AA, benchmarks, gel SemVer, matrice compat egui, CHANGELOG.
- **Sortie** : catalogue complet, production-ready, intégrable par un tiers en < 15 min.

---

## 4. Synthèse de maturité (cible v2.0)

| Domaine | v1.0 | Manque = | Levier principal |
|---|---|---|---|
| Layout | 🟢 ~80% | volume mineur | v1.7 |
| Widgets | 🟡 ~55% | **volume** (table, overlays, inputs) | v1.2–v1.4 |
| Patterns | 🟡 ~45% | volume | v1.8 |
| Motion | 🟡 ~40% | volume + enter/exit | v1.6 |
| Thèmes | 🔴 ~25% | volume + builder | v1.9 |
| Icônes | 🔴 ~10% | **architecture (SVG)** | v1.5 |
| Audio | ⚪ 0% | tout (différenciateur) | v1.10 |

Tout est du **volume sur fondations existantes** sauf deux décisions d'archi (ADR-0009 trait,
ADR-0008 icônes). Une fois ces verrous levés, chaque widget suit Token→Recipe→Widget — le travail
répétitif où une IA briefée est efficace.

---

## 5. Contraintes transverses (à chaque version)

- **Frozen core** : toute nouvelle recette via méthode `Theme` par défaut (ADR-0009), jamais
  breaking.
- **API egui vérifiée** : docs.rs **+** compilation locale avant commit (risque #1 du projet).
- **a11y dès l'ajout** : AccessKit/screen-reader exposé (cf. commit #32), focus visible,
  cibles ≥ 44 px, `prefers-reduced-motion`.
- **Tests** : `egui_kittest` (snapshot + AccessKit) par widget ; PR ≤ 400 LOC, sinon découper.
- **AI_CONTEXT.md** tenu à jour par crate touchée ; `graphify update .` après extraction.

---

## 6. SVG → painter : conception détaillée

### 6.1 Faisabilité
Pipeline Rust éprouvé : **`usvg` → `lyon`/`kurbo` → `epaint`**.
- `usvg` parse, normalise, résout les transforms, convertit cercles/rects/lignes en paths
  absolus (Move/Line/Cubic/Quad/Close).
- `lyon`/`kurbo` aplatissent les beziers en polylignes (tolérance réglable) — ou on conserve les
  beziers et on émet `CubicBezierShape`/`QuadraticBezierShape`.
- egui dessine via `Shape::Path(PathShape)`, `CubicBezierShape`, ou `Painter::line_segment`.
- Prior art direct : `svg2polylines` (usvg + lyon).

### 6.2 Source recommandée — Lucide
- Licence **ISC** (permissive, attribution légère), ~1700 icônes.
- viewBox **24×24**, stroke **2px**, round caps/joins, `currentColor` → mappe sur
  `tokens().colors.text` exactement comme l'`Icon` actuel.
- **Même style** que les 8 icônes maison → continuité visuelle parfaite.

### 6.2bis Panorama complet des 6 familles de solutions d'icônes egui

| # | Approche | Net au zoom | Recolor thème | Poids | Dep runtime | Ajout d'1 icône | Multicolore |
|---|---|---|---|---|---|---|---|
| A | Painter manuel *(actuel)* | ✅ | ✅ | 0 | aucune | ❌ tracer à la main | ❌ |
| B | **Codegen SVG→painter** (`usvg`+`lyon`→Rust) | ✅ | ✅ | faible | **aucune** | ✅ ajouter à la liste | 🟡 |
| C | Runtime SVG→painter (parse au démarrage) | ✅ | ✅ | faible | ❌ `usvg` | ✅ | 🟡 |
| D | SVG→texture (`egui_extras`+`resvg`) | 🟡 flou si zoom | 🟡 tint global | moyen | ❌ `resvg` | ✅✅ `Image::new` | ✅ |
| E | **Police d'icônes** (`egui-phosphor`, `egui-material-icons`) | ✅ | ✅ | font 100-400 Ko | faible | ✅✅ un codepoint | ❌ |
| F | Atlas/spritesheet PNG | ❌ | 🟡 | image | aucune | 🟡 régénérer | ✅ |

**Arbitrage** : B et E sont les seuls candidats sérieux pour des icônes d'UI à l'échelle.
- **E (police)** = standard de facto egui, ajout trivial, mais **monochrome** et **embarque un asset
  font** — contredit le principe « no font asset » du projet ([icons/lib.rs:1](../crates/lumen-ui-icons/src/lib.rs)).
- **B (codegen)** = seule option qui garde **zéro asset ET zéro dep runtime** tout en passant à
  l'échelle, recoloration tokenisée native, continuité visuelle avec les 8 icônes maison.
- D réservé aux illustrations multicolores (logos, empty-states), pas aux icônes d'UI.

**La question décisive B vs E** : « no font asset » est-il un principe à conserver ? Oui → B. Non,
chemin le plus court → E.

### 6.3 Approche retenue — codegen build-time (`xtask`) · ADR-0008
Préserve la philosophie « no asset, no runtime dep, vectoriel net, couleur tokenisée ».
1. `xtask gen-icons` lit une **liste curée** de SVG (pas les 1700 → poids binaire).
2. `usvg` normalise → `lyon`/`kurbo` produit segments/beziers en coords normalisées 0..1.
3. Émet du Rust au même format que [icons/lib.rs `paint()`](../crates/lumen-ui-icons/src/lib.rs)
   (un bras `match IconKind` par icône, ou un tableau statique `&[PathCmd]` + un painter générique).
4. Build final : **aucune** dépendance `usvg`/`lyon`/`resvg` embarquée.

### 6.4 Alternative — runtime `resvg` (egui_extras image loader)
Rasterise le SVG en texture. Plus simple, mais : perd la couleur tokenisée fine, perd la netteté
au zoom, embarque `resvg` au runtime. **Non aligné** avec la philosophie painter. À réserver à des
illustrations complexes multicolores, pas aux icônes d'UI.

### 6.5 Décisions à acter dans l'ADR-0008
1. **Codegen vs runtime** → codegen (recommandé).
2. **Bezier vs polyligne** → conserver les beziers (`CubicBezierShape`) pour netteté à tout zoom ;
   sinon flatten à tolérance fine.
3. **Stroke vs fill** → démarrer stroke (Lucide). Le fill (Material Symbols) demande `PathShape`
   rempli + winding even-odd — supporté plus tard.
4. **Curation** → liste d'inclusion configurable (feature ou fichier manifest), pas le set complet.

### Sources
- [svg2polylines (usvg + lyon)](https://github.com/dbrgn/svg2polylines)
- [egui Shape / PathShape](https://docs.rs/egui/latest/egui/enum.Shape.html)
- [egui Painter](https://docs.rs/egui/latest/egui/struct.Painter.html)
- [usvg Path](https://docs.rs/usvg/latest/usvg/struct.Path.html)
- [Lucide — licence ISC](https://lucide.dev/license)
- [Lucide — guide de design (24×24, stroke 2px)](https://lucide.dev/contribute/icon-design-guide)
