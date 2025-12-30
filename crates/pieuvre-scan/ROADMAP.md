# ROADMAP pieuvre-scan (Vision SOTA)

## Phase 1 : Consolidation (v0.8.x) - [EN COURS]
- [x] Intégration `yara-x` native.
- [x] Acquisition `SeDebugPrivilege`.
- [x] Forensique navigateur de base (Chrome/Firefox).
- [ ] Support complet des raccourcis `.lnk` via `parselnk`.
- [ ] Analyse des tâches planifiées via parsing XML direct.

## Phase 2 : Intelligence & Heuristique (v0.9.0)
- [ ] **Moteur de Réputation Local** : Bloom Filter basé sur MalwareBazaar (SHA256).
- [ ] **Détection de Détournement COM** : Audit systématique des CLSIDs `InProcServer32`.
- [ ] **Analyse de Mémoire** : Scan des chaînes de caractères dans les processus actifs via YARA-X.
- [ ] **WMI Audit** : Détection des `EventConsumers` malveillants.

## Phase 3 : Apogée Forensique (v1.0.0)
- [ ] **Moteur de Remédiation Atomique** : Suppression différée via `MoveFileExW` et gestion de quarantaine chiffrée.
- [ ] **Intégration LOLDrivers** : Blocage des drivers vulnérables connus.
- [ ] **Analyse de Flux Réseau** : Détection des domaines de télémétrie/C2 via inspection DNS.
- [ ] **Interface Graphique Dédiée** : Visualisation des menaces et graphes de persistance.
