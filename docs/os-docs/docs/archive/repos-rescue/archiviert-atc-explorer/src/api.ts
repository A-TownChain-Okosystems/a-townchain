// Backend API client
export class ApiClient {
  constructor(private baseUrl: string = 'http://localhost:4000') {}

  async get(path: string): Promise<any> {
    const res = await fetch(`${this.baseUrl}${path}`);
    if (!res.ok) throw new Error(`API error: ${res.status}`);
    return res.json();
  }

  async getBlock(height: number): Promise<any> { return this.get(`/blocks/${height}`); }
  async getTx(hash: string): Promise<any> { return this.get(`/tx/${hash}`); }
  async getAddress(addr: string): Promise<any> { return this.get(`/address/${addr}`); }
  async getStats(): Promise<any> { return this.get('/stats'); }
}
