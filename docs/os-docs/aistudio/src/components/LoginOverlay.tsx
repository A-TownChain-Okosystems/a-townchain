import React, { useState } from 'react';
import { Shield, Key, AlertTriangle, CheckCircle, ArrowRight, Lock, Bot, Eye, EyeOff, X, FileText, QrCode, Smartphone } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { jsPDF } from 'jspdf';
import { QRCodeSVG } from 'qrcode.react';

const SEED_WORDS = [
  "abstract", "balance", "camera", "danger", "economy", "fabric", "galaxy", "habit", 
  "ice", "jazz", "kingdom", "logic", "machine", "network", "ocean", "panel", 
  "quantum", "radar", "safety", "talent", "universe", "vessel", "wealth", "zone",
  "abandon", "ability", "able", "about", "above", "absent", "absorb", "abuse", "access", "accident", 
  "account", "accuse", "achieve", "acid", "acoustic", "acquire", "across", "act", "action", "actor"
];

interface LoginOverlayProps {
  onLogin: (duration: string) => void;
}

export function LoginOverlay({ onLogin }: LoginOverlayProps) {
  const [step, setStep] = useState<'login' | 'register' | 'verify-email' | 'generate' | 'seed' | 'forgot-password' | 'recovery-seed' | 'recovery-options' | 'reset-password' | 'offline-qr'>('login');
  const [seedPhrase, setSeedPhrase] = useState<string[]>([]);
  const [hasConfirmed, setHasConfirmed] = useState(false);
  const [loginDuration, setLoginDuration] = useState(() => localStorage.getItem('atc_session_duration') || 'heute');
  
  // Registration data
  const [firstName, setFirstName] = useState('');
  const [lastName, setLastName] = useState('');
  const [displayName, setDisplayName] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [passwordConfirm, setPasswordConfirm] = useState('');

  // Seed verification
  const [showPhrase, setShowPhrase] = useState(false);
  const [showVerification, setShowVerification] = useState(false);
  const [verificationMethod, setVerificationMethod] = useState<'select' | 'password' | 'email' | 'otp'>('select');
  const [verificationCode, setVerificationCode] = useState('');
  const [verificationError, setVerificationError] = useState('');

  const handleRegisterSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (password !== passwordConfirm) {
      alert("Passwörter stimmen nicht überein!");
      return;
    }
    setStep('verify-email');
  };

  const handleEmailVerification = (e: React.FormEvent) => {
    e.preventDefault();
    setStep('generate');
    setTimeout(() => {
      // Use Web Crypto API for secure random generation
      const array = new Uint32Array(24);
      window.crypto.getRandomValues(array);
      const phrase = Array.from(array).map(n => SEED_WORDS[n % SEED_WORDS.length]);
      setSeedPhrase(phrase);
      setShowPhrase(true); // Automatically reveal since email is verified
      setStep('seed');
    }, 2000);
  };

  const handleRevealClick = () => {
    if (showPhrase) {
      setShowPhrase(false);
    } else {
      setShowVerification(true);
      setVerificationMethod('select');
      setVerificationCode('');
      setVerificationError('');
    }
  };

  const downloadBackupPDF = () => {
    const doc = new jsPDF();
    
    // Add title
    doc.setFont("helvetica", "bold");
    doc.setFontSize(22);
    doc.text("ATC-OS Wallet Recovery Backup", 20, 30);
    
    // Add User Details
    doc.setFont("helvetica", "normal");
    doc.setFontSize(12);
    doc.text(`User: ${firstName} ${lastName} (${displayName})`, 20, 45);
    doc.text(`Email: ${email}`, 20, 52);
    
    // Add warning
    doc.setTextColor(220, 53, 69); // Red warning text
    doc.text("CRITICAL: Keep this document safe and offline.", 20, 65);
    doc.text("Anyone with this seed phrase can access your funds.", 20, 72);
    
    // Add Seed Phrase
    doc.setTextColor(0, 0, 0);
    doc.setFontSize(14);
    doc.setFont("helvetica", "bold");
    doc.text("Recovery Seed Phrase (24 Words):", 20, 90);
    
    doc.setFont("helvetica", "normal");
    doc.setFontSize(12);
    
    // Format seed phrase
    const startY = 100;
    const colWidth = 45;
    const rowHeight = 10;
    
    seedPhrase.forEach((word, index) => {
      const col = index % 4;
      const row = Math.floor(index / 4);
      const x = 20 + (col * colWidth);
      const y = startY + (row * rowHeight);
      
      doc.text(`${index + 1}. ${word}`, x, y);
    });
    
    doc.setFontSize(10);
    doc.setFont("helvetica", "italic");
    doc.text("Generated on: " + new Date().toLocaleString(), 20, 270);
    doc.text("Store this in a secure physical location.", 20, 277);
    
    doc.save(`atcos-seed-backup-${Date.now()}.pdf`);
  };

  const verifyAndReveal = () => {
    if (!verificationCode) {
      setVerificationError('Bitte Wert eingeben.');
      return;
    }
    if (verificationMethod === 'password' && verificationCode !== password) {
      setVerificationError('Falsches Kennwort.');
      return;
    }
    if (verificationMethod !== 'password' && verificationCode.length < 6) {
      setVerificationError('Ungültiger Code.');
      return;
    }
    // Success
    setShowVerification(false);
    setShowPhrase(true);
  };

  if (step === 'login') {
    return (
      <div className="flex items-center justify-center min-h-screen bg-slate-900 relative selection:bg-atc-cyan/20">
        <div className="absolute inset-0 bg-cover bg-center opacity-25" style={{ backgroundImage: `url('https://media.base44.com/images/public/6a2756186106d6f0fbb105b5/371a23000_file_0000000046a0720a8c30add446651917.png')` }} />
        <div className="absolute inset-0 bg-gradient-to-b from-slate-900/70 via-slate-900/85 to-slate-900/95" />
        <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.03]" />
        <div className="absolute top-0 right-0 w-[500px] h-[500px] bg-atc-cyan/10 rounded-full blur-[120px] -translate-y-1/2 translate-x-1/3" />
        <div className="absolute bottom-0 left-0 w-[500px] h-[500px] bg-atc-purple/10 rounded-full blur-[120px] translate-y-1/3 -translate-x-1/4" />

        <div className="relative z-10 w-full max-w-sm p-8 bg-slate-900 border border-white/10 rounded-2xl shadow-2xl backdrop-blur-sm">
          <div className="flex flex-col items-center mb-8">
            <Bot className="w-12 h-12 text-atc-cyan mb-3" />
            <h1 className="text-2xl font-bold text-white tracking-tight">ATC-OS Identity</h1>
            <p className="text-slate-400 text-sm mt-1">Unified OS & Wallet Access</p>
          </div>
          
          <form className="space-y-4" onSubmit={(e) => { e.preventDefault(); onLogin(loginDuration); }}>
            <div>
              <label className="block text-xs font-medium text-slate-400 mb-1.5 uppercase tracking-widest">Email / Wallet ID</label>
              <input type="text" required className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-atc-cyan transition-colors" placeholder="user@a-town" />
            </div>
            <div>
              <div className="flex justify-between items-center mb-1.5">
                <label className="block text-xs font-medium text-slate-400 uppercase tracking-widest">Passwort / Master Key</label>
                <button type="button" onClick={() => setStep('forgot-password')} className="text-xs text-atc-cyan hover:text-atc-cyan/80 transition-colors">Passwort vergessen?</button>
              </div>
              <input type="password" required className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-atc-cyan transition-colors" placeholder="••••••••" />
            </div>
            <div>
              <label className="block text-xs font-medium text-slate-400 mb-1.5 uppercase tracking-widest">Eingeloggt bleiben für</label>
              <select 
                value={loginDuration} 
                onChange={(e) => setLoginDuration(e.target.value)}
                className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-atc-cyan transition-colors cursor-pointer"
              >
                <option value="heute">Heute</option>
                <option value="3_tage">3 Tage</option>
                <option value="7_tage">7 Tage</option>
                <option value="14_tage">14 Tage</option>
                <option value="1_monat">1 Monat</option>
                <option value="restlicher_monat">Restlicher Monat</option>
              </select>
            </div>
            <div className="pt-4 flex flex-col gap-3">
              <button type="submit" className="w-full bg-atc-cyan hover:bg-atc-cyan/90 text-slate-900 font-bold py-2.5 rounded-lg transition-colors flex items-center justify-center gap-2">
                <Lock className="w-4 h-4" /> Entsperren & Anmelden
              </button>
              <button type="button" onClick={() => setStep('register')} className="w-full bg-white/5 hover:bg-white/10 text-slate-300 font-medium py-2.5 rounded-lg transition-colors border border-white/5">
                Neue Identität / Wallet erstellen
              </button>
            </div>
          </form>
        </div>
      </div>
    );
  }

  if (step === 'register') {
    return (
      <div className="flex items-center justify-center min-h-screen bg-slate-900 relative">
        <div className="absolute inset-0 bg-cover bg-center opacity-25" style={{ backgroundImage: `url('https://media.base44.com/images/public/6a2756186106d6f0fbb105b5/371a23000_file_0000000046a0720a8c30add446651917.png')` }} />
        <div className="absolute inset-0 bg-gradient-to-b from-slate-900/70 via-slate-900/85 to-slate-900/95" />
        <div className="absolute inset-0 bg-cover bg-center opacity-25" style={{ backgroundImage: `url('https://media.base44.com/images/public/6a2756186106d6f0fbb105b5/371a23000_file_0000000046a0720a8c30add446651917.png')` }} />
        <div className="absolute inset-0 bg-gradient-to-b from-slate-900/70 via-slate-900/85 to-slate-900/95" />
        <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.03]" />
        
        <div className="relative z-10 w-full max-w-sm p-8 bg-slate-900 border border-white/10 rounded-2xl shadow-2xl backdrop-blur-sm">
          <div className="flex flex-col items-center mb-6">
            <Key className="w-10 h-10 text-atc-cyan mb-3" />
            <h2 className="text-xl font-bold text-white tracking-tight">Identität Generieren</h2>
            <p className="text-slate-400 text-sm mt-1 text-center">Erstellen Sie Ihren OS Account und Ihre Krypto-Wallet in einem Schritt.</p>
          </div>
          
          <form className="space-y-4" onSubmit={handleRegisterSubmit}>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-xs font-medium text-slate-400 mb-1.5 uppercase tracking-widest">Vorname</label>
                <input 
                  type="text" 
                  value={firstName}
                  onChange={e => setFirstName(e.target.value)}
                  required 
                  className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-atc-cyan transition-colors" 
                  placeholder="Max" 
                />
              </div>
              <div>
                <label className="block text-xs font-medium text-slate-400 mb-1.5 uppercase tracking-widest">Nachname</label>
                <input 
                  type="text" 
                  value={lastName}
                  onChange={e => setLastName(e.target.value)}
                  required 
                  className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-atc-cyan transition-colors" 
                  placeholder="Mustermann" 
                />
              </div>
            </div>
            <div>
              <label className="block text-xs font-medium text-slate-400 mb-1.5 uppercase tracking-widest">Anzeigename</label>
              <input 
                type="text" 
                value={displayName}
                onChange={e => setDisplayName(e.target.value)}
                required 
                className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-atc-cyan transition-colors" 
                placeholder="@maxmuster" 
              />
            </div>
            <div>
              <label className="block text-xs font-medium text-slate-400 mb-1.5 uppercase tracking-widest">Email (Pflichtfeld zur Verifizierung)</label>
              <input 
                type="email" 
                value={email}
                onChange={e => setEmail(e.target.value)}
                required 
                className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-atc-cyan transition-colors" 
                placeholder="email@example.com" 
              />
            </div>
            <div>
              <label className="block text-xs font-medium text-slate-400 mb-1.5 uppercase tracking-widest">Lokales Passwort festlegen</label>
              <input 
                type="password" 
                value={password}
                onChange={e => setPassword(e.target.value)}
                required 
                className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-atc-cyan transition-colors" 
                placeholder="Min. 8 Zeichen für Wallet-Verschlüsselung" 
              />
            </div>
            <div>
              <label className="block text-xs font-medium text-slate-400 mb-1.5 uppercase tracking-widest">Passwort wiederholen</label>
              <input 
                type="password" 
                value={passwordConfirm}
                onChange={e => setPasswordConfirm(e.target.value)}
                required 
                className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-atc-cyan transition-colors" 
                placeholder="Passwort bestätigen" 
              />
            </div>
            
            <div className="pt-4 flex flex-col gap-3">
              <div className="flex gap-2">
                <button type="submit" className="flex-1 bg-atc-cyan hover:bg-atc-cyan/90 text-slate-900 font-bold py-2.5 rounded-lg transition-colors flex items-center justify-center gap-2 text-sm">
                  <Shield className="w-4 h-4" /> Account & Wallet erstellen
                </button>
                <button type="button" onClick={() => setStep('offline-qr')} className="bg-slate-800 hover:bg-slate-700 text-white font-medium py-2.5 px-4 rounded-lg transition-colors border border-slate-600 flex items-center justify-center" title="Offline Registrierung via Smartphone">
                  <QrCode className="w-5 h-5" />
                </button>
              </div>
              <button type="button" onClick={() => setStep('login')} className="w-full text-slate-500 hover:text-slate-300 text-sm py-2 transition-colors">
                Zurück zum Login
              </button>
            </div>
          </form>
        </div>
      </div>
    );
  }

  if (step === 'offline-qr') {
    const qrPayload = JSON.stringify({
      app: "ATC-OS",
      action: "offline-register",
      nonce: Math.random().toString(36).substring(7)
    });

    return (
      <div className="flex items-center justify-center min-h-screen bg-slate-900 relative">
        <div className="absolute inset-0 bg-cover bg-center opacity-25" style={{ backgroundImage: `url('https://media.base44.com/images/public/6a2756186106d6f0fbb105b5/371a23000_file_0000000046a0720a8c30add446651917.png')` }} />
        <div className="absolute inset-0 bg-gradient-to-b from-slate-900/70 via-slate-900/85 to-slate-900/95" />
        <div className="absolute inset-0 bg-cover bg-center opacity-25" style={{ backgroundImage: `url('https://media.base44.com/images/public/6a2756186106d6f0fbb105b5/371a23000_file_0000000046a0720a8c30add446651917.png')` }} />
        <div className="absolute inset-0 bg-gradient-to-b from-slate-900/70 via-slate-900/85 to-slate-900/95" />
        <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.03]" />
        
        <div className="relative z-10 w-full max-w-sm p-8 bg-slate-900 border border-white/10 rounded-2xl shadow-2xl backdrop-blur-sm">
          <div className="flex flex-col items-center mb-4">
            <Smartphone className="w-10 h-10 text-atc-cyan mb-3" />
            <h2 className="text-xl font-bold text-white tracking-tight">Offline Wallet Generierung</h2>
            <p className="text-slate-400 text-sm mt-1 text-center">QR-Code mit Ihrem Smartphone scannen, Wallet erstellen und Email-Token hier eingeben.</p>
          </div>
          
          <div className="flex justify-center bg-white p-4 rounded-xl mb-6 shadow-inner mx-auto max-w-[200px]">
             <QRCodeSVG value={qrPayload} size={160} />
          </div>

          <form className="space-y-4" onSubmit={handleEmailVerification}>
             <div>
              <label className="block text-xs font-medium text-slate-400 mb-1.5 uppercase tracking-widest text-center">Smartphone Email-Token</label>
              <input 
                type="text" 
                required 
                className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-3 text-white text-center tracking-[0.2em] focus:outline-none focus:border-atc-cyan transition-colors font-mono uppercase" 
                placeholder="TOKEN-1234" 
              />
            </div>
            <button type="submit" className="w-full bg-atc-cyan hover:bg-atc-cyan/90 text-slate-900 font-bold py-2.5 rounded-lg transition-colors mt-2">
              Validieren & Fortsetzen
            </button>
            <div className="text-center mt-2">
               <button type="button" onClick={() => setStep('register')} className="text-sm text-slate-500 hover:text-slate-300 transition-colors">
                 Zurück zur regulären Registrierung
               </button>
            </div>
          </form>
        </div>
      </div>
    );
  }

  if (step === 'verify-email') {
    return (
      <div className="flex items-center justify-center min-h-screen bg-slate-900 relative">
        <div className="absolute inset-0 bg-cover bg-center opacity-25" style={{ backgroundImage: `url('https://media.base44.com/images/public/6a2756186106d6f0fbb105b5/371a23000_file_0000000046a0720a8c30add446651917.png')` }} />
        <div className="absolute inset-0 bg-gradient-to-b from-slate-900/70 via-slate-900/85 to-slate-900/95" />
        <div className="absolute inset-0 bg-cover bg-center opacity-25" style={{ backgroundImage: `url('https://media.base44.com/images/public/6a2756186106d6f0fbb105b5/371a23000_file_0000000046a0720a8c30add446651917.png')` }} />
        <div className="absolute inset-0 bg-gradient-to-b from-slate-900/70 via-slate-900/85 to-slate-900/95" />
        <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.03]" />
        
        <div className="relative z-10 w-full max-w-sm p-8 bg-slate-900 border border-white/10 rounded-2xl shadow-2xl backdrop-blur-sm">
          <div className="flex flex-col items-center mb-6">
            <Lock className="w-10 h-10 text-atc-cyan mb-3" />
            <h2 className="text-xl font-bold text-white tracking-tight">Email Verifizierung</h2>
            <p className="text-slate-400 text-sm mt-1 text-center">Ein 6-stelliger Code wurde an <span className="text-white font-medium">{email || 'Ihre Email-Adresse'}</span> gesendet.</p>
          </div>
          
          <form className="space-y-4" onSubmit={handleEmailVerification}>
            <div>
              <label className="block text-xs font-medium text-slate-400 mb-1.5 uppercase tracking-widest text-center">Verifizierungscode Eingeben</label>
              <input 
                type="text" 
                required 
                maxLength={6}
                className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-3 text-white text-center tracking-[0.5em] focus:outline-none focus:border-atc-cyan transition-colors text-lg font-mono" 
                placeholder="000000" 
              />
            </div>
            <button type="submit" className="w-full bg-atc-cyan hover:bg-atc-cyan/90 text-slate-900 font-bold py-2.5 rounded-lg transition-colors mt-2 shadow-[0_0_15px_rgba(45,212,191,0.2)] hover:shadow-[0_0_20px_rgba(45,212,191,0.4)]">
              Verifizieren & Seed generieren
            </button>
            <div className="text-center mt-4">
              <button type="button" className="text-xs text-atc-cyan hover:text-atc-cyan/80 transition-colors">
                Code erneut senden
              </button>
            </div>
          </form>
        </div>
      </div>
    );
  }

  if (step === 'generate') {
    return (
      <div className="flex items-center justify-center min-h-screen bg-slate-900 relative">
        <div className="absolute inset-0 bg-cover bg-center opacity-25" style={{ backgroundImage: `url('https://media.base44.com/images/public/6a2756186106d6f0fbb105b5/371a23000_file_0000000046a0720a8c30add446651917.png')` }} />
        <div className="absolute inset-0 bg-gradient-to-b from-slate-900/70 via-slate-900/85 to-slate-900/95" />
        <div className="flex flex-col items-center gap-6 text-center max-w-sm p-8 bg-slate-900 border border-atc-cyan/20 rounded-2xl relative overflow-hidden">
          <div className="absolute inset-0 bg-atc-cyan/5 animate-pulse" />
          <Key className="w-16 h-16 text-atc-cyan animate-bounce relative z-10" />
          <div className="relative z-10">
            <h2 className="text-xl font-bold text-white tracking-tight mb-2">Kryptografische Wallet wird generiert...</h2>
            <p className="text-slate-400 text-sm">Erstellung des Master-Seeds und Verschlüsselung mit Ihrem Login-Passwort. Bitte warten.</p>
          </div>
        </div>
      </div>
    );
  }

  if (step === 'seed') {
    return (
      <div className="flex items-center justify-center min-h-screen bg-slate-900 p-6 relative">
        <div className="absolute inset-0 bg-cover bg-center opacity-25" style={{ backgroundImage: `url('https://media.base44.com/images/public/6a2756186106d6f0fbb105b5/371a23000_file_0000000046a0720a8c30add446651917.png')` }} />
        <div className="absolute inset-0 bg-gradient-to-b from-slate-900/70 via-slate-900/85 to-slate-900/95" />
        <div className="max-w-2xl w-full bg-[#060a16] border border-atc-cyan/30 rounded-2xl p-8 relative overflow-hidden shadow-2xl">
          <div className="absolute top-0 right-0 p-6 opacity-5">
            <Shield className="w-48 h-48" />
          </div>

          <div className="relative z-10">
            <div className="flex items-center justify-between mb-6">
              <div className="flex items-center gap-3">
                <AlertTriangle className="w-8 h-8 text-amber-400" />
                <div>
                  <h2 className="text-xl font-bold text-white tracking-tight">Recovery Seed Phrase</h2>
                  <p className="text-emerald-400/80 text-sm font-medium mt-1">Email erfolgreich verifiziert ({email})</p>
                </div>
              </div>
              <button 
                onClick={handleRevealClick}
                className="flex items-center gap-2 px-4 py-2 rounded-lg bg-white/5 hover:bg-white/10 text-white text-sm border border-white/10 transition-colors font-medium"
              >
                 {showPhrase ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
                 {showPhrase ? 'VERBERGEN' : 'ANZEIGEN / ENTSPERREN'}
              </button>
            </div>

            <p className="text-slate-300 text-sm mb-6 leading-relaxed">
              Ihre Identität und Wallet wurden erfolgreich erstellt. Der folgende 24-Wörter-Seed ist der einzige Weg, 
              um Ihre Wallet bei Verlust Ihres Passworts wiederherzustellen. Ohne diese Wörter sind Ihre Assets unwiederbringlich verloren.
            </p>

            <div className="bg-black/80 border border-white/10 rounded-xl p-6 mb-8 relative min-h-[160px] overflow-hidden">
              <AnimatePresence>
                {!showPhrase && (
                  <motion.div 
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    exit={{ opacity: 0 }}
                    className="absolute inset-0 bg-slate-900/90 backdrop-blur-md z-10 flex flex-col items-center justify-center rounded-xl border border-white/5"
                  >
                    <Lock className="w-8 h-8 text-slate-500 mb-3" />
                    <span className="text-slate-300 font-medium">Klicken Sie auf 'Anzeigen / Entsperren', um den Seed sichtbar zu machen</span>
                  </motion.div>
                )}
              </AnimatePresence>

              <div className="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-6 gap-x-4 gap-y-4">
                {seedPhrase.map((word, i) => (
                  <div key={i} className="flex gap-2 text-sm bg-white/5 p-2 rounded justify-center items-center border border-white/5">
                    <span className="text-slate-500 font-mono select-none text-[10px]">{i + 1}</span>
                    <span className="text-emerald-400 font-mono font-bold tracking-wide">{word}</span>
                  </div>
                ))}
              </div>
            </div>

            <div className="bg-white/5 border border-white/10 p-4 rounded-xl flex items-start gap-4 mb-8">
              <input
                type="checkbox"
                id="confirmSeed"
                checked={hasConfirmed}
                onChange={(e) => setHasConfirmed(e.target.checked)}
                className="mt-1 w-5 h-5 rounded border-slate-600 bg-slate-800 text-atc-cyan focus:ring-atc-cyan focus:ring-offset-slate-900 cursor-pointer"
              />
              <label htmlFor="confirmSeed" className="text-sm text-slate-300 cursor-pointer user-select-none">
                Ich bestätige hiermit, dass ich die oben genannte 24-Wörter Seed-Phrase sicher und analog (offline) notiert und verwahrt habe. Ich bin mir bewusst, dass das System diese nicht wiederherstellen kann.
              </label>
            </div>

            <div className="flex justify-end gap-4 mt-8 pt-6 border-t border-white/10">
              <button
                onClick={downloadBackupPDF}
                className="px-6 py-3 rounded-xl border border-white/10 text-white hover:bg-white/5 transition-colors font-bold text-sm flex items-center gap-2"
              >
                <FileText className="w-5 h-5" />
                Papier-Backup PDF Herunterladen
              </button>
              <button
                onClick={() => onLogin(loginDuration)}
                disabled={!hasConfirmed}
                className={`px-8 py-3 rounded-xl font-bold text-sm transition-all flex items-center gap-2
                  ${hasConfirmed 
                    ? 'bg-atc-cyan text-slate-900 hover:bg-atc-cyan/90 shadow-[0_0_20px_rgba(45,212,191,0.3)]' 
                    : 'bg-white/10 text-slate-500 cursor-not-allowed opacity-50'}`}
              >
                <CheckCircle className="w-5 h-5" />
                Wallet Sichern & ATC-OS Starten
              </button>
            </div>
          </div>
        </div>

        {/* Verification Modal */}
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
                
                <Shield className="w-8 h-8 text-atc-cyan mb-4" />
                <h3 className="text-xl font-bold text-white mb-2">Identität Bestätigen</h3>
                <p className="text-sm text-slate-400 mb-6">Bitte verifizieren Sie sich mit Ihren gewählten Login-Daten oder via Smartphone/Email, um die Seed-Phrase offenzulegen.</p>

                {verificationMethod === 'select' && (
                  <div className="flex flex-col gap-3">
                    <button onClick={() => setVerificationMethod('password')} className="p-4 rounded-xl border border-white/10 hover:border-atc-cyan/50 hover:bg-atc-cyan/5 text-left transition-all">
                      <div className="font-bold text-white mb-1">Lokales Passwort</div>
                      <div className="text-xs text-slate-500">Ihre bei der Registrierung gewählten Login-Daten</div>
                    </button>
                    <button onClick={() => setVerificationMethod('email')} className="p-4 rounded-xl border border-white/10 hover:border-atc-cyan/50 hover:bg-atc-cyan/5 text-left transition-all">
                      <div className="font-bold text-white mb-1">Email Bestätigung</div>
                      <div className="text-xs text-slate-500">6-stelliger Code an: {email || 'Ihre Email'}</div>
                    </button>
                    <button onClick={() => setVerificationMethod('otp')} className="p-4 rounded-xl border border-white/10 hover:border-atc-cyan/50 hover:bg-atc-cyan/5 text-left transition-all">
                      <div className="font-bold text-white mb-1">2FA / Smartphone App</div>
                      <div className="text-xs text-slate-500">Authenticator Code verwenden</div>
                    </button>
                  </div>
                )}

                {verificationMethod !== 'select' && (
                  <div className="flex flex-col gap-4">
                    <div className="flex items-center gap-2 mb-2">
                      <button onClick={() => setVerificationMethod('select')} className="text-slate-500 hover:text-white text-xs font-bold uppercase tracking-widest">&larr; Zurück</button>
                    </div>
                    <div>
                      <label className="block text-xs font-bold text-slate-400 uppercase tracking-widest mb-2">
                        {verificationMethod === 'password' && 'Login-Passwort eingeben'}
                        {verificationMethod === 'email' && 'Email Code eingeben'}
                        {verificationMethod === 'otp' && 'Authenticator Code eingeben'}
                      </label>
                      <input 
                        type={verificationMethod === 'password' ? 'password' : 'text'}
                        value={verificationCode}
                        onChange={e => { setVerificationCode(e.target.value); setVerificationError(''); }}
                        className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-atc-cyan transition-colors font-mono"
                        placeholder={verificationMethod === 'password' ? "••••••••" : "123456"}
                        autoFocus
                      />
                      {verificationError && <p className="text-xs text-red-500 mt-2 font-medium">{verificationError}</p>}
                    </div>
                    <button 
                      onClick={verifyAndReveal}
                      className="w-full py-3 bg-atc-cyan text-slate-900 font-bold rounded-xl mt-2 hover:bg-atc-cyan/90 transition-colors shadow-lg shadow-atc-cyan/20"
                    >
                      Bestätigen & Offenlegen
                    </button>
                  </div>
                )}
              </motion.div>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    );
  }

  if (step === 'forgot-password' || step === 'recovery-options') {
    return (
      <div className="flex items-center justify-center min-h-screen bg-slate-900 relative">
        <div className="absolute inset-0 bg-cover bg-center opacity-25" style={{ backgroundImage: `url('https://media.base44.com/images/public/6a2756186106d6f0fbb105b5/371a23000_file_0000000046a0720a8c30add446651917.png')` }} />
        <div className="absolute inset-0 bg-gradient-to-b from-slate-900/70 via-slate-900/85 to-slate-900/95" />
        <div className="absolute inset-0 bg-cover bg-center opacity-25" style={{ backgroundImage: `url('https://media.base44.com/images/public/6a2756186106d6f0fbb105b5/371a23000_file_0000000046a0720a8c30add446651917.png')` }} />
        <div className="absolute inset-0 bg-gradient-to-b from-slate-900/70 via-slate-900/85 to-slate-900/95" />
        <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.03]" />
        
        <div className="relative z-10 w-full max-w-sm p-8 bg-slate-900 border border-white/10 rounded-2xl shadow-2xl backdrop-blur-sm">
          <div className="flex flex-col items-center mb-6">
            <Lock className="w-10 h-10 text-amber-400 mb-3" />
            <h2 className="text-xl font-bold text-white tracking-tight">Account Wiederherstellung</h2>
            <p className="text-slate-400 text-sm mt-1 text-center">Wählen Sie eine Methode, um Ihr Passwort zurückzusetzen.</p>
          </div>
          
          <div className="space-y-3 mb-6">
            <button onClick={() => setStep('recovery-seed')} className="w-full p-4 rounded-xl border border-white/10 hover:border-atc-cyan/50 hover:bg-atc-cyan/5 text-left transition-all">
              <div className="font-bold text-white mb-1">Recovery Seed Phrase</div>
              <div className="text-xs text-slate-500">24-Wörter Seed verwenden</div>
            </button>
            <button onClick={() => setStep('recovery-options')} className="w-full p-4 rounded-xl border border-white/10 hover:border-atc-cyan/50 hover:bg-atc-cyan/5 text-left transition-all relative overflow-hidden group">
              <div className="font-bold text-white mb-1">Email / SMS Bestätigung</div>
              <div className="text-xs text-slate-500">Verifizierungslink senden</div>
              {step === 'recovery-options' && (
                 <div className="absolute inset-0 bg-atc-cyan/10 flex items-center justify-center backdrop-blur-sm">
                    <span className="text-xs font-bold text-atc-cyan">In dieser Demo deaktiviert</span>
                 </div>
              )}
            </button>
          </div>
          
          <button type="button" onClick={() => setStep('login')} className="w-full text-slate-500 hover:text-slate-300 text-sm py-2 transition-colors">
            Zurück zum Login
          </button>
        </div>
      </div>
    );
  }

  if (step === 'recovery-seed') {
    return (
      <div className="flex items-center justify-center min-h-screen bg-slate-900 relative">
        <div className="absolute inset-0 bg-cover bg-center opacity-25" style={{ backgroundImage: `url('https://media.base44.com/images/public/6a2756186106d6f0fbb105b5/371a23000_file_0000000046a0720a8c30add446651917.png')` }} />
        <div className="absolute inset-0 bg-gradient-to-b from-slate-900/70 via-slate-900/85 to-slate-900/95" />
        <div className="relative z-10 w-full max-w-sm p-8 bg-slate-900 border border-white/10 rounded-2xl shadow-2xl backdrop-blur-sm">
           <div className="flex flex-col items-center mb-6">
             <Key className="w-10 h-10 text-emerald-400 mb-3" />
             <h2 className="text-xl font-bold text-white tracking-tight">Seed Eingabe</h2>
             <p className="text-slate-400 text-sm mt-1 text-center">Geben Sie Ihre 24-Wörter Phrase durch Leerzeichen getrennt ein.</p>
           </div>
           
           <form className="space-y-4" onSubmit={(e) => { e.preventDefault(); setStep('reset-password'); }}>
              <div>
                 <textarea 
                   required
                   className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-3 text-white text-sm focus:outline-none focus:border-emerald-400 transition-colors font-mono resize-none h-32"
                   placeholder="abstract balance camera danger economy fabric galaxy habit ice jazz kingdom logic machine network ocean panel quantum radar safety talent universe vessel wealth zone"
                 ></textarea>
              </div>
              <button type="submit" className="w-full bg-emerald-500 hover:bg-emerald-400 text-slate-900 font-bold py-2.5 rounded-lg transition-colors">
                Verifizieren
              </button>
              <button type="button" onClick={() => setStep('forgot-password')} className="w-full text-slate-500 hover:text-slate-300 text-sm py-2 transition-colors">
                Zurück
              </button>
           </form>
        </div>
      </div>
    );
  }

  if (step === 'reset-password') {
    return (
      <div className="flex items-center justify-center min-h-screen bg-slate-900 relative">
        <div className="absolute inset-0 bg-cover bg-center opacity-25" style={{ backgroundImage: `url('https://media.base44.com/images/public/6a2756186106d6f0fbb105b5/371a23000_file_0000000046a0720a8c30add446651917.png')` }} />
        <div className="absolute inset-0 bg-gradient-to-b from-slate-900/70 via-slate-900/85 to-slate-900/95" />
        <div className="relative z-10 w-full max-w-sm p-8 bg-slate-900 border border-white/10 rounded-2xl shadow-2xl backdrop-blur-sm">
           <div className="flex flex-col items-center mb-6">
             <Shield className="w-10 h-10 text-atc-cyan mb-3" />
             <h2 className="text-xl font-bold text-white tracking-tight">Neues Passwort</h2>
             <p className="text-slate-400 text-sm mt-1 text-center">Legen Sie ein neues Passwort für Ihren Account fest.</p>
           </div>
           
           <form className="space-y-4" onSubmit={(e) => { e.preventDefault(); setStep('login'); }}>
              <div>
                <label className="block text-xs font-medium text-slate-400 mb-1.5 uppercase tracking-widest">Neues Passwort</label>
                <input type="password" required className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-atc-cyan transition-colors" placeholder="••••••••" />
              </div>
              <div>
                <label className="block text-xs font-medium text-slate-400 mb-1.5 uppercase tracking-widest">Passwort bestätigen</label>
                <input type="password" required className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-atc-cyan transition-colors" placeholder="••••••••" />
              </div>
              <button type="submit" className="w-full bg-atc-cyan hover:bg-atc-cyan/90 text-slate-900 font-bold py-2.5 rounded-lg transition-colors mt-2">
                Passwort speichern
              </button>
           </form>
        </div>
      </div>
    );
  }

  return null;
}

