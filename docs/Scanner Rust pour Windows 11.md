# **Rapport de Recherche et Développement : Conception d'un Moteur Unifié de Détection et de Remédiation des Menaces "Greyware" pour Windows 11 en Rust**

## **Introduction**

La cybersécurité moderne sur les postes de travail Windows 11 ne se limite plus à la simple détection de virus destructeurs ou de ransomwares chiffrant les données. Une catégorie insidieuse de menaces, souvent qualifiée de "greyware" ou de "zone grise", a émergé comme une nuisance majeure pour l'expérience utilisateur et la confidentialité des données. Cette catégorie englobe les Programmes Potentiellement Indésirables (PUPs ou PuPs), les adwares (logiciels publicitaires), les pirates de navigateur (browser hijackers) et les barres d'outils intrusives. Contrairement aux malwares traditionnels qui cherchent à détruire ou à voler de manière flagrante, ces logiciels opèrent souvent à la limite de la légalité, s'installant via des techniques de "bundling" (regroupement logiciel) et modifiant profondément les configurations système et les préférences des navigateurs pour monétiser le trafic utilisateur.1

L'objectif de ce rapport de recherche est de fournir une feuille de route technique exhaustive pour l'intégration d'un module de scan et de nettoyage au sein de l'outil "Pieuvre", développé en Rust. Ce module vise à égaler, voire surpasser, les capacités d'outils de référence tels qu'AdwCleaner ou ZHPCleaner, en tirant parti des garanties de sécurité mémoire et de performance offertes par le langage Rust. L'analyse se concentre spécifiquement sur l'environnement Windows 11, prenant en compte ses spécificités architecturales (Registre, Système de Fichiers, Gestion des Processus) et les mécanismes de persistance avancés utilisés par les menaces modernes.

Le choix de Rust pour cette tâche est stratégique. Les outils de remédiation doivent souvent opérer avec des privilèges élevés (SYSTEM ou Administrateur) et manipuler des données non fiables provenant du registre ou du système de fichiers. Les langages traditionnels comme C ou C++ exposent ces outils à des vulnérabilités de corruption de mémoire (buffer overflows, use-after-free) que des malwares sophistiqués pourraient exploiter pour désactiver le scanner. Rust, grâce à son modèle de propriété (ownership) et de vérification des emprunts (borrow checker) à la compilation, élimine mathématiquement ces classes d'erreurs, offrant une base robuste pour un outil de sécurité offensif et défensif.4

Ce document détaille l'architecture nécessaire pour interagir avec les API de bas niveau de Windows (Win32), l'analyse forensique des artefacts laissés par les adwares (clés de registre orphelines, tâches planifiées, fichiers JSON de préférences navigateurs), l'intégration de moteurs de signatures performants comme YARA via des implémentations natives Rust, et enfin, les stratégies de remédiation sécurisée, incluant la suppression au redémarrage et la gestion de la quarantaine.

## ---

**Chapitre 1 : Fondations de la Programmation Système Windows en Rust**

Le développement d'un scanner antimalware pour Windows 11 exige une interaction profonde avec le noyau du système d'exploitation. Il ne s'agit pas simplement de lire des fichiers, mais de manipuler des jetons d'accès (tokens), d'énumérer des processus protégés, d'injecter des instructions de suppression différée dans le gestionnaire de session, et de parcourir des ruches de registre potentiellement verrouillées. L'écosystème Rust a considérablement mûri, offrant désormais des projections directes des API Windows qui rivalisent avec le C++ en termes de contrôle, tout en conservant l'ergonomie de Rust.

### **1.1 Écosystème des Crates : windows-rs vs winapi**

Historiquement, la crate winapi était la solution standard pour l'interopérabilité Rust/Windows. Elle consistait en des liaisons manuelles vers les API C, souvent incomplètes ou obsolètes. Pour le projet Pieuvre, l'utilisation de winapi est déconseillée au profit des crates officielles de Microsoft : windows et windows-sys.

#### **Analyse Comparative et Recommandation Technique**

| Caractéristique | winapi (Obsolète) | windows-rs (Recommandé pour Haut Niveau) | windows-sys (Recommandé pour Performance) |
| :---- | :---- | :---- | :---- |
| **Maintenance** | Communautaire, sporadique 7 | Microsoft, automatisée via métadonnées 9 | Microsoft, automatisée 11 |
| **Couverture API** | Partielle, liée manuellement | Totale (Win32, COM, WinRT) | Totale (Win32 uniquement, types bruts) |
| **Sécurité** | Pointeurs bruts unsafe partout | Wrappers plus sûrs, types riches | Pointeurs bruts, surcharge minimale |
| **Compilation** | Rapide | Lente (génération de code massive) | Très rapide (déclarations extern pures) |

Pour un moteur de scan qui doit être performant et léger, l'architecture recommandée pour Pieuvre est hybride :

1. Utiliser **windows-sys** pour les appels système fréquents et critiques (boucles de scan de fichiers, énumération de processus) afin de minimiser l'overhead et le temps de compilation.11  
2. Utiliser **windows** (la crate complète) uniquement pour les interactions complexes nécessitant COM (Component Object Model), comme la manipulation avancée du Planificateur de Tâches (Task Scheduler) ou l'interaction avec WMI (Windows Management Instrumentation), où la gestion automatique des interfaces COM et des HRESULT par windows-rs simplifie considérablement le code.10

#### **Gestion des Chaînes de Caractères (Unicode/UTF-16)**

Windows utilise nativement l'encodage UTF-16 (WCHAR) pour toutes ses API système (les versions se terminant par 'W', ex: CreateFileW), tandis que Rust utilise UTF-8 par défaut. Une source fréquente de bugs et de vulnérabilités dans les outils de sécurité réside dans la conversion incorrecte des chemins de fichiers, que les malwares exploitent parfois avec des caractères Unicode invalides pour échapper aux scanners.

