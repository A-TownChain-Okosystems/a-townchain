# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.units import cm
from reportlab.lib import colors
from reportlab.platypus import (SimpleDocTemplate, Paragraph, Spacer, Table,
                                 TableStyle, HRFlowable, KeepTogether)
from reportlab.lib.enums import TA_LEFT, TA_CENTER, TA_RIGHT

# ── Farben ────────────────────────────────────────────
PURPLE  = colors.HexColor("#a259ff")
CYAN    = colors.HexColor("#00d1ff")
PINK    = colors.HexColor("#ff2d78")
GREEN   = colors.HexColor("#00c896")
ORANGE  = colors.HexColor("#ff8c00")
DARK    = colors.HexColor("#05080f")
PANEL   = colors.HexColor("#0d1528")
DIMTEXT = colors.HexColor("#4a6080")
WHITE   = colors.HexColor("#e0f0ff")
RED     = colors.HexColor("#ff3355")

doc = SimpleDocTemplate(
    "/app/ATC_Issues_Zusammenfassung.pdf",
    pagesize=A4,
    rightMargin=1.8*cm, leftMargin=1.8*cm,
    topMargin=1.8*cm, bottomMargin=1.8*cm,
    title="A-TownChain OS — Issue Übersicht",
    author="ShivaCoreDev × Aurora AI"
)

styles = getSampleStyleSheet()

def S(name, **kw):
    return ParagraphStyle(name, **kw)

sTitle = S("sTitle", fontSize=22, textColor=PURPLE, fontName="Helvetica-Bold",
           spaceAfter=4, alignment=TA_CENTER, leading=26)
sSubtitle = S("sSubtitle", fontSize=10, textColor=CYAN, fontName="Helvetica",
              spaceAfter=2, alignment=TA_CENTER)
sMeta = S("sMeta", fontSize=8, textColor=DIMTEXT, fontName="Helvetica",
          spaceAfter=12, alignment=TA_CENTER)
sSection = S("sSection", fontSize=13, textColor=CYAN, fontName="Helvetica-Bold",
             spaceBefore=14, spaceAfter=6, leading=16)
sBody = S("sBody", fontSize=9, textColor=colors.HexColor("#c0d0e8"),
          fontName="Helvetica", leading=13, spaceAfter=2)
sSmall = S("sSmall", fontSize=7.5, textColor=DIMTEXT, fontName="Helvetica", leading=11)
sBold = S("sBold", fontSize=9, textColor=WHITE, fontName="Helvetica-Bold", leading=13)
sFooter = S("sFooter", fontSize=7.5, textColor=DIMTEXT, fontName="Helvetica",
            alignment=TA_CENTER)
sPhase = S("sPhase", fontSize=11, textColor=PURPLE, fontName="Helvetica-Bold",
           spaceBefore=10, spaceAfter=4)

story = []

# ── HEADER ────────────────────────────────────────────
story.append(Spacer(1, 0.3*cm))
story.append(Paragraph("A-TownChain OS", sTitle))
story.append(Paragraph("Issue Übersicht &amp; Technische Zusammenfassung", sSubtitle))
story.append(Paragraph("ShivaCoreDev × Aurora AI  ·  v2.0.0 Genesis Release  ·  20. Mai 2026", sMeta))
story.append(HRFlowable(width="100%", thickness=1, color=PURPLE, spaceAfter=10))

