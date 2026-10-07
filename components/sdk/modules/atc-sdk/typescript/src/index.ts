export interface RpcRequest<P = unknown> {
  jsonrpc: "2.0";
  id: number;
  method: string;
  params: P;
}

export interface RpcError {
  code: number;
  message: string;
}

export interface RpcResponse<T = unknown> {
  jsonrpc: "2.0";
  id: number;
  result?: T;
  error?: RpcError;
}

export interface ChainInfo {
  chain_id: string;
  network: string;
  protocol_version: string;
}

export interface Balance {
  address: string;
  amount: string;
}

export class AtcSdkError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "AtcSdkError";
  }
}

export class ATCClient {
  readonly endpoint: URL;

  constructor(endpoint: string | URL) {
    const url = endpoint instanceof URL ? endpoint : new URL(endpoint);
    if (!["http:", "https:", "ws:", "wss:"].includes(url.protocol)) {
      throw new AtcSdkError(`Unsupported endpoint scheme: ${url.protocol}`);
    }
    this.endpoint = url;
  }

  request<P>(id: number, method: string, params: P): RpcRequest<P> {
    return { jsonrpc: "2.0", id, method, params };
  }

  unwrap<T>(response: RpcResponse<T>): T {
    if (response.error) {
      throw new AtcSdkError(`RPC ${response.error.code}: ${response.error.message}`);
    }
    if (response.result === undefined) {
      throw new AtcSdkError("RPC response contains neither result nor error");
    }
    return response.result;
  }
}
