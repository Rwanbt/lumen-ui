# Roadmap finale (verrouillée) — lumen-ui

> **Cible** : egui 0.34.3 (juin 2026, MSRV Rust 1.92). Licence MIT OR Apache-2.0.
> **Statut** : version consolidée après deux passes de review multi-IA. Les décisions
> ci-dessous sont **arbitrées, pas ouvertes**. Ce document remplace toutes les versions
> précédentes du plan.
>
> **Nom** : `lumen-ui` (préfixe crates `lumen-*`). À confirmer libre sur crates.io au
> moment de réserver ; alternatives : `egui-lumen`, `aura-ui`.

---

## Changelog de cette version (vs le plan précédent)

| Décision | Statut | Raison |
|----------|--------|--------|
| Motion minimal en **v0.2** (pas v0.5) | ADOPTÉ (consensus IA + moi) | Sinon re-câblage de tous les widgets en v0.5 |
| Recettes paramétrées par `state` dès **v0.1** | ADOPTÉ | Évite un breaking change du trait `Theme` (couche la plus fondamentale) |
| `Density { Compact, Comfortable, Touch }` dans le contexte dès **v0.1** | ADOPTÉ (idée ChatGPT) | Évite un 2e paramètre rétro-ajouté aux recettes |
| **Workspace dès le jalon 0** (pas de monorepo plat à splitter en v0.4) | TRANCHÉ CONTRE les IA | Voir §Décision d'architecture |
| Springs/timelines (motion avancé) en **v0.5** | maintenu | Risque + coût ; le minimal de v0.2 suffit d'ici là |
| Corrections d'API egui 0.34 | APPLIQUÉ | Le code des IA ne compilait pas (voir §Corrections d'API) |

---

## Décision d'architecture : workspace dès le départ (contre l'avis de plusieurs IA)

Plusieurs IA recommandent un monorepo plat (un seul `src/`) à splitter en workspace en
v0.4. **Je tranche contre**, pour trois raisons :

1. Un workspace cargo avec peu de crates n'a aucun coût administratif tant qu'on ne publie
   pas (et on ne publie rien avant v0.9). Le `Cargo.toml` racine s'écrit une fois.
2. Splitter en v0.4 = deux refactos risqués empilés : le découpage en crates **ET**
   l'intégration d'`egui_taffy` (la dépendance la plus instable) au même moment.
3. Le compilateur force la clarté des frontières (`pub` vs `pub(crate)`) dès le début. Pour
   un design system, cette discipline précoce est un atout, pas un frein.

**Conséquence concrète** : workspace dès le jalon 0, mais on ne crée que les crates
nécessaires à chaque version. On commence avec `lumen-ui-core` + `lumen-ui-widgets` + façade
`lumen-ui`. Les autres crates naissent quand leur version arrive. **Pas de « grand split ».**

Voir [docs/adr/0001-workspace-from-day-zero.md](docs/adr/0001-workspace-from-day-zero.md).

---

## Corrections d'API egui 0.34 (le code des IA contenait des erreurs)

Vérifié sur docs.rs (egui 0.34) **et confirmé par compilation locale** lors du bootstrap :

- `egui::Button` n'a **PAS** `.padding()` ni `.shadow()`. Méthodes réelles : `.fill()`,
  `.stroke()`, `.corner_radius()`, `.small()`, `.frame(bool)`, `.min_size()`, `.sense()`,
  `.selected()`. → Le padding/l'ombre se gèrent en enveloppant le bouton dans un
  `egui::Frame` (`Frame` a `inner_margin`, `fill`, `stroke`, `corner_radius`, `shadow`).
- `Button<'a>` porte une durée de vie (atomes de contenu). Le wrapper lumen en tient compte
  (il dessine au moment du `add`, sans stocker le `Button`).
- Pas de styliser-avant-dessiner fiable : `response.hovered()` n'est connu qu'**après**
  allocation. Donc l'état hover/active utilisé pour la recette est celui de la **frame
  précédente** (lu via `ctx.read_response(id)`), et c'est `animate_value_with_time` qui
  lisse la transition — d'où le motion en v0.2.
- `Frame` est le bon véhicule pour ombre + padding + corner radius cohérents par recette.
- **Corrections supplémentaires constatées au bootstrap** : `IdTypeMap::get_persisted`
  exige `ctx.data_mut(...)` (pas `data`) ; `Context::style_mut` est renommé
  `global_style_mut` en 0.34.

---

## A. Architecture cible finale (crates)