# ── STATS HEADER ──────────────────────────────────────
stats_data = [
    ["19", "13", "6", "3"],
    ["Gesamt Issues", "Haupt-Features", "Testnet Sub-Issues", "Milestones"],
]
stats_table = Table(stats_data, colWidths=[4.2*cm]*4)
stats_table.setStyle(TableStyle([
    ("BACKGROUND",    (0,0), (-1,-1), PANEL),
    ("TEXTCOLOR",     (0,0), (-1,0),  PURPLE),
    ("TEXTCOLOR",     (0,1), (-1,1),  DIMTEXT),
    ("FONTNAME",      (0,0), (-1,0),  "Helvetica-Bold"),
    ("FONTSIZE",      (0,0), (-1,0),  20),
    ("FONTNAME",      (0,1), (-1,1),  "Helvetica"),
    ("FONTSIZE",      (0,1), (-1,1),  8),
    ("ALIGN",         (0,0), (-1,-1), "CENTER"),
    ("VALIGN",        (0,0), (-1,-1), "MIDDLE"),
    ("TOPPADDING",    (0,0), (-1,-1), 10),
    ("BOTTOMPADDING", (0,0), (-1,-1), 8),
    ("GRID",          (0,0), (-1,-1), 0.5, colors.HexColor("#1a2540")),
    ("ROUNDEDCORNERS",(0,0), (-1,-1), [4,4,4,4]),
]))
story.append(stats_table)
story.append(Spacer(1, 0.5*cm))

# ── MILESTONE LEGENDE ─────────────────────────────────
story.append(Paragraph("Milestone-Übersicht", sSection))
milestone_data = [
    ["Milestone", "Issues", "Fokus", "Status"],
    ["v2.1.0", "#1 #2 #3 #4 #5 #6", "Sicherheit · Persistenz · AI · Battle · Explorer", "🔄 In Planung"],
    ["v2.2.0", "#7–#9 #11–#13 #14–#19", "Testnet · Governance · Marketplace · Build", "📋 Backlog"],
    ["v3.0.0", "#10", "Cross-Chain Bridge (EVM Interop)", "🔮 Future"],
]
ms_table = Table(milestone_data, colWidths=[2.5*cm, 4.5*cm, 7.5*cm, 2.7*cm])
ms_table.setStyle(TableStyle([
    ("BACKGROUND",    (0,0), (-1,0),  PURPLE),
    ("TEXTCOLOR",     (0,0), (-1,0),  colors.white),
    ("FONTNAME",      (0,0), (-1,0),  "Helvetica-Bold"),
    ("FONTSIZE",      (0,0), (-1,-1), 8),
    ("FONTNAME",      (0,1), (-1,-1), "Helvetica"),
    ("TEXTCOLOR",     (0,1), (-1,-1), colors.HexColor("#c0d0e8")),
    ("BACKGROUND",    (0,1), (-1,1),  colors.HexColor("#0d1a30")),
    ("BACKGROUND",    (0,2), (-1,2),  PANEL),
    ("BACKGROUND",    (0,3), (-1,3),  colors.HexColor("#0d1a30")),
    ("ALIGN",         (0,0), (-1,-1), "LEFT"),
    ("VALIGN",        (0,0), (-1,-1), "MIDDLE"),
    ("TOPPADDING",    (0,0), (-1,-1), 6),
    ("BOTTOMPADDING", (0,0), (-1,-1), 6),
    ("LEFTPADDING",   (0,0), (-1,-1), 8),
    ("GRID",          (0,0), (-1,-1), 0.4, colors.HexColor("#1a2540")),
]))
story.append(ms_table)
story.append(Spacer(1, 0.4*cm))

# ── ISSUES TABELLE ────────────────────────────────────
story.append(HRFlowable(width="100%", thickness=0.5, color=DIMTEXT, spaceAfter=8))
story.append(Paragraph("Alle 19 Issues — Vollständige Übersicht", sSection))

