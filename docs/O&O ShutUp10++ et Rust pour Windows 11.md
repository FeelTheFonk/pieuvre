# **Rapport d'Analyse Approfondie : Architecture de Confidentialité Windows 11 et Implémentation des Paramètres O\&O ShutUp10++ pour l'Automatisation Système**

## **1\. Introduction et Contexte Stratégique**

### **1.1 L'Évolution du Modèle de Données Windows et la Nécessité de l'Intervention**

L'écosystème des systèmes d'exploitation modernes a subi une transformation radicale au cours de la dernière décennie, passant d'un modèle de logiciel-produit statique à un modèle de "Software as a Service" (SaaS). Avec l'avènement de Windows 10 et sa maturation sous Windows 11, Microsoft a institutionnalisé la collecte de données télémétriques comme composante intrinsèque du fonctionnement de l'OS. Ce changement de paradigme ne vise plus uniquement le débogage technique via les rapports d'erreurs, mais s'étend à la modélisation comportementale de l'utilisateur, à l'optimisation de l'engagement via des suggestions algorithmiques, et plus récemment, à l'alimentation des modèles d'intelligence artificielle générative tels que Copilot.1

Pour les développeurs et administrateurs système soucieux de la souveraineté numérique, des outils tiers sont devenus indispensables pour naviguer dans cette complexité. Parmi eux, **O\&O ShutUp10++** s'est imposé comme une référence technique, non pas par la magie noire, mais par une application rigoureuse et documentée de modifications sur les stratégies de groupe locales (GPO) et les clés de registre système. Contrairement aux scripts "debloaters" agressifs qui risquent de corrompre l'intégrité du système en supprimant des fichiers binaires, cet utilitaire agit comme un orchestrateur de configuration, basculant des interrupteurs logiciels prévus (mais souvent cachés) par Microsoft.3

### **1.2 Objectifs du Rapport pour le Projet "Pieuvre"**

Ce document a pour vocation de servir de spécification technique exhaustive pour la mise à jour du programme Rust "Pieuvre". L'objectif est de transcender la simple énumération de fonctionnalités pour fournir une cartographie précise des clés de registre, des valeurs de données et des logiques d'application utilisées par les paramètres "Recommandés" (les réglages verts) de O\&O ShutUp10++.

L'analyse se concentrera sur la déconstruction des mécanismes de persistance, la distinction entre les configurations utilisateur (HKCU) et machine (HKLM), et les implications de sécurité de chaque modification. Nous fournirons les détails nécessaires pour permettre une implémentation programmatique robuste en Rust, capable de reproduire fidèlement le niveau de confidentialité "Recommandé" sans compromettre la stabilité de l'environnement Windows 11\.5

## ---

**2\. Architecture Technique de la Gestion de la Confidentialité sous Windows**

Pour implémenter efficacement des contrôles de confidentialité dans un langage système comme Rust, il est impératif de comprendre l'architecture sous-jacente que Windows utilise pour gérer ces préférences. O\&O ShutUp10++ n'invente pas de nouvelles fonctionnalités ; il manipule les interfaces de gestion existantes.

### **2.1 La Dualité des Ruches de Registre : Portée et Privilèges**

Toute modification de confidentialité s'opère via la base de registre Windows, une base de données hiérarchique qui stocke les paramètres de configuration. Pour le développeur de "Pieuvre", la distinction fondamentale réside entre deux ruches principales :

1. HKEY\_LOCAL\_MACHINE (HKLM) :  
   Cette ruche contient les paramètres globaux qui affectent le système d'exploitation indépendamment de l'utilisateur connecté. Les modifications ici sont critiques pour désactiver des services de fond tels que la télémétrie système, l'inventaire des applications ou les mises à jour automatiques des pilotes.  
   * *Implication pour Rust :* L'écriture dans HKLM nécessite impérativement des privilèges administratifs (élévation UAC). Le programme "Pieuvre" doit donc vérifier au démarrage s'il est exécuté avec les droits suffisants, sous peine d'échecs silencieux ou d'erreurs d'accès refusé (AccessDenied).  
2. HKEY\_CURRENT\_USER (HKCU) :  
   Cette ruche gère les préférences de la session active. Elle est utilisée pour les paramètres d'interface utilisateur (UI), tels que l'affichage des widgets, les suggestions du menu Démarrer ou les configurations de l'explorateur de fichiers.  
   * *Implication pour Rust :* Bien que l'écriture soit possible sans élévation, une stratégie de déploiement efficace doit souvent appliquer ces paramètres à tous les profils utilisateurs, ce qui peut nécessiter de monter la ruche de l'utilisateur par défaut ou d'itérer sur les profils existants.

