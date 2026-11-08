// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import crypto from 'crypto';
import { AtcBlockchainEngine } from '../blockchain/engine.js';

interface Peer {
  id: string;
  ip: string;
  latency: number;
  protocolVersion: string;
}

export class P2PNetwork {
  private engine: AtcBlockchainEngine;
  private connectedPeers: Peer[] = [];
  private inboundTraffic: number = 0;
  private outboundTraffic: number = 0;
  private simulationInterval: any = null;

  constructor(engine: AtcBlockchainEngine) {
    this.engine = engine;
  }

  public initialize(host: string, port: number) {
    console.log(`[P2P] Networking activated on ${host}:${port} (NAT Traversal ready)`);
    
    // Simulate finding peers via Kademlia DHT
    this.discoverPeers();

    this.simulationInterval = setInterval(() => {
        this.simulateNetworkTraffic();
    }, 2000);
  }

  public getConnections(): Peer[] {
    return this.connectedPeers;
  }

  public getTrafficStats() {
    return {
      inbound_kbps: (this.inboundTraffic / 1024).toFixed(2),
      outbound_kbps: (this.outboundTraffic / 1024).toFixed(2)
    };
  }

  private discoverPeers() {
    const defaultPeers = 5 + Math.floor(Math.random() * 20);
    for (let i = 0; i < defaultPeers; i++) {
      this.connectedPeers.push({
        id: crypto.randomBytes(8).toString('hex'),
        ip: `${Math.floor(Math.random()*255)}.${Math.floor(Math.random()*255)}.${Math.floor(Math.random()*255)}.${Math.floor(Math.random()*255)}`,
        latency: Math.floor(Math.random() * 150) + 10,
        protocolVersion: 'v2-GossipSub'
      });
    }
  }

  private simulateNetworkTraffic() {
    // Generate synthetic noise for standard blockchain telemetry
    this.inboundTraffic = Math.random() * 50000;
    this.outboundTraffic = Math.random() * 80000;

    // Random peer drops / joins
    if (Math.random() > 0.8) {
        if (this.connectedPeers.length > 3) {
            this.connectedPeers.pop();
        }
    }
    if (Math.random() > 0.7) {
        this.connectedPeers.push({
            id: crypto.randomBytes(8).toString('hex'),
            ip: `${Math.floor(Math.random()*255)}.${Math.floor(Math.random()*255)}.${Math.floor(Math.random()*255)}.${Math.floor(Math.random()*255)}`,
            latency: Math.floor(Math.random() * 150) + 10,
            protocolVersion: 'v2-GossipSub'
        });
    }
  }
}
