import React, { useState, useEffect } from "react";
import { User, Shield, Hash, Key, Clock, Database, CloudOff, Globe, QrCode } from "lucide-react";

export function UserProfileView() {
  const [profile, setProfile] = useState(() => {
    try {
      const stored = localStorage.getItem("atc_user_profile");
      if (stored) return JSON.parse(stored);
    } catch {}
    
    return {
      name: "Anonymous User",
      role: "Standard User",
      bio: "Local profile data.",
      did: "did:atc:5f9b4...2a1c",
      localSync: true,
      publicKey: "0x0482...a5f2",
      createdAt: new Date().toISOString()
    };
  });

  const [isEditing, setIsEditing] = useState(false);
  const [editForm, setEditForm] = useState(profile);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (!isEditing) {
      localStorage.setItem("atc_user_profile", JSON.stringify(profile));
    }
  }, [profile, isEditing]);

  const saveProfile = () => {
    setProfile(editForm);
    setIsEditing(false);
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="h-full bg-[#060a16] text-slate-300 rounded-xl border border-white/10 p-6 overflow-y-auto custom-scrollbar relative">
      <div className="absolute top-0 right-0 w-64 h-64 bg-indigo-500/10 rounded-full blur-[100px] opacity-50 pointer-events-none" />
      
      <div className="max-w-4xl mx-auto space-y-8 relative z-10">
        
        {/* Header Section */}
        <div className="flex flex-col sm:flex-row sm:items-end justify-between border-b border-white/10 pb-6 gap-6">
          <div className="flex items-center gap-6">
            <div className="relative group">
              <div className="w-24 h-24 rounded-2xl bg-black/40 border border-white/10 flex items-center justify-center overflow-hidden">
                <User className="w-10 h-10 text-indigo-400 opacity-60" />
                <div className="absolute inset-0 bg-gradient-to-tr from-indigo-500/20 to-transparent mix-blend-overlay" />
              </div>
              <div className="absolute -bottom-2 -right-2 w-8 h-8 rounded-full bg-[#060a16] border border-white/10 flex items-center justify-center pointer-events-none">
                <CloudOff className="w-4 h-4 text-amber-500 z-10" />
              </div>
            </div>
            
            <div className="flex flex-col">
              <h1 className="text-3xl font-bold text-white tracking-tight">{profile.name}</h1>
              <div className="flex items-center gap-2 mt-1">
                <span className="text-indigo-400 text-sm font-medium uppercase tracking-widest">{profile.role}</span>
                <span className="text-slate-500 font-mono text-xs px-2 py-0.5 rounded border border-white/10 bg-black/20">LOCAL ONLY</span>
              </div>
            </div>
          </div>
          
          <button 
            onClick={() => {
              if (isEditing) saveProfile();
              else setIsEditing(true);
            }}
            className={`px-6 py-2 rounded-lg font-medium text-sm transition-colors shadow-lg ${
              isEditing 
                ? 'bg-emerald-500 hover:bg-emerald-600 text-white shadow-emerald-500/20' 
                : 'bg-indigo-500 hover:bg-indigo-600 text-white shadow-indigo-500/20'
            }`}
          >
            {isEditing ? 'Save Profile' : 'Edit Profile'}
          </button>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          
          {/* Identity Info Panel */}
          <div className="lg:col-span-2 space-y-6">
            <div className="bg-white/5 border border-white/10 rounded-xl p-6">
              <div className="flex items-center gap-3 mb-6">
                <div className="p-2 bg-indigo-500/10 rounded-lg">
                  <Hash className="w-5 h-5 text-indigo-400" />
                </div>
                <h3 className="text-sm font-bold text-white uppercase tracking-widest">
                  Identity Details
                </h3>
              </div>
              
              <div className="space-y-4">
                {isEditing ? (
                  <div className="space-y-4">
                    <div>
                      <label className="text-xs text-slate-500 font-bold uppercase tracking-widest">Display Name</label>
                      <input 
                        type="text" 
                        value={editForm.name} 
                        onChange={e => setEditForm({...editForm, name: e.target.value})}
                        className="w-full mt-1 bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-white outline-none focus:border-indigo-500 transition-colors"
                      />
                    </div>
                    <div>
                      <label className="text-xs text-slate-500 font-bold uppercase tracking-widest">Bio / Description</label>
                      <textarea 
                        value={editForm.bio} 
                        onChange={e => setEditForm({...editForm, bio: e.target.value})}
                        className="w-full mt-1 bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-white outline-none focus:border-indigo-500 transition-colors h-24 resize-none"
                      />
                    </div>
                  </div>
                ) : (
                  <div className="space-y-4">
                    <div>
                      <label className="text-xs text-slate-500 font-bold uppercase tracking-widest">Display Name</label>
                      <div className="mt-1 text-white text-lg">{profile.name}</div>
                    </div>
                    <div>
                      <label className="text-xs text-slate-500 font-bold uppercase tracking-widest">Bio / Description</label>
                      <div className="mt-1 text-slate-300">{profile.bio}</div>
                    </div>
                  </div>
                )}
              </div>
            </div>

            {/* My Assets */}
            <div className="bg-white/5 border border-white/10 rounded-xl p-6">
                <div className="flex items-center gap-3 mb-6">
                  <div className="p-2 bg-indigo-500/10 rounded-lg">
                    <Database className="w-5 h-5 text-indigo-400" />
                  </div>
                  <h3 className="text-sm font-bold text-white uppercase tracking-widest">
                    My Assets & Modules
                  </h3>
                </div>
                
                <div className="space-y-4">
                  {(() => {
                    let assets: any[] = [];
                    try {
                      assets = JSON.parse(localStorage.getItem('atc_user_apps') || '[]');
                    } catch {}
                    
                    if (assets.length === 0) {
                      return <div className="text-center text-sm text-slate-500 py-4">No assets installed yet.</div>;
                    }
                    return assets.map((asset, i) => (
                      <div key={i} className="flex justify-between items-center p-3 rounded-lg border border-white/5 bg-black/40">
                         <div className="flex flex-col">
                            <span className="text-white font-bold">{asset.name}</span>
                            <span className="text-xs text-slate-400">{asset.rarity} {asset.element}</span>
                         </div>
                         <span className="text-[10px] text-indigo-400 font-mono flex items-center gap-1">
                            <Globe className="w-3 h-3" /> ONLINE
                         </span>
                      </div>
                    ));
                  })()}
                </div>
            </div>

            <div className="bg-white/5 border border-white/10 rounded-xl p-6">
               <div className="flex items-center gap-3 mb-6">
                <div className="p-2 bg-emerald-500/10 rounded-lg">
                  <Key className="w-5 h-5 text-emerald-400" />
                </div>
                <h3 className="text-sm font-bold text-white uppercase tracking-widest">
                  Cryptographic Anchors
                </h3>
              </div>

              <div className="space-y-4">
                <div className="bg-black/40 border border-white/5 rounded-lg p-4 flex flex-col justify-between">
                  <div className="flex justify-between items-center mb-2">
                    <span className="text-xs text-slate-500 font-bold uppercase tracking-wider">Decentralized Identifier (DID)</span>
                    <button 
                      onClick={() => copyToClipboard(profile.did)}
                      className="text-[10px] text-indigo-400 hover:text-white uppercase tracking-widest bg-indigo-500/10 px-2 py-1 rounded transition-colors"
                    >
                      {copied ? 'Copied' : 'Copy'}
                    </button>
                  </div>
                  <span className="text-sm text-emerald-400 font-mono tracking-wider break-all">{profile.did}</span>
                </div>
                
                <div className="bg-black/40 border border-white/5 rounded-lg p-4 flex flex-col justify-between">
                  <div className="flex justify-between items-center mb-2">
                    <span className="text-xs text-slate-500 font-bold uppercase tracking-wider">Public Key (secp256k1)</span>
                  </div>
                  <span className="text-sm text-slate-400 font-mono tracking-wider break-all">{profile.publicKey}</span>
                </div>
              </div>
            </div>
          </div>
          
          {/* Status & Sync Panel */}
          <div className="space-y-6">
            <div className="bg-white/5 border border-amber-500/20 rounded-xl p-6 relative overflow-hidden group">
              <div className="absolute top-0 right-0 w-32 h-32 bg-amber-500/5 rounded-full blur-[40px] pointer-events-none" />
              <div className="flex items-center gap-3 mb-4">
                <div className="p-2 bg-amber-500/10 rounded-lg">
                  <Database className="w-5 h-5 text-amber-500" />
                </div>
                <h3 className="text-sm font-bold text-white uppercase tracking-widest">
                  Local Context
                </h3>
              </div>
              
              <div className="space-y-3">
                <div className="flex justify-between items-center">
                  <span className="text-xs text-slate-400">Sync Status</span>
                  <span className="text-xs font-bold text-amber-500 flex items-center gap-1"><CloudOff className="w-3 h-3" /> Offline</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-xs text-slate-400">Profile Data</span>
                  <span className="text-xs font-mono text-slate-300">IndexedDB</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-xs text-slate-400">Keys</span>
                  <span className="text-xs font-mono text-slate-300">Secure Enclave</span>
                </div>
                <div className="flex justify-between items-center mt-4 pt-4 border-t border-white/5">
                  <span className="text-xs text-slate-400">Created</span>
                  <span className="text-xs font-mono text-slate-300">{new Date(profile.createdAt).toLocaleDateString()}</span>
                </div>
              </div>
            </div>

            <div className="bg-white/5 border border-white/10 rounded-xl p-6 flex flex-col items-center justify-center text-center">
               <QrCode className="w-24 h-24 text-slate-600 mb-4 opacity-50" />
               <h4 className="text-sm font-medium text-white mb-2">Device Pairing</h4>
               <p className="text-xs text-slate-400">
                 Scan this code from a trusted device to securely transmit your DID across local networks.
               </p>
               <button className="mt-4 px-4 py-2 border border-white/10 bg-black/40 hover:bg-white/5 rounded-lg text-xs font-bold text-slate-300 transition-colors uppercase tracking-widest w-full">
                 Show Code
               </button>
            </div>
          </div>

        </div>
      </div>
    </div>
  );
}
