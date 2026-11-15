import { create } from 'zustand';
import { api, BackendInstance, Loader } from '../lib/api';
export type Instance = { id: string; name: string; version: string; loader: 'Vanilla' | 'Fabric' | 'Forge' | 'Quilt'; color: string };
type Store = { instances: Instance[]; selectedId: string; hydrated: boolean; setSelected: (id: string) => void; hydrate: () => Promise<void>; addInstance: (instance: Instance) => void; removeInstance: (id: string) => void; toast: string; notify: (message: string) => void };
const initial: Instance[] = [{ id: 'survival', name: 'Survival World', version: '1.21.1', loader: 'Vanilla', color: 'sunset' }, { id: 'modded', name: 'Modded Adventures', version: '1.20.1', loader: 'Fabric', color: 'forest' }, { id: 'creative', name: 'Creative Lab', version: '1.21.1', loader: 'Vanilla', color: 'ocean' }];
const colors = ['forest', 'sunset', 'ocean'];
const toBackend = (instance: Instance): BackendInstance => ({ id: instance.id, name: instance.name, version: instance.version, loader: instance.loader.toLowerCase() as Loader, game_dir: '' });
export const useLauncher = create<Store>((set, get) => ({
  instances: initial,
  selectedId: 'survival',
  hydrated: false,
  setSelected: (selectedId) => set({ selectedId }),
  hydrate: async () => {
    if (!api.isDesktop()) { set({ hydrated: true }); return; }
    try {
      const saved = await api.listInstances();
      const instances = saved.map((item, index) => ({ id: item.id, name: item.name, version: item.version, loader: (item.loader[0].toUpperCase() + item.loader.slice(1)) as Instance['loader'], color: colors[index % colors.length] }));
      set({ instances, selectedId: instances[0]?.id ?? '', hydrated: true });
    } catch { set({ hydrated: true }); get().notify('Could not load saved instances'); }
  },
  addInstance: (instance) => {
    set((state) => ({ instances: [...state.instances, instance], selectedId: instance.id }));
    if (api.isDesktop()) api.createInstance({ name: instance.name, version: instance.version, loader: instance.loader.toLowerCase() as Loader })
      .then((saved) => set((state) => ({ instances: state.instances.map((item) => item.id === instance.id ? { ...item, id: saved.id } : item), selectedId: state.selectedId === instance.id ? saved.id : state.selectedId })))
      .catch(() => { set((state) => ({ instances: state.instances.filter((item) => item.id !== instance.id) })); get().notify('Could not save instance'); });
  },
  removeInstance: (id) => {
    set((state) => ({ instances: state.instances.filter((item) => item.id !== id), selectedId: state.selectedId === id ? (state.instances.find((item) => item.id !== id)?.id ?? '') : state.selectedId }));
    if (api.isDesktop()) api.deleteInstance(id).catch(() => get().notify('Could not delete saved instance'));
  },
  toast: '',
  notify: (toast) => { set({ toast }); window.setTimeout(() => set((state) => state.toast === toast ? { toast: '' } : state), 3000); },
}));
