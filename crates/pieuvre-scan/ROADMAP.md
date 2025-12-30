# ROADMAP pieuvre-scan

## Phase 1 : Consolidation (v0.8.x) - [COMPLETE]
- [x] Integration `yara-x` native.
- [x] Acquisition `SeDebugPrivilege`.
- [x] Analyse de navigateurs (Chrome/Firefox/Edge).
- [x] Support des raccourcis `.lnk` via `parselnk`.
- [x] Structure `Threat` pour donnees structurees.
- [x] Support navigateur Brave.
- [x] 8 regles YARA-X par defaut.
- [x] 10 cles ASEP dans le registre.
- [ ] Analyse des taches planifiees via parsing XML.

## Phase 2 : Analyse Avancee (v0.9.0)
- [ ] **Moteur de Reputation Local** : Bloom Filter base sur MalwareBazaar.
- [ ] **Audit CLSID** : Detection de detournement COM (`InProcServer32`).
- [ ] **Analyse Memoire** : Scan des chaines de caracteres dans les processus actifs.
- [ ] **Audit WMI** : Detection des `EventConsumers` persistants.
- [ ] **Taches Planifiees** : Parsing XML et detection de persistence.

## Phase 3 : Remediation et Protection (v1.0.0)
- [ ] **Moteur de Remediation** : Suppression differee via `MoveFileExW` et gestion de quarantaine.
- [ ] **Integration LOLDrivers** : Blocage des drivers vulnerables.
- [ ] **Analyse Reseau** : Detection des domaines de telemetrie via inspection DNS.
- [ ] **Interface Graphique** : Visualisation des menaces et graphes de persistance.