### **2.2 Mécanisme de Priorité : Politiques vs Préférences**

Une nuance technique cruciale exploitée par O\&O ShutUp10++ est la différence entre les clés de "Politique" et les clés de "Préférence".

* Les Clés de Politique (Software\\Policies\\Microsoft\\...) :  
  Elles correspondent aux objets de stratégie de groupe (GPO). Lorsque Windows détecte une valeur dans cette branche, elle prend le pas sur toute autre configuration. C'est le mécanisme de "verrouillage" utilisé en entreprise. Par exemple, définir AllowTelemetry à 0 dans la branche Policies grisera l'option dans l'interface des paramètres Windows, affichant la mention "Ce paramètre est géré par votre organisation".  
  * *Recommandation pour Pieuvre :* Privilégier systématiquement l'écriture dans les branches Policies lorsque cela est possible. Cela garantit la persistance du réglage face aux tentatives de modification par l'utilisateur ou par des mises à jour mineures.6  
* Les Clés de Préférence (Software\\Microsoft\\...) :  
  Elles reflètent l'état actuel de l'interface graphique. Elles sont plus volatiles et peuvent être réinitialisées par le système si une mise à jour majeure ("Feature Update") redéploie les paramètres par défaut.

### **2.3 Le Défi de la Persistance et des Mises à Jour**

Windows 10 et 11 intègrent des mécanismes d'auto-guérison (self-healing) qui peuvent restaurer des services désactivés. Le service WaaSMedicSvc (Windows Update Medic Service), par exemple, peut réparer des composants de mise à jour altérés. O\&O ShutUp10++ contourne cela en utilisant des clés de registre officielles supportées par le moteur de politique de groupe, qui sont respectées par le système comme des directives administratives légitimes plutôt que comme des corruptions à réparer. C'est cette approche "propre" que "Pieuvre" doit émuler pour éviter les effets de bord instables.3

## ---

**3\. Analyse Exhaustive des Paramètres Recommandés (Catégorie par Catégorie)**

Cette section détaille les paramètres spécifiques classés comme "Recommandés" (Verts) par O\&O ShutUp10++. Pour chaque paramètre, nous fournissons le contexte technique, la justification de confidentialité, et les chemins de registre précis à implémenter.

### **3.1 Télémétrie et Collecte de Données Diagnostiques**

La télémétrie est le canal par lequel Windows exfiltre des données techniques et comportementales vers les serveurs Microsoft (vortex.data.microsoft.com, etc.). C'est la priorité absolue de tout outil de confidentialité.

#### **3.1.1 Désactivation de la Télémétrie (Niveau Sécurité)**

Par défaut, Windows 11 Home et Pro ne permettent pas de désactiver complètement la télémétrie via l'interface graphique ; le niveau minimum sélectionnable est "Requis" (Basic). Cependant, le moteur de politique accepte une valeur plus restrictive, réservée aux éditions Entreprise, mais souvent respectée ou du moins minimisée sur les autres éditions lorsqu'elle est forcée par le registre.

* **Mécanisme Technique :** Le service DiagTrack (Expériences des utilisateurs connectés) vérifie la clé AllowTelemetry.  
* **Implémentation Registre :**

| Ruche | Chemin | Nom de la Valeur | Type | Donnée Recommandée |
| :---- | :---- | :---- | :---- | :---- |
| **HKLM** | SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection | AllowTelemetry | DWORD | 0 |

*Note d'expert :* La valeur 0 correspond au niveau "Security", 1 à "Basic", 2 à "Enhanced" (obsolète), et 3 à "Full". Bien que Microsoft indique que 0 n'est effectif que sur les versions Enterprise/Education, O\&O l'applique universellement pour garantir le niveau le plus bas possible accepté par le binaire compattelrunner.exe.8

#### **3.1.2 Désactivation de l'Identifiant Publicitaire (Advertising ID)**

L'Advertising ID est un UUID unique généré par l'OS pour chaque utilisateur. Il permet aux développeurs d'applications et aux réseaux publicitaires de suivre le comportement de l'utilisateur à travers différentes applications pour construire un profil de consommation.

* Implémentation Registre :  
  O\&O ShutUp10++ agit sur deux fronts : désactiver l'ID actuel (HKCU) et interdire sa génération future par politique (HKLM).

