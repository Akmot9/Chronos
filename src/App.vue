<template>
  <div class="chronometer">
    <!-- Display the time -->
    <div class="time-display">{{ time }}</div>
    <button @click="toggleChronometer">toggle</button>
    <button @click="resetChronometer">Reset</button>
  </div>
</template>

<script>
import { event } from '@tauri-apps/api';
import { invoke } from '@tauri-apps/api/tauri';

export default {
  data() {
    return {
      time: '00:00:00.000', // Définition initiale de la propriété réactive time

    };
  },
  methods: {
    async toggleChronometer() {
      try {
        await invoke('toggle_chronometer');
      } catch (error) {
        console.error('Failed to toggle the chronometer:', error);
      }
    },
    async resetChronometer() {
    try {
      await invoke('reset_chronometer');
      this.time = '00:00:00.000'; // Reset the time on the frontend
    } catch (error) {
      console.error('Failed to reset the chronometer:', error);
    }
  },
  },
  mounted() {
  // Ajout de l'écouteur d'événement quand le composant est monté
    event.listen('chronometer-update', (response) => {
      // Update the time with the received payload, regardless of its value
      this.time = response.payload.message;
    });
  },

};
</script>

<style scoped>
.chronometer {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background-color: #282c34;
  color: #61dafb;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.time-display {
  font-size: 3rem;
  letter-spacing: 0.1rem;
  padding: 20px;
  border-radius: 10px;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.3);
  background: #1e2128;
}

button {
  margin-top: 20px;
  padding: 10px 20px;
  font-size: 1rem;
  color: white;
  background-color: #61dafb;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  outline: none;
}
button:hover {
  background-color: #42a0f5;
}
button:active {
  background-color: #1e90ff;
}
</style>


