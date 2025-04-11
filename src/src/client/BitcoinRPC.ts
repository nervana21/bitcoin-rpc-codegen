import axios, { AxiosInstance } from 'axios';
import { RPCResponse, RPCError } from '../types/rpc';

export interface BitcoinRPCConfig {
  url: string;
  username: string;
  password: string;
  timeout?: number;
}

export class BitcoinRPC {
  private client: AxiosInstance;

  constructor(config: BitcoinRPCConfig) {
    this.client = axios.create({
      baseURL: config.url,
      auth: {
        username: config.username,
        password: config.password
      },
      timeout: config.timeout || 30000,
      headers: {
        'Content-Type': 'application/json'
      }
    });
  }

  private async request<T>(method: string, params: any[] = []): Promise<T> {
    try {
      const response = await this.client.post<RPCResponse<T>>('', {
        jsonrpc: '2.0',
        id: Date.now(),
        method,
        params
      });

      if (response.data.error) {
        throw new RPCError(response.data.error);
      }

      return response.data.result;
    } catch (error) {
      if (error instanceof RPCError) {
        throw error;
      }
      throw new RPCError({
        code: -32603,
        message: error instanceof Error ? error.message : 'Unknown error occurred'
      });
    }
  }

  // Blockchain RPC methods
  async getblockchaininfo() {
    return this.request<any>('getblockchaininfo');
  }

  async getblockcount() {
    return this.request<number>('getblockcount');
  }

  async getblockhash(height: number) {
    return this.request<string>('getblockhash', [height]);
  }

  async getblock(blockhash: string, verbosity: number = 1) {
    return this.request<any>('getblock', [blockhash, verbosity]);
  }

  // Transaction RPC methods
  async sendrawtransaction(hexstring: string, allowhighfees: boolean = false) {
    return this.request<string>('sendrawtransaction', [hexstring, allowhighfees]);
  }

  async getrawtransaction(txid: string, verbose: boolean = false) {
    return this.request<any>('getrawtransaction', [txid, verbose]);
  }

  // Wallet RPC methods
  async getbalance(dummy: string = '*', minconf: number = 1, include_watchonly: boolean = true) {
    return this.request<number>('getbalance', [dummy, minconf, include_watchonly]);
  }

  async getnewaddress(label: string = '', address_type: string = '') {
    return this.request<string>('getnewaddress', [label, address_type]);
  }

  // Network RPC methods
  async getnetworkinfo() {
    return this.request<any>('getnetworkinfo');
  }

  async getpeerinfo() {
    return this.request<any[]>('getpeerinfo');
  }
} 