| Ruche | Chemin | Nom de la Valeur | Type | Donnée Recommandée |
| :---- | :---- | :---- | :---- | :---- |
| **HKLM** | SOFTWARE\\Policies\\Microsoft\\Windows\\AdvertisingInfo | DisabledByGroupPolicy | DWORD | 1 |
| **HKCU** | Software\\Microsoft\\Windows\\CurrentVersion\\AdvertisingInfo | Enabled | DWORD | 0 |

L'application de DisabledByGroupPolicy est essentielle pour empêcher une réactivation accidentelle via les paramètres de confidentialité de l'interface moderne.10

#### **3.1.3 Programme d'Amélioration de l'Expérience Client (CEIP/SQM)**

Le CEIP (Customer Experience Improvement Program) utilise le mécanisme SQM (Service Quality Metrics) pour envoyer des données sur la configuration matérielle et l'utilisation des logiciels. Bien que ces données soient théoriquement anonymisées, elles contribuent au "fingerprinting" de la machine.

* **Implémentation Registre :**

| Ruche | Chemin | Nom de la Valeur | Type | Donnée Recommandée |
| :---- | :---- | :---- | :---- | :---- |
| **HKLM** | SOFTWARE\\Policies\\Microsoft\\SQMClient\\Windows | CEIPEnable | DWORD | 0 |
| **HKLM** | SOFTWARE\\Microsoft\\SQMClient\\Windows | CEIPEnable | DWORD | 0 |

Il est recommandé de définir cette valeur à la fois dans la branche Policies et la branche standard pour assurer une couverture maximale.12

### ---

**3.2 Services Cognitifs et Intelligence Artificielle (Windows AI)**

L'introduction de l'IA dans Windows 11 via Copilot et Recall représente un vecteur de collecte de données massif, analysant potentiellement le contenu de l'écran et les interactions textuelles.

#### **3.2.1 Désactivation de Windows Copilot**

Copilot est intégré comme un assistant web via Edge, mais dispose de points d'entrée système (barre des tâches, raccourci Win+C). Sa désactivation est devenue une recommandation standard "Verte" dans les versions récentes de O\&O (v1.9.1440 et supérieures).13

* **Implémentation Registre :**

| Ruche | Chemin | Nom de la Valeur | Type | Donnée Recommandée |
| :---- | :---- | :---- | :---- | :---- |
| **HKCU** | Software\\Policies\\Microsoft\\Windows\\WindowsCopilot | TurnOffWindowsCopilot | DWORD | 1 |
| **HKLM** | SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsCopilot | TurnOffWindowsCopilot | DWORD | 1 |

Pour les versions récentes de Windows 11 (23H2/24H2), cette clé supprime efficacement l'icône et désactive le lancement du service associé.6

#### **3.2.2 Blocage de Windows Recall (Fonctionnalité de "Mémoire Photographique")**

Recall enregistre des instantanés (snapshots) de l'activité utilisateur pour permettre une recherche sémantique ultérieure. Les risques de confidentialité sont immenses si la base de données locale est exfiltrée. O\&O applique une désactivation préventive.

* **Implémentation Registre :**

| Ruche | Chemin | Nom de la Valeur | Type | Donnée Recommandée |
| :---- | :---- | :---- | :---- | :---- |
| **HKLM** | SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsAI | DisableAIDataAnalysis | DWORD | 1 |
| **HKLM** | SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsAI | AllowRecallEnablement | DWORD | 0 |

Ces clés bloquent l'analyse des données par l'IA locale et empêchent l'activation de la fonctionnalité de prise de vue.15

### ---

**3.3 Interface Utilisateur et "Shell Experience"**

Microsoft utilise l'interface utilisateur (Shell) pour injecter du contenu dynamique (souvent publicitaire ou promotionnel) via Bing et MSN. Ces fonctionnalités nécessitent des connexions réseau constantes et une analyse des centres d'intérêt.

#### **3.3.1 Désactivation des Widgets et "News and Interests"**

Le panneau de widgets est un navigateur Edge WebView2 déguisé qui charge du contenu MSN. Il consomme des ressources (processus Widgets.exe) et collecte des données de navigation.

* **Implémentation Registre :**

| Ruche | Chemin | Nom de la Valeur | Type | Donnée Recommandée |
| :---- | :---- | :---- | :---- | :---- |
| **HKLM** | SOFTWARE\\Policies\\Microsoft\\Dsh | AllowNewsAndInterests | DWORD | 0 |
| **HKCU** | Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced | TaskbarDa | DWORD | 0 |

