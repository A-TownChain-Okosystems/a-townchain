import React, { createContext, useContext, useState, ReactNode } from 'react';
import { generateSeedPhrase } from '../utils/crypto';

interface WalletContextType {
  address: string | null;
  balance: number;
  isConnecting: boolean;
  seedPhrase: string[];
  connectWallet: () => void;
  setBalance: React.Dispatch<React.SetStateAction<number>>;
}

const WalletContext = createContext<WalletContextType | undefined>(undefined);

export function WalletProvider({ children }: { children: ReactNode }) {
  const [address, setAddress] = useState<string | null>(null);
  const [balance, setBalance] = useState(0);
  const [isConnecting, setIsConnecting] = useState(false);
  const [seedPhrase, setSeedPhrase] = useState<string[]>([]);

  const connectWallet = () => {
    setIsConnecting(true);
    // Simulate generation or connection delay
    setTimeout(() => {
      setAddress('0xATC' + Math.random().toString(16).substring(2, 10).toUpperCase() + '...A1F02');
      setBalance(1337.42);
      setSeedPhrase(generateSeedPhrase());
      setIsConnecting(false);
    }, 1500);
  };

  return (
    <WalletContext.Provider value={{ address, balance, isConnecting, seedPhrase, connectWallet, setBalance }}>
      {children}
    </WalletContext.Provider>
  );
}

export function useWallet() {
  const context = useContext(WalletContext);
  if (context === undefined) {
    throw new Error('useWallet must be used within a WalletProvider');
  }
  return context;
}
