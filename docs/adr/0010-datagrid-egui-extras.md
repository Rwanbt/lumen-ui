# ADR-0010 : DataGrid — adopter `egui_extras::TableBuilder` ou le réécrire à la main ?

**Date** : 2026-06-16 | **Statut** : **Accepté — Option A** (arbitré 2026-06-16) | Auteur : session v1.8

> **Décision actée** : Option A (`egui_extras::TableBuilder`). Réponses aux questions ouvertes,
> retenues comme plan (réversibles tant que le code n'est pas écrit) :
> 1. **Feature-gate `datagrid`** — oui (défaut « zéro dep runtime » préservé).
> 2. `DataGrid` dans **`lumen-ui-widgets`** sous la feature `datagrid`.
> 3. Tri : **`SortState` émis**, l'appelant trie ses données.
> 4. Cellules : **`String` d'abord** (parité `Table`), cellule riche (closure `FnMut(&mut Ui)`)
>    dans un lot ultérieur.
>
> Implémentation différée : on enchaîne d'abord un autre lot v1.8 (Drawer / TreeView / Wizard).

## Contexte

La v1.3 livre un [`Table`](../../crates/lumen-ui-widgets/src/table.rs) bâti sur `egui::Grid` :
en-tête + lignes de cellules `String`, zébrage optionnel, thémé via `TableRecipe`. Sa portée est
explicitement limitée — cellules statiques, **pas de tri, pas de largeurs de colonnes, pas de
virtualisation**. Pour des données paginées on appaire avec `Pagination`.

La roadmap ([ROADMAP-v2-catalogue.md](../ROADMAP-v2-catalogue.md) §2.4) qualifie le **DataGrid**
(tri, sélection, pagination, **virtualisation**) de « plus gros manque » du catalogue. La v1.3 le
liste comme `[ ] DataGrid (tri/virtualisation)`. Avant d'écrire la moindre ligne, il faut trancher :
**construit-on le DataGrid sur `egui_extras::TableBuilder`, ou le réécrit-on à la main ?**

### État des dépendances (vérifié)
- `egui_extras` **n'est pas** une dépendance du workspace. Seul `egui_taffy 0.12` l'est, **isolé
  derrière la feature `layout`** — précédent : on accepte une crate de l'écosystème egui derrière
  un flag plutôt que dans le cœur.
- L'[ADR-0004](0004-msrv-egui-pin.md) pin `egui = 0.34.3` strictement (egui casse ~3× / an) ;
  toute dépendance liée à la version d'egui élargit la surface de breaking changes à suivre.

### Ce que `TableBuilder` apporte réellement (vérifié docs.rs egui_extras 0.34)
- Dimensionnement de colonnes : `Column::auto/remainder/exact/initial/clip`, colonnes
  **redimensionnables** à la souris, en-tête **sticky**.
- **Virtualisation** du corps : `body.rows(height, total_rows, |row| …)` ne rend que les lignes
  visibles → indispensable au-delà de quelques centaines de lignes.
- Zébrage, sélection de ligne (via `row.set_selected` + réponse cliquable).
- **Ne fournit PAS le tri** : trier reste la logique de l'appelant (trier le `Vec`, re-render).
  `TableBuilder` ne fait que *rendre* ; un en-tête « triable » est un bouton qui émet un état de tri.

## Décision proposée (à arbitrer)

**Option A — Adopter `egui_extras::TableBuilder`, derrière une feature, pour un widget `DataGrid`
distinct du `Table` actuel. (RECOMMANDÉE)**

- Nouvelle dépendance `egui_extras = "0.34.3"` (compat egui 0.34.3 vérifiée), **feature-gated**
  (`datagrid`), comme `egui_taffy` l'est derrière `layout`. Le défaut « zéro dep runtime » reste
  intact pour qui n'active pas la feature.
- `DataGrid` vit dans `lumen-ui-widgets` (feature `datagrid`) ou un module dédié. Le `Table`
  `Grid` actuel **reste** pour les petits tableaux statiques (pas de breaking change).
- Thématisation : on enveloppe les cellules avec une recette pure `DataGridRecipe::resolve(tokens,
  ctx)` (pattern ADR-0009) — `TableBuilder` ne lit pas nos tokens, on applique couleurs/tailles
  cellule par cellule comme le fait déjà `Table`.
- Tri : `DataGrid` expose des en-têtes triables et **émet un `SortState` (colonne + sens)** ;
  l'appelant trie ses données. Sélection via un `&mut Option<usize>` / `&mut Selection`.

*Pourquoi recommandée* : la virtualisation EST la raison d'être d'un DataGrid. La réécrire
correctement (fenêtrage + colonnes resizables + header sticky) est précisément le genre de
complexité profonde qu'il vaut mieux déléguer à une crate maintenue — `egui_extras` est le choix
canonique de l'écosystème, déjà précédé par `egui_taffy`. Coût marginal réel : +1 dep liée à egui.

## Alternatives rejetées (ou à retenir si tu tranches autrement)

**Option B — Réécrire à la main sur `egui::Grid` + `ScrollArea::show_rows`.**
- ✅ Zéro nouvelle dépendance, contrôle total du thème, fidèle à la philosophie « no dep runtime ».
- ❌ `ScrollArea::show_rows` donne la virtualisation des lignes à bon compte, mais **colonnes
  redimensionnables + header sticky + dimensionnement auto** sont non triviaux à réimplémenter et
  porteurs de bugs. On posséderait tout ce code (maintenance, edge cases d'alignement déjà vus sur
  le `Table` actuel). Réinvente ce qu'`egui_extras` fait déjà bien.

**Option C — Ne pas faire de DataGrid maintenant : enrichir le `Table` actuel (tri + pagination),
différer la virtualisation.**
- ✅ Zéro dep, livre de la valeur immédiate (en-têtes triables + données paginées) — couvre la
  majorité des cas (petits/moyens tableaux).
- ❌ Pas de virtualisation → inadapté à 10 k+ lignes ; le « plus gros manque » n'est que
  partiellement traité. Reporte la vraie décision.

## Conséquences (si Option A retenue)

- +1 dépendance (`egui_extras`) liée à la version d'egui → à inscrire au registre des risques de
  [ROADMAP.md](../../ROADMAP.md) et à la matrice de compat egui (v2.0).
- Asymétrie assumée : `Table` (Grid, statique, défaut) vs `DataGrid` (egui_extras, feature
  `datagrid`, données dynamiques). À documenter (quand utiliser lequel).
- `DataGridRecipe` pure (ADR-0009), testable sans thème ; tests comportementaux egui_kittest
  (tri émis, sélection routée). PR estimée > 400 LOC → à découper (recette+rendu, puis tri+sélection).

## Questions ouvertes (pour l'arbitrage)
1. **Feature-gate** `datagrid` (recommandé) ou dépendance ferme dans `widgets` ?
2. `DataGrid` dans `lumen-ui-widgets` ou un module/crate dédié ?
3. Tri : émettre un `SortState` (appelant trie) — OK ? Ou DataGrid trie lui-même des cellules
   `String` (limité, mais zéro travail appelant) ?
4. Cellules : `String` only (comme `Table`) au départ, ou cellule = closure `FnMut(&mut Ui)` pour
   des cellules riches (badges, boutons) dès la v1 ?

## Délai de review : à trancher en séance avant tout code DataGrid.
