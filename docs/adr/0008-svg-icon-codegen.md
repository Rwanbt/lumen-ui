# ADR-0008 : Pipeline d'icônes SVG → painter par codegen build-time

**Date** : 2026-06-16 | **Statut** : Accepté

## Contexte

`lumen-ui-icons` v1.0 contient **8 icônes dessinées à la main** au painter egui
([icons/lib.rs](../../crates/lumen-ui-icons/src/lib.rs)) : chaque icône est une fonction `paint()`
qui appelle `line_segment`/`circle_stroke` sur des coordonnées normalisées 0..1. Ce choix garantit
trois propriétés que le projet veut préserver :

1. **No asset** — aucune police ni fichier embarqué (commentaire ligne 1 du module).
2. **No runtime dep** — aucune bibliothèque SVG/raster dans le binaire final.
3. **Vectoriel + tokenisé** — net à toute taille, couleur par défaut = `tokens().colors.text`.

Le coût : ajouter une icône = tracer ses coordonnées à la main. Cela ne passe pas l'échelle
au-delà de ~30 icônes, alors que le catalogue v2 (cf. [ROADMAP-v2-catalogue.md](../ROADMAP-v2-catalogue.md))
vise la parité pratique avec un set web (Lucide ≈ 1700 icônes). Il faut un mécanisme d'import
d'icônes SVG qui **conserve les trois propriétés ci-dessus**.

Six familles de solutions ont été évaluées (détail §6.2bis de la roadmap) : painter manuel (actuel),
codegen SVG→painter, runtime SVG→painter, SVG→texture (`resvg`), police d'icônes
(`egui-phosphor`…), atlas PNG.

## Décision

**Génération de code au build-time** via un utilitaire `xtask gen-icons` :

1. Une **liste curée** de fichiers SVG (sous-ensemble de Lucide — licence ISC, viewBox 24×24,
   stroke 2px, `currentColor` ; même style que les 8 icônes maison) est la source.
2. `usvg` parse, normalise et résout les transforms → paths absolus
   (Move/Line/Cubic/Quad/Close), cercles/rects convertis en paths.
3. `lyon`/`kurbo` conservent les beziers (émis en `CubicBezierShape`) ou les aplatissent à
   tolérance fine selon le cas.
4. L'outil **émet du code Rust** au même format que la fonction `paint()` existante (un bras de
   `match IconKind` par icône, ou un tableau statique de commandes + un painter générique), en
   coordonnées normalisées 0..1.
5. L'API publique (`Icon::name()`, `.size()`, `.color()`) et la tokenisation de couleur restent
   **inchangées**.

**Conséquence clé** : `usvg`/`lyon`/`kurbo` ne sont des dépendances que de l'`xtask` (outil de dev),
**jamais du crate publié `lumen-ui-icons`**. Le binaire final n'embarque aucune dépendance SVG.

Sous-décisions actées :
- **Bezier conservé** par défaut (netteté à tout zoom) ; flatten seulement si nécessaire.
- **Stroke d'abord** (Lucide est stroke-only). Le fill (Material Symbols : `PathShape` rempli +
  winding even-odd) est différé.
- **Curation configurable** (manifest d'inclusion / feature par familles), jamais le set complet
  embarqué — maîtrise du poids binaire.

Cible : implémentation en v1.5 de la roadmap.

## Alternatives rejetées

- **Runtime SVG→texture (`egui_extras` + `resvg`)** : rasterise en texture. Rejeté pour des icônes
  d'UI car perd la netteté au zoom, la recoloration n'est qu'un tint global, et embarque `resvg`
  au runtime — viole « no runtime dep ». Réservé éventuellement aux illustrations multicolores
  (logos, empty-states), hors périmètre icônes.
- **Police d'icônes (`egui-phosphor`, `egui-material-icons`)** : standard de facto egui, ajout
  trivial (un codepoint), vectoriel et recolorable. Rejeté car **monochrome only** et surtout
  **embarque un asset font** — viole le principe « no font asset » fondateur du module. C'était le
  concurrent le plus sérieux ; le seul critère décisif a été la volonté de conserver ce principe.
- **Runtime SVG→painter (parse au démarrage)** : garde le vectoriel et la tokenisation, mais
  embarque `usvg` au runtime et paie un coût de parsing à l'init — inutile quand le codegen produit
  le même résultat sans dépendance ni coût runtime.
- **Atlas/spritesheet PNG** : pixelisé au zoom, recoloration limitée — inadapté à des icônes
  vectorielles nettes.
- **Continuer le painter manuel** : ne passe pas l'échelle ; c'est précisément le problème à
  résoudre.

## Conséquences

- **Principes préservés** : le crate publié reste sans asset et sans dépendance runtime ; les
  icônes restent vectorielles et tokenisées. Continuité visuelle parfaite avec les 8 icônes
  existantes (même style stroke).
- **Passage à l'échelle** : ajouter une icône = ajouter une entrée à la liste curée + régénérer —
  plus de tracé manuel.
- **Nouvelle surface outillage** : un `xtask`/`tools/lumen-icon-gen` à maintenir, avec ses deps de
  dev (`usvg`, `lyon`/`kurbo`). Le code généré est commité (reproductible, diffable, auditable).
- **Attribution** : la licence ISC de Lucide impose de conserver l'avis de licence ; à documenter
  dans `lumen-ui-icons` (NOTICE / en-tête du fichier généré).
- **Poids binaire maîtrisé** par la curation ; pas d'explosion vers 1700 icônes par défaut.
- **Limite assumée** : multicolore/gradients non couverts par ce pipeline (stroke/fill simple
  uniquement) — acceptable pour des icônes d'UI.

## Sources

- [svg2polylines — prior art usvg + lyon](https://github.com/dbrgn/svg2polylines)
- [egui Shape / PathShape](https://docs.rs/egui/latest/egui/enum.Shape.html)
- [usvg](https://docs.rs/usvg/latest/usvg/)
- [Lucide — licence ISC](https://lucide.dev/license) · [guide de design 24×24/2px](https://lucide.dev/contribute/icon-design-guide)
