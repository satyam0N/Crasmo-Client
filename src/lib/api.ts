import { invoke, isTauri } from '@tauri-apps/api/core';

export type LauncherError = { code: 'NOT_READY' | 'IO' | 'NETWORK' | 'INVALID_INPUT' | 'NOT_FOUND'; message: string };
export type Loader = 'vanilla' | 'fabric' | 'forge' | 'quilt';
export type BackendInstance = { id: string; name: string; version: string; loader: Loader; game_dir: string };
export type NewInstance = Pick<BackendInstance, 'name' | 'version' | 'loader'>;
export type Settings = { memory_min_gb: number; memory_max_gb: number; memory_recommended_gb: number; close_on_launch: boolean; show_snapshots: boolean; game_directory: string | null };
export type VersionEntry = { id: string; type: string; url: string; release_time: string };
export type VersionManifest = { latest: Record<string, string>; versions: VersionEntry[] };

export const api = {
  isDesktop: isTauri,
  ping: () => invoke<string>('ping'),
  listInstances: () => invoke<BackendInstance[]>('list_instances'),
  createInstance: (input: NewInstance) => invoke<BackendInstance>('create_instance', { input }),
  deleteInstance: (id: string) => invoke<void>('delete_instance', { id }),
  getSettings: () => invoke<Settings>('get_settings'),
  saveSettings: (settings: Settings) => invoke<void>('save_settings', { settings }),
  getVersionManifest: (includeSnapshots: boolean) => invoke<VersionManifest>('get_version_manifest', { includeSnapshots }),
};