La clé TaskbarDa (Taskbar Dashboard) contrôle spécifiquement la visibilité de l'icône dans la barre des tâches pour l'utilisateur courant, tandis que la politique Dsh (Device Shell) désactive la fonctionnalité au niveau système.17

#### **3.3.2 Suppression des Suggestions du Menu Démarrer**

Le menu Démarrer de Windows 11 contient une section "Nos recommandations" qui affiche les fichiers récents et suggère parfois de nouvelles applications du Store.

* **Implémentation Registre :**

| Ruche | Chemin | Nom de la Valeur | Type | Donnée Recommandée |
| :---- | :---- | :---- | :---- | :---- |
| **HKLM** | SOFTWARE\\Policies\\Microsoft\\Windows\\Explorer | HideRecommendedSection | DWORD | 1 |

Cette clé force le menu Démarrer à ne pas afficher la section de recommandations, épurant l'interface et limitant l'analyse locale des habitudes d'ouverture de fichiers.19

#### **3.3.3 Désactivation des "Search Highlights" (Bing dans la Recherche)**

La barre de recherche affiche des images quotidiennes et des faits divers ("Search Highlights"). Cela transforme un outil de recherche locale en portail web.

* **Implémentation Registre :**

| Ruche | Chemin | Nom de la Valeur | Type | Donnée Recommandée |
| :---- | :---- | :---- | :---- | :---- |
| **HKLM** | SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search | AllowSearchHighlights | DWORD | 0 |
| **HKLM** | SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search | DisableWebSearch | DWORD | 1 |

La combinaison de ces deux clés est cruciale : la première désactive les éléments visuels (dessins, faits du jour), la seconde empêche l'envoi des requêtes de recherche locales vers Bing.21

### ---

**3.4 Sécurité des Réseaux et Mises à Jour**

#### **3.4.1 Optimisation de la Distribution (WUDO \- Peer-to-Peer)**

Windows Update Delivery Optimization (WUDO) permet à votre PC de servir de serveur de mise à jour pour d'autres machines. Par défaut, cela peut inclure des machines sur Internet, consommant votre bande passante montante.

* Implémentation Registre :  
  O\&O recommande de restreindre cela au réseau local (LAN) ou de le désactiver, mais ne le désactive pas totalement pour ne pas briser les mises à jour du Store. La recommandation "Verte" est souvent le mode LAN (1) ou Simple (99/0 selon version).

| Ruche | Chemin | Nom de la Valeur | Type | Donnée Recommandée |
| :---- | :---- | :---- | :---- | :---- |
| **HKLM** | SOFTWARE\\Policies\\Microsoft\\Windows\\DeliveryOptimization | DODownloadMode | DWORD | 0 (HTTP) ou 1 (LAN) |

La valeur 0 force le téléchargement direct depuis les serveurs Microsoft (mode HTTP simple), éliminant tout trafic P2P sortant ou entrant non sollicité.9

#### **3.4.2 Désactivation de Wi-Fi Sense**

Bien que la fonctionnalité de partage automatique de mot de passe ait été retirée, des composants de Wi-Fi Sense restent actifs pour la détection de réseaux ouverts (Hotspots 2.0).

* **Implémentation Registre :**

| Ruche | Chemin | Nom de la Valeur | Type | Donnée Recommandée |
| :---- | :---- | :---- | :---- | :---- |
| **HKLM** | SOFTWARE\\Microsoft\\WcmSvc\\wifinetworkmanager\\config | AutoConnectAllowedOEM | DWORD | 0 |

Cette clé empêche le gestionnaire de réseau de tenter des connexions automatiques à des réseaux suggérés ou ouverts, réduisant la surface d'attaque sur les réseaux publics.22

### ---

**3.5 Gestion des Permissions Applicatives (Capability Access Manager)**

Windows 11 centralise les permissions des applications "Modernes" (UWP) via le *Capability Access Manager*. O\&O ShutUp10++ offre un contrôle granulaire ici. La recommandation "Verte" est généralement de bloquer l'accès global aux applications pour les capteurs les plus sensibles, forçant l'utilisateur à les réactiver explicitement s'il en a besoin.

#### **3.5.1 Blocage Global : Localisation, Caméra, Microphone**

Ces réglages utilisent des valeurs de type chaîne (String) plutôt que des DWORDs, avec les valeurs "Allow" ou "Deny".