```
lumen-ui/                       (workspace dès le jalon 0)
├── Cargo.toml                  (workspace : membres + versions partagées)
├── crates/
│   ├── lumen-ui-core/             Tokens, Density/UiContext, trait Theme, recettes, install()
│   ├── lumen-ui-motion/           Springs, easings, transitions, timelines (v0.5)
│   ├── lumen-ui-widgets/          Composants (consomment recettes ; motion minimal dès v0.2)
│   ├── lumen-ui-layout/           Wrappers egui_taffy + breakpoints (v0.4)
│   ├── lumen-ui-patterns/         Dashboard, Sidebar, Inspector, Settings, LogPanel (v0.6)
│   ├── lumen-ui-icons/            Set d'icônes (feature icons) (v0.7)
│   ├── lumen-ui-themes/           Dark, Light, AudioDark, HighContrast (v0.7)
│   ├── lumen-material/         Adaptateurs egui-material3 (feature material) (v0.8)
│   └── lumen-ui/               Façade : re-export, prélude, features flags
├── tools/lumen-theme-gen/      CLI génération/édition de thèmes (v0.7)
├── examples/                   minimal, gallery, dashboard, audio_plugin, responsive
├── docs/                       mdBook
├── assets/ · .github/workflows/ · README · CHANGELOG · CONTRIBUTING
└── LICENSE-MIT / LICENSE-APACHE
```

### Features de la façade `lumen-ui`

```toml
[features]
default  = ["theme", "widgets", "themes"]
tokens   = ["dep:lumen-ui-core"]
theme    = ["tokens"]
motion   = ["theme", "dep:lumen-ui-motion"]
widgets  = ["theme", "dep:lumen-ui-widgets"]        # motion minimal intégré sans dep lourde
layout   = ["theme", "dep:lumen-ui-layout"]         # tire egui_taffy
patterns = ["widgets", "layout", "dep:lumen-ui-patterns"]
icons    = ["dep:lumen-ui-icons"]
themes   = ["theme", "dep:lumen-ui-themes"]
material = ["widgets", "dep:lumen-material"]      # tire egui-material3
serde    = ["lumen-ui-core/serde"]
full     = ["widgets", "motion", "layout", "patterns", "icons", "themes", "serde"]
```

> **État au bootstrap** : seules `tokens`, `theme`, `widgets`, `serde`, `full` (réduit)
> sont branchées, car seules `lumen-ui-core` et `lumen-ui-widgets` existent. Les autres features
> sont ajoutées à la façade quand leur crate naît.

---

## B. Roadmap versionnée (chaque version est publiable et utilisable)

### v0.1 — Socle fondateur (1–2 j) — **EN COURS / bootstrap fait**

- `lumen-ui-core` : `Tokens`, `Density`/`UiContext`, trait `Theme` avec **recettes
  paramétrées par `(variant, state, ctx)`** dès maintenant, `install()`, `UiThemeExt`.
- `apply_to_ctx` mappe les tokens sur `Style`/`Visuals`/`Spacing`/`Widgets`.
- Thème `DarkTheme`. Widget `Button` de validation (via `Frame` + `Button`).
- Workspace + CI verte. `examples/minimal.rs`.
- **Validation obligatoire sur une app réelle existante.**
- **Sortie** : changer de thème change toute l'app sans toucher la logique.

### v0.2 — Widgets fondamentaux + motion minimal (2–3 sem)

- Button (Primary/Secondary/Ghost/Danger), Input/TextField, Card, Badge, Switch, Checkbox,
  RadioGroup, Slider, Label, Heading.
- États : Normal/Hovered/Active/Disabled, lus sur la frame précédente.
- **Motion minimal intégré** : interpolation couleur/opacité via `ctx.animate_value_with_time`
  + helper `lumen_ui_core::anim::lerp_color`. Pas de dep lourde.
- `examples/gallery.rs` avec switch de thème live.
- **Sortie** : 10 widgets cohérents, transitions hover/focus fluides, zéro état dans `App`.

### v0.3 — Composants composés & headless (2–3 sem)

- Modal, Toast (file + auto-dismiss), Tabs, Dropdown/Select, Tooltip, Accordion, ContextMenu,
  Popover. État dans `ctx.data` via `Id`, dessin délégué au thème.
- `examples/dashboard.rs`.
- **Sortie** : ouvrir une modale/un toast = 1 ligne, sans booléen externe.

### v0.4 — Layout CSS & responsive (2–4 sem)

- `lumen-ui-layout` : wrappers sur `egui_taffy` (row/column/grid, `.gap`, `.justify_*`,
  `.align_*`, `.grow`), activation `max_passes = 2`.
- Breakpoints `Xs..Xl` + `responsive(ui, |bp| …)`.
- `examples/responsive.rs`.
- **Sortie** : un layout flex/grid survit au resize sans code manuel.

### v0.5 — Moteur d'animation avancé (3–4 sem)

- `lumen-ui-motion` : springs (stiffness/damping/mass), easings (cubic-bezier), transitions
  (Fade/Slide/Scale), timelines. Les widgets v0.2 basculent leur motion minimal sur
  `lumen-ui-motion` **sans changer leur API publique** (tout l'intérêt d'avoir prévu les états
  dès v0.1).
- **Sortie** : transitions fluides 60 fps, < 1 ms/frame.

### v0.6 — Patterns métier (2–3 sem)

- `lumen-ui-patterns` : DashboardLayout (sidebar/content/inspector), Sidebar, InspectorPanel
  (paramètres audio), SettingsPage, LogPanel, Toolbar, StatusBar, CommandPalette.
- `examples/dashboard.rs` + `audio_plugin.rs`.
- **Sortie** : structurer une nouvelle app = < 20 lignes.

