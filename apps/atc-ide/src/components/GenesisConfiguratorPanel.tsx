import React, { useState, useMemo } from 'react';
import {
  Server,
  Network,
  Cpu,
  Key,
  Shield,
  Coins,
  Flame,
  FileCode,
  Download,
  Copy,
  Check,
  Plus,
  Trash2,
  RefreshCw,
  Zap,
  Globe,
  CheckCircle2,
  AlertTriangle,
  FileJson,
  Layers,
  Terminal,
  Activity
} from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

interface ValidatorEntry {
  id: string;
  moniker: string;
  blsPubKey: string;
  edConsAddress: string;
  power: number;
  commissionRate: number; // e.g. 0.05 = 5%
}

interface GenesisAccountEntry {
  id: string;
  label: string;
  address: string;
  balance: number; // in ATC
  vestingMonths?: number;
}

interface GenesisConfiguratorPanelProps {
  onSaveWorkspaceFile?: (fileName: string, content: string) => void;
  onShowToast?: (msg: string, type: 'info' | 'warning' | 'error') => void;
}

export function GenesisConfiguratorPanel({
  onSaveWorkspaceFile,
  onShowToast
}: GenesisConfiguratorPanelProps) {
  // Preset environment
  const [networkPreset, setNetworkPreset] = useState<'devnet' | 'testnet' | 'mainnet'>('devnet');

  // Chain parameters
  const [chainId, setChainId] = useState<string>('atc-devnet-local-1');
  const [blockTimeMs, setBlockTimeMs] = useState<number>(500);
  const [maxBlockBytes, setMaxBlockBytes] = useState<number>(2097152); // 2 MB
  const [epochLength, setEpochLength] = useState<number>(5000);
  const [targetGasLimit, setTargetGasLimit] = useState<number>(30000000);
  const [baseFeeBurnRatio, setBaseFeeBurnRatio] = useState<number>(50); // 50% burnt

  // Slashing & Governance
  const [doubleSignSlashPercent, setDoubleSignSlashPercent] = useState<number>(5.0);
  const [downtimeSlashPercent, setDowntimeSlashPercent] = useState<number>(0.01);
  const [minValidatorStake, setMinValidatorStake] = useState<number>(10000);

  // Initial Validators
  const [validators, setValidators] = useState<ValidatorEntry[]>([
    {
      id: 'val-1',
      moniker: 'shiva-node-alpha',
      blsPubKey: '0x8f192b49e218c39e018274a9df340b61a9e8b7c4d5e6f1a2b3c4d5e6f7a8b9c0',
      edConsAddress: 'atcvalcons1z89e9x04928374829348923489234823948239',
      power: 250000,
      commissionRate: 5,
    },
    {
      id: 'val-2',
      moniker: 'shiva-node-bravo',
      blsPubKey: '0x7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b',
      edConsAddress: 'atcvalcons1k72j9m83746281938472918273948271928371',
      power: 250000,
      commissionRate: 5,
    },
    {
      id: 'val-3',
      moniker: 'shiva-node-charlie',
      blsPubKey: '0x3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d',
      edConsAddress: 'atcvalcons1v93l8k27192837462819283746192837461928',
      power: 250000,
      commissionRate: 5,
    },
    {
      id: 'val-4',
      moniker: 'shiva-node-delta',
      blsPubKey: '0x1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b',
      edConsAddress: 'atcvalcons1m48p7q19283746192837461928374619283746',
      power: 250000,
      commissionRate: 5,
    },
  ]);

  // Genesis Accounts
  const [accounts, setAccounts] = useState<GenesisAccountEntry[]>([
    {
      id: 'acc-1',
      label: 'Staking & Validator Rewards Pool',
      address: 'atc1poolstaking00000000000000000000000000000000',
      balance: 400000000, // 400M ATC
    },
    {
      id: 'acc-2',
      label: 'Ecosystem & Community Treasury',
      address: 'atc1communitytreasury0000000000000000000000000',
      balance: 250000000, // 250M ATC
    },
    {
      id: 'acc-3',
      label: 'Core Protocol Engineering & Research',
      address: 'atc1coredevfoundation000000000000000000000000',
      balance: 200000000, // 200M ATC
      vestingMonths: 48,
    },
    {
      id: 'acc-4',
      label: 'Public Faucet & Developer Grants',
      address: 'atc1faucetdevnet0000000000000000000000000000000',
      balance: 150000000, // 150M ATC
    },
  ]);

  const [activeOutputTab, setActiveOutputTab] = useState<'genesis' | 'config' | 'keys' | 'docker'>('genesis');
  const [copiedTab, setCopiedTab] = useState<boolean>(false);

  // Switch presets
  const handleSelectPreset = (preset: 'devnet' | 'testnet' | 'mainnet') => {
    setNetworkPreset(preset);
    if (preset === 'devnet') {
      setChainId('atc-devnet-local-1');
      setBlockTimeMs(500);
      setEpochLength(1000);
      setBaseFeeBurnRatio(50);
    } else if (preset === 'testnet') {
      setChainId('atc-shiva-testnet-1');
      setBlockTimeMs(1000);
      setEpochLength(5000);
      setBaseFeeBurnRatio(75);
    } else {
      setChainId('atc-shiva-mainnet-1');
      setBlockTimeMs(1000);
      setEpochLength(10000);
      setBaseFeeBurnRatio(85);
    }
  };

  // Total allocated supply
  const totalSupply = useMemo(() => {
    return accounts.reduce((sum, a) => sum + a.balance, 0);
  }, [accounts]);

  // Total validator voting power
  const totalVotingPower = useMemo(() => {
    return validators.reduce((sum, v) => sum + v.power, 0);
  }, [validators]);

  // Generate genesis.json content
  const genesisJsonContent = useMemo(() => {
    const genesisObj = {
      genesis_time: '2026-09-02T00:00:00.000000000Z',
      chain_id: chainId,
      initial_height: '1',
      consensus_params: {
        block: {
          max_bytes: maxBlockBytes.toString(),
          max_gas: targetGasLimit.toString(),
          time_iota_ms: blockTimeMs.toString(),
        },
        evidence: {
          max_age_num_blocks: '100000',
          max_age_duration: '172800000000000',
        },
        validator: {
          pub_key_types: ['bls12-381', 'ed25519'],
        },
        slashing: {
          double_sign_slash_percent: `${doubleSignSlashPercent}%`,
          downtime_slash_percent: `${downtimeSlashPercent}%`,
          min_signed_per_window: '0.90',
        },
        gas_and_fees: {
          base_fee_burn_ratio: `${baseFeeBurnRatio}%`,
          min_gas_price: '1000000000', // 1 Gwei
        },
      },
      app_state: {
        tokenomics: {
          symbol: 'ATC',
          decimals: 18,
          total_genesis_supply: totalSupply.toString(),
          accounts: accounts.map((a) => ({
            label: a.label,
            address: a.address,
            coins: [{ denom: 'uatc', amount: (a.balance * 1000000).toString() }],
            vesting_months: a.vestingMonths || 0,
          })),
        },
        validators: validators.map((v) => ({
          moniker: v.moniker,
          bls_pub_key: v.blsPubKey,
          consensus_address: v.edConsAddress,
          power: v.power.toString(),
          commission_rate: `${v.commissionRate}%`,
        })),
        atc_vm: {
          version: '1.2.0',
          capability_pipeline_enabled: true,
          zero_register_r0_immutable: true,
        },
      },
    };

    return JSON.stringify(genesisObj, null, 2);
  }, [
    chainId,
    maxBlockBytes,
    targetGasLimit,
    blockTimeMs,
    doubleSignSlashPercent,
    downtimeSlashPercent,
    baseFeeBurnRatio,
    totalSupply,
    accounts,
    validators,
  ]);

  // Generate config.toml content
  const configTomlContent = useMemo(() => {
    return `# ==============================================================
# A-TownChain Node Configuration (config.toml)
# Auto-generated for: ${chainId}
# ==============================================================

proxy_app = "tcp://127.0.0.1:26658"
moniker = "node-primary"
fast_sync = true
db_backend = "goleveldb"
log_level = "info"

[rpc]
laddr = "tcp://0.0.0.0:26657"
cors_allowed_origins = ["*"]
max_open_connections = 1000

[p2p]
laddr = "tcp://0.0.0.0:26656"
seeds = ""
persistent_peers = ""
max_num_inbound_peers = 40
max_num_outbound_peers = 10

[mempool]
recheck = true
broadcast = true
wal_dir = "data/mempool.wal"
size = 5000
max_tx_bytes = 1048576

[consensus]
wal_file = "data/cs.wal/wal"
timeout_propose = "${Math.round(blockTimeMs * 0.6)}ms"
timeout_prevote = "${Math.round(blockTimeMs * 0.2)}ms"
timeout_precommit = "${Math.round(blockTimeMs * 0.2)}ms"
timeout_commit = "${Math.round(blockTimeMs * 0.2)}ms"
create_empty_blocks = true
create_empty_blocks_interval = "3s"

[instrumentation]
prometheus = true
prometheus_listen_addr = ":26660"
`;
  }, [chainId, blockTimeMs]);

  // Generate validator_keys.json content
  const validatorKeysContent = useMemo(() => {
    const keys = validators.map((v, i) => ({
      moniker: v.moniker,
      validator_index: i + 1,
      bls_public_key: v.blsPubKey,
      consensus_address: v.edConsAddress,
      p2p_id: `12D3KooW${v.blsPubKey.substring(2, 26)}`,
      status: 'ACTIVE_BFT_SIGNER',
    }));
    return JSON.stringify({ chain_id: chainId, validators: keys }, null, 2);
  }, [chainId, validators]);

  // Generate docker-compose.devnet.yml
  const dockerComposeContent = useMemo(() => {
    return `# ==============================================================
# A-TownChain Multi-Node Local Devnet Launcher
# Runs 4 BFT Validator Nodes with instant consensus & RPC
# ==============================================================
version: '3.8'

services:
${validators
  .map(
    (v, i) => `  ${v.moniker}:
    image: atownchain/core:latest
    container_name: ${v.moniker}
    restart: unless-stopped
    command: start --home /atc/data --rpc.laddr tcp://0.0.0.0:2665${7 + i}
    ports:
      - "${26657 + i}:26657"
      - "${26656 + i}:26656"
    volumes:
      - ./config:/atc/config:ro
      - ./genesis.json:/atc/config/genesis.json:ro
    environment:
      - ATC_CHAIN_ID=${chainId}
      - ATC_NODE_MONIKER=${v.moniker}
`
  )
  .join('\n')}

  block-explorer:
    image: atownchain/explorer:latest
    ports:
      - "8080:80"
    environment:
      - RPC_URL=http://shiva-node-alpha:26657
    depends_on:
      - shiva-node-alpha
`;
  }, [validators, chainId]);

  // Copy active tab content
  const handleCopyCurrent = () => {
    let content = '';
    if (activeOutputTab === 'genesis') content = genesisJsonContent;
    else if (activeOutputTab === 'config') content = configTomlContent;
    else if (activeOutputTab === 'keys') content = validatorKeysContent;
    else if (activeOutputTab === 'docker') content = dockerComposeContent;

    navigator.clipboard.writeText(content);
    setCopiedTab(true);
    setTimeout(() => setCopiedTab(false), 2000);
    onShowToast?.('Konfiguration in die Zwischenablage kopiert.', 'info');
  };

  // Save ALL 4 files to workspace
  const handleSaveAllToWorkspace = () => {
    if (!onSaveWorkspaceFile) {
      onShowToast?.('Workspace-Handler nicht bereit.', 'error');
      return;
    }

    onSaveWorkspaceFile('config/genesis.json', genesisJsonContent);
    onSaveWorkspaceFile('config/config.toml', configTomlContent);
    onSaveWorkspaceFile('config/validator_keys.json', validatorKeysContent);
    onSaveWorkspaceFile('docker-compose.devnet.yml', dockerComposeContent);

    onShowToast?.('Genesis-Paket (genesis.json, config.toml, keys & docker-compose) im Workspace gesichert!', 'info');
  };

  return (
    <div className="flex-1 flex flex-col bg-[#0b0f19] text-slate-200 overflow-hidden font-sans">
      {/* Top Header Bar */}
      <div className="flex flex-wrap items-center justify-between px-4 py-3 bg-[#0d1322] border-b border-white/10 gap-3 shrink-0">
        <div className="flex items-center gap-3">
          <div className="w-9 h-9 rounded-lg bg-indigo-500/20 border border-indigo-500/40 flex items-center justify-center text-indigo-400">
            <Server className="w-5 h-5" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-sm font-bold text-white tracking-wide">
                Genesis Block & Devnet Node Configurator
              </h2>
              <span className="px-2 py-0.5 rounded-full text-[10px] font-mono font-bold bg-indigo-500/20 text-indigo-300 border border-indigo-500/40">
                Shiva-1 Tendermint/BFT
              </span>
              <span className="px-2 py-0.5 rounded-full text-[10px] font-mono bg-cyan-500/20 text-cyan-300 border border-cyan-500/40">
                Chain: {chainId}
              </span>
            </div>
            <p className="text-xs text-slate-400">
              Generiert genesis.json, Tendermint config.toml, BLS12-381 Validator-Keys und Multi-Node Docker Devnet
            </p>
          </div>
        </div>

        {/* Preset Switcher & Workspace Export */}
        <div className="flex items-center gap-2">
          <div className="flex bg-white/5 p-1 rounded-lg border border-white/10 text-xs font-semibold">
            {(['devnet', 'testnet', 'mainnet'] as const).map((p) => (
              <button
                key={p}
                onClick={() => handleSelectPreset(p)}
                className={`px-3 py-1 rounded-md uppercase transition-all ${
                  networkPreset === p
                    ? 'bg-indigo-500/25 text-indigo-300 border border-indigo-500/40 shadow-sm'
                    : 'text-slate-400 hover:text-slate-200'
                }`}
              >
                {p}
              </button>
            ))}
          </div>

          {onSaveWorkspaceFile && (
            <button
              onClick={handleSaveAllToWorkspace}
              className="px-3 py-1.5 bg-indigo-600 hover:bg-indigo-500 text-white rounded-lg text-xs font-bold flex items-center gap-1.5 shadow-lg shadow-indigo-600/20 transition-all cursor-pointer"
            >
              <Download className="w-3.5 h-3.5" />
              <span>Genesis-Paket in Workspace sichern</span>
            </button>
          )}
        </div>
      </div>

      {/* Main Split: Left Config Editor | Right Output Preview */}
      <div className="flex-1 grid grid-cols-12 gap-0 overflow-hidden">
        {/* LEFT COLUMN: Configuration Controls (6 cols) */}
        <div className="col-span-12 lg:col-span-6 border-r border-white/10 flex flex-col bg-[#080d1a] overflow-y-auto p-4 space-y-5">
          {/* Section 1: Chain Parameters */}
          <div className="space-y-3">
            <h3 className="text-xs font-bold text-slate-300 uppercase tracking-wider flex items-center gap-1.5">
              <Globe className="w-3.5 h-3.5 text-cyan-400" />
              <span>1. Basis Chain- & Konsensus-Parameter</span>
            </h3>

            <div className="grid grid-cols-2 gap-3 text-xs">
              <div>
                <label className="text-slate-400 block mb-1">Chain-ID</label>
                <input
                  type="text"
                  value={chainId}
                  onChange={(e) => setChainId(e.target.value)}
                  className="w-full bg-black/40 border border-white/10 rounded-lg px-2.5 py-1.5 font-mono text-cyan-300 outline-none focus:border-cyan-500"
                />
              </div>

              <div>
                <label className="text-slate-400 block mb-1">Block-Intervall (ms)</label>
                <input
                  type="number"
                  value={blockTimeMs}
                  onChange={(e) => setBlockTimeMs(parseInt(e.target.value, 10) || 500)}
                  className="w-full bg-black/40 border border-white/10 rounded-lg px-2.5 py-1.5 font-mono text-slate-200 outline-none focus:border-cyan-500"
                />
              </div>

              <div>
                <label className="text-slate-400 block mb-1">Max Block Size (Bytes)</label>
                <input
                  type="number"
                  value={maxBlockBytes}
                  onChange={(e) => setMaxBlockBytes(parseInt(e.target.value, 10) || 2097152)}
                  className="w-full bg-black/40 border border-white/10 rounded-lg px-2.5 py-1.5 font-mono text-slate-200 outline-none focus:border-cyan-500"
                />
              </div>

              <div>
                <label className="text-slate-400 block mb-1">Target Gas Limit</label>
                <input
                  type="number"
                  value={targetGasLimit}
                  onChange={(e) => setTargetGasLimit(parseInt(e.target.value, 10) || 30000000)}
                  className="w-full bg-black/40 border border-white/10 rounded-lg px-2.5 py-1.5 font-mono text-slate-200 outline-none focus:border-cyan-500"
                />
              </div>
            </div>
          </div>

          {/* Section 2: Tokenomics & Genesis Allocation */}
          <div className="space-y-3 pt-2 border-t border-white/10">
            <div className="flex items-center justify-between">
              <h3 className="text-xs font-bold text-slate-300 uppercase tracking-wider flex items-center gap-1.5">
                <Coins className="w-3.5 h-3.5 text-amber-400" />
                <span>2. Tokenomics & Initial Supply</span>
              </h3>
              <span className="font-mono text-xs text-emerald-400 font-bold">
                Total: {totalSupply.toLocaleString()} ATC
              </span>
            </div>

            <div className="space-y-2">
              {accounts.map((acc, idx) => (
                <div
                  key={acc.id}
                  className="p-2.5 rounded-lg bg-white/5 border border-white/5 flex flex-col gap-1.5 text-xs"
                >
                  <div className="flex items-center justify-between">
                    <span className="font-bold text-slate-200">{acc.label}</span>
                    <span className="font-mono text-cyan-300 font-bold">
                      {acc.balance.toLocaleString()} ATC ({Math.round((acc.balance / totalSupply) * 100)}%)
                    </span>
                  </div>
                  <div className="flex items-center justify-between text-[11px] font-mono text-slate-400 truncate">
                    <span>{acc.address}</span>
                    {acc.vestingMonths ? (
                      <span className="text-purple-300 bg-purple-500/20 px-1.5 py-0.2 rounded">
                        {acc.vestingMonths} Mo Vesting
                      </span>
                    ) : (
                      <span className="text-emerald-400">Liquid</span>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Section 3: Initial BFT Validators */}
          <div className="space-y-3 pt-2 border-t border-white/10">
            <div className="flex items-center justify-between">
              <h3 className="text-xs font-bold text-slate-300 uppercase tracking-wider flex items-center gap-1.5">
                <Shield className="w-3.5 h-3.5 text-indigo-400" />
                <span>3. Initial BFT Validator Set ({validators.length})</span>
              </h3>
              <span className="font-mono text-xs text-indigo-400 font-bold">
                Power: {totalVotingPower.toLocaleString()}
              </span>
            </div>

            <div className="space-y-2">
              {validators.map((val) => (
                <div
                  key={val.id}
                  className="p-2.5 rounded-lg bg-white/5 border border-white/5 flex items-center justify-between text-xs"
                >
                  <div>
                    <div className="font-bold text-slate-200 flex items-center gap-2">
                      <span>{val.moniker}</span>
                      <span className="text-[10px] text-indigo-300 bg-indigo-500/20 px-1.5 py-0.2 rounded font-mono">
                        Comm: {val.commissionRate}%
                      </span>
                    </div>
                    <div className="text-[10px] text-slate-500 font-mono mt-0.5 truncate max-w-xs">
                      BLS: {val.blsPubKey.substring(0, 24)}...
                    </div>
                  </div>

                  <div className="text-right font-mono">
                    <span className="font-bold text-cyan-400">{val.power.toLocaleString()} Power</span>
                    <div className="text-[10px] text-slate-500">
                      {Math.round((val.power / totalVotingPower) * 100)}% Anteil
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* RIGHT COLUMN: Real-Time Output Preview (6 cols) */}
        <div className="col-span-12 lg:col-span-6 flex flex-col bg-[#060a14] overflow-hidden">
          {/* Subtabs for generated artifacts */}
          <div className="flex items-center justify-between px-3 py-2 bg-[#0a0f1d] border-b border-white/10 shrink-0 text-xs">
            <div className="flex items-center gap-1">
              {[
                { id: 'genesis', label: 'genesis.json', icon: FileJson },
                { id: 'config', label: 'config.toml', icon: FileCode },
                { id: 'keys', label: 'validator_keys.json', icon: Key },
                { id: 'docker', label: 'docker-compose.yml', icon: Terminal },
              ].map((tab) => {
                const Icon = tab.icon;
                return (
                  <button
                    key={tab.id}
                    onClick={() => setActiveOutputTab(tab.id as any)}
                    className={`px-2.5 py-1 rounded-md font-semibold flex items-center gap-1.5 transition-colors ${
                      activeOutputTab === tab.id
                        ? 'bg-indigo-500/25 text-indigo-300 border border-indigo-500/40 shadow-sm'
                        : 'text-slate-400 hover:text-slate-200'
                    }`}
                  >
                    <Icon className="w-3.5 h-3.5" />
                    <span>{tab.label}</span>
                  </button>
                );
              })}
            </div>

            <button
              onClick={handleCopyCurrent}
              className="px-2.5 py-1 bg-white/5 hover:bg-white/10 text-slate-300 rounded-md text-xs font-semibold flex items-center gap-1.5 transition-colors"
            >
              {copiedTab ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
              <span>Kopieren</span>
            </button>
          </div>

          {/* Code Viewer */}
          <div className="flex-1 overflow-y-auto p-3 font-mono text-xs text-slate-300 leading-relaxed select-text bg-[#040710]">
            <pre className="whitespace-pre-wrap break-all">
              {activeOutputTab === 'genesis' && genesisJsonContent}
              {activeOutputTab === 'config' && configTomlContent}
              {activeOutputTab === 'keys' && validatorKeysContent}
              {activeOutputTab === 'docker' && dockerComposeContent}
            </pre>
          </div>
        </div>
      </div>
    </div>
  );
}
