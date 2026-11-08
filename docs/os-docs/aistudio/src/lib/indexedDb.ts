// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ATC-OS Indexed Database Wrapper
export class DatabaseIndex {
  private dbName: string;
  private version: number;
  private db: IDBDatabase | null = null;
  private storeName: string;

  constructor(dbName: string = 'atc_os_database', storeName: string = 'key_value_store', version: number = 1) {
    this.dbName = dbName;
    this.storeName = storeName;
    this.version = version;
  }

  async connect(): Promise<IDBDatabase> {
    if (this.db) return this.db;
    
    return new Promise((resolve, reject) => {
      const request = indexedDB.open(this.dbName, this.version);

      request.onerror = () => {
        reject('Fehler beim Öffnen der IndexedDB');
      };

      request.onsuccess = (event: any) => {
        this.db = event.target.result;
        resolve(this.db!);
      };

      request.onupgradeneeded = (event: any) => {
        const db = event.target.result;
        if (!db.objectStoreNames.contains(this.storeName)) {
          db.createObjectStore(this.storeName, { keyPath: 'id' });
        }
      };
    });
  }

  async set(id: string, value: any): Promise<void> {
    const db = await this.connect();
    return new Promise((resolve, reject) => {
      const transaction = db.transaction([this.storeName], 'readwrite');
      const store = transaction.objectStore(this.storeName);
      const request = store.put({ id, value, timestamp: Date.now() });

      request.onsuccess = () => resolve();
      request.onerror = () => reject('Fehler beim Speichern');
    });
  }

  async get(id: string): Promise<any> {
    const db = await this.connect();
    return new Promise((resolve, reject) => {
      const transaction = db.transaction([this.storeName], 'readonly');
      const store = transaction.objectStore(this.storeName);
      const request = store.get(id);

      request.onsuccess = () => resolve(request.result?.value || null);
      request.onerror = () => reject('Fehler beim Abrufen');
    });
  }

  async getAll(): Promise<any[]> {
    const db = await this.connect();
    return new Promise((resolve, reject) => {
      const transaction = db.transaction([this.storeName], 'readonly');
      const store = transaction.objectStore(this.storeName);
      const request = store.getAll();

      request.onsuccess = () => resolve(request.result || []);
      request.onerror = () => reject('Fehler beim Abrufen aller Daten');
    });
  }

  async remove(id: string): Promise<void> {
    const db = await this.connect();
    return new Promise((resolve, reject) => {
      const transaction = db.transaction([this.storeName], 'readwrite');
      const store = transaction.objectStore(this.storeName);
      const request = store.delete(id);

      request.onsuccess = () => resolve();
      request.onerror = () => reject('Fehler beim Löschen');
    });
  }
}

export const atcDatabase = new DatabaseIndex();