issues = [
    # Nr, Titel, Kategorie, Prio, Milestone, Kurzbeschreibung
    ("#1",  "Smart Contract Implementation",      "Blockchain",  "HIGH",   "v2.1", "ATC-001, ATC-8300 burn/pause/snapshot, BaseContract Basisklasse"),
    ("#2",  "Gemini AI Integration",              "AI",          "HIGH",   "v2.1", "Gemini 2.0 Flash API · System-Prompt · SSE Streaming · Token-Logger"),
    ("#3",  "Shivamon Battle UI",                 "Frontend",    "HIGH",   "v2.1", "Animierter Kampf · HP-Bars · Damage Numbers · XP-Animation"),
    ("#4",  "NFT Persistenz (SQLite)",            "Backend",     "HIGH",   "v2.1", "SQLite-Schema · Wallet/Shivamon/TX/Block Repos · Migration Script"),
    ("#5",  "ATC Blockchain Explorer",            "Frontend",    "MEDIUM", "v2.1", "Block/TX/Adress-Browser · Globale Suche · 10s Auto-Refresh"),
    ("#6",  "ECDSA Signatur",                     "Security",    "HIGH",   "v2.1", "secp256k1 · sign/verify · Nonce-System · Gateway Middleware"),
    ("#7",  "Build System EXE/AppImage",          "DevOps",      "MEDIUM", "v2.2", "PyInstaller · Launcher · GitHub Actions CI/CD · Win + Linux"),
    ("#8",  "Multi-Node Testnet",                 "Blockchain",  "HIGH",   "v2.2", "Übergeordnetes Issue für alle 6 Testnet Sub-Issues (#14–#19)"),
    ("#9",  "Governance Contract (ATC-9900)",     "Blockchain",  "MEDIUM", "v2.2", "DAO Voting · Quorum 10% · Proposal · Delegated Voting"),
    ("#10", "Cross-Chain Bridge",                 "Blockchain",  "LOW",    "v3.0", "Lock/Mint · Multi-Sig Relayer · ETH/Polygon · Timelock Security"),
    ("#11", "Shivamon Breeding Gen 2",            "Game",        "MEDIUM", "v2.2", "DNA-Mix · Stat-Vererbung ±10% · 24h Cooldown · 25 ATC Kosten"),
    ("#12", "Solidity On-Chain Contracts",        "Blockchain",  "MEDIUM", "v2.2", "ATCToken.sol · Shivamon.sol · Governance.sol · Hardhat Tests"),
    ("#13", "ATC Marketplace",                    "Game",        "MEDIUM", "v2.2", "List/Buy/Cancel · 2.5% Fee · Floor Price · Offer-System"),
    ("#14", "[Testnet] Bootstrap Node",           "Networking",  "HIGH",   "v2.2", "UDP Discovery · Peer-Registry · Handshake · Peer-Persistenz"),
    ("#15", "[Testnet] Block Propagation",        "Networking",  "HIGH",   "v2.2", "TCP P2P · broadcast_block() · Duplikat-Filter · Orphan-Pool"),
    ("#16", "[Testnet] Initial Sync",             "Networking",  "HIGH",   "v2.2", "Batch-Download (50 Blöcke) · Checkpoint-Validierung · Fortschritt"),
    ("#17", "[Testnet] Longest-Chain-Rule",       "Networking",  "HIGH",   "v2.2", "Fork-Erkennung · Chain-Reorg · TX-Revert in Mempool"),
    ("#18", "[Testnet] Docker Compose",           "DevOps",      "MEDIUM", "v2.2", "5 Nodes lokal · Healthcheck · Faucet-Script · docker-compose up"),
    ("#19", "[Testnet] Node-Monitoring",          "Frontend",    "MEDIUM", "v2.2", "Live-Dashboard · Node-Cards · 5s Auto-Refresh · Netzwerk-Stats"),
]

# Farben pro Priorität
prio_colors = {
    "HIGH":   RED,
    "MEDIUM": ORANGE,
    "LOW":    GREEN,
}
cat_colors = {
    "Blockchain": PURPLE,
    "AI":         CYAN,
    "Frontend":   colors.HexColor("#00bfff"),
    "Backend":    colors.HexColor("#7b68ee"),
    "Security":   PINK,
    "Game":       colors.HexColor("#ff6b35"),
    "Networking": colors.HexColor("#20c997"),
    "DevOps":     colors.HexColor("#ffd700"),
}

header_row = [
    Paragraph("<b>#</b>", S("h", fontSize=8, textColor=colors.white, fontName="Helvetica-Bold", alignment=TA_CENTER)),
    Paragraph("<b>Issue</b>", S("h", fontSize=8, textColor=colors.white, fontName="Helvetica-Bold")),
    Paragraph("<b>Kategorie</b>", S("h", fontSize=8, textColor=colors.white, fontName="Helvetica-Bold", alignment=TA_CENTER)),
    Paragraph("<b>Prio</b>", S("h", fontSize=8, textColor=colors.white, fontName="Helvetica-Bold", alignment=TA_CENTER)),
    Paragraph("<b>MS</b>", S("h", fontSize=8, textColor=colors.white, fontName="Helvetica-Bold", alignment=TA_CENTER)),
    Paragraph("<b>Technische Beschreibung</b>", S("h", fontSize=8, textColor=colors.white, fontName="Helvetica-Bold")),
]

