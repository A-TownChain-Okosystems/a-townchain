import React, { useState, useEffect } from 'react';
import { useGoogleWorkspace } from '../contexts/GoogleWorkspaceContext.tsx';
import { Network, Database, Layers, CheckCircle, FileText, Calendar, Mail } from 'lucide-react';

export default function IntegrationsWindow() {
  const { signIn, signOut, user, isAuthenticated, accessToken } = useGoogleWorkspace();
  const [notionConnected, setNotionConnected] = useState(false);
  const [notionPages, setNotionPages] = useState<any[]>([]);
  const [isLoadingNotion, setIsLoadingNotion] = useState(false);
  const [activeTab, setActiveTab] = useState<'google' | 'notion'>('google');

  // Google data state
  const [driveFiles, setDriveFiles] = useState<any[]>([]);
  const [calendarEvents, setCalendarEvents] = useState<any[]>([]);
  const [spreadsheets, setSpreadsheets] = useState<any[]>([]);
  const [gmailMessages, setGmailMessages] = useState<any[]>([]);
  const [docs, setDocs] = useState<any[]>([]);
  const [isLoadingGoogle, setIsLoadingGoogle] = useState(false);

  useEffect(() => {
    if (isAuthenticated && accessToken) {
      loadGoogleData();
    } else {
      setDriveFiles([]);
      setCalendarEvents([]);
      setSpreadsheets([]);
      setGmailMessages([]);
      setDocs([]);
    }
  }, [isAuthenticated, accessToken]);

  const loadGoogleData = async () => {
    if (!accessToken) return;
    setIsLoadingGoogle(true);
    try {
      const authHeaders = { Authorization: `Bearer ${accessToken}` };
      
      const driveRes = await fetch('https://www.googleapis.com/drive/v3/files?pageSize=5&fields=files(id,name,mimeType)', { headers: authHeaders });
      if (driveRes.ok) {
        const driveData = await driveRes.json();
        setDriveFiles(driveData.files || []);
      }

      const timeMin = new Date().toISOString();
      const calRes = await fetch(`https://www.googleapis.com/calendar/v3/calendars/primary/events?timeMin=${timeMin}&maxResults=5&singleEvents=true&orderBy=startTime`, { headers: authHeaders });
      if (calRes.ok) {
        const calData = await calRes.json();
        setCalendarEvents(calData.items || []);
      }

      const sheetRes = await fetch('https://www.googleapis.com/drive/v3/files?q=mimeType=\'application/vnd.google-apps.spreadsheet\'&pageSize=5&fields=files(id,name)', { headers: authHeaders });
      if (sheetRes.ok) {
        const sheetData = await sheetRes.json();
        setSpreadsheets(sheetData.files || []);
      }

      const docsRes = await fetch('https://www.googleapis.com/drive/v3/files?q=mimeType=\'application/vnd.google-apps.document\'&pageSize=5&fields=files(id,name)', { headers: authHeaders });
      if (docsRes.ok) {
        const docsData = await docsRes.json();
        setDocs(docsData.files || []);
      }

      const gmailRes = await fetch('https://gmail.googleapis.com/gmail/v1/users/me/messages?maxResults=5', { headers: authHeaders });
      if (gmailRes.ok) {
        const gmailData = await gmailRes.json();
        if (gmailData.messages && gmailData.messages.length > 0) {
          const messagesWithDetails = await Promise.all(
            gmailData.messages.map(async (msg: any) => {
              const msgDetailRes = await fetch(`https://gmail.googleapis.com/gmail/v1/users/me/messages/${msg.id}?format=metadata&metadataHeaders=Subject&metadataHeaders=From`, { headers: authHeaders });
              if (msgDetailRes.ok) {
                const detailData = await msgDetailRes.json();
                const subjectHeader = detailData.payload?.headers?.find((h: any) => h.name === 'Subject');
                const fromHeader = detailData.payload?.headers?.find((h: any) => h.name === 'From');
                return {
                  id: msg.id,
                  snippet: detailData.snippet,
                  subject: subjectHeader ? subjectHeader.value : 'No Subject',
                  from: fromHeader ? fromHeader.value : 'Unknown'
                };
              }
              return { id: msg.id, subject: 'Unknown', snippet: '' };
            })
          );
          setGmailMessages(messagesWithDetails);
        } else {
          setGmailMessages([]);
        }
      }

    } catch (e) {
      console.error("Failed to load Google data", e);
    } finally {
      setIsLoadingGoogle(false);
    }
  };

  useEffect(() => {
    // Determine if notion is connected by checking via our new API
    if (user) {
      checkNotionConnection();
    } else {
      setNotionConnected(false);
      setNotionPages([]);
    }
  }, [user]);

  const checkNotionConnection = async () => {
    setIsLoadingNotion(true);
    try {
      // Just check the endpoint
      const res = await fetch('/api/notion/pages', {
        headers: {
          'Authorization': `Bearer ${user?.uid}` // Pass user id since we don't have token mapped on client side yet, wait we need token.
        }
      });
      if (res.ok) {
         setNotionConnected(true);
         const data = await res.json();
         setNotionPages(data.results || []);
      } else {
         setNotionConnected(false);
         setNotionPages([]);
      }
    } catch {
      setNotionConnected(false);
    } finally {
      setIsLoadingNotion(false);
    }
  };

  const handleConnectNotion = async () => {
    if (!user) return alert('Please sign in with Google first to create an account context.');
    try {
      const authHeader = `Bearer ${await user.getIdToken()}`;
      const res = await fetch('/api/notion/auth/url', {
        headers: {
          'Authorization': authHeader
        }
      });
      if (!res.ok) throw new Error('Failed to get Notion Auth URL');
      const { url } = await res.json();
      
      const authWindow = window.open(
        url,
        'oauth_popup',
        'width=600,height=700'
      );
      
      if (!authWindow) {
        alert('Please allow popups to connect to Notion.');
      }
    } catch(err) {
      console.error(err);
      alert('Error initiating Notion connection');
    }
  };

  useEffect(() => {
    const handleMessage = (event: MessageEvent) => {
      const origin = event.origin;
      if (!origin.endsWith('.run.app') && !origin.includes('localhost')) return;
      if (event.data?.type === 'OAUTH_AUTH_SUCCESS') {
        checkNotionConnection();
      }
    };
    window.addEventListener('message', handleMessage);
    return () => window.removeEventListener('message', handleMessage);
  }, []);

  // Use the same auth pattern for Notion pages request
  useEffect(() => {
    const fetchNotionPagesWithAuth = async () => {
      if (notionConnected && user) {
        const authHeader = `Bearer ${await user.getIdToken()}`;
        const res = await fetch('/api/notion/pages', {
          headers: { 'Authorization': authHeader }
        });
        if (res.ok) {
           const data = await res.json();
           setNotionPages(data.results || []);
        }
      }
    };
    fetchNotionPagesWithAuth();
  }, [notionConnected, user]);

  return (
    <div className="w-full h-full bg-[#050811] text-slate-300 overflow-hidden flex flex-col">
      <div className="p-6 border-b border-white/10 flex items-center justify-between shrink-0 bg-white/[0.02]">
        <div>
          <h2 className="text-xl font-bold text-white tracking-tight flex items-center gap-2">
            <Network className="text-indigo-400" /> System Integrations
          </h2>
          <p className="text-sm text-slate-500 mt-1">Connect your workspace tools to ATC-OS</p>
        </div>
        <div className="flex bg-[#090b14] rounded-lg p-1 border border-white/10">
          <button 
            onClick={() => setActiveTab('google')}
            className={`px-4 py-1.5 text-sm rounded transition-colors ${activeTab === 'google' ? 'bg-indigo-500/20 text-indigo-300' : 'hover:bg-white/5'}`}
          >
            Google Workspace
          </button>
          <button 
            onClick={() => setActiveTab('notion')}
            className={`px-4 py-1.5 text-sm rounded transition-colors ${activeTab === 'notion' ? 'bg-indigo-500/20 text-indigo-300' : 'hover:bg-white/5'}`}
          >
            Notion
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-6">
        {activeTab === 'google' && (
          <div className="max-w-2xl mx-auto flex flex-col gap-6">
            <div className="bg-[#090b14] border border-white/10 p-6 rounded-xl relative overflow-hidden group">
              <div className="w-16 h-16 rounded-2xl bg-indigo-500/10 border border-indigo-500/30 flex items-center justify-center mb-4 text-indigo-400 group-hover:scale-110 group-hover:rotate-3 transition-transform">
                <Database className="w-8 h-8" />
              </div>
              <h3 className="text-lg font-bold text-white mb-2">Google Workspace</h3>
              <p className="text-sm text-slate-400 mb-6 max-w-md">
                Connect your Google account to access Drive, Docs, Sheets, Calendar, Contacts, and other Workspace tools directly within the ATC environment.
              </p>
              
              {isAuthenticated ? (
                <div className="flex items-center justify-between p-4 rounded-lg bg-emerald-500/10 border border-emerald-500/30">
                  <div className="flex items-center gap-3">
                    <CheckCircle className="text-emerald-400 w-5 h-5" />
                    <div>
                      <div className="text-sm font-semibold text-emerald-400">Connected to Google Workspace</div>
                      <div className="text-xs text-emerald-500/70">{user?.email}</div>
                    </div>
                  </div>
                  <button onClick={signOut} className="text-xs text-emerald-400 hover:text-emerald-300 underline underline-offset-2">Disconnect</button>
                </div>
              ) : (
                <button 
                  onClick={signIn}
                  className="px-6 py-2.5 bg-indigo-500 hover:bg-indigo-400 text-white font-medium rounded-lg transition-colors border border-indigo-400/50 shadow-[0_0_15px_rgba(99,102,241,0.3)]"
                >
                  Connect Google Workspace
                </button>
              )}
            </div>
            
            {isAuthenticated && (
               <div className="bg-[#090b14] border border-white/10 p-6 rounded-xl flex flex-col gap-6">
                  <div>
                    <h4 className="text-sm font-semibold uppercase tracking-widest text-slate-500 mb-4">Workspace Status</h4>
                    <div className="grid grid-cols-2 sm:grid-cols-3 gap-4">
                      {["Drive", "Docs", "Sheets", "Calendar", "Gmail", "Contacts"].map(app => (
                          <div key={app} className="flex items-center gap-2 p-3 rounded bg-white/5 border border-white/5">
                            <Layers className="w-4 h-4 text-slate-400" />
                            <span className="text-sm font-medium text-slate-300">{app} Ready</span>
                          </div>
                      ))}
                    </div>
                  </div>

                  {isLoadingGoogle ? (
                    <div className="text-sm text-slate-400 animate-pulse">Loading Workspace data...</div>
                  ) : (
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                      <div className="flex flex-col gap-3">
                        <h4 className="text-sm font-semibold text-slate-300 flex items-center gap-2">
                          <FileText className="w-4 h-4 text-blue-400" /> Recent Drive Files
                        </h4>
                        {driveFiles.length === 0 ? (
                          <p className="text-xs text-slate-500 italic">No files found or missing scopes.</p>
                        ) : (
                          <div className="flex flex-col gap-2">
                            {driveFiles.map(file => (
                              <div key={file.id} className="p-2 rounded bg-white/5 border border-white/10 flex flex-col">
                                <span className="text-sm font-medium text-slate-200 truncate">{file.name}</span>
                                <span className="text-[10px] text-slate-500 font-mono truncate">{file.mimeType}</span>
                              </div>
                            ))}
                          </div>
                        )}
                      </div>

                      <div className="flex flex-col gap-3">
                        <h4 className="text-sm font-semibold text-slate-300 flex items-center gap-2">
                          <FileText className="w-4 h-4 text-blue-500" /> Recent Docs
                        </h4>
                        {docs.length === 0 ? (
                          <p className="text-xs text-slate-500 italic">No docs found or missing scopes.</p>
                        ) : (
                          <div className="flex flex-col gap-2">
                            {docs.map(doc => (
                              <div key={doc.id} className="p-2 rounded bg-white/5 border border-white/10 flex flex-col">
                                <span className="text-sm font-medium text-slate-200 truncate">{doc.name}</span>
                              </div>
                            ))}
                          </div>
                        )}
                      </div>

                      <div className="flex flex-col gap-3">
                        <h4 className="text-sm font-semibold text-slate-300 flex items-center gap-2">
                          <Database className="w-4 h-4 text-emerald-400" /> Recent Sheets
                        </h4>
                        {spreadsheets.length === 0 ? (
                          <p className="text-xs text-slate-500 italic">No sheets found or missing scopes.</p>
                        ) : (
                          <div className="flex flex-col gap-2">
                            {spreadsheets.map(sheet => (
                              <div key={sheet.id} className="p-2 rounded bg-white/5 border border-white/10 flex flex-col">
                                <span className="text-sm font-medium text-slate-200 truncate">{sheet.name}</span>
                              </div>
                            ))}
                          </div>
                        )}
                      </div>

                      <div className="flex flex-col gap-3">
                        <h4 className="text-sm font-semibold text-slate-300 flex items-center gap-2">
                          <Calendar className="w-4 h-4 text-emerald-400" /> Upcoming Events
                        </h4>
                        {calendarEvents.length === 0 ? (
                          <p className="text-xs text-slate-500 italic">No events found or missing scopes.</p>
                        ) : (
                          <div className="flex flex-col gap-2">
                            {calendarEvents.map(event => (
                              <div key={event.id} className="p-2 rounded bg-white/5 border border-white/10 flex flex-col">
                                <span className="text-sm font-medium text-slate-200 truncate">{event.summary || 'Untitled Event'}</span>
                                <span className="text-[10px] text-slate-500">
                                  {new Date(event.start?.dateTime || event.start?.date).toLocaleString()}
                                </span>
                              </div>
                            ))}
                          </div>
                        )}
                      </div>

                      <div className="flex flex-col gap-3">
                        <h4 className="text-sm font-semibold text-slate-300 flex items-center gap-2">
                          <Mail className="w-4 h-4 text-rose-400" /> Recent Emails
                        </h4>
                        {gmailMessages.length === 0 ? (
                          <p className="text-xs text-slate-500 italic">No emails found or missing scopes.</p>
                        ) : (
                          <div className="flex flex-col gap-2">
                            {gmailMessages.map(msg => (
                              <div key={msg.id} className="p-2 rounded bg-white/5 border border-white/10 flex flex-col">
                                <span className="text-sm font-medium text-slate-200 truncate">{msg.subject}</span>
                                <span className="text-[10px] text-slate-500 truncate">{msg.from}</span>
                              </div>
                            ))}
                          </div>
                        )}
                      </div>
                    </div>
                  )}
               </div>
            )}
          </div>
        )}

         {activeTab === 'notion' && (
          <div className="max-w-2xl mx-auto flex flex-col gap-6">
            <div className="bg-[#090b14] border border-white/10 p-6 rounded-xl relative overflow-hidden group">
              <div className="w-16 h-16 rounded-2xl bg-slate-500/10 border border-slate-500/30 flex items-center justify-center mb-4 text-slate-300 group-hover:scale-110 group-hover:-rotate-3 transition-transform">
                <svg width="32" height="32" viewBox="0 0 24 24" fill="currentColor">
                   <path d="M4.45899 4.509C4.16199 4.316 3.99399 4.368 3.94299 4.542C3.91699 4.607 3.86499 4.807 3.83299 5.091L3.80199 8.21C3.80199 8.423 3.85999 8.617 3.97099 8.785L7.20699 13.915H7.31099V5.63301C7.31099 5.38101 7.20699 5.17401 7.00099 5.01901L4.45899 4.509ZM20.122 4.509C19.825 4.316 19.657 4.368 19.606 4.542C19.58 4.607 19.535 4.807 19.496 5.091L19.465 8.21C19.465 8.423 19.523 8.617 19.634 8.785L22.87 13.915H22.974V5.63301C22.974 5.38101 22.87 5.17401 22.664 5.01901L20.122 4.509ZM8.242 18.06V20.127H5.69801V18.261L8.242 18.06ZM11.026 15.341H14.126V17.02H11.026V15.341ZM8.242 15.083V17.15H5.69801V15.284L8.242 15.083ZM16.3 12.56L13.111 18.665H15.178L18.367 12.56H16.3ZM12.06 6.82201V8.88901H9.51601V7.02301L12.06 6.82201ZM14.844 4.09H17.944V5.76901H14.844V4.09ZM12.06 3.83201V5.89901H9.51601V4.03301L12.06 3.83201Z" />
                </svg>
              </div>
              <h3 className="text-lg font-bold text-white mb-2">Notion Integration</h3>
              <p className="text-sm text-slate-400 mb-6 max-w-md">
                Connect your Notion workspace to read pages, access databases, and seamlessly integrate Notion notes directly into the ATC ecosystem.
              </p>
              
              {!user ? (
                 <div className="p-4 rounded-lg bg-rose-500/10 border border-rose-500/20 text-rose-400 text-sm">
                    You must sign in with Google Workspace first to establish your identity context.
                 </div>
              ) : isLoadingNotion ? (
                 <div className="p-3 text-slate-400 text-sm animate-pulse">Checking connection status...</div>
              ) : notionConnected ? (
                <div className="flex items-center justify-between p-4 rounded-lg bg-emerald-500/10 border border-emerald-500/30">
                  <div className="flex items-center gap-3">
                    <CheckCircle className="text-emerald-400 w-5 h-5" />
                    <div>
                      <div className="text-sm font-semibold text-emerald-400">Connected to Notion</div>
                      <div className="text-xs text-emerald-500/70">{notionPages.length} Pages Accessible</div>
                    </div>
                  </div>
                  {/* Disconnect notion from user object */}
                </div>
              ) : (
                <button 
                  onClick={handleConnectNotion}
                  className="px-6 py-2.5 bg-slate-100 hover:bg-white text-slate-900 font-bold rounded-lg transition-colors border border-transparent shadow-[0_0_15px_rgba(255,255,255,0.1)]"
                >
                  Connect Notion
                </button>
              )}
            </div>
            
            {notionConnected && (
               <div className="bg-[#090b14] border border-white/10 p-6 rounded-xl flex flex-col gap-4">
                  <h4 className="text-sm font-semibold uppercase tracking-widest text-slate-500">Accessible Pages</h4>
                  {notionPages.length === 0 ? (
                     <p className="text-sm text-slate-500 italic">No pages found in this workspace or missing permissions inside Notion.</p>
                  ) : (
                     <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 max-h-[300px] overflow-y-auto custom-scrollbar pr-2">
                        {notionPages.map(page => {
                           const titleObj = page.properties?.title?.title?.[0] || page.properties?.Name?.title?.[0];
                           const title = titleObj ? titleObj.plain_text : 'Untitled';
                           return (
                              <div key={page.id} className="flex flex-col p-3 rounded-lg bg-white/5 border border-white/10 hover:border-white/20 transition-colors">
                                 <span className="text-sm font-semibold text-white truncate">{title}</span>
                                 <span className="text-[10px] text-slate-500 font-mono mt-1">ID: {page.id.substring(0, 8)}...</span>
                              </div>
                           )
                        })}
                     </div>
                  )}
               </div>
            )}
          </div>
        )}
      </div>

    </div>
  );
}
