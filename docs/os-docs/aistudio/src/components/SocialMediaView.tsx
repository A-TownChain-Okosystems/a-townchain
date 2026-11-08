import React, { useState } from 'react';
import { Users, MessageSquare, LayoutList, ThumbsUp, ThumbsDown, Share2, Send, Search, UserPlus, MoreVertical } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

const mockFriends = [
  { id: '1', name: 'Alice Node', status: 'online', avatar: 'A' },
  { id: '2', name: 'Bob Chain', status: 'offline', avatar: 'B' },
  { id: '3', name: 'Charlie Hash', status: 'online', avatar: 'C' }
];

const mockPosts = [
  { id: '1', author: 'Alice Node', content: 'Just deployed my first smart contract on ATC!', timestamp: '2h ago', likes: 12, dislikes: 1, shares: 3 },
  { id: '2', author: 'Charlie Hash', content: 'ATC Network is running so smooth today. Incredible block times.', timestamp: '4h ago', likes: 24, dislikes: 0, shares: 1 }
];

const mockChat = [
  { id: '1', sender: 'Alice Node', text: 'Hey, are you free to look at this pull request?', time: '10:30 AM', isMe: false },
  { id: '2', sender: 'Me', text: 'Sure, I will check it out in 5 minutes.', time: '10:32 AM', isMe: true }
];

