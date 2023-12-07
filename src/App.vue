<template>
  <div class="chronometer">
    <div class="time-display">{{ time }}</div>
    
    <div class="buttons">
      <button class="button reset" @click="resetChronometer">Reset</button>
      <button class="button start" @click="toggleChronometer">Toggle</button>
      <button class="button start" @click="saveLap">Lap</button>
    </div>

    <div v-if="laps.length" class="lap-times">
      <div v-for="(lap, index) in laps" :key="index" class="lap">
        Tour {{ index + 1 }}: {{ lap }}
      </div>
    </div>
  </div>
</template>

<script>
import { event } from '@tauri-apps/api';
import { invoke } from '@tauri-apps/api/tauri';

export default {
  data() {
    return {
      time: '00:00:00.000', // Définition initiale de la propriété réactive time
      laps: [],
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
        this.laps = []; // Also reset the laps
      } catch (error) {
        console.error('Failed to reset the chronometer:', error);
      }
    },
    async saveLap() {
      try {
        const lapTime = await invoke('save_lap');
        this.laps.push(lapTime); // Add the new lap time to the array
      } catch (error) {
        console.error('Failed to save the lap:', error);
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

<style>
  .chronometer {
    display: flex;
    justify-content: center;
    flex-direction: column;
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

  .buttons {
    display: flex;
    justify-content: space-around;
    width: 100%;
    margin-bottom: 10px;
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

  .lap-times {
 width: 100%;
 display: flex;
 flex-direction: column;
 align-items: center;
 justify-content: flex-start;
}

  .lap {
    margin-bottom: 10px;
  }
 
</style>