# common_graph

`common_graph` fournit les fondations structurelles communes aux graphes Rust de l’écosystème BezotCorp.

## Objectifs

- fournir des identifiants fortement typés pour les nœuds et les arêtes ;
- générer les identifiants internes avec UUIDv7 ;
- encapsuler entièrement la dépendance `uuid` ;
- empêcher le mélange entre `NodeId` et `EdgeId` ;
- fournir des bases communes réutilisables ;
- permettre la composition de plusieurs formes de graphes ;
- rester indépendant de tout domaine métier.

## Types actuels

- `NodeId`
- `EdgeId`
- `NodeBase`
- `EdgeBase`
- `OrientedEdge`

Les nœuds métier, les relations métier et leurs enums doivent être définis dans les crates consommatrices.

## Utilisation dans le workspace

Ajoutez la dépendance commune au workspace, puis utilisez :

`common_graph.workspace = true`

Les consommateurs utilisent les types de `common_graph` sans dépendre directement du crate `uuid`.

## Principes

- l’identité interne des nœuds et des arêtes repose sur UUIDv7 ;
- `NodeId` et `EdgeId` restent des types Rust distincts ;
- la représentation UUID reste interne à la bibliothèque ;
- `EdgeBase` n’impose aucune orientation ;
- les formes d’arêtes sont construites avec des structs composées ;
- les connaissances métier ne doivent pas entrer dans cette bibliothèque.

## Développement

Commandes principales :

- `cargo fmt`
- `cargo check`
- `cargo test`

## Branches

- `main` : version stable ;
- `dev` : intégration ;
- `feature/graph` : développement des fondations du graphe.

## Licence

Ce projet est distribué sous Apache License 2.0.

Consultez les fichiers `LICENSE` et `NOTICE`.