* **Implémentation Registre :**

| Fonction | Chemin (HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore...) | Nom | Type | Donnée |
| :---- | :---- | :---- | :---- | :---- |
| **Localisation** | ...\\location | Value | String | Deny |
| **Caméra** | ...\\webcam | Value | String | Deny |
| **Microphone** | ...\\microphone | Value | String | Deny |
| **Notifs** | ...\\userNotification | Value | String | Deny |

*Avertissement pour l'implémentation Rust :* L'écriture de la valeur "Deny" dans HKLM affecte toutes les applications du système. Si l'utilisateur souhaite utiliser Zoom ou Teams, il devra manuellement réactiver l'accès dans les Paramètres. C'est pourquoi ces réglages sont puissants mais peuvent générer des appels au support. O\&O les classe souvent en "Vert" mais avec des avertissements contextuels.23

#### **3.5.2 Applications en Arrière-plan**

Windows 11 a modifié la gestion des applications en arrière-plan par rapport à Windows 10, rendant la désactivation globale plus complexe. Cependant, la politique de désactivation reste pertinente pour économiser des ressources et limiter la télémétrie passive des applications.

| Ruche | Chemin | Nom de la Valeur | Type | Donnée Recommandée |
| :---- | :---- | :---- | :---- | :---- |
| **HKLM** | SOFTWARE\\Policies\\Microsoft\\Windows\\AppPrivacy | LetAppsRunInBackground | DWORD | 2 (Force Deny) |

### ---

**3.6 Fonctionnalités Diverses et Héritées**

#### **3.6.1 Bouton de Révélation du Mot de Passe**

