import type { GatewayMessage, GatewayResponse } from '@/types';

type MessageHandler = (response: GatewayResponse) => void;

class GatewayClient {
  private ws: WebSocket | null = null;
  private url: string = '';
  private token: string = '';
  private handlers: Map<string, MessageHandler> = new Map();
  private reconnectTimer: number | null = null;
  private messageId = 0;
  private isConnected = false;
  private onConnectCallback: (() => void) | null = null;
  private onDisconnectCallback: (() => void) | null = null;

  connect(url: string, token: string) {
    this.url = url;
    this.token = token;
    this.createConnection();
  }

  private createConnection() {
    const wsUrl = this.url.replace(/^http/, 'ws') + '/ws' + (this.token ? `?token=${this.token}` : '');
    
    try {
      this.ws = new WebSocket(wsUrl);
      
      this.ws.onopen = () => {
        this.isConnected = true;
        this.onConnectCallback?.();
        if (this.reconnectTimer) {
          clearTimeout(this.reconnectTimer);
          this.reconnectTimer = null;
        }
      };
      
      this.ws.onclose = () => {
        this.isConnected = false;
        this.onDisconnectCallback?.();
        this.scheduleReconnect();
      };
      
      this.ws.onerror = () => {
        this.isConnected = false;
      };
      
      this.ws.onmessage = (event) => {
        try {
          const response = JSON.parse(event.data) as GatewayResponse;
          const id = (response as { id?: string }).id;
          if (id && this.handlers.has(id)) {
            const handler = this.handlers.get(id);
            this.handlers.delete(id);
            handler?.(response);
          }
        } catch (e) {
          console.error('Failed to parse message:', e);
        }
      };
    } catch (e) {
      console.error('WebSocket connection error:', e);
      this.scheduleReconnect();
    }
  }

  private scheduleReconnect() {
    if (this.reconnectTimer) return;
    this.reconnectTimer = window.setTimeout(() => {
      this.reconnectTimer = null;
      if (this.url && this.token) {
        this.createConnection();
      }
    }, 3000);
  }

  disconnect() {
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
    this.isConnected = false;
  }

  onConnect(callback: () => void) {
    this.onConnectCallback = callback;
  }

  onDisconnect(callback: () => void) {
    this.onDisconnectCallback = callback;
  }

  get connected() {
    return this.isConnected;
  }

  async request<T = unknown>(method: string, params?: Record<string, unknown>): Promise<T> {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      throw new Error('WebSocket not connected');
    }

    const id = `msg_${++this.messageId}`;
    
    return new Promise((resolve, reject) => {
      const message: GatewayMessage & { id: string } = { method, params, id };
      this.ws?.send(JSON.stringify(message));
      
      this.handlers.set(id, (response) => {
        if (response.error) {
          reject(new Error(response.error));
        } else {
          resolve(response.ok as T);
        }
      });
      
      setTimeout(() => {
        if (this.handlers.has(id)) {
          this.handlers.delete(id);
          reject(new Error('Request timeout'));
        }
      }, 30000);
    });
  }

  setToken(token: string) {
    this.token = token;
  }
}

export const gateway = new GatewayClient();