La fonction `toggle_chronometer` est une commande Tauri qui joue un rôle clé dans la gestion du chronomètre. Explorons ce que fait cette fonction ligne par ligne :

### Préparation des Variables

1. **Obtention des Verrous sur les Variables Globales :**
   ```rust
   let mut is_running = IS_RUNNING.lock().unwrap();
   let mut elapsed = ELAPSED_TIME.lock().unwrap();
   let mut start_time = START_TIME.lock().unwrap();
   ```
   Ces lignes obtiennent des verrous sur les variables globales. Le `lock().unwrap()` est une façon de dire : "J'ai besoin d'accéder à cette ressource, et je suis sûr qu'elle est disponible". Cela empêche d'autres threads d'accéder à ces variables en même temps, assurant ainsi une modification sûre.

### Logique de Commutation

2. **Vérification de l'État du Chronomètre :**
   ```rust
   if *is_running {
       ...
   } else {
       ...
   }
   ```
   Cette structure conditionnelle vérifie si le chronomètre est en cours d'exécution (`*is_running` est `true`) ou s'il est arrêté (`false`).

3. **Si le Chronomètre est en Cours d'Exécution :**
   - **Enregistrement du Temps Écoulé :**
     ```rust
     *elapsed += start_time.elapsed();
     ```
     Si le chronomètre est actif, cette ligne ajoute le temps écoulé depuis le dernier `start_time` à `elapsed`. Cela permet de garder une trace du temps total écoulé jusqu'à ce point.
   - **Réinitialisation de `start_time` :**
     ```rust
     *start_time = Instant::now();
     ```
     Réinitialise `start_time` à l'instant actuel. C'est nécessaire pour préparer la prochaine reprise du chronomètre après sa pause.

4. **Si le Chronomètre est Arrêté :**
   - **Définition de `start_time` pour la Reprise :**
     ```rust
     *start_time = Instant::now();
     ```
     Si le chronomètre est arrêté, cette ligne définit `start_time` à l'instant actuel, afin que, lorsque le chronomètre est repris, il commence à compter à partir de ce moment.

5. **Inversion de l'État du Chronomètre :**
   ```rust
   *is_running = !*is_running;
   ```
   Finalement, l'état du chronomètre est inversé. Si le chronomètre était en cours d'exécution, il s'arrête; s'il était arrêté, il commence à fonctionner.

### Résumé

La fonction `toggle_chronometer` gère l'activation et la désactivation du chronomètre. Elle met à jour les variables globales pour refléter l'état actuel du chronomètre, soit en accumulant le temps écoulé lorsqu'il est mis en pause, soit en se préparant à compter à partir de l'instant actuel lorsqu'il est repris. Ce mécanisme assure un suivi précis du temps écoulé, essentiel pour le fonctionnement du chronomètre.