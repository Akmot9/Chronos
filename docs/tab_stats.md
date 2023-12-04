Voici un tableau qui décrit les états des variables globales `IS_RUNNING`, `ELAPSED_TIME`, et `START_TIME` dans différentes situations dans votre application de chronomètre :

| État / Action         | `IS_RUNNING`        | `ELAPSED_TIME`                                             | `START_TIME`                    | Description                                                                                     |
|-----------------------|---------------------|------------------------------------------------------------|---------------------------------|-------------------------------------------------------------------------------------------------|
| **Initialisation**    | `false`             | `Duration::new(0, 0)`                                      | Temps actuel (`Instant::now()`) | Au démarrage, le chronomètre est arrêté, sans temps écoulé, et `START_TIME` est initialisé.      |
| **Démarrage**         | `true`              | Inchangé                                                   | Temps actuel (`Instant::now()`) | Le chronomètre démarre ; `START_TIME` est réinitialisé pour marquer le début de cette session.   |
| **En Cours**          | `true`              | Incrémente avec le temps écoulé                            | Inchangé                        | Tant que le chronomètre est en cours, `ELAPSED_TIME` augmente avec le temps écoulé.             |
| **Pause**             | `false`             | `ELAPSED_TIME` + Temps depuis dernier `START_TIME`         | Inchangé                        | Lors de la pause, le temps écoulé depuis le dernier démarrage est ajouté à `ELAPSED_TIME`.       |
| **Reprise**           | `true`              | Inchangé                                                   | Temps actuel (`Instant::now()`) | Lors de la reprise, `START_TIME` est réinitialisé pour comptabiliser le nouveau temps écoulé.    |
| **Réinitialisation**  | `false`             | `Duration::new(0, 0)`                                      | Temps actuel (`Instant::now()`) | Réinitialise le chronomètre : `ELAPSED_TIME` est remis à zéro et `START_TIME` est réinitialisé.  |

### Explications :

- **`IS_RUNNING`** indique si le chronomètre est actif (`true`) ou arrêté (`false`).
- **`ELAPSED_TIME`** représente le temps total écoulé lorsque le chronomètre est en pause.
- **`START_TIME`** marque l'instant précis où le chronomètre a été démarré ou repris après une pause.

Ce tableau aide à comprendre comment ces trois variables interagissent et évoluent en fonction des actions de l'utilisateur sur l'application de chronomètre.