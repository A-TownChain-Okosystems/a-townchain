import React from 'react';
import { CreditCard, DollarSign, Wallet2, History, ArrowRightLeft, ShieldCheck } from 'lucide-react';

export function PaymentSystemView() {
  const transactions = [
    { id: "TX-9921", user: "Acme Corp", amount: "$12,450.00", status: "Cleared", type: "Enterprise License" },
    { id: "TX-9922", user: "0x44f...2a1", amount: "4,500 ATC", status: "Processing", type: "Node Staking" },
    { id: "TX-9923", user: "John Doe", amount: "$49.99", status: "Cleared", type: "Pro Subscription" },
  ];

  return (
    <div className="flex flex-col gap-10 mt-8 pb-12 font-sans">
      <div className="mb-4 flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-emerald-500/10 border border-emerald-500/30 flex items-center justify-center text-emerald-400 shadow-[0_0_15px_rgba(16,185,129,0.2)]">
          <CreditCard className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
            Payment & Billing System
          </h2>
          <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
            Fiat Clearing, Krypto-Tokenomics, Subscription Management und Rechnungsstellungspipelines.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div className="p-5 bg-gradient-to-br from-[#090b14] to-[#041a10] border border-emerald-500/20 rounded-2xl">
          <div className="flex items-center gap-2 text-emerald-400 mb-2">
            <DollarSign className="w-4 h-4" />
            <span className="text-xs uppercase font-bold tracking-widest">MRR (Fiat)</span>
          </div>
          <div className="text-3xl font-mono font-bold text-white">$142k</div>
        </div>
        <div className="p-5 bg-gradient-to-br from-[#090b14] to-[#10041a] border border-purple-500/20 rounded-2xl">
          <div className="flex items-center gap-2 text-purple-400 mb-2">
            <Wallet2 className="w-4 h-4" />
            <span className="text-xs uppercase font-bold tracking-widest">TVL (Crypto)</span>
          </div>
          <div className="text-3xl font-mono font-bold text-white">2.4M ATC</div>
        </div>
        <div className="p-5 bg-[#090b14] border border-white/5 rounded-2xl">
          <div className="flex items-center gap-2 text-slate-400 mb-2">
            <History className="w-4 h-4" />
            <span className="text-xs uppercase font-bold tracking-widest">Active Subs</span>
          </div>
          <div className="text-3xl font-mono font-bold text-white">3,892</div>
        </div>
        <div className="p-5 bg-[#090b14] border border-white/5 rounded-2xl">
          <div className="flex items-center gap-2 text-blue-400 mb-2">
            <ShieldCheck className="w-4 h-4" />
            <span className="text-xs uppercase font-bold tracking-widest">Fraud Rate</span>
          </div>
          <div className="text-3xl font-mono font-bold text-white">0.01%</div>
        </div>
      </div>

      <div className="p-6 rounded-2xl bg-[#090b14] border border-white/5">
        <h3 className="text-lg font-semibold text-white mb-6 flex items-center gap-2">
          <ArrowRightLeft className="w-5 h-5 text-emerald-400" /> Recent Clearing Transactions
        </h3>
        <div className="overflow-x-auto">
          <table className="w-full text-left text-sm text-slate-300">
            <thead className="bg-black/40 text-xs uppercase font-medium text-slate-500">
              <tr>
                <th className="p-4 rounded-tl-lg">Tx ID</th>
                <th className="p-4">Customer / Address</th>
                <th className="p-4">Amount</th>
                <th className="p-4">Type</th>
                <th className="p-4 rounded-tr-lg">Status</th>
              </tr>
            </thead>
            <tbody>
              {transactions.map((tx, i) => (
                <tr key={i} className="border-b border-white/5 hover:bg-white/5">
                  <td className="p-4 font-mono text-xs text-slate-400">{tx.id}</td>
                  <td className="p-4 font-medium">{tx.user}</td>
                  <td className="p-4 font-mono font-bold text-emerald-400">{tx.amount}</td>
                  <td className="p-4 text-slate-400">{tx.type}</td>
                  <td className="p-4">
                    <span className={`px-2 py-1 text-[10px] uppercase font-bold rounded ${tx.status === 'Cleared' ? 'bg-emerald-500/10 text-emerald-400' : 'bg-amber-500/10 text-amber-400'}`}>
                      {tx.status}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
