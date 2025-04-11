/// <reference types="jest" />
import { BitcoinRPC } from '../BitcoinRPC';
import { RPCError } from '../../types/rpc';
import axios from 'axios';

jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

describe('BitcoinRPC', () => {
  let rpc: BitcoinRPC;
  const config = {
    url: 'http://localhost:8332',
    username: 'test',
    password: 'test'
  };

  beforeEach(() => {
    rpc = new BitcoinRPC(config);
    jest.clearAllMocks();
  });

  describe('getblockchaininfo', () => {
    it('should return blockchain info', async () => {
      const mockResponse = {
        data: {
          jsonrpc: '2.0',
          id: 1,
          result: {
            chain: 'main',
            blocks: 123456,
            headers: 123456,
            bestblockhash: '000000000000000000000000000000000000000000000000000000000000000'
          }
        }
      };

      mockedAxios.create.mockReturnValue({
        post: jest.fn().mockResolvedValue(mockResponse)
      } as any);

      const result = await rpc.getblockchaininfo();
      expect(result).toEqual(mockResponse.data.result);
    });

    it('should handle RPC errors', async () => {
      const mockError = {
        data: {
          jsonrpc: '2.0',
          id: 1,
          error: {
            code: -32601,
            message: 'Method not found'
          }
        }
      };

      mockedAxios.create.mockReturnValue({
        post: jest.fn().mockRejectedValue(mockError)
      } as any);

      await expect(rpc.getblockchaininfo()).rejects.toThrow(RPCError);
    });
  });
}); 