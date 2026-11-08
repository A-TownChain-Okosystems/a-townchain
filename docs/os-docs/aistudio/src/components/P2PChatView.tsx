import React, { useState, useEffect, useRef } from 'react';
import { Send, Users, ShieldCheck, Key, Lock, Phone, Video } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

interface ChatMessage {
  id: string;
  senderId: string;
  senderName: string;
  text: string;
  timestamp: Date;
  isMe: boolean;
  status: 'sent' | 'delivered' | 'read';
}

const INITIAL_FRIENDS = [
  { id: '1', name: 'Alice Node', status: 'online', avatar: 'A', typing: false },
  { id: '2', name: 'Bob Chain', status: 'offline', avatar: 'B', typing: false },
  { id: '3', name: 'Charlie Hash', status: 'online', avatar: 'C', typing: false }
];

export function P2PChatView() {
  const [friends, setFriends] = useState(INITIAL_FRIENDS);
  const [activeFriendId, setActiveFriendId] = useState(INITIAL_FRIENDS[0].id);
  const [messages, setMessages] = useState<Record<string, ChatMessage[]>>({
    '1': [
      { id: '1', senderId: '1', senderName: 'Alice Node', text: 'Hey, are you free to look at this pull request?', timestamp: new Date(Date.now() - 3600000), isMe: false, status: 'read' },
      { id: '2', senderId: 'me', senderName: 'Me', text: 'Sure, I will check it out in 5 minutes.', timestamp: new Date(Date.now() - 3500000), isMe: true, status: 'read' },
    ]
  });
  const [inputText, setInputText] = useState('');
  const [wsStatus, setWsStatus] = useState<'connecting' | 'connected' | 'disconnected'>('connecting');
  const chatEndRef = useRef<HTMLDivElement>(null);

  const activeFriend = friends.find(f => f.id === activeFriendId);
  const activeMessages = messages[activeFriendId] || [];

  // Mock WebSocket Connection Setup
  useEffect(() => {
    setWsStatus('connecting');
    const timer = setTimeout(() => {
      setWsStatus('connected');
    }, 1500);
    return () => clearTimeout(timer);
  }, []);

  // Mock Incoming Messages
  useEffect(() => {
    if (wsStatus !== 'connected' || !activeFriend || activeFriend.status !== 'online') return;
    
    // Random incoming chatter
    const interval = setInterval(() => {
      if (Math.random() > 0.8) {
        // Show typing
        setFriends(prev => prev.map(f => f.id === activeFriendId ? { ...f, typing: true } : f));
        
        setTimeout(() => {
          setFriends(prev => prev.map(f => f.id === activeFriendId ? { ...f, typing: false } : f));
          const responses = ["Interesting perspective.", "Can you send the contract hash?", "I'm deploying to the testnet now.", "Let's review it during the sync."];
          const text = responses[Math.floor(Math.random() * responses.length)];
          const msg: ChatMessage = {
            id: Date.now().toString(),
            senderId: activeFriendId,
            senderName: activeFriend.name,
            text,
            timestamp: new Date(),
            isMe: false,
            status: 'delivered'
          };
          setMessages(prev => ({
            ...prev,
            [activeFriendId]: [...(prev[activeFriendId] || []), msg]
          }));
          
          window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: `New message from ${activeFriend.name}` }));
        }, 2000);
      }
    }, 15000);

    return () => clearInterval(interval);
  }, [wsStatus, activeFriendId, activeFriend]);

  useEffect(() => {
    chatEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [activeMessages]);

  const handleSendMessage = () => {
    if (!inputText.trim() || !activeFriend) return;
    
    const msg: ChatMessage = {
      id: Date.now().toString(),
      senderId: 'me',
      senderName: 'Me',
      text: inputText,
      timestamp: new Date(),
      isMe: true,
      status: 'sent'
    };
    
    setMessages(prev => ({
      ...prev,
      [activeFriendId]: [...(prev[activeFriendId] || []), msg]
    }));
    setInputText('');
    
    // Simulate delivery
    setTimeout(() => {
      setMessages(prev => {
        const msgs = [...(prev[activeFriendId] || [])];
        const m = msgs.find(x => x.id === msg.id);
        if (m) m.status = 'delivered';
        return { ...prev, [activeFriendId]: msgs };
      });
    }, 500);
  };

  return (
    <div className="flex h-full bg-[#050811] text-slate-200">
      {/* Sidebar / Friends List */}
      <div className="w-64 border-r border-white/5 bg-black/20 flex flex-col hidden sm:flex">
        <div className="p-4 border-b border-white/5 bg-black/40">
          <div className="flex items-center justify-between mb-4">
            <h3 className="font-bold font-mono text-white tracking-widest text-sm flex items-center gap-2">
              <Key className="w-4 h-4 text-emerald-400" />
              P2P NETWORK
            </h3>
          </div>
          <div className="flex items-center gap-2 text-xs">
            <div className={`w-2 h-2 rounded-full ${wsStatus === 'connected' ? 'bg-emerald-500 animate-pulse' : wsStatus === 'connecting' ? 'bg-amber-500 animate-bounce' : 'bg-red-500'}`} />
            <span className={wsStatus === 'connected' ? 'text-emerald-400' : 'text-slate-400'}>
              {wsStatus === 'connected' ? 'Encrypted Tunnel Active' : wsStatus === 'connecting' ? 'Establishing Handshake...' : 'Disconnected'}
            </span>
          </div>
        </div>
        <div className="flex-1 overflow-y-auto custom-scrollbar p-2 space-y-1">
          {friends.map(friend => (
            <button 
              key={friend.id} 
              onClick={() => setActiveFriendId(friend.id)}
              className={`w-full flex items-center gap-3 p-3 rounded-xl transition-all text-left group ${activeFriendId === friend.id ? 'bg-indigo-500/20 ring-1 ring-indigo-500/30' : 'hover:bg-white/5'}`}
            >
              <div className="relative">
                <div className={`w-10 h-10 rounded-full flex items-center justify-center font-bold text-slate-300 transition-colors ${activeFriendId === friend.id ? 'bg-indigo-500/50' : 'bg-slate-800 group-hover:bg-slate-700'}`}>
                  {friend.avatar}
                </div>
                <div className={`absolute bottom-0 right-0 w-3 h-3 rounded-full border-2 border-[#090b14] ${friend.status === 'online' ? 'bg-emerald-500' : 'bg-slate-500'}`} />
              </div>
              <div className="flex-1 overflow-hidden">
                <div className="text-sm font-medium text-white truncate">{friend.name}</div>
                <div className="text-xs text-slate-500 truncate flex items-center gap-1">
                  {friend.typing ? <span className="text-indigo-400 italic">typing...</span> : (friend.status === 'online' ? 'Online' : 'Offline')}
                </div>
              </div>
            </button>
          ))}
        </div>
      </div>

      {/* Main Chat Area */}
      <div className="flex-1 flex flex-col bg-[#050811] relative">
        {wsStatus !== 'connected' && (
          <div className="absolute inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center">
            <div className="text-center p-6 bg-white/5 border border-white/10 rounded-xl shadow-2xl">
               <ShieldCheck className="w-12 h-12 text-indigo-400 mx-auto mb-4 animate-pulse" />
               <h3 className="text-white font-medium mb-1">Securing Connection</h3>
               <p className="text-sm text-slate-400 font-mono">Negotiating RSA keys for end-to-end encryption...</p>
            </div>
          </div>
        )}

        <div className="h-16 border-b border-white/5 bg-black/40 flex items-center justify-between px-4 sm:px-6">
          {activeFriend ? (
            <div className="flex items-center gap-4">
              <div className="relative sm:hidden">
                <div className="w-10 h-10 rounded-full bg-indigo-500/50 flex items-center justify-center font-bold text-white">
                   {activeFriend.avatar}
                </div>
                <div className={`absolute bottom-0 right-0 w-3 h-3 rounded-full border-2 border-[#090b14] ${activeFriend.status === 'online' ? 'bg-emerald-500' : 'bg-slate-500'}`} />
              </div>
              <div>
                <h2 className="text-white font-medium flex items-center gap-2">
                  {activeFriend.name}
                  <Lock className="w-3 h-3 text-emerald-400" />
                </h2>
                <p className="text-xs text-emerald-400/80 font-mono">P2P Secure Channel Established</p>
              </div>
            </div>
          ) : (
            <div className="text-slate-400">Select a peer to start messaging</div>
          )}
          
          <div className="flex items-center gap-2">
            <button className="p-2 text-slate-400 hover:text-white hover:bg-white/10 rounded-lg transition-colors hidden sm:block">
              <Phone className="w-4 h-4" />
            </button>
            <button className="p-2 text-slate-400 hover:text-white hover:bg-white/10 rounded-lg transition-colors hidden sm:block">
              <Video className="w-4 h-4" />
            </button>
          </div>
        </div>

        <div className="flex-1 overflow-y-auto p-4 sm:p-6 space-y-6 custom-scrollbar bg-gradient-to-b from-[#050811] to-[#090d1c]">
          {activeMessages.length === 0 && (
             <div className="flex flex-col items-center justify-center h-full text-slate-500 gap-4 opacity-60">
                <Lock className="w-16 h-16" />
                <p className="text-sm text-center max-w-sm">No messages yet.<br/>Your communication is secured by A-TownChain's decentralized P2P routing.</p>
             </div>
          )}
          {activeMessages.map((msg, i) => {
            const showHeader = i === 0 || activeMessages[i-1].senderId !== msg.senderId || (msg.timestamp.getTime() - activeMessages[i-1].timestamp.getTime() > 300000);
            
            return (
              <motion.div 
                key={msg.id} 
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                className={`flex flex-col ${msg.isMe ? 'items-end' : 'items-start'}`}
              >
                {showHeader && (
                  <div className="text-xs text-slate-500 mb-1 ml-1 mr-1">
                    {msg.senderName} • {msg.timestamp.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                  </div>
                )}
                <div className={`max-w-[85%] sm:max-w-[70%] rounded-2xl px-4 py-2.5 shadow-sm ${
                  msg.isMe 
                    ? 'bg-indigo-600/90 text-white rounded-tr-sm shadow-indigo-500/10' 
                    : 'bg-white/10 text-slate-200 border border-white/5 rounded-tl-sm'
                }`}>
                  <p className="text-[15px] leading-relaxed whitespace-pre-wrap">{msg.text}</p>
                </div>
                {msg.isMe && i === activeMessages.length - 1 && (
                  <div className="text-[10px] text-slate-500 mt-1 mr-1">
                    {msg.status === 'delivered' ? 'Delivered' : 'Sent'}
                  </div>
                )}
              </motion.div>
            );
          })}
          {activeFriend?.typing && (
            <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }} className="flex items-start">
               <div className="bg-white/5 border border-white/10 text-slate-400 rounded-2xl rounded-tl-sm px-4 py-3 flex gap-1.5 items-center">
                 <div className="w-1.5 h-1.5 bg-slate-500 rounded-full animate-bounce" style={{ animationDelay: '0ms' }} />
                 <div className="w-1.5 h-1.5 bg-slate-500 rounded-full animate-bounce" style={{ animationDelay: '150ms' }} />
                 <div className="w-1.5 h-1.5 bg-slate-500 rounded-full animate-bounce" style={{ animationDelay: '300ms' }} />
               </div>
            </motion.div>
          )}
          <div ref={chatEndRef} />
        </div>

        <div className="p-4 bg-black/40 border-t border-white/5 backdrop-blur-xl">
          <div className="max-w-4xl mx-auto flex items-center gap-3">
             <textarea
               value={inputText}
               onChange={e => setInputText(e.target.value)}
               onKeyDown={e => {
                 if (e.key === 'Enter' && !e.shiftKey) {
                   e.preventDefault();
                   handleSendMessage();
                 }
               }}
               placeholder="Type an encrypted message..."
               className="flex-1 bg-white/5 border border-white/10 hover:border-white/20 rounded-xl px-4 py-3 text-sm text-white placeholder-slate-500 focus:outline-none focus:border-indigo-500/50 resize-none max-h-32 min-h-[44px] custom-scrollbar"
               rows={1}
             />
             <button 
               onClick={handleSendMessage}
               disabled={!inputText.trim() || wsStatus !== 'connected'}
               className="p-3 bg-indigo-500 hover:bg-indigo-600 disabled:opacity-50 disabled:hover:bg-indigo-500 rounded-xl text-white transition-all shadow-lg shadow-indigo-500/20"
             >
               <Send className="w-5 h-5 -ml-0.5 mt-0.5" />
             </button>
          </div>
        </div>
      </div>
    </div>
  );
}
