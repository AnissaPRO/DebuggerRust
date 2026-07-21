# Debugger BPCE SI

## Présentation

Ce projet a été réalisé dans le cadre de mon alternance au sein de **BPCE SI** dans le contexte de la modernisation des outils internes.

L'objectif est de proposer une nouvelle version d'un visualiseur de journaux d'événements (logs) permettant aux équipes de développement et de support d'analyser rapidement les traces applicatives produites par le Portail utilisé par les conseillers bancaires.

L'application a été développée en **Rust** et repose sur une architecture modulaire séparant l'interface graphique des traitements métier.

---

## Fonctionnalités

- Ouverture de fichiers de logs au format JSON
- Lecture optimisée de fichiers volumineux
- Désérialisation automatique des événements
- Suppression des séquences ANSI
- Formatage des dates
- Filtrage par niveau de journalisation :
  - ERROR
  - WARN
  - INFO
  - DEBUG

- Interface graphique développée avec Slint
- Gestion robuste des erreurs
- Tests unitaires intégrés

---

### Module app

Contient l'interface graphique développée avec Slint.

Responsabilités :

- affichage des journaux ;
- gestion des interactions utilisateur ;
- application des filtres ;
- communication avec la bibliothèque métier.

### Module core_logger

Bibliothèque métier indépendante de l'interface graphique.

Responsabilités :

- lecture des fichiers ;
- désérialisation JSON ;
- suppression des séquences ANSI ;
- formatage des données ;
- filtrage des événements ;
- gestion des erreurs ;
- tests unitaires.

---

## Prérequis

Avant de compiler le projet, installer :

- Rust
- Cargo

Vérification :

```bash
rustc --version
cargo --version
```

---

## Compilation

Compilation du projet :

```bash
cargo build
```

L'exécutable généré est disponible dans :

```text
target/release/
```

---

## Exécution

Lancer l'application :

```bash
cargo run
```

---

## Exécution des tests

Le projet intègre un harnais de tests unitaires permettant de valider les principaux traitements métier.

Exécuter l'ensemble des tests :

```bash
cargo test
```

Les tests couvrent notamment :

- la lecture des fichiers ;
- la désérialisation JSON ;
- la suppression des séquences ANSI ;
- le filtrage des journaux ;
- la gestion des erreurs.

---

## Gestion des erreurs

L'application utilise un type d'erreur dédié permettant de traiter les situations suivantes :

- fichier introuvable ;
- erreur de lecture ;
- journal mal formaté ;
- erreur de désérialisation JSON.

Les lignes invalides sont ignorées afin de poursuivre l'analyse du reste du fichier.

---

## Sécurité et robustesse

Le projet bénéficie des garanties offertes par Rust :

- sécurité mémoire ;
- prévention des accès concurrents non sécurisés ;
- contrôle des références à la compilation ;
- limitation des erreurs d'exécution.

Cette approche contribue à améliorer la fiabilité du logiciel.

---

## Évolutions possibles

Les évolutions envisagées sont :

- recherche textuelle avancée ;
- filtres combinés ;
- export des résultats ;
- lecture de flux temps réel ;

---

## Auteur

Projet réalisé dans le cadre du titre :

**Expert en développement logiciel (RNCP 39583)**

Entreprise d'accueil : **BPCE SI**

Année : 2025–2026
