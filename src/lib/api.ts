import { invoke } from '@tauri-apps/api/core';
export type LauncherError = { code: 'NOT_READY' | 'IO' | 'NETWORK' | 'INVALID_INPUT'; message: string };
export const api = {
  ping: () => invoke<string>('ping'),
  listInstances: () => invoke<unknown[]>('list_instances'),
  getSettings: () => invoke<Record<string, unknown>>('get_settings'),
  saveSettings: (settings: Record<string, unknown>) => invoke<void>('save_settings', { settings }),
};