rows = [header_row]
for idx, (nr, titel, kat, prio, ms, desc) in enumerate(issues):
    bg = colors.HexColor("#0d1528") if idx % 2 == 0 else colors.HexColor("#080e1e")
    pc = prio_colors.get(prio, ORANGE)
    kc = cat_colors.get(kat, DIMTEXT)
    rows.append([
        Paragraph(f"<b>{nr}</b>", S(f"nr{idx}", fontSize=8, textColor=CYAN, fontName="Helvetica-Bold", alignment=TA_CENTER)),
        Paragraph(titel, S(f"t{idx}", fontSize=8, textColor=WHITE, fontName="Helvetica-Bold")),
        Paragraph(kat, S(f"k{idx}", fontSize=7.5, textColor=kc, fontName="Helvetica", alignment=TA_CENTER)),
        Paragraph(prio, S(f"p{idx}", fontSize=7.5, textColor=pc, fontName="Helvetica-Bold", alignment=TA_CENTER)),
        Paragraph(ms, S(f"m{idx}", fontSize=7.5, textColor=DIMTEXT, fontName="Helvetica", alignment=TA_CENTER)),
        Paragraph(desc, S(f"d{idx}", fontSize=7.5, textColor=colors.HexColor("#8aa0c0"), fontName="Helvetica", leading=10)),
    ])

col_w = [1.0*cm, 4.2*cm, 2.0*cm, 1.3*cm, 1.1*cm, 7.6*cm]
issues_table = Table(rows, colWidths=col_w, repeatRows=1)

ts = TableStyle([
    ("BACKGROUND",    (0,0), (-1,0),  colors.HexColor("#1a0a3a")),
    ("ROWBACKGROUNDS",(0,1), (-1,-1), [colors.HexColor("#0d1528"), colors.HexColor("#080e1e")]),
    ("ALIGN",         (0,0), (-1,-1), "LEFT"),
    ("VALIGN",        (0,0), (-1,-1), "MIDDLE"),
    ("TOPPADDING",    (0,0), (-1,-1), 5),
    ("BOTTOMPADDING", (0,0), (-1,-1), 5),
    ("LEFTPADDING",   (0,0), (-1,-1), 5),
    ("GRID",          (0,0), (-1,-1), 0.3, colors.HexColor("#1a2540")),
    ("LINEBELOW",     (0,0), (-1,0),  1.0, PURPLE),
])
issues_table.setStyle(ts)
story.append(issues_table)
story.append(Spacer(1, 0.5*cm))

# ── EMPFOHLENE REIHENFOLGE ────────────────────────────
story.append(HRFlowable(width="100%", thickness=0.5, color=DIMTEXT, spaceAfter=8))
story.append(Paragraph("Empfohlene Implementierungsreihenfolge", sSection))