Ce bouton (l'œil) dans les champs de mot de passe est un risque de sécurité physique (visualisation par un tiers).

* **Implémentation Registre :**

| Ruche | Chemin | Nom de la Valeur | Type | Donnée Recommandée |
| :---- | :---- | :---- | :---- | :---- |
| **HKLM** | SOFTWARE\\Policies\\Microsoft\\Windows\\CredUI | DisablePasswordReveal | DWORD | 1 |

.25

#### **3.6.2 Enregistreur d'Actions Utilisateur (Steps Recorder)**

Bien que déprécié, cet outil peut être activé pour le diagnostic. Le désactiver prévient son utilisation malveillante ou accidentelle.

* **Implémentation Registre :**

| Ruche | Chemin | Nom de la Valeur | Type | Donnée Recommandée |
| :---- | :---- | :---- | :---- | :---- |
| **HKLM** | SOFTWARE\\Policies\\Microsoft\\Windows\\AppCompat | DisableUAR | DWORD | 1 |

#### **3.6.3 Collecteur d'Inventaire (Inventory Collector)**

Ce composant (CompatTelRunner) scanne les fichiers pour la compatibilité des mises à jour.

* **Implémentation Registre :**

| Ruche | Chemin | Nom de la Valeur | Type | Donnée Recommandée |
| :---- | :---- | :---- | :---- | :---- |
| **HKLM** | SOFTWARE\\Policies\\Microsoft\\Windows\\AppCompat | DisableInventory | DWORD | 1 |

.27

## ---

**4\. Stratégie d'Implémentation pour le Programme Rust "Pieuvre"**

L'intégration de ces paramètres dans une application Rust demande une rigueur particulière concernant la sécurité du type de données et la gestion des erreurs système.

### **4.1 Choix des Bibliothèques et Architecture**

La bibliothèque standard Rust (std) ne couvre pas les API spécifiques à Windows. L'utilisation de la crate winreg est le standard industriel pour interagir avec le registre.

* **Dépendance Cargo.toml :**  
  Ini, TOML  
  \[dependencies\]  
  winreg \= "0.10"

### **4.2 Gestion des Privilèges et de l'Architecture 64-bits**

Windows 11 fonctionne sur une architecture 64-bits. Une application 32-bits (si compilée ainsi par défaut) serait redirigée par le système vers la branche Wow6432Node du registre, rendant les modifications inopérantes pour le système OS réel.

* **Recommandation Critique :** Lors de l'ouverture des clés de registre, il est impératif d'utiliser le flag KEY\_WOW64\_64KEY pour forcer l'accès à la vue 64-bits, même si l'application "Pieuvre" est compilée en 32-bits (bien qu'une compilation en x86\_64-pc-windows-msvc soit préférable).

### **4.3 Structure de Données Modulaire**

Pour gérer la complexité des dizaines de clés, il est déconseillé d'écrire des appels impératifs séquentiels. Une approche déclarative est préférable.

Rust

// Exemple de structure de données pour définir une "Règle" O\&O  
enum RegType {  
    Dword(u32),  
    Sz(String),  
}

struct PrivacySetting {  
    name: String,  
    hive: HKEY, // HKLM ou HKCU  
    path: String,  
    key: String,  
    value: RegType,  
    description: String,  
}

// Exemple d'instantiation pour la télémétrie  
let telemetry\_rule \= PrivacySetting {  
    name: "Disable Telemetry".to\_string(),  
    hive: HKEY\_LOCAL\_MACHINE,  
    path: "SOFTWARE\\\\Policies\\\\Microsoft\\\\Windows\\\\DataCollection".to\_string(),  
    key: "AllowTelemetry".to\_string(),  
    value: RegType::Dword(0),  
    description: "Force le niveau de télémétrie à Sécurité".to\_string(),  
};

### **4.4 Logique de "Toggle" et Sauvegarde**

Une fonctionnalité clé de O\&O ShutUp10++ est la capacité d'annuler les modifications. Votre programme Rust doit :

1. **Lire** la valeur actuelle avant modification.  
2. **Stocker** cette valeur (dans un fichier JSON local ou une base SQLite légère).  
3. **Appliquer** la nouvelle valeur.  
4. Si la clé n'existait pas, noter qu'elle devra être supprimée lors de la restauration, et non pas simplement remise à 0\.

### **4.5 Gestion des Erreurs : Clés Manquantes**

Beaucoup de clés "Policies" n'existent pas par défaut. La méthode open\_subkey de winreg échouera.

* **Logique Rust :** Utiliser create\_subkey (qui ouvre si existe, crée sinon) au lieu de open\_subkey pour les opérations d'écriture.  
* Assurer une gestion d'erreur robuste (Result\<(), io::Error\>) pour informer l'utilisateur si une clé est verrouillée par le système (Permission Denied), ce qui indique souvent un manque de droits Administrateur.

## ---

**5\. Analyse des Risques et Considérations Finales**

L'automatisation de ces paramètres via "Pieuvre" offre une puissance considérable, mais comporte des risques inhérents à toute modification système.

### **5.1 Faux Positifs et Sécurité**

La désactivation de certaines fonctionnalités (comme *SmartScreen* ou le rapport d'erreurs *WER*) peut être considérée par les analystes de sécurité comme une réduction de la posture de sécurité, car elle prive Microsoft de données sur les nouvelles menaces. Cependant, dans un contexte de confidentialité "Verte", O\&O maintient généralement SmartScreen activé pour la navigation web (filtre anti-phishing) tout en désactivant l'envoi des URL visitées. "Pieuvre" devrait suivre cette nuance : ne pas désactiver le moteur de sécurité local, mais couper le canal de rapport vers le cloud.

### **5.2 Impact sur les Fonctionnalités "Modernes"**

Les utilisateurs doivent être informés que la désactivation de la géolocalisation globale ou de l'accès au microphone via le *ConsentStore* (HKLM) est une "option nucléaire". Cela brisera les conférences Teams ou Zoom si l'utilisateur ne réactive pas manuellement l'accès. C'est une friction acceptable pour un outil orienté confidentialité, mais qui nécessite une documentation claire (tooltips) dans votre interface utilisateur.

### **5.3 Pérennité face à Windows Update**

Microsoft réinitialise périodiquement certaines préférences lors des "Feature Updates" annuelles. L'approche basée sur les clés Policies (GPO) adoptée dans ce rapport est la plus résiliente, car Windows est programmé pour respecter ces clés lors des migrations. Néanmoins, il est recommandé que "Pieuvre" inclue une fonction de "Vérification d'état" au démarrage pour réappliquer les règles si une dérive de configuration (configuration drift) est détectée.

En conclusion, l'intégration de ces paramètres dans "Pieuvre" transformera l'outil en une solution de confidentialité de niveau professionnel, capable de rivaliser avec les standards établis par O\&O ShutUp10++, tout en offrant la transparence et l'auditabilité d'un code source ouvert (Rust).

#### **Sources des citations**

1. Tweak and Debloat Your Windows 10 or 11 Computer Settings for Free with O\&O ShutUp10++ \- YouTube, consulté le décembre 30, 2025, [https://www.youtube.com/watch?v=R98FXZBI-Kw](https://www.youtube.com/watch?v=R98FXZBI-Kw)  
2. How to Disable ALL Microsoft Windows spying\! FREE UTILITY\! \- YouTube, consulté le décembre 30, 2025, [https://www.youtube.com/watch?v=IJSie\_3ncc8](https://www.youtube.com/watch?v=IJSie_3ncc8)  
3. PSA: ShutUp10++ lets you control every single setting where Windows 10 \+ 11 can accesss and share your private data, easy for free and without any installation. : r/pcgaming \- Reddit, consulté le décembre 30, 2025, [https://www.reddit.com/r/pcgaming/comments/sy1gb2/psa\_shutup10\_lets\_you\_control\_every\_single/](https://www.reddit.com/r/pcgaming/comments/sy1gb2/psa_shutup10_lets_you_control_every_single/)  
4. O\&O Shutup10++, AppBuster \- Windows detox software \- Dedoimedo, consulté le décembre 30, 2025, [https://www.dedoimedo.com/computers/oo-shutup-appbuster-review.html](https://www.dedoimedo.com/computers/oo-shutup-appbuster-review.html)  
5. LeDragoX/Win-Debloat-Tools: Re-imagining Windows like a minimal OS install, already debloated with minimal impact for most functionality. \- GitHub, consulté le décembre 30, 2025, [https://github.com/LeDragoX/Win-Debloat-Tools](https://github.com/LeDragoX/Win-Debloat-Tools)  
6. How to Disable Windows 11 Copilot Through Registry File or Group Policy Editor, consulté le décembre 30, 2025, [https://www.techrepublic.com/article/how-to-disable-copilot-windows-11/](https://www.techrepublic.com/article/how-to-disable-copilot-windows-11/)  
7. How To Disable Copilot on Windows 11 and 10 \[2025\] \- Secure Data Recovery Services, consulté le décembre 30, 2025, [https://www.securedatarecovery.com/blog/how-to-disable-copilot-windows](https://www.securedatarecovery.com/blog/how-to-disable-copilot-windows)  
8. Windows 11 Privacy Settings: Complete Setup Guide \- Aardwolf Security, consulté le décembre 30, 2025, [https://aardwolfsecurity.com/how-to-set-up-windows-11-for-maximum-privacy/](https://aardwolfsecurity.com/how-to-set-up-windows-11-for-maximum-privacy/)  
9. Cleaning the bloat from Windows 10 | \[H\]ard|Forum, consulté le décembre 30, 2025, [https://hardforum.com/threads/cleaning-the-bloat-from-windows-10.1870618/](https://hardforum.com/threads/cleaning-the-bloat-from-windows-10.1870618/)  
10. Disable Ads in Windows 11, consulté le décembre 30, 2025, [https://www.elevenforum.com/t/disable-ads-in-windows-11.8004/](https://www.elevenforum.com/t/disable-ads-in-windows-11.8004/)  
11. Enable or Disable Advertising ID for Personalized Ads in Apps in Windows 11, consulté le décembre 30, 2025, [https://www.elevenforum.com/t/enable-or-disable-advertising-id-for-personalized-ads-in-apps-in-windows-11.3730/](https://www.elevenforum.com/t/enable-or-disable-advertising-id-for-personalized-ads-in-apps-in-windows-11.3730/)  
12. PC-Tuning/docs/registry-opts.md at main \- GitHub, consulté le décembre 30, 2025, [https://github.com/valleyofdoom/PC-Tuning/blob/main/docs/registry-opts.md](https://github.com/valleyofdoom/PC-Tuning/blob/main/docs/registry-opts.md)  
13. NEW: O\&O ShutUp10++ 1.9.1440\! Deactivation of Windows Copilot \+ Recall now also under Windows 10 & more\! \- Blog, consulté le décembre 30, 2025, [https://blog.oo-software.com/en/new-oo-shutup10-1-9-1440-deactivation-of-windows-copilot-recall-now-also-under-windows-10-more/](https://blog.oo-software.com/en/new-oo-shutup10-1-9-1440-deactivation-of-windows-copilot-recall-now-also-under-windows-10-more/)  
14. Disable Microsoft COPILOT Without Group Policy Editor \- YouTube, consulté le décembre 30, 2025, [https://www.youtube.com/watch?v=FZUOIteH7Ts](https://www.youtube.com/watch?v=FZUOIteH7Ts)  
15. How to disable Recall in Windows 11 (Registry script to turn off Recall AI) \- Windows Latest, consulté le décembre 30, 2025, [https://www.windowslatest.com/2025/04/28/disable-remove-recall-feature-in-windows-11/](https://www.windowslatest.com/2025/04/28/disable-remove-recall-feature-in-windows-11/)  
16. Registry entries to disable Copilot+ Recall feature : r/Windows11 \- Reddit, consulté le décembre 30, 2025, [https://www.reddit.com/r/Windows11/comments/1d1ef1q/registry\_entries\_to\_disable\_copilot\_recall\_feature/](https://www.reddit.com/r/Windows11/comments/1d1ef1q/registry_entries_to_disable_copilot_recall_feature/)  
17. How to remove widgets in Windows 11 \- PDQ, consulté le décembre 30, 2025, [https://www.pdq.com/blog/how-to-remove-widgets-in-windows-11/](https://www.pdq.com/blog/how-to-remove-widgets-in-windows-11/)  
18. Is there a way to fully disable Widgets processes? : r/Windows11 \- Reddit, consulté le décembre 30, 2025, [https://www.reddit.com/r/Windows11/comments/x739nt/is\_there\_a\_way\_to\_fully\_disable\_widgets\_processes/](https://www.reddit.com/r/Windows11/comments/x739nt/is_there_a_way_to_fully_disable_widgets_processes/)  
19. Simple way to hide Recommended Start menu section : r/Windows11 \- Reddit, consulté le décembre 30, 2025, [https://www.reddit.com/r/Windows11/comments/1jewouy/simple\_way\_to\_hide\_recommended\_start\_menu\_section/](https://www.reddit.com/r/Windows11/comments/1jewouy/simple_way_to_hide_recommended_start_menu_section/)  
20. 3 Ways to Remove Recommended Section from Start Menu on Windows 11 \- WiseCleaner, consulté le décembre 30, 2025, [https://www.wisecleaner.com/think-tank/621-3-Ways-to-Remove-Recommended-Section-from-Start-Menu-on-Windows-11.html](https://www.wisecleaner.com/think-tank/621-3-Ways-to-Remove-Recommended-Section-from-Start-Menu-on-Windows-11.html)  
21. How to Turn Off Search Highlights on Windows 11 \- GeeksforGeeks, consulté le décembre 30, 2025, [https://www.geeksforgeeks.org/techtips/turn-off-search-highlights-on-windows/](https://www.geeksforgeeks.org/techtips/turn-off-search-highlights-on-windows/)  
22. Configure Wi-Fi Sense and Paid Wi-Fi Services \- Windows Client | Microsoft Learn, consulté le décembre 30, 2025, [https://learn.microsoft.com/en-us/troubleshoot/windows-client/networking/configure-wifi-sense-and-paid-wifi-service](https://learn.microsoft.com/en-us/troubleshoot/windows-client/networking/configure-wifi-sense-and-paid-wifi-service)  
23. Windows-Optimize-Harden-Debloat/sos-optimize-windows.ps1 at master \- GitHub, consulté le décembre 30, 2025, [https://github.com/simeononsecurity/Windows-Optimize-Harden-Debloat/blob/master/sos-optimize-windows.ps1](https://github.com/simeononsecurity/Windows-Optimize-Harden-Debloat/blob/master/sos-optimize-windows.ps1)  
24. consulté le décembre 30, 2025, [https://raw.githubusercontent.com/undergroundwires/privacy.sexy/0.13.5/src/application/collections/windows.yaml](https://raw.githubusercontent.com/undergroundwires/privacy.sexy/0.13.5/src/application/collections/windows.yaml)  
25. How to disable the Password Reveal button on the Sign-in screen on Windows 10, consulté le décembre 30, 2025, [https://www.windowscentral.com/how-disable-password-reveal-button-sign-screen-windows-10](https://www.windowscentral.com/how-disable-password-reveal-button-sign-screen-windows-10)  
26. 2 Methods to Remove Password Reveal Button in Windows 10, consulté le décembre 30, 2025, [https://www.top-password.com/blog/remove-password-reveal-button-in-windows-10/](https://www.top-password.com/blog/remove-password-reveal-button-in-windows-10/)  
27. Windows Tweaks.bat \- GitHub, consulté le décembre 30, 2025, [https://github.com/HakanFly/Windows-Tweaks/blob/main/Windows%20Tweaks.bat](https://github.com/HakanFly/Windows-Tweaks/blob/main/Windows%20Tweaks.bat)