L'implémentation dans Pieuvre doit inclure un trait utilitaire pour convertir sans allocation excessive les chaînes Rust en vecteurs u16 terminés par un caractère nul, format attendu par Windows.

Rust

use std::ffi::OsStr;  
use std::os::windows::ffi::OsStrExt;

// Conversion essentielle pour interagir avec l'API Win32  
pub fn to\_pcwstr(text: &str) \-\> Vec\<u16\> {  
    OsStr::new(text)  
       .encode\_wide()  
       .chain(std::iter::once(0)) // Terminaison nulle obligatoire  
       .collect()  
}

Cette fonction sera omniprésente lors de l'appel de fonctions comme RegOpenKeyExW ou MoveFileExW.12

### **1.2 Gestion des Privilèges : Le Cas Critique de SeDebugPrivilege**

Pour scanner efficacement un système Windows 11 infecté, l'outil ne peut se contenter des droits d'administrateur standard. Il doit être capable d'inspecter des processus système (comme lsass.exe ou csrss.exe) et d'accéder à des clés de registre protégées appartenant au compte SYSTEM. Pour cela, le privilège SeDebugPrivilege est indispensable.14

Par défaut, même un processus lancé "en tant qu'administrateur" possède ce privilège dans son jeton d'accès (token), mais il est à l'état *désactivé*. L'activation explicite est une étape obligatoire au démarrage du moteur Pieuvre.

#### **Mécanisme d'Élévation en Rust**

L'activation de SeDebugPrivilege implique une séquence précise d'appels API Win32, qui doit être implémentée avec rigueur en Rust via windows-sys ou windows.16

1. **Ouverture du Jeton de Processus (OpenProcessToken)** : Il faut obtenir un handle sur le token du processus courant avec les droits TOKEN\_ADJUST\_PRIVILEGES et TOKEN\_QUERY.  
2. **Recherche du LUID (LookupPrivilegeValueW)** : Le système n'utilise pas le nom "SeDebugPrivilege" directement, mais un identifiant local unique (LUID).  
3. **Ajustement du Jeton (AdjustTokenPrivileges)** : C'est l'étape critique. Il faut construire une structure TOKEN\_PRIVILEGES contenant le LUID et l'attribut SE\_PRIVILEGE\_ENABLED.

Un point d'attention particulier pour le développeur Rust est la structure TOKEN\_PRIVILEGES. En C, elle utilise un "flexible array member" (tableau de taille variable à la fin de la structure), ce qui n'est pas directement représentable en Rust safe. L'implémentation doit souvent recourir à une manipulation de pointeurs brute ou à la définition manuelle d'une structure compatible.17

**Implémentation Technique Recommandée :**

Rust

// Pseudo-code illustrant la logique nécessaire avec windows-sys  
use windows\_sys::Win32::Security::{AdjustTokenPrivileges, LookupPrivilegeValueW, TOKEN\_PRIVILEGES, SE\_PRIVILEGE\_ENABLED};

// La structure TOKEN\_PRIVILEGES doit être initialisée avec soin.  
// En Rust, on ne peut pas facilement redimensionner la struct C,   
// donc pour un seul privilège, la struct par défaut suffit souvent.  
let mut tp: TOKEN\_PRIVILEGES \= std::mem::zeroed();  
tp.PrivilegeCount \= 1;  
tp.Privileges.Luid \= luid\_recupere;  
tp.Privileges.Attributes \= SE\_PRIVILEGE\_ENABLED;

// Appel de AdjustTokenPrivileges...  
// ATTENTION : GetLastError() doit être vérifié même si la fonction retourne TRUE (succès),  
// car elle peut retourner TRUE tout en échouant partiellement (ERROR\_NOT\_ALL\_ASSIGNED).

L'absence de ce privilège rendrait le scanner aveugle face aux rootkits ou aux services malveillants s'exécutant avec les droits SYSTEM.19

## ---

**Chapitre 2 : Analyse Forensique du Registre et Persistance**

Le Registre Windows est le système nerveux central où les adwares et PuPs s'ancrent pour garantir leur survie au redémarrage. Contrairement aux virus classiques qui peuvent infecter des exécutables, les adwares s'appuient presque exclusivement sur des clés de configuration légitimes détournées de leur usage.

### **2.1 Points d'Extension de Démarrage Automatique (ASEP)**

L'analyse doit couvrir de manière exhaustive les clés ASEP (Auto-Start Extensibility Points). Une simple vérification de la clé Run est insuffisante pour un outil moderne.21

#### **Les Clés "Run" et "RunOnce"**

C'est le niveau élémentaire de la persistance. Le scanner Pieuvre doit itérer sur ces clés pour :

