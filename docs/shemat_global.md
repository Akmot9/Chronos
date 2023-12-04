sequenceDiagram
    participant UI as Interface Utilisateur
    participant Main as Thread Principal
    participant Chrono as Thread Chronomètre
    participant Cmd as Commandes Tauri

    Note over UI, Cmd: Interaction entre l'utilisateur et le backend

    UI->>Cmd: Action (Démarrer/Pause, Réinitialiser)
    Cmd->>Main: Modifier IS_RUNNING/ELAPSED_TIME/START_TIME
    loop Vérification et Calcul dans le Thread Chronomètre
        Main->>Chrono: Vérifie IS_RUNNING
        alt Si IS_RUNNING est vrai
            Chrono->>Chrono: Calcule le temps écoulé
        else Si IS_RUNNING est faux
            Chrono->>Chrono: Conserve le temps actuel
        end
        Chrono->>UI: Envoie le temps via canal mpsc
    end

    Note over Chrono: Boucle continue pour mise à jour et calcul du temps
