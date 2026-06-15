# ADR-0006 : Accessibilité — audit de contraste WCAG AA automatisé, portée « état au repos »

**Date** : 2026-06-15 | **Statut** : Accepté

## Contexte

Critère de sortie de v0.8 : « audit WCAG 2.1 AA passé sur tous les composants ». Il fallait
décider **où** vit la logique de contraste, **quelles paires** sont auditées, et **à quel état
d'interaction** la conformité AA est garantie.

Un point de tension concret : en mode sombre, les états hover/active **éclaircissent** le fill
(`emph = lighten`), ce qui *réduit* le contraste d'un texte clair posé dessus. Un bleu de marque
vibrant avec texte blanc ne peut pas rester à 4.5:1 à travers un éclaircissement de 18 %.

## Décision

1. **Logique dans `lumen-ui-core::a11y`** (fonctions pures) : `relative_luminance`, `contrast_ratio`,
   `meets`/`meets_aa`, `ContrastLevel`, et `audit_colors(&Colors) -> AuditReport`. Aucune
   dépendance au trait `Theme` — l'audit travaille sur la palette.
2. **Audit automatisé en test** : chaque thème intégré (`DarkTheme`, `LightTheme`, `audio_dark`,
   `high_contrast`) a un test qui échoue si une paire texte/fond passe sous AA. Un thème
   inaccessible est attrapé par la CI, pas par un utilisateur qui ne peut pas lire l'écran.
3. **Paires auditées** : toutes celles où un widget peint du texte sur un fill — `text`/`text_muted`
   sur background et surfaces, et chaque `on_*` sur son fill sémantique (labels de boutons, badges).
4. **Portée = état au repos (Normal)**, tenu à **AA (4.5:1)** avec marge (cible ~5.0:1 pour les
   fills sombres) afin que le hover reste proche d'AA. C'est la portée standard de « WCAG AA sur
   tous les composants » ; garantir AA y compris sur l'état `Active` transitoire (le plus
   éclairci) imposerait des accents nettement plus ternes. `text_muted` est tenu à AA (contenu
   réel : légendes, labels secondaires), pas exempté comme du texte désactivé.

Conséquence directe : ajustement des palettes intégrées pour atteindre AA au repos —
DarkTheme `primary`/`danger`, LightTheme `success`/`warning`, audio_dark `danger`.

## Alternatives rejetées

- **Garantir AA dans tous les états (hover/active inclus)** : force des accents très désaturés
  (`primary` → bleu ardoise foncé), régression esthétique forte sur le thème phare pour un gain
  marginal sur un état transitoire. Les utilisateurs exigeant AAA peuvent définir un `PaletteTheme`.
- **Inverser l'emphase (assombrir au hover en mode sombre)** : changement transverse du `builder`,
  comportement contre-intuitif (bouton qui s'assombrit dans une UI sombre).
- **Logique de contraste dans un crate séparé `lumen-a11y`** : sur-découpage ; c'est de la math
  pure sur `Color32`, sa place est dans la couche fondation.

## Conséquences

- Les 4 thèmes intégrés sont garantis AA au repos, vérifié en CI.
- Les auteurs de thèmes tiers peuvent réutiliser `audit_colors` pour valider leurs palettes.
- La portée (repos vs tous états) est explicite : pas de fausse promesse AAA.
