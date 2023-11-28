<template>
  <div class="chronometer">
    <!-- Display the time -->
    <div class="time-display">{{ time }}</div>
  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue';
import { event } from '@tauri-apps/api';

// Reactive property to hold the time
const time = ref('00:00:00.000');

onMounted(() => {
  // Add the event listener when the Vue component is mounted
  event.listen('event-name', (response) => {
    console.log('Event received:', response.payload);
    // Update the reactive property with the time received from the event
    time.value = response.payload.message;
  });
});
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
</style>
