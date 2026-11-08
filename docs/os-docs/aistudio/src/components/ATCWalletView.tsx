import React, { useState, useEffect } from 'react';
import { Wallet, Send, RefreshCw, Activity, ArrowRight, ShieldCheck, Coins, Key, Copy, Check, Lock, Eye, EyeOff, X, FileText } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { generateSeedPhrase, encryptSeedPhrase } from '../utils/crypto';
import { useWallet } from '../contexts/WalletContext';
import { jsPDF } from 'jspdf';

export function ATCWalletView() {
  const { address, balance, isConnecting, seedPhrase, connectWallet, setBalance } = useWallet();
  const [recipient, setRecipient] = useState('');
  const [amount, setAmount] = useState('');
  const [transactions, setTransactions] = useState<any[]>([
    { id: '1', hash: '0x9f...4a2b', type: 'receive', amount: 50, from: '0x12..88', date: '2 Mins ago', status: 'confirmed' },
    { id: '2', hash: '0x32...11ff', type: 'send', amount: 12.5, to: '0x77..21', date: '5 Hours ago', status: 'confirmed' },
    { id: '3', hash: '0x88...cc44', type: 'receive', amount: 1300, from: '0x00..01', date: '1 Day ago', status: 'confirmed' },
  ]);

  const [activeTab, setActiveTab] = useState<'wallet' | 'keys'>('wallet');
  const [seedStep, setSeedStep] = useState<'show_seed' | 'saved'>('show_seed');
  const [showPhrase, setShowPhrase] = useState(false);
  const [hasCopied, setHasCopied] = useState(false);
  const [showConfirmDialog, setShowConfirmDialog] = useState(false);
  const [password, setPassword] = useState('');
  
  const [showVerification, setShowVerification] = useState(false);
  const [verificationMethod, setVerificationMethod] = useState<'select' | 'password' | 'email' | 'otp'>('select');
  const [verificationCode, setVerificationCode] = useState('');
  const [verificationError, setVerificationError] = useState('');

  const exportWallet = () => {
    const encryptedData = localStorage.getItem('atc_encrypted_wallet');
    if (!encryptedData) return;
    const exportObject = {
      version: '1.0',
      timestamp: new Date().toISOString(),
      wallet: JSON.parse(encryptedData)
    };
    const blob = new Blob([JSON.stringify(exportObject, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `atc_wallet_backup_${Date.now()}.json`;
    a.click();
    localStorage.setItem('atc_last_backup_download', Date.now().toString());
  };

  const copyToClipboard = () => {
    navigator.clipboard.writeText(seedPhrase.join(' '));
    setHasCopied(true);
    setTimeout(() => setHasCopied(false), 2000);
  };

  const downloadBackupPDF = () => {
    const doc = new jsPDF();
    
    // Add title
    doc.setFont("helvetica", "bold");
    doc.setFontSize(22);
    doc.text("ATC-OS Wallet Recovery Backup", 20, 30);
    
    // Add warning
    doc.setFont("helvetica", "normal");
    doc.setFontSize(12);
    doc.setTextColor(220, 53, 69);
    doc.text("CRITICAL: Keep this document safe and offline.", 20, 45);
    doc.text("Anyone with this seed phrase can access your funds.", 20, 52);
    
    // Add Address
    doc.setTextColor(0, 0, 0);
    doc.setFontSize(14);
    doc.setFont("helvetica", "bold");
    doc.text("Wallet Address:", 20, 70);
    doc.setFont("helvetica", "normal");
    doc.setFontSize(11);
    doc.text(address || "Not connected", 20, 78);
    
    // Add Seed Phrase
    doc.setFontSize(14);
    doc.setFont("helvetica", "bold");
    doc.text("Recovery Seed Phrase (24 Words):", 20, 95);
    
    doc.setFont("helvetica", "normal");
    doc.setFontSize(12);
    
    // Format seed phrase into a grid
    const startY = 105;
    const colWidth = 45;
    const rowHeight = 10;
    
    seedPhrase.forEach((word, index) => {
      const col = index % 4;
      const row = Math.floor(index / 4);
      const x = 20 + (col * colWidth);
      const y = startY + (row * rowHeight);
      
      doc.text(`${index + 1}. ${word}`, x, y);
    });
    
    // Add footer instructions
    doc.setFontSize(10);
    doc.setFont("helvetica", "italic");
    doc.text("Generated on: " + new Date().toLocaleString(), 20, 270);
    doc.text("Store this in a secure physical location.", 20, 277);
    
    // Save PDF
    doc.save(`atcos-paper-backup-${Date.now()}.pdf`);
    localStorage.setItem('atc_last_backup_download', Date.now().toString());
  };

  const handleSend = () => {
    if (!recipient || !amount || parseFloat(amount) > balance) return;
    const newTx = {
      id: Date.now().toString(),
      hash: `0x${Math.random().toString(16).substr(2, 8)}...${Math.random().toString(16).substr(2, 4)}`,
      type: 'send',
      amount: parseFloat(amount),
      to: recipient.substr(0, 8) + '...',
      date: 'Just now',
      status: 'pending'
    };
    setTransactions([newTx, ...transactions]);
    setBalance(prev => prev - parseFloat(amount));
    setRecipient('');
    setAmount('');
    setTimeout(() => {
      setTransactions(prev => prev.map(t => t.id === newTx.id ? { ...t, status: 'confirmed' } : t));
    }, 3000);
  };

  return (
    <div className="flex flex-col h-full bg-[#05080f] text-slate-300">
      <div className="flex items-center justify-between border-b border-atc-border p-4">
         <div className="flex items-center gap-3">
            <div className="p-2 bg-atc-cyan/10 rounded-lg">
               <Wallet className="w-6 h-6 text-atc-cyan" />
            </div>
            <div>
               <h2 className="font-bold text-white tracking-tight">ATC-OS Identity & Wallet</h2>
               <p className="text-xs text-slate-500">Unified Standard Access</p>
            </div>
         </div>
         <div className="flex bg-black/40 border border-atc-border/50 rounded-lg p-1">
            <button 
              onClick={() => setActiveTab('wallet')}
              className={`px-4 py-1.5 rounded-md text-sm font-medium transition-colors ${activeTab === 'wallet' ? 'bg-white/10 text-white shadow-sm' : 'text-slate-400 hover:text-white'}`}
            >Wallet</button>
            <button 
              onClick={() => setActiveTab('keys')}
              className={`px-4 py-1.5 rounded-md text-sm font-medium transition-colors ${activeTab === 'keys' ? 'bg-white/10 text-white shadow-sm' : 'text-slate-400 hover:text-white'}`}
            >Recovery / Backup</button>
         </div>
      </div>

      <div className="flex-1 overflow-y-auto p-6 custom-scrollbar">
        {activeTab === 'wallet' && (
          !address ? (
            <div className="flex flex-col items-center justify-center h-full max-w-sm mx-auto text-center gap-6 mt-12">
              <div className="w-24 h-24 rounded-full bg-atc-cyan/10 flex items-center justify-center border border-atc-cyan/20 animate-pulse">
                 <ShieldCheck className="w-12 h-12 text-atc-cyan" />
              </div>
              <div>
                <h3 className="text-xl font-bold text-white mb-2">Wallet Not Connected</h3>
                <p className="text-sm text-slate-400">Connect your local ATC Wallet or generate a new private identity to access funds.</p>
              </div>
              <button 
                onClick={connectWallet}
                disabled={isConnecting}
                className="w-full flex items-center justify-center gap-2 py-3 rounded-xl bg-atc-cyan/20 text-atc-cyan font-bold transition-all hover:bg-atc-cyan/30 hover:scale-[1.02] border border-atc-cyan/30"
              >
                {isConnecting ? <RefreshCw className="w-5 h-5 animate-spin" /> : <Wallet className="w-5 h-5" />}
                {isConnecting ? 'GENERATING...' : 'CONNECT WALLET'}
              </button>
            </div>
          ) : (
          <div className="max-w-4xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-6">
            
            {/* Balance Card */}
            <motion.div initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} className="flex flex-col gap-6">
              <div className="bg-[#090b14] border border-atc-border/50 rounded-2xl p-6 relative overflow-hidden">
                 <div className="absolute top-0 right-0 p-6 opacity-5 z-0">
                    <Coins className="w-32 h-32 text-atc-cyan" />
                 </div>
                 <div className="relative z-10 flex flex-col gap-4">
                    <p className="text-sm font-medium text-slate-400 uppercase tracking-wider">A-TOWN OS Unified Identity</p>
                    <div className="flex items-baseline gap-2">
                       <span className="text-5xl font-mono font-bold text-white tracking-tighter">{balance.toFixed(2)}</span>
                       <span className="text-xl font-bold text-atc-cyan">ATC</span>
                    </div>
                    <div className="mt-4 flex items-center justify-between p-3 bg-white/5 border border-white/10 rounded-xl">
                       <div>
                         <p className="text-xs text-slate-500 mb-0.5">Identity Address</p>
                         <p className="text-sm text-atc-cyan font-mono">{address}</p>
                       </div>
                    </div>
                 </div>
              </div>

              {/* Send Transaction */}
              <div className="bg-[#090b14] border border-atc-border/50 rounded-2xl p-6 flex flex-col gap-4 relative">
                 <div className="absolute top-4 right-4 flex items-center gap-2 text-[10px] font-mono bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 px-2 py-1 rounded">
                   <ShieldCheck className="w-3 h-3" /> PRE-EXECUTION FIREWALL: ON
                 </div>
                 <h3 className="font-bold text-white mt-2">Transfer Assets</h3>
                 <div>
                    <label className="text-xs font-medium text-slate-500 mb-1 block">Recipient / Identity</label>
                    <input 
                      type="text" 
                      value={recipient}
                      onChange={(e) => setRecipient(e.target.value)}
                      placeholder="0x... or @identity"
                      className="w-full bg-black/40 border border-atc-border/50 rounded-lg px-4 py-2 text-sm text-white focus:outline-none focus:border-atc-cyan transition-colors"
                    />
                 </div>
                 <div>
                    <label className="text-xs font-medium text-slate-500 mb-1 block">Amount (ATC)</label>
                    <div className="relative">
                      <input 
                        type="number" 
                        value={amount}
                        onChange={(e) => setAmount(e.target.value)}
                        placeholder="0.00"
                        className="w-full bg-black/40 border border-atc-border/50 rounded-lg pl-4 pr-16 py-2 text-sm text-white focus:outline-none focus:border-atc-cyan transition-colors"
                      />
                      <button 
                        onClick={() => setAmount(balance.toString())}
                        className="absolute right-2 top-1/2 -translate-y-1/2 text-xs font-bold text-atc-cyan bg-atc-cyan/10 hover:bg-atc-cyan/20 px-2 py-1 rounded transition-colors"
                      >MAX</button>
                    </div>
                 </div>
                 <button 
                   onClick={handleSend}
                   disabled={!recipient || !amount || parseFloat(amount) > balance}
                   className="mt-2 w-full flex items-center justify-center gap-2 py-3 rounded-xl bg-atc-cyan text-slate-900 font-bold transition-all hover:bg-atc-cyan/90 disabled:opacity-50 disabled:cursor-not-allowed shadow-[0_0_15px_rgba(45,212,191,0.2)]"
                 >
                   <Send className="w-4 h-4" /> SEND TRANSACTION
                 </button>
              </div>
            </motion.div>

            {/* Activities */}
            <motion.div initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.1 }} className="flex flex-col gap-4">
               <h3 className="font-bold text-white px-2">Recent Activities</h3>
               <div className="bg-[#090b14] border border-atc-border/50 rounded-2xl flex flex-col overflow-hidden">
                 {transactions.map((tx, i) => (
                   <div key={tx.id} className={`flex items-center justify-between p-4 ${i !== transactions.length - 1 ? 'border-b border-atc-border/50' : ''}`}>
                      <div className="flex items-center gap-3">
                         <div className={`w-10 h-10 rounded-full flex items-center justify-center ${tx.type === 'send' ? 'bg-purple-500/10 text-purple-400' : 'bg-emerald-500/10 text-emerald-400'}`}>
                           {tx.type === 'send' ? <ArrowRight className="w-5 h-5 -rotate-45" /> : <ArrowRight className="w-5 h-5 rotate-[135deg]" />}
                         </div>
                         <div>
                            <p className="text-sm font-bold text-white">{tx.type === 'send' ? 'Send' : 'Receive'}</p>
                            <p className="text-xs text-slate-500 font-mono">{tx.type === 'send' ? tx.to : tx.from}</p>
                         </div>
                      </div>
                      <div className="text-right">
                         <p className={`text-sm font-bold ${tx.type === 'send' ? 'text-white' : 'text-emerald-400'}`}>
                           {tx.type === 'send' ? '-' : '+'}{tx.amount} ATC
                         </p>
                         <p className="text-xs text-slate-500 flex items-center gap-1 justify-end mt-0.5">
                           {tx.status === 'pending' ? <RefreshCw className="w-3 h-3 animate-spin text-amber-500" /> : <Check className="w-3 h-3 text-atc-cyan" />}
                           {tx.date}
                         </p>
                      </div>
                   </div>
                 ))}
               </div>
            </motion.div>
          </div>
          )
        )}

        {activeTab === 'keys' && (
          <div className="max-w-2xl mx-auto flex flex-col gap-6">
            {seedStep === 'show_seed' && (
              <motion.div 
                initial={{ opacity: 0 }} 
                animate={{ opacity: 1 }}
                className="flex flex-col gap-6 bg-[#090b14] border border-atc-border/50 rounded-2xl p-8 relative"
              >
                 <div className="flex justify-between items-start mb-2">
                    <div>
                      <h3 className="text-lg font-bold text-atc-cyan">Recovery Seed Phrase (Unified Backup)</h3>
                      <p className="text-sm text-slate-400 mt-1">This seed phrase recovers your ATC-OS identity and wallet assets.</p>
                    </div>
                    <div className="flex items-center gap-2">
                      <button 
                        onClick={() => {
                          if (showPhrase) {
                            setShowPhrase(false);
                          } else {
                            setShowVerification(true);
                            setVerificationMethod('select');
                            setVerificationCode('');
                            setVerificationError('');
                          }
                        }} 
                        className="flex items-center gap-2 px-3 py-1.5 rounded bg-white/5 hover:bg-white/10 text-white text-xs border border-white/10 transition-colors"
                      >
                         {showPhrase ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
                         {showPhrase ? 'HIDE' : 'REVEAL / VERIFY'}
                      </button>
                      <button onClick={copyToClipboard} className="flex items-center gap-2 px-3 py-1.5 rounded bg-white/5 hover:bg-white/10 text-white text-xs border border-white/10 transition-colors">
                         {hasCopied ? <Check className="w-4 h-4 text-emerald-400" /> : <Copy className="w-4 h-4" />}
                         {hasCopied ? 'COPIED' : 'COPY'}
                      </button>
                    </div>
                 </div>
                 <div className="bg-black/50 border border-white/10 rounded-xl p-6 relative overflow-hidden min-h-[160px]">
                   <AnimatePresence>
                     {!showPhrase && (
                        <motion.div 
                          initial={{ opacity: 0 }}
                          animate={{ opacity: 1 }}
                          exit={{ opacity: 0 }}
                          className="absolute inset-0 bg-black/90 backdrop-blur-md z-10 flex flex-col gap-2 items-center justify-center rounded-xl"
                        >
                           <Lock className="w-8 h-8 text-slate-500" />
                           <span className="text-slate-300 font-medium text-sm">Click Reveal and verify identity to view phrase</span>
                        </motion.div>
                     )}
                   </AnimatePresence>
                   <div className="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-6 gap-x-4 gap-y-4">
                     {seedPhrase.map((word, i) => (
                       <motion.div 
                         key={i} 
                         initial={{ opacity: 0, y: 10 }}
                         animate={{ opacity: 1, y: 0 }}
                         transition={{ delay: i * 0.05, duration: 0.3 }}
                         className="flex gap-2 text-sm justify-center items-center bg-white/5 py-2 px-1 rounded"
                       >
                         <span className="text-slate-500 font-mono select-none text-[10px]">{i + 1}</span>
                         <span className="text-emerald-400 font-mono font-medium tracking-wide">{word}</span>
                       </motion.div>
                     ))}
                   </div>
                 </div>
                 
                 <div className="flex flex-col sm:flex-row gap-4 mt-2 relative z-0">
                   <button 
                     onClick={() => setShowConfirmDialog(true)}
                     className="flex-1 px-6 py-4 rounded-xl bg-slate-800 text-white font-bold transition-all hover:bg-slate-700 border border-slate-600 flex items-center justify-center gap-2"
                   >
                     CREATE ENCRYPTED BACKUP
                   </button>
                   <button 
                     onClick={downloadBackupPDF}
                     className="flex-1 px-6 py-4 rounded-xl bg-white/5 text-white font-bold transition-all hover:bg-white/10 border border-white/10 flex items-center justify-center gap-2"
                   >
                     <FileText className="w-5 h-5" />
                     DOWNLOAD PAPER PDF
                   </button>
                 </div>
              </motion.div>
            )}
            
            {seedStep === 'saved' && (
              <div className="flex flex-col items-center justify-center p-12 bg-[#090b14] border border-atc-cyan/30 rounded-2xl text-center gap-4">
                 <div className="w-16 h-16 rounded-full bg-atc-cyan/10 flex items-center justify-center mb-2">
                    <Check className="w-8 h-8 text-atc-cyan" />
                 </div>
                 <h3 className="text-xl font-bold text-white">Backup Secured</h3>
                 <p className="text-sm text-slate-400 max-w-sm">Your encrypted wallet backup has been saved. Store your password safely.</p>
                 <button 
                   onClick={exportWallet}
                   className="mt-6 px-6 py-2 rounded-lg border border-white/10 text-white hover:bg-white/5 transition-colors font-medium text-sm"
                 >
                   DOWNLOAD BACKUP .JSON
                 </button>
              </div>
            )}
          </div>
        )}
      </div>

      <AnimatePresence>
        {showVerification && (
          <motion.div 
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="absolute inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
          >
            <motion.div 
              initial={{ scale: 0.95 }}
              animate={{ scale: 1 }}
              exit={{ scale: 0.95 }}
              className="bg-slate-900 border border-white/10 rounded-2xl p-6 w-full max-w-md shadow-2xl relative"
            >
              <button 
                onClick={() => setShowVerification(false)}
                className="absolute top-4 right-4 text-slate-500 hover:text-white transition-colors"
              >
                <X className="w-5 h-5" />
              </button>
              
              <Lock className="w-8 h-8 text-atc-cyan mb-4" />
              <h3 className="text-xl font-bold text-white mb-2">Verify Identity</h3>
              <p className="text-sm text-slate-400 mb-6">Please verify your OS login identity to reveal the seed phrase.</p>

              {verificationMethod === 'select' && (
                <div className="flex flex-col gap-3">
                  <button onClick={() => setVerificationMethod('password')} className="p-4 rounded-xl border border-white/10 hover:border-atc-cyan/50 hover:bg-atc-cyan/5 text-left transition-all">
                    <div className="font-bold text-white mb-1">OS Login Password</div>
                    <div className="text-xs text-slate-500">Verify using your OS encryption password</div>
                  </button>
                  <button onClick={() => setVerificationMethod('email')} className="p-4 rounded-xl border border-white/10 hover:border-atc-cyan/50 hover:bg-atc-cyan/5 text-left transition-all">
                    <div className="font-bold text-white mb-1">Email Code</div>
                    <div className="text-xs text-slate-500">Send a 6-digit code to your identity email</div>
                  </button>
                  <button onClick={() => setVerificationMethod('otp')} className="p-4 rounded-xl border border-white/10 hover:border-atc-cyan/50 hover:bg-atc-cyan/5 text-left transition-all">
                    <div className="font-bold text-white mb-1">Smartphone App (OTP)</div>
                    <div className="text-xs text-slate-500">Use your linked authenticator app</div>
                  </button>
                </div>
              )}

              {verificationMethod !== 'select' && (
                <div className="flex flex-col gap-4">
                  <div className="flex items-center gap-2 mb-2">
                    <button onClick={() => setVerificationMethod('select')} className="text-slate-500 hover:text-white text-xs font-bold uppercase tracking-widest">&larr; Back</button>
                  </div>
                  <div>
                    <label className="block text-xs font-bold text-slate-400 uppercase tracking-widest mb-2">
                      {verificationMethod === 'password' && 'Enter Password'}
                      {verificationMethod === 'email' && 'Enter 6-digit Email Code'}
                      {verificationMethod === 'otp' && 'Enter Authenticator Code'}
                    </label>
                    <input 
                      type={verificationMethod === 'password' ? 'password' : 'text'}
                      value={verificationCode}
                      onChange={e => { setVerificationCode(e.target.value); setVerificationError(''); }}
                      className="w-full bg-black/40 border border-atc-border/50 rounded-lg px-4 py-3 text-sm text-white focus:outline-none focus:border-atc-cyan transition-colors font-mono"
                      placeholder={verificationMethod === 'password' ? "••••••••" : "123456"}
                      autoFocus
                    />
                    {verificationError && <p className="text-xs text-red-500 mt-2">{verificationError}</p>}
                  </div>
                  <button 
                    onClick={() => {
                      if (!verificationCode) {
                        setVerificationError('Field is required.');
                        return;
                      }
                      if (verificationMethod !== 'password' && verificationCode.length < 6) {
                        setVerificationError('Invalid code.');
                        return;
                      }
                      // Simulate successful verification using the provided credentials
                      setShowVerification(false);
                      setShowPhrase(true);
                    }}
                    className="w-full py-3 bg-atc-cyan text-slate-900 font-bold rounded-xl mt-2 hover:bg-atc-cyan/90 transition-colors"
                  >
                    Verify & Reveal
                  </button>
                </div>
              )}
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>
      
      <AnimatePresence>
        {showConfirmDialog && (
          <motion.div initial={{opacity:0}} animate={{opacity:1}} exit={{opacity:0}} className="absolute inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm">
             <motion.div initial={{scale:0.95}} animate={{scale:1}} className="bg-slate-900 border border-white/10 rounded-2xl p-6 w-full max-w-md shadow-2xl relative">
                <button onClick={() => setShowConfirmDialog(false)} className="absolute top-4 right-4 text-slate-500"><X className="w-5 h-5"/></button>
                <Lock className="w-8 h-8 text-atc-cyan mb-4" />
                <h3 className="text-xl font-bold text-white mb-2">Encrypt Backup File</h3>
                <p className="text-sm text-slate-400 mb-6">Enter a password to encrypt your unified wallet identity backup file.</p>
                <div className="space-y-4">
                  <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} placeholder="Encryption Password" 
                         className="w-full bg-black/40 border border-atc-border/50 rounded-lg px-4 py-3 text-white" />
                 <button onClick={async () => {
                    if(!password) return;
                    try {
                      const encryptedBlob = await encryptSeedPhrase(seedPhrase, password);
                      localStorage.setItem('atc_encrypted_wallet', JSON.stringify({ 
                        encrypted: true, 
                        date: Date.now(), 
                        data: encryptedBlob 
                      }));
                      setShowConfirmDialog(false);
                      setSeedStep('saved');
                    } catch (e) {
                      console.error("Encryption failed", e);
                    }
                  }} className="w-full py-3 bg-atc-cyan text-slate-900 font-bold rounded-xl mt-2">
                    Encrypt & Save
                  </button>
                </div>
             </motion.div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
