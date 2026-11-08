# 🏗️ Architektur — atc-ide

Das IDE-Repository kapselt Plugins für Entwickler-Editoren und verbindet diese mit dem ATCLang LSP Server.

## Flussdiagramm

```
[ VS Code / JetBrains / Vim ]
           |
     JSON-RPC Protocol
           v
   [ ATCLang LSP Server ]
      /             \
     v               v
[ Lexer/Parser ]  [ ATVM Debugger ]
```
