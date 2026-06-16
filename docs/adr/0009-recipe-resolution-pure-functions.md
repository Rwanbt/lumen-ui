# ADR-0009 : Résolution des recettes par fonctions pures (tokens-as-data) pour l'expansion du catalogue

**Date** : 2026-06-16 | **Statut** : Accepté

## Contexte

Le trait `Theme` ([theme.rs](../../crates/lumen-ui-core/src/theme.rs)) expose **une méthode par
famille de widget** (`button_recipe`, `card_recipe`, `badge_recipe`, …), sans implémentation par
défaut. Conséquence : ajouter un widget thémé = ajouter une méthode au trait = **breaking change**
pour les 4 thèmes internes (`DarkTheme`, `LightTheme`, `audio_dark`, `high_contrast`) et tout thème
tiers. L'expansion du catalogue v2 (cf. [ROADMAP-v2-catalogue.md](../ROADMAP-v2-catalogue.md), des
dizaines de widgets) est impossible ainsi.

Constat décisif en lisant le code : **les thèmes ne contiennent aucune logique de style.** Toute la
résolution recette-depuis-tokens vit dans un seul endroit, [builder.rs](../../crates/lumen-ui-core/src/builder.rs),
paramétré par `(tokens, emphasis_fn, …)`. Un thème ne fournit que sa **palette** et sa **direction
d'emphase** (`lighten` en dark, `darken` en light). `PaletteTheme` le prouve : un thème est
**entièrement déterminé par `(Tokens, ThemeMode)`**. Aucun des 4 thèmes n'exploite la capacité du
trait à surcharger la *logique* d'une recette — seules les couleurs (via tokens) et l'emphase
diffèrent. La flexibilité « override logique par thème » est donc spéculative et non utilisée.

C'est exactement le modèle des design systems web pro (Tailwind, shadcn, Radix Themes, Material 3,
standard W3C Design Tokens) : **le thème est de la DATA** (tokens), les recettes sont calculées par
des **fonctions pures**, les composants consomment les tokens sans connaître le thème.

## Décision

Pour les **nouveaux** widgets du catalogue, la résolution de recette passe par des **fonctions
pures publiques**, pas par de nouvelles méthodes du trait `Theme` :

```rust
// recipe.rs — la logique de résolution est attachée à la recette, publique et pure :
impl TooltipRecipe {
    pub fn resolve(tokens: &Tokens, mode: ThemeMode, state: WidgetState, ctx: &UiContext) -> Self {
        let emph = mode.emphasis();   // pub(crate), interne au cœur
        // … dérive depuis les tokens …
    }
}

// widget — consomme la fonction pure, sans toucher le trait :
let r = TooltipRecipe::resolve(ui.theme().tokens(), ui.theme().mode(), state, &ui.ui_ctx());
```

**Un seul ajout au trait** : `fn mode(&self) -> ThemeMode`, **avec implémentation par défaut** par
heuristique de luminance du fond (`relative_luminance(background) > 0.5 → Light`). Additif et
non-breaking : les 4 thèmes existants héritent du défaut sans changer une ligne. `PaletteTheme`
surcharge `mode()` pour honorer son mode explicite (cas des thèmes à fond atypique).

`ThemeMode::emphasis()` passe `pub(crate)` pour que les fonctions `*::resolve` (situées dans le
cœur) y accèdent. Le type `Emphasis` reste interne (jamais exposé dans une signature publique).

### Propriétés obtenues
- Ajouter un widget = struct `Recipe` + `Recipe::resolve()` + le widget. **0 changement au trait,
  0 changement aux 4 thèmes, 0 breaking change.**
- Modèle « web-like » : tokens (data) → fonction pure → widget.
- Les 7 recettes **existantes** restent inchangées (méthodes de trait) — pas de migration risquée
  du frozen core dans le même lot. Une unification ultérieure (donner aussi un défaut aux 7
  recettes, réduire les thèmes à `(tokens, mode)`) est possible plus tard, derrière son propre ADR.

## Alternatives rejetées

- **Méthode par recette dans le trait, avec défaut** (« option 1 » initiale) : additif et non-breaking
  aussi, mais fait grossir le trait à chaque widget et reste du *code* plutôt que de la *data*. La
  résolution par fonction pure est plus web-like et ne touche pas le trait du tout.
- **Registre générique** `fn recipe(kind, state, ctx) -> ComponentRecipe` (« option 2 ») : perd le
  typage par recette (type union large / `Box<dyn Any>`), gros `match` verbeux, moins découvrable
  pour l'IA, casse la cohérence des 7 recettes typées. Rejeté.
- **Migration immédiate des 7 recettes existantes vers fonctions pures** (réduire les thèmes à
  `(tokens, mode)`) : c'est le point d'arrivée idéal, mais c'est un refactor du frozen core à isoler
  dans son propre ADR/lot, pas à empiler avec l'ajout du catalogue (règle : isoler migration et
  feature — [AGENTS.md](../../AGENTS.md)).
- **`mode()` sans méthode de trait, déduit côté widget** depuis la luminance : empêcherait un thème
  (ex. `PaletteTheme`) de déclarer un mode explicite. Rejeté au profit d'une méthode de trait avec
  défaut surchargeable.

## Conséquences

- Le frozen core ne bouge quasiment pas (1 méthode additive). Risque minimal sur les thèmes.
- Légère asymétrie temporaire : widgets historiques via `ui.theme().xxx_recipe(...)`, nouveaux
  widgets via `XxxRecipe::resolve(...)`. Documentée ; unifiable plus tard (ADR dédié).
- `Tokens` reste l'unique source de vérité ; toute recette en dérive par fonction pure (DRY,
  testable sans thème — `XxxRecipe::resolve` est testable en isolation).
- Validation : `cargo test -p lumen-ui-core` reste vert (21 tests + tests `mode()`), thèmes
  inchangés.