* HKEY\_LOCAL\_MACHINE (HKLM) : Affecte tous les utilisateurs. Nécessite des droits Admin pour écrire, mais souvent lisible.  
* HKEY\_CURRENT\_USER (HKCU) : Affecte l'utilisateur actuel. Cible privilégiée des adwares installés sans droits admin.  
* **Wow6432Node** : Sur un Windows 11 64-bit, les applications 32-bit (comme beaucoup d'anciens adwares) écrivent dans HKLM\\SOFTWARE\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Run. Ignorer cette branche laisserait passer 30 à 40% des infections.23

Heuristiques de Détection dans le Registre :  
L'analyse des valeurs contenues dans ces clés doit dépasser la simple liste noire.

1. **Chemins Suspects :** Tout exécutable pointant vers %AppData%, %LocalAppData%, ou %Temp% est hautement suspect. Les logiciels légitimes s'installent généralement dans Program Files.  
2. **Arguments Obfusqués :** La présence de commandes comme powershell \-w hidden \-e... ou rundll32 javascript:... indique une charge utile "fileless" (sans fichier) stockée directement dans le registre, une technique popularisée par le malware Kovter.24

### **2.2 Techniques de Persistance Avancées**

Pour égaler des outils comme ZHPCleaner, Pieuvre doit inspecter des vecteurs moins évidents mais très efficaces.25

#### **Image File Execution Options (IFEO)**

Située dans HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options, cette clé permet aux développeurs de déboguer des applications.

* **Le Détournement :** Un attaquant crée une sous-clé nommée chrome.exe et ajoute une valeur Debugger pointant vers son malware.  
* **Résultat :** Chaque fois que l'utilisateur lance Chrome, Windows lance en réalité le malware (qui peut ensuite lancer Chrome pour masquer son action).  
* **Détection :** Scanner toutes les sous-clés. Si une valeur Debugger existe et pointe vers un fichier inconnu ou suspect, c'est une alerte rouge.24

#### **AppInit\_DLLs**

Située dans HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Windows.

* **Mécanisme :** Les DLLs listées ici sont injectées dans *chaque* processus qui charge User32.dll (c'est-à-dire presque toutes les applications graphiques).  
* **Contexte Windows 11 :** Bien que le "Secure Boot" désactive souvent cette fonctionnalité, elle reste un vecteur actif si le Secure Boot est désactivé ou sur des systèmes hérités. Le scanner doit vérifier la valeur LoadAppInit\_DLLs. Si elle est à 1, la liste des DLLs doit être analysée rigoureusement.23

#### **Détournement COM (CLSID)**

Les objets COM (Component Object Model) sont fondamentaux pour l'interopérabilité Windows. Les adwares s'enregistrent souvent comme des extensions de l'explorateur ou des "Browser Helper Objects" (BHO).

* **Cible :** HKCR\\CLSID\\{GUID}\\InProcServer32.  
* **Analyse :** Cette clé indique quelle DLL charger pour un GUID donné. Si la DLL par défaut (Default) pointe vers un chemin utilisateur (User Profile) au lieu de System32, il s'agit presque toujours d'un hijacking COM.27  
* **Données Exploitables :** Il existe des bases de données de CLSIDs malveillants connus (adware classes). Pieuvre devrait intégrer une base de hachage de ces GUIDs pour une détection rapide.

### **2.3 Le Planificateur de Tâches : Le Repaire Caché**

Les tâches planifiées sont devenues le vecteur de persistance prédominant pour les adwares modernes, car elles permettent une exécution avec des privilèges élevés et sont plus difficiles à auditer que les clés Run.28

#### **Architecture et Détection**

Les tâches sont définies par des fichiers XML dans C:\\Windows\\System32\\Tasks, mais indexées dans le registre sous HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Schedule\\TaskCache\\Tree.

* **Discordance Registre/Disque :** Une technique de dissimulation (utilisée par Hafnium et des adwares avancés) consiste à supprimer le fichier XML ou la clé Tree tout en laissant la tâche active en mémoire ou partiellement référencée, rendant l'outil Task Scheduler de Windows aveugle.  
* **Analyse Rust :** Pieuvre ne doit pas utiliser l'API haut niveau de Windows pour lister les tâches, car elle peut être trompée. Il doit :  
  1. Parcourir récursivement le dossier System32\\Tasks.  
  2. Parcourir la clé de registre TaskCache\\Tree.  
  3. Comparer les deux. Toute orpheline (présente dans l'un mais pas l'autre) est suspecte.  
  4. Analyser le contenu XML des tâches pour trouver des actions lançant cmd.exe, powershell, ou ouvrant des URLs (adware "pop-up").29

## ---

**Chapitre 3 : Forensique des Navigateurs (Le Cœur de la Cible)**

L'objectif premier d'un adware est la monétisation du trafic web. Cela implique le détournement de la page d'accueil, du moteur de recherche par défaut, et l'injection de publicités via des extensions malveillantes. Chaque navigateur stocke ces configurations différemment, nécessitant des parsers spécifiques en Rust.

### **3.1 Google Chrome et Microsoft Edge (Architecture Chromium)**

Ces deux navigateurs partagent la même architecture de configuration basée sur le format JSON. Les fichiers clés sont situés dans %LocalAppData%\\Google\\Chrome\\User Data\\Default\\ (ou Microsoft\\Edge...).

#### **Le Fichier Preferences**

Ce fichier JSON contient l'essentiel de la configuration.

* **Extensions (extensions.settings) :** Cette section liste toutes les extensions installées. Chaque clé est l'ID de l'extension (32 caractères).  
  * *Indicateurs de Compromission :* Recherchez les extensions dont le chemin (path) pointe hors du dossier d'installation standard (extensions "externes" forcées).  
  * *Moteur de Recherche (default\_search\_provider) :* Vérifiez l'URL de recherche. Les adwares remplacent souvent Google/Bing par des moteurs "feed" (ex: Trovi, Conduit).30  
* **Secure Preferences :** Chrome protège certaines préférences critiques (comme la page d'accueil) avec un HMAC (Hash-based Message Authentication Code) stocké dans le nœud protection. Si Pieuvre modifie le fichier Preferences manuellement (ex: pour supprimer une extension) sans recalculer ce HMAC, Chrome détectera la corruption et réinitialisera tout le profil.  
  * *Stratégie de Remédiation :* Au lieu d'éditer le JSON (risqué), la méthode recommandée est d'utiliser les politiques de suppression (Policy) ou de supprimer les dossiers de l'extension et de laisser Chrome effectuer son auto-réparation ("Soft Reset").

#### **Politiques de Gestion (Enterprise Policies)**

Les adwares utilisent abusivement les mécanismes de gestion de flotte d'entreprise pour empêcher l'utilisateur de désinstaller une extension.

* **Clé Registre :** HKLM\\SOFTWARE\\Policies\\Google\\Chrome\\ExtensionInstallForcelist.  
* **Analyse :** Sur une machine "Grand Public" (Home Edition), cette clé ne devrait généralement pas exister ou être vide. Si elle contient des IDs d'extensions, il s'agit à 99% d'un navigateur détourné. Pieuvre doit offrir la possibilité de purger cette clé.32

### **3.2 Mozilla Firefox : Une Approche Hybride**

La configuration de Firefox est plus fragmentée et moins centralisée que celle de Chrome.

#### **extensions.json : L'Inventaire des Add-ons**

Situé à la racine du profil, ce fichier JSON liste les extensions. Contrairement à Chrome, il fournit des métadonnées précieuses pour le diagnostic.34

* **Champs Critiques :**  
  * userDisabled (booléen) : Si false, l'extension est active.  
  * foreignInstall (booléen) : Si true, l'extension a été installée par un programme externe (sideloading), méthode favorite des adwares.  
  * sourceURI : L'origine du téléchargement. Une URL non-Mozilla ici est suspecte.  
* **Remédiation :** Modifier ce fichier pour passer userDisabled à true est une méthode efficace pour neutraliser une extension sans casser le profil, car Firefox relit ce fichier au démarrage.

#### **prefs.js et user.js : Configuration JavaScript**

Le fichier prefs.js n'est pas du JSON, mais un script contenant des appels user\_pref("clé", valeur);.

* **Le Danger de user.js :** Firefox charge le fichier user.js au démarrage pour *écraser* les préférences de prefs.js. Les adwares inscrivent leur page d'accueil dans user.js pour rendre le changement persistant même si l'utilisateur modifie ses réglages via l'interface.  
* **Action Pieuvre :** Le scanner doit détecter la présence de user.js. S'il contient des URLs ou des paramètres de recherche, il doit être signalé, mis en quarantaine et supprimé. Il est rarement utilisé par les utilisateurs lambdas.37

#### **Base de Données places.sqlite**

Contient l'historique et les favoris. Les adwares insèrent des "Smart Bookmarks" ou polluent l'historique pour influencer l'autocomplétion de la barre d'adresse.

* **Outil Rust :** Utiliser la crate rusqlite pour exécuter des requêtes SQL de nettoyage (ex: DELETE FROM moz\_places WHERE url LIKE '%conduit%').40

## ---

**Chapitre 4 : Moteur de Détection (Signatures et Heuristiques)**

Une fois les artefacts (fichiers, clés de registre) collectés par les "walkers" (parcours récursifs), ils doivent être analysés pour déterminer leur nature malveillante. C'est ici que l'analyse par signatures entre en jeu.

### **4.1 Intégration de YARA en Rust**

YARA est le standard de facto pour la description de modèles de malwares. Pour Pieuvre, l'utilisation de la bibliothèque C historique (libyara) via des bindings est déconseillée pour des raisons de complexité de déploiement et de sécurité mémoire.

#### **Le Choix de la Modernité : YARA-X ou Boreal**

Deux réimplémentations de YARA en Rust pur existent et sont recommandées :

1. **YARA-X (par VirusTotal) :** Vise à remplacer YARA. Elle est plus sûre, souvent plus rapide, et supporte l'analyse de la mémoire et des fichiers. Elle offre une API Rust idiomatique.42  
2. **Boreal :** Une alternative conçue pour être un remplacement "drop-in" complet, souvent plus performante sur les très grands jeux de règles grâce à des optimisations d'automates finis.44

**Recommandation :** Intégrer **YARA-X** pour sa robustesse et son support actif par VirusTotal. Cela permet de compiler les règles YARA directement dans le binaire de Pieuvre, rendant l'outil autonome (portable) sans dépendances DLL externes.

#### **Syntaxe des Règles Adware**

Les règles pour Adware diffèrent de celles pour les virus. Elles cherchent des chaînes de caractères (URLs, noms de clés, GUIDs) plutôt que du code binaire polymorphe.

Extrait de code

rule Adware\_Conduit\_Registry {  
    meta:  
        description \= "Détecte les traces de registre de Conduit Search"  
        author \= "Pieuvre R\&D"  
        threat\_level \= 5  
    strings:  
        $clsid \= "{3c471948-f874-49f5-b338-4f214a2ee0b1}" nocase  
        $url \= "search.conduit.com" ascii wide  
        $publisher \= "Client Connect LTD" ascii wide  
    condition:  
        $clsid or ($url and $publisher)  
}

L'utilisation de conditions comme nocase (insensible à la casse) et ascii wide (compatible UTF-8 et UTF-16) est cruciale pour l'environnement Windows.45

### **4.2 Optimisation avec Aho-Corasick**

Scanner chaque fichier avec des milliers de règles YARA est coûteux en CPU. Pour optimiser, Pieuvre doit implémenter un pré-filtre utilisant l'algorithme Aho-Corasick.

* **Principe :** Cet algorithme permet de rechercher simultanément des milliers de mots-clés (patterns) dans un texte en une seule passe.  
* **Implémentation :** Utiliser la crate aho-corasick.  
* **Flux :**  
  1. Compiler une liste de 5000+ chaînes uniques caractéristiques (noms de dossiers PuP comme "Babylon", "SweetIM", "MyWebSearch").  
  2. Scanner les chemins de fichiers et les valeurs de registre avec Aho-Corasick.  
  3. Uniquement en cas de correspondance ("Hit"), lancer le scan YARA complet sur le fichier pour confirmer la variante exacte.46

### **4.3 Sources de Données Exploitables**

Pour alimenter la base de données de Pieuvre, plusieurs sources ouvertes sont disponibles :

1. **Bases ClamAV (.cvd) :** Bien que binaires, ces bases contiennent des millions de signatures MD5/SHA256. La crate clamav-client permet d'interagir avec un démon, mais pour un outil portable, il est préférable d'écrire un parser Rust pour extraire les signatures "PUA" (Potentially Unwanted Application) des fichiers .ndb (signatures normalisées) de ClamAV.48  
2. **MalwareBazaar :** Utiliser l'API pour récupérer quotidiennement les hashs tagués "Adware" et "Heur.PUP". Pieuvre peut inclure une base SQLite locale de hashs récents pour une détection rapide.50  
3. **Dépôts YARA Communautaires :** Le dépôt Neo23x0/signature-base contient des dossiers spécifiques pour les "PUP" et "Adware" qui sont directement intégrables.52

## ---

**Chapitre 5 : Stratégies de Nettoyage et Remédiation**

Détecter est une chose, nettoyer sans briser le système en est une autre. La remédiation sous Windows 11 présente des défis spécifiques liés au verrouillage des fichiers et à la stabilité du système.

### **5.1 Suppression au Redémarrage (L'Arme Absolue)**

Les adwares injectent souvent des DLLs dans explorer.exe ou d'autres processus système, rendant la suppression immédiate impossible (erreur "Fichier en cours d'utilisation").  
La méthode standard, utilisée par AdwCleaner, repose sur l'API Windows MoveFileExW avec le drapeau MOVEFILE\_DELAY\_UNTIL\_REBOOT (0x4).

* **Mécanisme Sous-jacent :** Cette API n'efface pas le fichier immédiatement. Elle ajoute une entrée dans la valeur de registre PendingFileRenameOperations sous HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager.  
* **Fonctionnement :** Au prochain démarrage, le Gestionnaire de Session (smss.exe) lit cette clé et supprime les fichiers *avant* de charger l'environnement utilisateur et les DLLs, contournant ainsi le verrouillage.  
* **Implémentation Rust :**  
  Rust  
  use windows\_sys::Win32::Storage::FileSystem::{MoveFileExW, MOVEFILE\_DELAY\_UNTIL\_REBOOT};

  // Pour supprimer (et non déplacer), le deuxième argument (lpNewFileName) doit être NULL.  
  // Le cast en PCWSTR est nécessaire.  
  unsafe {  
      let path \= to\_pcwstr("C:\\\\ProgramData\\\\Malicious\\\\adware.dll");  
      MoveFileExW(path.as\_ptr(), std::ptr::null(), MOVEFILE\_DELAY\_UNTIL\_REBOOT);  
  }

.13

### **5.2 Neutralisation des Processus : Suspendre avant de Tuer**

Les adwares modernes utilisent des processus "chiens de garde" (watchdogs). Si vous terminez le processus A, le processus B détecte l'arrêt et relance A immédiatement.

* **La Stratégie "Suspend" :** Au lieu d'utiliser TerminateProcess, Pieuvre doit utiliser l'API native (via NtSuspendProcess ou en suspendant tous les threads) pour *figer* tous les processus identifiés comme malveillants simultanément. Une fois tous figés, ils ne peuvent plus se relancer mutuellement. Le nettoyage peut alors procéder à la terminaison séquentielle.

### **5.3 Système de Quarantaine**

Aucune suppression ne doit être définitive sans filet de sécurité.

* **Architecture :**  
  1. Créer un dossier sécurisé (ex: C:\\Pieuvre\\Quarantine).  
  2. Avant suppression, copier le fichier en le chiffrant (un simple XOR avec une clé fixe suffit pour empêcher l'exécution accidentelle).  
  3. Maintenir un index JSON (quarantine\_log.json) mappant le chemin d'origine, le hash, la date et le nom de la détection.  
* **Restauration :** Cette structure permet à Pieuvre d'offrir une fonction "Annuler" (Undo), essentielle en cas de faux positif sur un fichier système critique.1

## ---

**Chapitre 6 : Architecture Recommandée pour "Pieuvre"**

Pour intégrer ces capacités dans l'outil existant, une architecture modulaire exploitant la concurrence de Rust est nécessaire.

### **6.1 Concurrence : Modèle Producteur-Consommateur**

Le scan d'un disque dur entier (centaines de milliers de fichiers) est lent s'il est séquentiel (I/O bound).

* **Producteurs (Walkers) :** Utiliser la crate walkdir ou jwalk (parallèle) pour parcourir l'arborescence disque et envoyer les chemins de fichiers dans un canal (crossbeam-channel).  
* **Consommateurs (Scanners) :** Un pool de threads (géré par rayon ou tokio) récupère les chemins, effectue les lectures de fichiers et applique les règles YARA/Aho-Corasick.  
* **Avantage :** Cette séparation permet de saturer les capacités I/O du disque sans bloquer le CPU pour l'analyse des signatures.6

### **6.2 Tableaux de Données : Structure du Rapport de Scan**

Pour faciliter le traitement par l'interface utilisateur ou l'exportation, les résultats doivent être structurés.

| Type de Menace | Emplacement (Artefact) | Famille (Signature) | Action Recommandée | Statut |
| :---- | :---- | :---- | :---- | :---- |
| **Registre** | HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run\\Update | PUP.Optional.Conduit | Supprimer | En attente |
| **Fichier** | C:\\Users\\User\\AppData\\Roaming\\Babylon\\bin.exe | Adware.Babylon | Quarantaine \+ Suppr | En attente |
| **Navigateur** | Chrome Prefs: homepage | Hijacker.SearchPage | Réinitialiser Valeur | En attente |
| **Tâche** | Task: \\Microsoft\\Windows\\Maintenance\\WinDefendr | Trojan.Poweliks | Supprimer Tâche | En attente |

### **6.3 Sécurité et Allowlisting**

Pour éviter les catastrophes (suppression de explorer.exe ou de clés de démarrage Windows), Pieuvre doit intégrer une liste blanche (allowlist) "en dur".

* **Vérification de Signature Authenticode :** Avant de flagger un fichier comme suspect uniquement sur heuristique, vérifier s'il est signé numériquement par Microsoft, Google ou Mozilla. La crate windows permet d'appeler WinVerifyTrust. C'est une barrière de sécurité indispensable contre les faux positifs.

## ---

**Conclusion**

Le développement d'un moteur antimalware en Rust pour Windows 11 représente une opportunité majeure de moderniser l'outillage de sécurité. En combinant la sécurité mémoire native de Rust avec la puissance des API windows-rs et la rapidité des moteurs YARA nouvelle génération (yara-x), il est possible de créer un outil plus résilient, plus rapide et plus sûr que ses prédécesseurs écrits en C/C++.

La réussite du projet "Pieuvre" reposera moins sur la complexité du code que sur la finesse de l'analyse forensique : la capacité à parser correctement les fichiers de préférences obscurcis des navigateurs, à débusquer les tâches planifiées cachées, et à nettoyer proprement le registre sans laisser de résidus corrompus. Les éléments fournis dans ce rapport constituent les briques techniques fondamentales pour atteindre cet objectif.

#### **Sources des citations**

1. AdwCleaner wants to get rid of about 2 dozen Registry Keys, what do I do? \- Reddit, consulté le décembre 30, 2025, [https://www.reddit.com/r/techsupport/comments/37cqfa/adwcleaner\_wants\_to\_get\_rid\_of\_about\_2\_dozen/](https://www.reddit.com/r/techsupport/comments/37cqfa/adwcleaner_wants_to_get_rid_of_about_2_dozen/)  
2. ZHPCleaner (R) | PDF | Windows Registry | Operating System Technology \- Scribd, consulté le décembre 30, 2025, [https://pt.scribd.com/document/546304994/ZHPCleaner-R](https://pt.scribd.com/document/546304994/ZHPCleaner-R)  
3. List of Common PUPs and Adware Applications \- ToolsLib Blog, consulté le décembre 30, 2025, [https://blog.toolslib.net/2024/11/03/list-of-common-pups-and-adware-applications/](https://blog.toolslib.net/2024/11/03/list-of-common-pups-and-adware-applications/)  
4. When developing for Windows, what are the (dis)advantages of winapi vs. windows crate?, consulté le décembre 30, 2025, [https://users.rust-lang.org/t/when-developing-for-windows-what-are-the-dis-advantages-of-winapi-vs-windows-crate/84721](https://users.rust-lang.org/t/when-developing-for-windows-what-are-the-dis-advantages-of-winapi-vs-windows-crate/84721)  
5. Verify the Safety of the Rust Standard Library | AWS Open Source Blog, consulté le décembre 30, 2025, [https://aws.amazon.com/blogs/opensource/verify-the-safety-of-the-rust-standard-library/](https://aws.amazon.com/blogs/opensource/verify-the-safety-of-the-rust-standard-library/)  
6. Rust Performance Optimizations Compared to Other Programming Languages \- Medium, consulté le décembre 30, 2025, [https://medium.com/@kaly.salas.7/rust-performance-optimizations-compared-to-other-programming-languages-c2e3685163e2](https://medium.com/@kaly.salas.7/rust-performance-optimizations-compared-to-other-programming-languages-c2e3685163e2)  
7. Windows programmers, which crate do you use to call Windows API? : r/rust \- Reddit, consulté le décembre 30, 2025, [https://www.reddit.com/r/rust/comments/169pobp/windows\_programmers\_which\_crate\_do\_you\_use\_to/](https://www.reddit.com/r/rust/comments/169pobp/windows_programmers_which_crate_do_you_use_to/)  
8. What's the difference between the winapi and windows-sys crates in Rust? \- Reddit, consulté le décembre 30, 2025, [https://www.reddit.com/r/rust/comments/12b6c5u/whats\_the\_difference\_between\_the\_winapi\_and/](https://www.reddit.com/r/rust/comments/12b6c5u/whats_the_difference_between_the_winapi_and/)  
9. Rust for Windows, and the windows crate | Microsoft Learn, consulté le décembre 30, 2025, [https://learn.microsoft.com/en-us/windows/dev-environment/rust/rust-for-windows](https://learn.microsoft.com/en-us/windows/dev-environment/rust/rust-for-windows)  
10. microsoft/windows-rs: Rust for Windows \- GitHub, consulté le décembre 30, 2025, [https://github.com/microsoft/windows-rs](https://github.com/microsoft/windows-rs)  
11. windows-sys \- crates.io: Rust Package Registry, consulté le décembre 30, 2025, [https://crates.io/crates/windows-sys](https://crates.io/crates/windows-sys)  
12. MOVEFILE\_DELAY\_UNTIL\_REB, consulté le décembre 30, 2025, [https://stdrs.dev/nightly/x86\_64-pc-windows-gnu/std/sys/windows/c/windows\_sys/constant.MOVEFILE\_DELAY\_UNTIL\_REBOOT.html](https://stdrs.dev/nightly/x86_64-pc-windows-gnu/std/sys/windows/c/windows_sys/constant.MOVEFILE_DELAY_UNTIL_REBOOT.html)  
13. MoveFileExW function (winbase.h) \- Win32 apps | Microsoft Learn, consulté le décembre 30, 2025, [https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefileexw](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefileexw)  
14. oscp-cpts-notes/windows-privilege-escalation/user-privileges/sedebugprivilege.md at main, consulté le décembre 30, 2025, [https://github.com/dollarboysushil/oscp-cpts-notes/blob/main/windows-privilege-escalation/user-privileges/sedebugprivilege.md](https://github.com/dollarboysushil/oscp-cpts-notes/blob/main/windows-privilege-escalation/user-privileges/sedebugprivilege.md)  
15. SeDebugPrivilege | OSCP-CPTS NOTES, consulté le décembre 30, 2025, [https://notes.dollarboysushil.com/windows-privilege-escalation/user-privileges/sedebugprivilege](https://notes.dollarboysushil.com/windows-privilege-escalation/user-privileges/sedebugprivilege)  
16. Enabling and Disabling Privileges in C++ \- Win32 apps | Microsoft Learn, consulté le décembre 30, 2025, [https://learn.microsoft.com/en-us/windows/win32/secauthz/enabling-and-disabling-privileges-in-c--](https://learn.microsoft.com/en-us/windows/win32/secauthz/enabling-and-disabling-privileges-in-c--)  
17. Only one element in the array field "privileges" of TOKEN\_PRIVILEGES is reachable · Issue \#2375 · microsoft/windows-rs \- GitHub, consulté le décembre 30, 2025, [https://github.com/microsoft/windows-rs/issues/2375](https://github.com/microsoft/windows-rs/issues/2375)  
18. Windows-rs and raw point to array of struct \- help \- The Rust Programming Language Forum, consulté le décembre 30, 2025, [https://users.rust-lang.org/t/windows-rs-and-raw-point-to-array-of-struct/64579](https://users.rust-lang.org/t/windows-rs-and-raw-point-to-array-of-struct/64579)  
19. Windows Vista/Windows 7 privilege: SeDebugPrivilege & OpenProcess \- Stack Overflow, consulté le décembre 30, 2025, [https://stackoverflow.com/questions/2932461/windows-vista-windows-7-privilege-sedebugprivilege-openprocess](https://stackoverflow.com/questions/2932461/windows-vista-windows-7-privilege-sedebugprivilege-openprocess)  
20. Rusty Rootkit \- Windows Kernel Rookit in Rust (Codename: Eagle) \- memN0ps, consulté le décembre 30, 2025, [https://memn0ps.github.io/rusty-windows-kernel-rootkit/](https://memn0ps.github.io/rusty-windows-kernel-rootkit/)  
21. Windows Malware Persistence: Common Techniques Explained \- Cofense, consulté le décembre 30, 2025, [https://cofense.com/blog/windows-persistence-explained-techniques,-risks,-and-what-defenders-should-know](https://cofense.com/blog/windows-persistence-explained-techniques,-risks,-and-what-defenders-should-know)  
22. Windows Persistence: Registry Run Keys | by Fahri Korkmaz \- Medium, consulté le décembre 30, 2025, [https://r4bb1t.medium.com/windows-persistence-registry-run-keys-e9acb20c4a7d](https://r4bb1t.medium.com/windows-persistence-registry-run-keys-e9acb20c4a7d)  
23. Event Triggered Execution: AppInit DLLs, Sub-technique T1546.010 \- MITRE ATT\&CK®, consulté le décembre 30, 2025, [https://attack.mitre.org/techniques/T1546/010/](https://attack.mitre.org/techniques/T1546/010/)  
24. Advanced Windows Persistence, Part 1: Remaining Inside the Windows Target, consulté le décembre 30, 2025, [https://hackers-arise.com/advanced-windows-persistence-part-1-remaining-inside-the-windows-target/](https://hackers-arise.com/advanced-windows-persistence-part-1-remaining-inside-the-windows-target/)  
25. From Registry With Love: Malware Registry Abuses \- Splunk, consulté le décembre 30, 2025, [https://www.splunk.com/en\_us/blog/security/from-registry-with-love-malware-registry-abuses.html](https://www.splunk.com/en_us/blog/security/from-registry-with-love-malware-registry-abuses.html)  
26. Registry Persistence via AppInit DLL | Elastic Security \[8.19\], consulté le décembre 30, 2025, [https://www.elastic.co/guide/en/security/8.19/registry-persistence-via-appinit-dll.html](https://www.elastic.co/guide/en/security/8.19/registry-persistence-via-appinit-dll.html)  
27. Persistence Techniques That Persist \- CyberArk, consulté le décembre 30, 2025, [https://www.cyberark.com/resources/threat-research-blog/persistence-techniques-that-persist](https://www.cyberark.com/resources/threat-research-blog/persistence-techniques-that-persist)  
28. Exploring Persistence Mechanisms in Windows Scheduled Tasks \- Security Blue Team, consulté le décembre 30, 2025, [https://www.securityblue.team/blog/posts/persistence-mechanisms-windows-scheduled-tasks](https://www.securityblue.team/blog/posts/persistence-mechanisms-windows-scheduled-tasks)  
29. Defending Against Scheduled Task Attacks in Windows Environments \- Qualys Blog, consulté le décembre 30, 2025, [https://blog.qualys.com/vulnerabilities-threat-research/2022/06/20/defending-against-scheduled-task-attacks-in-windows-environments](https://blog.qualys.com/vulnerabilities-threat-research/2022/06/20/defending-against-scheduled-task-attacks-in-windows-environments)  
30. Set Chrome policies for users or browsers \- Chrome Enterprise and Education Help, consulté le décembre 30, 2025, [https://support.google.com/chrome/a/answer/2657289?hl=en](https://support.google.com/chrome/a/answer/2657289?hl=en)  
31. Setting the Default Search Provider on Chrome via a script \- Stack Overflow, consulté le décembre 30, 2025, [https://stackoverflow.com/questions/6657472/setting-the-default-search-provider-on-chrome-via-a-script](https://stackoverflow.com/questions/6657472/setting-the-default-search-provider-on-chrome-via-a-script)  
32. Configure ExtensionSettings policy \- Chrome Enterprise and Education Help, consulté le décembre 30, 2025, [https://support.google.com/chrome/a/answer/9867568?hl=en](https://support.google.com/chrome/a/answer/9867568?hl=en)  
33. chrome/app/policy/policy\_templates.json \- chromium/src \- Git at Google, consulté le décembre 30, 2025, [https://chromium.googlesource.com/chromium/src/+/39682d17faa74c02c75b6e6d3b73421c32ab57f0/chrome/app/policy/policy\_templates.json](https://chromium.googlesource.com/chromium/src/+/39682d17faa74c02c75b6e6d3b73421c32ab57f0/chrome/app/policy/policy_templates.json)  
34. All of my addons are disabled and won't re-enable\! | Firefox Support Forum, consulté le décembre 30, 2025, [https://support.mozilla.org/gl/questions/1499472](https://support.mozilla.org/gl/questions/1499472)  
35. Where does firefox store the data whether an extension is enabled or disabled? \[closed\], consulté le décembre 30, 2025, [https://stackoverflow.com/questions/77856717/where-does-firefox-store-the-data-whether-an-extension-is-enabled-or-disabled](https://stackoverflow.com/questions/77856717/where-does-firefox-store-the-data-whether-an-extension-is-enabled-or-disabled)  
36. Firefox/Privacy \- ArchWiki, consulté le décembre 30, 2025, [https://wiki.archlinux.org/title/Firefox/Privacy](https://wiki.archlinux.org/title/Firefox/Privacy)  
37. How does user.js work in firefox in detail \- javascript \- Stack Overflow, consulté le décembre 30, 2025, [https://stackoverflow.com/questions/10602504/how-does-user-js-work-in-firefox-in-detail](https://stackoverflow.com/questions/10602504/how-does-user-js-work-in-firefox-in-detail)  
38. user.js and prefs.js : r/firefox \- Reddit, consulté le décembre 30, 2025, [https://www.reddit.com/r/firefox/comments/f93on8/userjs\_and\_prefsjs/](https://www.reddit.com/r/firefox/comments/f93on8/userjs_and_prefsjs/)  
39. Cutting Mozilla Out of Firefox With a user.js\! \- Trafotin.com, consulté le décembre 30, 2025, [https://trafotin.com/v/firefox-userjs/](https://trafotin.com/v/firefox-userjs/)  
40. Processing places.sqlite file with Linux \- firefox \- Stack Overflow, consulté le décembre 30, 2025, [https://stackoverflow.com/questions/8303139/processing-places-sqlite-file-with-linux](https://stackoverflow.com/questions/8303139/processing-places-sqlite-file-with-linux)  
41. How do I repair a corrupted Firefox places.sqlite database? \- Super User, consulté le décembre 30, 2025, [https://superuser.com/questions/111998/how-do-i-repair-a-corrupted-firefox-places-sqlite-database](https://superuser.com/questions/111998/how-do-i-repair-a-corrupted-firefox-places-sqlite-database)  
42. YARA-X, consulté le décembre 30, 2025, [https://virustotal.github.io/yara-x/](https://virustotal.github.io/yara-x/)  
43. YARA is dead, long live YARA-X, consulté le décembre 30, 2025, [https://virustotal.github.io/yara-x/blog/yara-is-dead-long-live-yara-x/](https://virustotal.github.io/yara-x/blog/yara-is-dead-long-live-yara-x/)  
44. vthib/boreal: Safe and performant YARA rules evaluator in Rust \- GitHub, consulté le décembre 30, 2025, [https://github.com/vthib/boreal](https://github.com/vthib/boreal)  
45. YARA \- The pattern matching swiss knife for malware researchers, consulté le décembre 30, 2025, [https://virustotal.github.io/yara/](https://virustotal.github.io/yara/)  
46. The Aho-Corasick Paradigm in Modern Antivirus Engines: A Cornerstone of Signature-Based Malware Detection \- MDPI, consulté le décembre 30, 2025, [https://www.mdpi.com/1999-4893/18/12/742](https://www.mdpi.com/1999-4893/18/12/742)  
47. ahocorasick-rs \- PyPI, consulté le décembre 30, 2025, [https://pypi.org/project/ahocorasick-rs/](https://pypi.org/project/ahocorasick-rs/)  
48. Hash-based Signatures \- ClamAV Documentation, consulté le décembre 30, 2025, [https://docs.clamav.net/manual/Signatures/HashSignatures.html](https://docs.clamav.net/manual/Signatures/HashSignatures.html)  
49. clamav-client \- crates.io: Rust Package Registry, consulté le décembre 30, 2025, [https://crates.io/crates/clamav-client](https://crates.io/crates/clamav-client)  
50. MalwareBazaar | Malware sample exchange, consulté le décembre 30, 2025, [https://bazaar.abuse.ch/](https://bazaar.abuse.ch/)  
51. Adware \- MalwareBazaar, consulté le décembre 30, 2025, [https://bazaar.abuse.ch/browse/tag/Adware/](https://bazaar.abuse.ch/browse/tag/Adware/)  
52. Neo23x0/signature-base: YARA signature and IOC database for my scanners and tools, consulté le décembre 30, 2025, [https://github.com/Neo23x0/signature-base](https://github.com/Neo23x0/signature-base)  
53. How does Windows remove locked files in the next reboot when you uninstall a program?, consulté le décembre 30, 2025, [https://stackoverflow.com/questions/21641006/how-does-windows-remove-locked-files-in-the-next-reboot-when-you-uninstall-a-pro](https://stackoverflow.com/questions/21641006/how-does-windows-remove-locked-files-in-the-next-reboot-when-you-uninstall-a-pro)