import React, { useState } from "react";
import {
  MessageSquare,
  Calendar,
  Search,
  Clock,
  Bot,
  User,
  Filter,
  Download,
} from "lucide-react";

export function ChatbotArchivePanel() {
  const [searchQuery, setSearchQuery] = useState("");

  const sessions = [
    {
      id: "sess-8a9d",
      date: "2026-06-03 10:15",
      topic: "API Integration Debugging",
      messages: 24,
      duration: "45m",
    },
    {
      id: "sess-7b3f",
      date: "2026-06-02 14:30",
      topic: "UI Layout Optimierung",
      messages: 12,
      duration: "18m",
    },
    {
      id: "sess-2c1a",
      date: "2026-06-01 09:00",
      topic: "Datenbankschema Entwurf",
      messages: 45,
      duration: "1h 20m",
    },
    {
      id: "sess-9f5e",
      date: "2026-05-28 16:45",
      topic: "Authentifizierung Setup",
      messages: 8,
      duration: "12m",
    },
  ];

  const dummyChat = [
    {
      role: "user",
      content:
        "Kannst du mir helfen, die JWT Validierung im Express Server einzubauen?",
    },
    {
      role: "assistant",
      content:
        "Natürlich! Hier ist ein Beispiel, wie du eine JWT Middleware in Express implementieren kannst...",
    },
    { role: "user", content: "Wo setze ich das Secret?" },
    {
      role: "assistant",
      content:
        "Das Secret solltest du zwingend in deinen Umgebungsvariablen (.env) speichern, niemals direkt im Code.",
    },
  ];

  const [activeSession, setActiveSession] = useState(sessions[0].id);

  return (
    <div className="flex-1 flex flex-col font-sans h-full bg-[#0c0c0e]">
      <div className="p-6 border-b border-white/10 flex items-center justify-between bg-black/20">
        <div className="flex items-center gap-3">
          <div className="p-2 bg-blue-500/10 rounded-lg">
            <MessageSquare className="w-5 h-5 text-blue-400" />
          </div>
          <div>
            <h2 className="text-xl font-bold text-slate-200">Chatbot Archiv</h2>
            <p className="text-sm text-slate-500 mt-1">
              Vergangene Konversationen, KI-Sitzungen und Prompts durchsuchen.
            </p>
          </div>
        </div>
        <div className="flex gap-2">
          <div className="relative">
            <Search className="w-4 h-4 text-slate-500 absolute left-3 top-1/2 -translate-y-1/2" />
            <input
              type="text"
              placeholder="Suchen in Chats..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-64 bg-black/50 border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-slate-200 outline-none focus:border-blue-500/50"
            />
          </div>
          <button
            className="p-2 rounded bg-white/5 hover:bg-white/10 text-slate-400 transition-colors"
            title="Filter"
          >
            <Filter className="w-4 h-4" />
          </button>
        </div>
      </div>

      <div className="flex flex-1 overflow-hidden">
        {/* Session List */}
        <div className="w-80 border-r border-white/10 bg-black/20 flex flex-col overflow-y-auto">
          <div className="p-3 space-y-2">
            {sessions.map((session) => (
              <div
                key={session.id}
                onClick={() => setActiveSession(session.id)}
                className={`p-3 rounded-lg cursor-pointer border transition-all ${
                  activeSession === session.id
                    ? "bg-blue-500/10 border-blue-500/30"
                    : "bg-white/[0.02] border-transparent hover:bg-white/5"
                }`}
              >
                <h4
                  className={`font-bold text-sm mb-1 ${activeSession === session.id ? "text-blue-400" : "text-slate-200"}`}
                >
                  {session.topic}
                </h4>
                <div className="flex items-center justify-between text-xs text-slate-500 mt-2">
                  <span className="flex items-center gap-1">
                    <Calendar className="w-3 h-3" />{" "}
                    {session.date.split(" ")[0]}
                  </span>
                  <span className="flex items-center gap-1">
                    <Clock className="w-3 h-3" /> {session.duration}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Chat Viewer */}
        <div className="flex-1 flex flex-col bg-black/40 relative">
          <div className="absolute top-4 right-4 z-10">
            <button className="flex items-center gap-2 px-3 py-1.5 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-xs font-bold text-slate-300 transition-colors">
              <Download className="w-3.5 h-3.5" /> Export
            </button>
          </div>
          <div className="flex-1 p-6 overflow-y-auto space-y-6">
            {dummyChat.map((msg, idx) => (
              <div
                key={idx}
                className={`flex gap-4 ${msg.role === "assistant" ? "" : "flex-row-reverse"}`}
              >
                <div
                  className={`w-8 h-8 rounded-full flex items-center justify-center shrink-0 ${
                    msg.role === "assistant"
                      ? "bg-blue-500/20 text-blue-400"
                      : "bg-emerald-500/20 text-emerald-400"
                  }`}
                >
                  {msg.role === "assistant" ? (
                    <Bot className="w-4 h-4" />
                  ) : (
                    <User className="w-4 h-4" />
                  )}
                </div>
                <div
                  className={`px-4 py-3 rounded-2xl max-w-[80%] ${
                    msg.role === "assistant"
                      ? "bg-white/5 border border-white/10 text-slate-300 rounded-tl-none"
                      : "bg-blue-500/10 border border-blue-500/20 text-blue-100 rounded-tr-none"
                  }`}
                >
                  <p className="text-sm leading-relaxed">{msg.content}</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
