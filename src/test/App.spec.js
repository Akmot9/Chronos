import { describe, beforeEach, afterEach, it, expect, vi } from 'vitest';
import { mount } from '@vue/test-utils';


import App from '../App.vue';

// Mock module
vi.mock('@tauri-apps/api/tauri', () => {
  return {
    invoke: vi.fn((cmd, args) => {
      // Provide mock responses based on the command
      if (cmd === 'toggle_chronometer') {
        // Mock response for toggle_chronometer
      } else if (cmd === 'reset_chronometer') {
        // Mock response for reset_chronometer
      } else if (cmd === 'save_lap') {
        // Mock response for save_lap
      }
    }),
  };
});

describe('App.vue', () => {
  let wrapper;

  beforeEach(() => {
    wrapper = mount(App);
  });

  afterEach(() => {
    vi.clearAllMocks(); // Clear mocks after each test
  });

  it('toggles the chronometer', async () => {
    const toggleButton = wrapper.find('.button.start');
    await toggleButton.trigger('click');
    // Add assertions to check if the correct invoke command was called
  });

  // Add more tests for reset and lap functionality
});