### v0.7 — Thèmes, icônes & outillage (2–3 sem)

- `lumen-ui-themes` (Dark/Light/AudioDark/HighContrast + signature), `lumen-ui-icons`,
  `tools/lumen-theme-gen` (CLI `.ron` ↔ code, preview live).
- **Sortie** : créer un thème complet sans écrire de Rust.

### v0.8 — Accessibilité & adaptateurs (2–3 sem)

- Contraste AA (AccessKit présent dans egui), navigation clavier complète, focus visible,
  cibles tactiles ≥ 44 px. Feature `material` finalisée.
- **Sortie** : audit WCAG 2.1 AA passé sur tous les composants.

### v0.9 — Doc, exemples, polish (2 sem)

- mdBook complet, galerie WASM en ligne (GitHub Pages), screenshots/GIFs, benchmarks.
- **Gel des API publiques.**
- **Sortie** : intégration par un tiers en < 15 min via la doc seule.

### v1.0 — Release stable

- API gelée, SemVer strict, CHANGELOG, matrice compat egui. Publication coordonnée crates.io.
  Annonces (r/rust, forum egui, This Week in Rust).
- **Sortie** : production-ready, utilisé en réel par au moins une de tes apps.

### Post-1.0 (backlog)

- Vrai backdrop-blur (offscreen + shader, feature `blur`, backend wgpu).
- DataTable (tri/pagination/virtualisation), DatePicker, RichText editor.
- Thèmes communautaires. Intégration nih-plug documentée (plugins audio).

---

## C. Détail technique (signatures corrigées pour egui 0.34)

Les signatures de référence vivent désormais dans le code (`crates/lumen-ui-core/`,
`crates/lumen-ui-widgets/`) qui **compile, passe clippy `-D warnings` et `fmt --check`**.

- **C.1 Tokens & contexte** → [`lumen-ui-core/src/tokens.rs`](crates/lumen-ui-core/src/tokens.rs),
  [`context.rs`](crates/lumen-ui-core/src/context.rs)
- **C.2 Recettes paramétrées par état** → [`recipe.rs`](crates/lumen-ui-core/src/recipe.rs)
- **C.3 Trait `Theme` + injection persistante** → [`theme.rs`](crates/lumen-ui-core/src/theme.rs)
- **C.4 Widget `Button` (Frame + Button)** → [`lumen-ui-widgets/src/button.rs`](crates/lumen-ui-widgets/src/button.rs)
- **C.5 Motion minimal** → [`anim.rs`](crates/lumen-ui-core/src/anim.rs)
- **C.6 Springs (v0.5)** : bascule transparente — les widgets appellent un helper
  `lumen_ui_motion::value(...)` qui délègue à `animate_value_with_time` en v0.2 et utilise le
  solveur de ressort en v0.5 ; **même signature, aucun changement d'API publique des widgets**.

---

## D. Qualité, tests, CI/CD

- **CI** : `fmt --check`, `clippy -D warnings`, `test --workspace --all-features`, build de
  tous les exemples, build doc, matrice Linux/macOS/Windows + WASM.
- **Tests visuels** : `egui_kittest` (snapshots) sur les widgets clés.
- **Bench** : `criterion` sur coût/frame des animations et du layout taffy.
- **Release** : `cargo release`, ordre des dépendances
  `core → motion → widgets/layout → themes/icons → patterns → material → façade`.
- **Doc** : mdBook sur GitHub Pages + docs.rs.

---

## E. Registre des risques

| Risque | Impact | Mitigation |
|--------|--------|------------|
| API egui inventée par génération de code | Élevé | Toute signature vérifiée sur docs.rs **et par compilation** avant commit ; CI casse tôt |
| Breaking changes egui (3 mineures/an) | Élevé | Couche d'adaptation unique dans `lumen-ui-core`, pin strict, matrice compat |
| `egui_taffy` instable | Moyen | Isolé feature `layout` + wrapper maison ; v0.4 seulement |
| Hover/active connus après dessin | Moyen | État lu sur frame N-1 + lissage `animate_value_with_time` (résolu C.4/C.5) |
| Sur-périmètre | Moyen | Critères de sortie verrouillés ; pas de v0.N+1 sans v0.N validée |
| Nom pris sur crates.io | Faible | Vérifier `lumen-ui` ; alternatives `egui-lumen`, `aura-ui` |

---

## F. Décisions arbitrées (résumé pour la prochaine review)

1. **Motion minimal v0.2 + recettes par état dès v0.1** → adopté du retour des IA.
2. **Density dès v0.1** → adopté (ChatGPT).
3. **Workspace dès le jalon 0, pas de monorepo-à-splitter** → tranché contre la majorité des
   IA, argumenté (§Décision d'architecture, ADR-0001).
4. **API egui 0.34 corrigée** (Button sans padding/shadow → Frame ; lifetime ; hover en frame
   N-1 ; `data_mut` ; `global_style_mut`) → le code des IA ne compilait pas ; corrigé et
   vérifié par compilation.

Ces quatre points sont les seuls écarts par rapport au consensus. Tout le reste du plan est
stable et validé par les deux passes.