reihenfolge = [
    ("1.", "#6 ECDSA Signatur",          "HIGH", "Sicherheitsfundament — Blocker für alle TX"),
    ("2.", "#4 SQLite Persistenz",       "HIGH", "Datenstabilität — Blocker für Production"),
    ("3.", "#1 Smart Contracts",         "HIGH", "ATC-8300 vollständig + ATC-001 Genesis"),
    ("4.", "#2 Gemini AI",               "HIGH", "KI-Gehirn des OS — Streaming Chat"),
    ("5.", "#3 Battle UI",               "HIGH", "Frontend für fertigen Battle-Contract"),
    ("6.", "#5 Blockchain Explorer",     "MED",  "Transparenz-Tool für Chain & TXs"),
    ("7.", "#14 Bootstrap Node",         "HIGH", "Testnet-Fundament — P2P Discovery"),
    ("8.", "#15 Block Propagation",      "HIGH", "Blöcke ans Netzwerk senden"),
    ("9.", "#16 Initial Sync",           "HIGH", "Neue Nodes synchronisieren"),
    ("10.","#17 Longest-Chain-Rule",     "HIGH", "Fork-Auflösung"),
    ("11.","#18 Docker Compose",         "MED",  "5 Nodes lokal via docker-compose up"),
    ("12.","#19 Node-Monitoring",        "MED",  "Live-Dashboard aller Nodes"),
    ("13.","#9 Governance DAO",          "MED",  "ATC-9900 Voting"),
    ("14.","#11 Shivamon Breeding",      "MED",  "Gen 2 NFTs durch Züchtung"),
    ("15.","#12 Solidity Contracts",     "MED",  "On-Chain EVM-Version"),
    ("16.","#13 Marketplace",            "MED",  "NFT kaufen & verkaufen"),
    ("17.","#7 Build System",            "MED",  "EXE / AppImage Installer"),
    ("18.","#10 Cross-Chain Bridge",     "LOW",  "ATC ↔ Ethereum / Polygon"),
]

rf_header = [
    Paragraph("<b>Schritt</b>", S("rfh", fontSize=8, textColor=colors.white, fontName="Helvetica-Bold", alignment=TA_CENTER)),
    Paragraph("<b>Issue</b>", S("rfh2", fontSize=8, textColor=colors.white, fontName="Helvetica-Bold")),
    Paragraph("<b>Prio</b>", S("rfh3", fontSize=8, textColor=colors.white, fontName="Helvetica-Bold", alignment=TA_CENTER)),
    Paragraph("<b>Begründung</b>", S("rfh4", fontSize=8, textColor=colors.white, fontName="Helvetica-Bold")),
]
rf_rows = [rf_header]
for step, issue, prio, grund in reihenfolge:
    pc = RED if prio == "HIGH" else (ORANGE if prio == "MED" else GREEN)
    rf_rows.append([
        Paragraph(step, S(f"rs{step}", fontSize=8, textColor=DIMTEXT, fontName="Helvetica", alignment=TA_CENTER)),
        Paragraph(issue, S(f"ri{step}", fontSize=8, textColor=CYAN, fontName="Helvetica-Bold")),
        Paragraph(prio, S(f"rp{step}", fontSize=7.5, textColor=pc, fontName="Helvetica-Bold", alignment=TA_CENTER)),
        Paragraph(grund, S(f"rg{step}", fontSize=7.5, textColor=colors.HexColor("#8aa0c0"), fontName="Helvetica")),
    ])

rf_table = Table(rf_rows, colWidths=[1.5*cm, 4.5*cm, 1.3*cm, 9.9*cm], repeatRows=1)
rf_table.setStyle(TableStyle([
    ("BACKGROUND",    (0,0), (-1,0),  colors.HexColor("#1a0a3a")),
    ("ROWBACKGROUNDS",(0,1), (-1,-1), [colors.HexColor("#0d1528"), colors.HexColor("#080e1e")]),
    ("ALIGN",         (0,0), (-1,-1), "LEFT"),
    ("VALIGN",        (0,0), (-1,-1), "MIDDLE"),
    ("TOPPADDING",    (0,0), (-1,-1), 5),
    ("BOTTOMPADDING", (0,0), (-1,-1), 5),
    ("LEFTPADDING",   (0,0), (-1,-1), 5),
    ("GRID",          (0,0), (-1,-1), 0.3, colors.HexColor("#1a2540")),
    ("LINEBELOW",     (0,0), (-1,0),  1.0, PURPLE),
]))
story.append(rf_table)
story.append(Spacer(1, 0.5*cm))

# ── FOOTER ────────────────────────────────────────────
story.append(HRFlowable(width="100%", thickness=0.5, color=PURPLE, spaceAfter=6))
story.append(Paragraph(
    "A-TownChain OS · ShivaCoreDev × Aurora AI · v2.0.0 Genesis Release · Mai 2026  |  "
    "github.com/ShivaCoreDev/a-townchain-os",
    sFooter
))

doc.build(story)
print("✅ PDF erstellt:", doc.filename)
