# ROADMAP pieuvre-scan

## Phase 1 : Consolidation (v0.8.x) - [EN COURS]
- [x] Intégration `yara-x` native.
- [x] Acquisition `SeDebugPrivilege`.
- [x] Analyse de navigateurs (Chrome/Firefox).
- [ ] Support des raccourcis `.lnk` via `parselnk`.
- [ ] Analyse des tâches planifiées via parsing XML.

## Phase 2 : Analyse Avancée (v0.9.0)
- [ ] **Moteur de Réputation Local** : Bloom Filter basé sur MalwareBazaar.
- [ ] **Audit CLSID** : Détection de détournement COM (`InProcServer32`).
- [ ] **Analyse Mémoire** : Scan des chaînes de caractères dans les processus actifs.
- [ ] **Audit WMI** : Détection des `EventConsumers` persistants.

## Phase 3 : Remédiation & Protection (v1.0.0)
- [ ] **Moteur de Remédiation** : Suppression différée via `MoveFileExW` et gestion de quarantaine.
- [ ] **Intégration LOLDrivers** : Blocage des drivers vulnérables.
- [ ] **Analyse Réseau** : Détection des domaines de télémétrie via inspection DNS.
- [ ] **Interface Graphique** : Visualisation des menaces et graphes de persistance.
