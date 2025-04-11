export interface RPCError {
  code: number;
  message: string;
}

export class RPCError extends Error {
  code: number;

  constructor(error: RPCError) {
    super(error.message);
    this.code = error.code;
    this.name = 'RPCError';
  }
}

export interface RPCResponse<T> {
  jsonrpc: string;
  id: number;
  result: T;
  error?: RPCError;
} 