export function SocialMediaView({ initialTab = 'wall', initialPostContent = '' }: { initialTab?: 'wall' | 'chat' | 'friends', initialPostContent?: string }) {
  const [activeTab, setActiveTab] = useState<'wall' | 'chat' | 'friends'>(initialTab);
  const [posts, setPosts] = useState(mockPosts);
  const [newPost, setNewPost] = useState(initialPostContent);
  const [chatMessage, setChatMessage] = useState('');
  const [activeChat, setActiveChat] = useState(mockFriends[0]);

  const handleLike = (id: string) => {
    setPosts(prev => prev.map(p => p.id === id ? { ...p, likes: p.likes + 1 } : p));
  };

  const handleDislike = (id: string) => {
    setPosts(prev => prev.map(p => p.id === id ? { ...p, dislikes: p.dislikes + 1 } : p));
  };

  const handleShare = (id: string) => {
    const post = posts.find(p => p.id === id);
    if (post) {
      window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: `Shared post by ${post.author}` }));
      setPosts(prev => prev.map(p => p.id === id ? { ...p, shares: p.shares + 1 } : p));
    }
  };

  const handlePost = () => {
    if (!newPost.trim()) return;
    const post = {
      id: Date.now().toString(),
      author: 'Me',
      content: newPost,
      timestamp: 'just now',
      likes: 0,
      dislikes: 0,
      shares: 0
    };
    setPosts([post, ...posts]);
    setNewPost('');
    window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: 'Post created successfully.' }));
  };

  React.useEffect(() => {
    const handleOpenPost = (e: any) => {
      setActiveTab('wall');
      setNewPost(e.detail);
    };
    window.addEventListener('ATC_OPEN_SOCIAL_POST', handleOpenPost);
    return () => window.removeEventListener('ATC_OPEN_SOCIAL_POST', handleOpenPost);
  }, []);

  return (
    <div className="flex h-full bg-[#050811] text-slate-200">
      {/* Sidebar Navigation */}
      <div className="w-16 sm:w-64 border-r border-white/5 bg-black/40 flex flex-col">
        <div className="p-4 border-b border-white/5 hidden sm:block">
          <h2 className="text-lg font-bold text-white font-mono tracking-wider">ATC Social</h2>
        </div>
        <div className="flex-1 py-4 space-y-1 px-2">
          <button 
            onClick={() => setActiveTab('wall')}
            className={`w-full flex items-center gap-3 px-3 py-3 sm:py-2 rounded-xl transition-all ${activeTab === 'wall' ? 'bg-indigo-500/20 text-indigo-400' : 'hover:bg-white/5 text-slate-400 hover:text-white'}`}
          >
            <LayoutList className="w-5 h-5 mx-auto sm:mx-0" />
            <span className="hidden sm:block font-medium">Pinnwand</span>
          </button>
          <button 
            onClick={() => window.dispatchEvent(new CustomEvent('ATC_OPEN_APP', { detail: 'p2p_chat' }))}
            className={`w-full flex items-center gap-3 px-3 py-3 sm:py-2 rounded-xl transition-all hover:bg-white/5 text-slate-400 hover:text-white`}
          >
            <MessageSquare className="w-5 h-5 mx-auto sm:mx-0" />
            <span className="hidden sm:block font-medium">P2P Chat</span>
          </button>
          <button 
            onClick={() => setActiveTab('friends')}
            className={`w-full flex items-center gap-3 px-3 py-3 sm:py-2 rounded-xl transition-all ${activeTab === 'friends' ? 'bg-indigo-500/20 text-indigo-400' : 'hover:bg-white/5 text-slate-400 hover:text-white'}`}
          >
            <Users className="w-5 h-5 mx-auto sm:mx-0" />
            <span className="hidden sm:block font-medium">Freundesliste</span>
          </button>
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 flex flex-col overflow-hidden relative">
        <AnimatePresence mode="wait">
          {activeTab === 'wall' && (
            <motion.div key="wall" initial={{ opacity: 0, x: 20 }} animate={{ opacity: 1, x: 0 }} exit={{ opacity: 0, x: -20 }} className="flex-1 overflow-y-auto p-4 sm:p-6 custom-scrollbar">
              <div className="max-w-2xl mx-auto space-y-6">
                
                {/* Create Post */}
                <div className="bg-white/5 border border-white/10 rounded-2xl p-4">
                  <textarea 
                     value={newPost}
                     onChange={e => setNewPost(e.target.value)}
                     placeholder="Was gibt's Neues im Netzwerk?"
                     className="w-full bg-black/20 border border-white/5 rounded-xl p-3 text-sm text-white placeholder-slate-500 focus:outline-none focus:border-indigo-500/50 resize-none min-h-[100px]"
                  />
                  <div className="flex justify-end mt-3">
                    <button onClick={handlePost} className="px-4 py-2 bg-indigo-500 hover:bg-indigo-600 text-white rounded-lg text-sm font-medium transition-colors flex items-center gap-2">
                      <Send className="w-4 h-4" /> Posten
                    </button>
                  </div>
                </div>

                {/* Feed */}
                <div className="space-y-4">
                  {posts.map(post => (
                    <div key={post.id} className="bg-white/5 border border-white/10 rounded-2xl p-4 sm:p-5">
                      <div className="flex justify-between items-start mb-3">
                        <div className="flex items-center gap-3">
                          <div className="w-10 h-10 rounded-full bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center font-bold text-white shadow-lg">
                            {post.author[0]}
                          </div>
                          <div>
                            <div className="font-medium text-white text-sm">{post.author}</div>
                            <div className="text-xs text-slate-400">{post.timestamp}</div>
                          </div>
                        </div>
                        <button className="text-slate-500 hover:text-white transition-colors"><MoreVertical className="w-4 h-4" /></button>
                      </div>
                      
                      <p className="text-slate-200 text-sm mb-4 whitespace-pre-wrap">{post.content}</p>
                      
                      <div className="flex items-center gap-1 sm:gap-4 border-t border-white/5 pt-3">
                        <button onClick={() => handleLike(post.id)} className="flex-1 sm:flex-none flex items-center justify-center gap-2 px-3 py-1.5 rounded-lg hover:bg-white/5 text-slate-400 hover:text-emerald-400 transition-colors">
                          <ThumbsUp className="w-4 h-4" /> <span className="text-xs font-medium">{post.likes}</span>
                        </button>
                        <button onClick={() => handleDislike(post.id)} className="flex-1 sm:flex-none flex items-center justify-center gap-2 px-3 py-1.5 rounded-lg hover:bg-white/5 text-slate-400 hover:text-red-400 transition-colors">
                          <ThumbsDown className="w-4 h-4" /> <span className="text-xs font-medium">{post.dislikes}</span>
                        </button>
                        <button onClick={() => handleShare(post.id)} className="flex-1 sm:flex-none flex items-center justify-center gap-2 px-3 py-1.5 rounded-lg hover:bg-white/5 text-slate-400 hover:text-indigo-400 transition-colors">
                          <Share2 className="w-4 h-4" /> <span className="text-xs font-medium">{post.shares}</span>
                        </button>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </motion.div>
          )}

          {activeTab === 'chat' && (
            <motion.div key="chat" initial={{ opacity: 0, x: 20 }} animate={{ opacity: 1, x: 0 }} exit={{ opacity: 0, x: -20 }} className="flex-1 flex h-full">
              {/* Chat List */}
              <div className="w-48 sm:w-64 border-r border-white/5 flex flex-col bg-black/20">
                <div className="p-4 border-b border-white/5">
                  <div className="relative">
                    <Search className="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-slate-500" />
                    <input type="text" placeholder="Suchen..." className="w-full bg-white/5 border border-white/10 rounded-lg pl-9 pr-3 py-2 text-sm text-white placeholder-slate-500 focus:outline-none focus:border-indigo-500/50" />
                  </div>
                </div>
                <div className="flex-1 overflow-y-auto custom-scrollbar p-2 space-y-1">
                  {mockFriends.map(friend => (
                    <button 
                      key={friend.id} 
                      onClick={() => setActiveChat(friend)}
                      className={`w-full flex items-center gap-3 p-2 rounded-xl transition-all text-left ${activeChat.id === friend.id ? 'bg-indigo-500/20' : 'hover:bg-white/5'}`}
                    >
                      <div className="relative">
                        <div className="w-10 h-10 rounded-full bg-slate-800 flex items-center justify-center font-bold text-slate-300">
                          {friend.avatar}
                        </div>
                        <div className={`absolute bottom-0 right-0 w-3 h-3 rounded-full border-2 border-[#090b14] ${friend.status === 'online' ? 'bg-emerald-500' : 'bg-slate-500'}`} />
                      </div>
                      <div className="flex-1 overflow-hidden">
                        <div className="text-sm font-medium text-white truncate">{friend.name}</div>
                        <div className="text-xs text-slate-500 truncate">{friend.status === 'online' ? 'Online' : 'Offline'}</div>
                      </div>
                    </button>
                  ))}
                </div>
              </div>
              
              {/* Chat View */}
              <div className="flex-1 flex flex-col bg-[#050811]">
                <div className="p-4 border-b border-white/5 flex items-center gap-3 bg-black/20">
                  <div className="w-10 h-10 rounded-full bg-slate-800 flex items-center justify-center font-bold text-slate-300">
                     {activeChat.avatar}
                  </div>
                  <div>
                    <div className="text-sm font-medium text-white">{activeChat.name}</div>
                    <div className="text-xs text-emerald-400">P2P Encrypted</div>
                  </div>
                </div>
                
                <div className="flex-1 overflow-y-auto p-4 space-y-4 custom-scrollbar">
                  <div className="text-center text-xs text-slate-500 my-4">Connection securely established.</div>
                  {mockChat.map(msg => (
                    <div key={msg.id} className={`flex ${msg.isMe ? 'justify-end' : 'justify-start'}`}>
                      <div className={`max-w-[75%] rounded-2xl p-3 ${msg.isMe ? 'bg-indigo-500 text-white rounded-tr-sm' : 'bg-white/10 text-slate-200 rounded-tl-sm border border-white/5'}`}>
                        <div className="text-sm">{msg.text}</div>
                        <div className={`text-[10px] mt-1 text-right ${msg.isMe ? 'text-indigo-200' : 'text-slate-400'}`}>{msg.time}</div>
                      </div>
                    </div>
                  ))}
                </div>
                
                <div className="p-4 border-t border-white/5 bg-black/20">
                  <div className="flex items-center gap-2">
                    <input 
                      type="text" 
                      value={chatMessage}
                      onChange={e => setChatMessage(e.target.value)}
                      onKeyDown={e => e.key === 'Enter' && setChatMessage('')}
                      placeholder="Sichere Nachricht senden..." 
                      className="flex-1 bg-white/5 border border-white/10 rounded-xl px-4 py-2 text-sm text-white placeholder-slate-500 focus:outline-none focus:border-indigo-500/50"
                    />
                    <button className="p-2.5 bg-indigo-500 hover:bg-indigo-600 rounded-xl text-white transition-colors">
                      <Send className="w-4 h-4" />
                    </button>
                  </div>
                </div>
              </div>
            </motion.div>
          )}

          {activeTab === 'friends' && (
            <motion.div key="friends" initial={{ opacity: 0, x: 20 }} animate={{ opacity: 1, x: 0 }} exit={{ opacity: 0, x: -20 }} className="flex-1 overflow-y-auto p-4 sm:p-6 custom-scrollbar">
              <div className="max-w-4xl mx-auto">
                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 mb-6">
                  <h3 className="text-lg font-bold text-white flex items-center gap-2">
                    <Users className="w-5 h-5 text-indigo-400" /> Meine Kontakte
                  </h3>
                  <div className="flex items-center gap-3 w-full sm:w-auto">
                    <div className="relative flex-1 sm:w-64">
                      <Search className="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-slate-500" />
                      <input type="text" placeholder="Nach Node ID oder Name..." className="w-full bg-white/5 border border-white/10 rounded-lg pl-9 pr-3 py-2 text-sm text-white placeholder-slate-500 focus:outline-none focus:border-indigo-500/50" />
                    </div>
                    <button className="p-2 bg-indigo-500 hover:bg-indigo-600 rounded-lg text-white transition-colors" title="Kontakt hinzufügen">
                      <UserPlus className="w-4 h-4" />
                    </button>
                  </div>
                </div>
                
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {mockFriends.map(friend => (
                    <div key={friend.id} className="bg-white/5 border border-white/10 rounded-2xl p-4 flex items-center gap-4 hover:border-white/20 transition-colors">
                      <div className="relative">
                        <div className="w-12 h-12 rounded-full bg-gradient-to-br from-slate-700 to-slate-800 flex items-center justify-center font-bold text-xl text-white shadow-lg border border-white/10">
                          {friend.avatar}
                        </div>
                        <div className={`absolute bottom-0 right-0 w-3.5 h-3.5 rounded-full border-2 border-[#090b14] ${friend.status === 'online' ? 'bg-emerald-500' : 'bg-slate-500'}`} />
                      </div>
                      <div className="flex-1 overflow-hidden">
                        <div className="font-semibold text-white truncate">{friend.name}</div>
                        <div className="text-xs text-slate-400 flex items-center gap-1 truncate">
                           ID: ATC-{Math.random().toString(16).substring(2, 10).toUpperCase()}
                        </div>
                      </div>
                      <button 
                        className="p-2 text-indigo-400 hover:bg-indigo-500/20 rounded-lg transition-colors"
                        onClick={() => {
                          setActiveChat(friend);
                          setActiveTab('chat');
                        }}
                      >
                        <MessageSquare className="w-4 h-4" />
                      </button>
                    </div>
                  ))}
                </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
}
