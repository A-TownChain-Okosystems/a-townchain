# ATC-06 — Inter-Node Latency Optimization & Routing

> **Standard-ID:** ATC-06
> **Status:** FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.2
> **Sprint:** 2.2 — P2P + Testnet
> **Issue:** #83
> **Task:** T-007
> **Wiki:** Kap. 37 (P2P-Netzwerk) + Kap. 67 (Technische Referenz)
> **Autor:** Aurora (MasterBrain · Base44)
> **Stand:** 05.07.2026 | Version 1.0.0
> **Kategorie:** P2P Networking  
> **Tier:** Tier 1 — Blockchain Infrastructure  

---

## 1. Überblick

ATC-06 definiert das Latency-Optimierungs- und Routing-System für das A-TownChain P2P-Netzwerk. Es sorgt dafür, dass Nachrichten (Transaktionen, Blöcke, Konsens-Votes) über die Pfade mit der geringsten Latenz propagiert werden, anstatt zufällig verteilt zu werden.

### Problemstellung

In einem Blockchain-P2P-Netzwerk ohne Latency-Optimierung:
- Transaktionen benötigen unnötig lange zur Propagation (hohe Stakeholder-Latenz)
- Blöcke erreichen Validatoren zu unterschiedlichen Zeitpunkten → Forks
- Schlechte Peers (hohe Latenz, Packet Loss) blockieren das Netzwerk
- Keine Priorisierung von kritischen Nachrichten (Konsens-Votes > Transaktionen)

### Lösung

ATC-06 implementiert ein **Latency-Aware Routing** mit folgenden Komponenten:
1. **RTT-Monitoring** — Kontinuierliche Latency-Messung zu allen Peers
2. **Peer-Ranking** — Peers werden nach Latency, Uptime und Reliability bewertet
3. **Optimal-Path-Selection** — Dijkstra-basierte Pfadfindung im Peer-Graph
4. **Message-Priorization** — Konsens-Votes haben höhere Priorität als Transaktionen
5. **Adaptive Fanout** — Anzahl der Weiterleitungs-Peers abhängig von Netzwerkauslastung

### Architektur

```
modules/network/
├── routing.atc          # Hauptrouting-Logik (ATC-06)
├── rtt_monitor.atc      # Round-Trip-Time Messung
├── peer_ranking.atc     # Peer-Bewertung und -Ranking
├── path_selector.atc    # Dijkstra/A* Pfadfindung
└── message_priority.atc # Nachrichten-Priorisierung
```

---

## 2. RTT-Monitoring (`rtt_monitor.atc`)

### 2.1 Datenstrukturen

```atclang
struct RTTSample {
    peer_id: PeerId,
    timestamp: u64,       // Unix ms
    rtt_ms: u32,          // Round-Trip-Time in Millisekunden
    success: bool,        // Ping erfolgreich?
}

struct PeerMetrics {
    peer_id: PeerId,
    samples: Queue<RTTSample>,     // Letzte 100 Samples (Ring-Buffer)
    ewma_rtt: f64,                  // Exponentially Weighted Moving Average
    min_rtt: u32,                   // Beste gemessene RTT
    max_rtt: u32,                   // Schlechteste gemessene RTT
    success_rate: f64,              // 0.0–1.0
    last_ping: u64,                 // Timestamp des letzten Pings
    consecutive_failures: u32,      // Aufeinanderfolgende Fehlschläge
}

struct RTTMonitor {
    peers: Map<PeerId, PeerMetrics>,
    ping_interval_ms: u32,          // Default: 5000 (5 Sekunden)
    timeout_ms: u32,                // Default: 2000 (2 Sekunden)
    sample_window: u32,             // Default: 100 Samples
    ewma_alpha: f64,                // Default: 0.3 (Gewichtung neuer Samples)
}
```

### 2.2 Ping/Pong-Protokoll

```atclang
// Ping-Nachricht an Peer senden
fn ping(peer: PeerId) -> void {
    let timestamp = now();
    let msg = PingMessage { timestamp, nonce: random_u64() };
    net_send(peer, encode_ping(msg));
    // Timer für Timeout starten
    schedule_timeout(peer, msg.nonce, timeout_ms);
}

// Pong-Antwort empfangen und RTT berechnen
fn handle_pong(peer: PeerId, msg: PongMessage) -> void {
    let rtt = now() - msg.original_timestamp;
    let sample = RTTSample {
        peer_id: peer,
        timestamp: now(),
        rtt_ms: rtt,
        success: true,
    };
    rtt_monitor.record_sample(peer, sample);
}

// Timeout-Handler — Peer hat nicht reagiert
fn handle_timeout(peer: PeerId, nonce: u64) -> void {
    let sample = RTTSample {
        peer_id: peer,
        timestamp: now(),
        rtt_ms: timeout_ms,
        success: false,
    };
    rtt_monitor.record_sample(peer, sample);
    rtt_monitor.peers[peer].consecutive_failures += 1;
    
    // Nach 5 Fehlschlägen: Peer als "unhealthy" markieren
    if rtt_monitor.peers[peer].consecutive_failures >= 5 {
        peer_ranking.mark_unhealthy(peer);
    }
}
```

### 2.3 EWMA-Berechnung

```atclang
// Exponentially Weighted Moving Average — reagiert schnell auf Änderungen
fn update_ewma(peer: PeerId, new_rtt: u32) -> void {
    let metrics = &mut rtt_monitor.peers[peer];
    if metrics.ewma_rtt == 0.0 {
        metrics.ewma_rtt = f64::from(new_rtt);
    } else {
        metrics.ewma_rtt = ewma_alpha * f64::from(new_rtt) 
                         + (1.0 - ewma_alpha) * metrics.ewma_rtt;
    }
}

// Gas-Cost: 20 (Update + Map-Lookup)
```

### 2.4 Monitoring-Loop

```atclang
// Wird alle ping_interval_ms aufgerufen
fn monitoring_tick() -> void {
    for peer in rtt_monitor.peers.keys() {
        if now() - rtt_monitor.peers[peer].last_ping >= ping_interval_ms {
            ping(peer);
            rtt_monitor.peers[peer].last_ping = now();
        }
    }
}
```

---

## 3. Peer-Ranking (`peer_ranking.atc`)

### 3.1 Ranking-Algorithmus

```atclang
struct PeerScore {
    peer_id: PeerId,
    latency_score: f64,     // 0.0–1.0 (1.0 = beste Latency)
    uptime_score: f64,      // 0.0–1.0 (1.0 = 100% Uptime)
    reliability_score: f64, // 0.0–1.0 (1.0 = nie fehlgeschlagen)
    bandwidth_score: f64,   // 0.0–1.0 (1.0 = hohe Bandbreite)
    total_score: f64,       // Gewichteter Gesamtscore
    rank: u32,              // Position im Ranking (1 = bester)
    is_healthy: bool,
    is_blacklisted: bool,
}
```

### 3.2 Score-Berechnung

```atclang
// Gewichtete Summe der Einzel-Scores
fn calculate_score(metrics: &PeerMetrics) -> PeerScore {
    // Latency-Score: niedrigere RTT = höherer Score
    // Formel: 1.0 - (ewma_rtt / max_acceptable_rtt)
    // max_acceptable_rtt = 1000 ms (konfigurierbar)
    let latency_score = 1.0 - clamp(
        metrics.ewma_rtt / 1000.0, 0.0, 1.0
    );
    
    // Uptime-Score: basiert auf success_rate
    let uptime_score = metrics.success_rate;
    
    // Reliability-Score: abnehmend mit consecutive_failures
    let reliability_score = 1.0 - clamp(
        f64::from(metrics.consecutive_failures) / 10.0, 0.0, 1.0
    );
    
    // Bandwidth-Score: aus letzten Übertragungen geschätzt
    // (Bytes/sec der letzten 10 Nachrichten)
    let bandwidth_score = estimate_bandwidth(metrics.peer_id);
    
    // Gewichtung: Latency 40%, Uptime 25%, Reliability 25%, Bandwidth 10%
    let total = 0.40 * latency_score 
              + 0.25 * uptime_score 
              + 0.25 * reliability_score 
              + 0.10 * bandwidth_score;
    
    return PeerScore {
        peer_id: metrics.peer_id,
        latency_score,
        uptime_score,
        reliability_score,
        bandwidth_score,
        total_score: total,
        rank: 0,  // wird in update_rankings() gesetzt
        is_healthy: total > 0.3 && metrics.consecutive_failures < 5,
        is_blacklisted: false,
    };
}
```

### 3.3 Ranking-Update

```atclang
// Wird nach jeder RTT-Monitoring-Runde aufgerufen
fn update_rankings() -> void {
    // 1. Scores für alle Peers berechnen
    let scores: Array<PeerScore> = array_new();
    for peer in rtt_monitor.peers.keys() {
        let score = calculate_score(&rtt_monitor.peers[peer]);
        array_push(scores, score);
    }
    
    // 2. Nach total_score absteigend sortieren
    array_sort(scores, |a, b| {
        if a.total_score > b.total_score { Ordering::Less }
        else { Ordering::Greater }
    });
    
    // 3. Ranks zuweisen
    for i in 0..scores.len() {
        scores[i].rank = i + 1;
        peer_ranking.scores[scores[i].peer_id] = scores[i];
    }
    
    // 4. Blacklist überprüfen
    for score in scores {
        if score.total_score < 0.1 && !score.is_blacklisted {
            blacklist_peer(score.peer_id, reason: "Score < 0.1");
        }
    }
}
```

### 3.4 Peer-Blacklisting

```atclang
fn blacklist_peer(peer: PeerId, reason: String) -> void {
    peer_ranking.scores[peer].is_blacklisted = true;
    peer_ranking.scores[peer].is_healthy = false;
    net_disconnect(peer);
    emit_event("PeerBlacklisted", encode_blacklist_event(peer, reason));
    
    // Auto-Unblacklist nach 5 Minuten (Rehabilitation)
    schedule_unblacklist(peer, 300_000); // 5 min in ms
}

fn unblacklist_peer(peer: PeerId) -> void {
    if peer_ranking.scores[peer].is_blacklisted {
        peer_ranking.scores[peer].is_blacklisted = false;
        peer_ranking.scores[peer].consecutive_failures = 0;
        net_connect(peer_address(peer));
        emit_event("PeerRehabilitated", encode_rehab_event(peer));
    }
}
```

---

## 4. Optimal-Path-Selection (`path_selector.atc`)

### 4.1 Peer-Graph

```atclang
struct PeerGraph {
    nodes: Map<PeerId, PeerNode>,
    edges: Map<(PeerId, PeerId), EdgeWeight>,
}

struct PeerNode {
    peer_id: PeerId,
    address: String,
    is_self: bool,       // Dieser Node
    is_validator: bool,
    is_bootstrap: bool,
}

struct EdgeWeight {
    latency_ms: u32,     // EWMA RTT
    bandwidth_bps: u64,  // Bytes/sec
    reliability: f64,    // 0.0–1.0
    weight: f64,         // Berechnetes Gewicht für Dijkstra
}
```

### 4.2 Gewichtung

```atclang
// Edge-Weight: Kombination aus Latency und Reliability
// Niedrigeres Gewicht = besserer Pfad
fn calculate_edge_weight(edge: &EdgeWeight) -> f64 {
    // Basis: Latency in ms
    let base = f64::from(edge.latency_ms);
    
    // Reliability-Multiplikator: unzuverlässige Pfade werden "teurer"
    let reliability_multiplier = 1.0 + (1.0 - edge.reliability) * 2.0;
    
    // Bandwidth-Bonus: hohe Bandbreite reduziert Gewicht leicht
    let bandwidth_factor = 1.0 - clamp(
        f64::from(edge.bandwidth_bps) / 10_000_000.0, // 10 MB/s Referenz
        0.0, 0.2 // Max 20% Reduktion
    );
    
    return base * reliability_multiplier * bandwidth_factor;
}
```

### 4.3 Dijkstra-Algorithmus

```atclang
// Findet den Pfad mit dem geringsten Gewicht von source zu target
fn find_optimal_path(
    graph: &PeerGraph, 
    source: PeerId, 
    target: PeerId
) -> Option<Array<PeerId>> {
    // Distanzen initialisieren
    let mut distances: Map<PeerId, f64> = map_new();
    let mut previous: Map<PeerId, Option<PeerId>> = map_new();
    let mut visited: Set<PeerId> = set_new();
    let mut queue: PriorityQueue<(f64, PeerId)> = pq_new();
    
    // Alle Distanzen auf unendlich, Source auf 0
    for node in graph.nodes.keys() {
        map_insert(distances, node, f64::MAX);
    }
    map_insert(distances, source, 0.0);
    pq_push(queue, (0.0, source));
    
    // Dijkstra-Hauptschleife
    while let Some((dist, current)) = pq_pop(queue) {
        if set_contains(visited, &current) { continue; }
        set_insert(visited, current);
        
        // Ziel erreicht
        if current == target {
            return reconstruct_path(previous, source, target);
        }
        
        // Nachbarn relaxieren
        for (neighbor, edge) in get_neighbors(graph, current) {
            if set_contains(visited, &neighbor) { continue; }
            if is_blacklisted(neighbor) { continue; }
            
            let edge_weight = calculate_edge_weight(edge);
            let new_dist = dist + edge_weight;
            
            if new_dist < distances[neighbor] {
                map_insert(distances, neighbor, new_dist);
                map_insert(previous, neighbor, Some(current));
                pq_push(queue, (new_dist, neighbor));
            }
        }
    }
    
    return None; // Kein Pfad gefunden
}

// Pfad aus previous-Map rekonstruieren
fn reconstruct_path(
    previous: Map<PeerId, Option<PeerId>>,
    source: PeerId,
    target: PeerId
) -> Option<Array<PeerId>> {
    let mut path = array_new();
    let mut current = target;
    
    while current != source {
        array_push(path, current);
        match map_get(previous, &current) {
            Some(prev) => current = *prev,
            None => return None, // Pfad unvollständig
        }
    }
    array_push(path, source);
    array_reverse(path);
    return Some(path);
}
```

### 4.4 A* für große Netzwerke

```atclang
// Für Netzwerke > 50 Nodes: A* mit Heuristik
// Heuristik: Geografische Distanz (IP-Geolocation)
fn a_star_heuristic(a: PeerId, b: PeerId) -> f64 {
    // Geschätzte Latency basierend auf geografischer Distanz
    let geo_a = get_geolocation(a);
    let geo_b = get_geolocation(b);
    let distance_km = haversine(geo_a, geo_b);
    // Daumenregel: ~1ms pro 200km (fiber optic)
    return distance_km / 200.0;
}
```

---

## 5. Message-Priorization (`message_priority.atc`)

### 5.1 Prioritäts-Level

```atclang
enum MessagePriority {
    Critical,    // Konsens-Votes, Fork-Resolution — sofort
    High,        // Block-Announcements — nächste Tick
    Normal,      // Transaktionen — queued
    Low,         // State-Sync, Peer-Discovery — Hintergrund
}
```

### 5.2 Prioritäts-Queue

```atclang
struct PriorityMessageQueue {
    critical: Queue<(PeerId, Bytes)>,
    high: Queue<(PeerId, Bytes)>,
    normal: Queue<(PeerId, Bytes)>,
    low: Queue<(PeerId, Bytes)>,
    max_critical_per_tick: u32,   // Default: 100
    max_high_per_tick: u32,       // Default: 50
    max_normal_per_tick: u32,     // Default: 30
    max_low_per_tick: u32,        // Default: 10
}

// Nachricht einreihen
fn enqueue(peer: PeerId, msg: Bytes, priority: MessagePriority) -> void {
    match priority {
        Critical => queue_enqueue(pq.critical, (peer, msg)),
        High => queue_enqueue(pq.high, (peer, msg)),
        Normal => queue_enqueue(pq.normal, (peer, msg)),
        Low => queue_enqueue(pq.low, (peer, msg)),
    }
}

// Nachrichten pro Tick verarbeiten (Strict Priority)
fn process_tick() -> void {
    // 1. Alle Critical-Nachrichten senden (kein Limit bei Konsens)
    while let Some((peer, msg)) = queue_dequeue(pq.critical) {
        send_with_optimal_path(peer, msg);
    }
    
    // 2. High-Nachrichten bis zum Limit
    let mut count = 0;
    while count < pq.max_high_per_tick {
        match queue_dequeue(pq.high) {
            Some((peer, msg)) => send_with_optimal_path(peer, msg),
            None => break,
        }
        count += 1;
    }
    
    // 3. Normal und Low analog...
}
```

### 5.3 Priority-Klassifikation

```atclang
fn classify_message(msg: &NetworkMessage) -> MessagePriority {
    match msg.type {
        MessageType::ConsensusVote => Critical,
        MessageType::ForkResolution => Critical,
        MessageType::BlockAnnounce => High,
        MessageType::BlockRequest => High,
        MessageType::Transaction => Normal,
        MessageType::StateSync => Low,
        MessageType::PeerDiscovery => Low,
        MessageType::Ping | MessageType::Pong => Low,
    }
}
```

---

## 6. Adaptive Fanout

```atclang
struct FanoutConfig {
    base_fanout: u32,          // Default: 4 (an 4 Peers weiterleiten)
    max_fanout: u32,           // Default: 8
    min_fanout: u32,           // Default: 2
    network_load_threshold: f64, // Default: 0.7 (70% Auslastung)
}

// Adaptive Fanout-Größe basierend auf Netzwerkauslastung
fn calculate_fanout(msg_priority: MessagePriority) -> u32 {
    let load = get_network_load(); // 0.0–1.0
    
    match msg_priority {
        Critical => max_fanout,    // Konsens immer max Fanout
        High => {
            if load > network_load_threshold {
                base_fanout  // Bei hoher Last: Standard
            } else {
                base_fanout + 2  // Bei niedriger Last: mehr Peers
            }
        }
        Normal => {
            if load > network_load_threshold {
                min_fanout  // Bei hoher Last: minimale Weiterleitung
            } else {
                base_fanout
            }
        }
        Low => min_fanout,  // Hintergrund-Traffic immer minimal
    }
}

// Nachricht mit optimalem Fanout weiterleiten
fn broadcast_with_fanout(msg: Bytes, priority: MessagePriority) -> void {
    let fanout = calculate_fanout(priority);
    let top_peers = peer_ranking.get_top_n_healthy(fanout);
    
    for peer in top_peers {
        send_with_optimal_path(peer, msg);
    }
}
```

---

## 7. Gas-Cost Tabelle

| Operation | Modul | Gas-Cost | Bemerkung |
|-----------|-------|----------|-----------|
| ping | rtt_monitor | 10 | Pro Ping |
| record_sample | rtt_monitor | 20 | Pro RTT-Sample |
| update_ewma | rtt_monitor | 20 | EWMA-Update |
| calculate_score | peer_ranking | 50 | Pro Peer |
| update_rankings | peer_ranking | 50 × n | Alle Peers |
| blacklist_peer | peer_ranking | 100 | + Event |
| find_optimal_path | path_selector | 200 × hops | Dijkstra |
| a_star_path | path_selector | 150 × hops | A* (mit Heuristik) |
| enqueue | message_priority | 10 | Pro Nachricht |
| process_tick | message_priority | 10 × msgs | Pro Tick |
| calculate_fanout | routing | 30 | Pro Broadcast |
| broadcast_with_fanout | routing | 30 + send × n | Fanout-Broadcast |

---

## 8. Konfiguration

```atclang
// Default-Konfiguration (node.toml)
[network.latency]
ping_interval_ms = 5000          // 5 Sekunden
ping_timeout_ms = 2000           // 2 Sekunden
max_acceptable_rtt = 1000        // 1 Sekunde
sample_window = 100              // 100 Samples pro Peer
ewma_alpha = 0.3                 // Gewichtung neuer Samples
consecutive_failure_threshold = 5  // Nach 5 Fehlschlägen: unhealthy
blacklist_threshold = 0.1        // Score < 0.1 → Blacklist
blacklist_duration_ms = 300000   // 5 Minuten Rehabilitation

[network.routing]
base_fanout = 4
max_fanout = 8
min_fanout = 2
network_load_threshold = 0.7
use_a_star_threshold = 50        // >50 Peers → A* statt Dijkstra

[network.priority]
max_critical_per_tick = 100      // Eigentlich unbegrenzt
max_high_per_tick = 50
max_normal_per_tick = 30
max_low_per_tick = 10
```

---

## 9. Context-Isolation

| Context | rtt_monitor | peer_ranking | path_selector | message_priority |
|---------|-------------|--------------|---------------|------------------|
| **Node** | ✅ Full | ✅ Full | ✅ Full | ✅ Full |
| **Smart Contract** | ❌ Panic | ❌ Panic | ❌ Panic | ❌ Panic |
| **Test** | ✅ Mock | ✅ Mock | ✅ Mock | ✅ Mock |

**Smart Contracts** haben keinen Zugriff auf das Routing-System. Die Netzwerk-Propagation ist ausschließlich Node-Context.

---

## 10. Testing-Strategie

### Unit-Tests (20+)

```
tests/network/
├── test_rtt_monitor.atc      # RTT-Messung, EWMA, Timeout (5 tests)
├── test_peer_ranking.atc     # Score-Berechnung, Ranking, Blacklist (5 tests)
├── test_path_selector.atc    # Dijkstra, A*, Pfad-Rekonstruktion (4 tests)
├── test_message_priority.atc # Priority-Queue, Klassifikation (3 tests)
└── test_fanout.atc           # Adaptive Fanout, Network-Load (3 tests)
```

### Test-Coverage-Ziele

| Modul | Coverage | Tests |
|-------|----------|-------|
| rtt_monitor.atc | 95% | 5+ |
| peer_ranking.atc | 90% | 5+ |
| path_selector.atc | 90% | 4+ |
| message_priority.atc | 85% | 3+ |
| fanout (routing.atc) | 85% | 3+ |
| **Total** | **90%+** | **20+** |

### Integration-Tests

```atclang
// test_latency_optimization.atc — Full RTT → Ranking → Path → Send cycle
fn test_full_routing_cycle() {
    // 1. Simuliere RTT-Messungen für 5 Peers
    let peers = setup_5_peers();
    for peer in peers {
        rtt_monitor.record_sample(peer, RTTSample {
            peer_id: peer, timestamp: now(), rtt_ms: 50, success: true
        });
    }
    
    // 2. Rankings aktualisieren
    peer_ranking.update_rankings();
    
    // 3. Optimalen Pfad finden
    let path = path_selector.find_optimal_path(
        graph, self_peer, peers[0]
    );
    assert(path.is_some());
    
    // 4. Nachricht mit Priority senden
    message_priority.enqueue(peers[0], b"test", Critical);
    message_priority.process_tick();
    
    // 5. Verify: Nachricht wurde über optimalen Pfad gesendet
    assert(was_sent_via_path(peers[0], path.unwrap()));
}

// test_blacklist_recovery.atc — Peer wird geblacklistet und rehabilitiert
fn test_blacklist_recovery() {
    let bad_peer = setup_peer_with_high_latency(2000); // 2s RTT
    rtt_monitor.record_sample(bad_peer, failure_sample);
    // ... 5 failures
    peer_ranking.update_rankings();
    assert(peer_ranking.is_blacklisted(bad_peer));
    
    // Nach Rehabilitation
    wait(300_000); // 5 min
    assert(!peer_ranking.is_blacklisted(bad_peer));
}

// test_priority_ordering.atc — Critical wird vor Normal gesendet
fn test_priority_ordering() {
    message_priority.enqueue(peer_a, b"normal_msg", Normal);
    message_priority.enqueue(peer_b, b"critical_msg", Critical);
    message_priority.process_tick();
    assert(sent_order[0] == "critical_msg");
    assert(sent_order[1] == "normal_msg");
}
```

---

## 11. Abhängigkeiten & Voraussetzungen

| Abhängigkeit | Issue | Beschreibung |
|--------------|-------|--------------|
| ATCLang Compiler (ATC-92) | #72 | Für Kompilierung der .atc-Module |
| ATCLang VM (ATC-93) | #73 | Für Ausführung |
| ATCLang Stdlib (ATC-94) | #81 | crypto, collections, io |
| Core Node Protocol (ATC-01) | #82 | P2P-Mesh Basis |

### Implementation-Reihenfolge

1. **rtt_monitor.atc** — Basis-Messung, hängt von io (net_send) ab
2. **peer_ranking.atc** — Hängt von rtt_monitor (PeerMetrics) ab
3. **path_selector.atc** — Hängt von peer_ranking (Scores) ab
4. **message_priority.atc** — Unabhängig, nutzt collections (Queue)
5. **routing.atc** — Hauptmodul, integriert alle anderen

---

## 12. Sprint-Zuweisung

- **Sprint 2.2** — P2P + Testnet
- **Task:** T-007
- **Issue:** #83
- **Priorität:** MEDIUM
- **Aufwandsschätzung:** 2-3 Tage
- **Deliverable:** `modules/network/routing.atc` + 4 Submodule mit 20+ Unit-Tests

---

## 13. Metriken & Observability

### Prometheus-Metriken

```
# RTT-Metriken
kai_peer_rtt_ms{peer="..."}           # EWMA RTT pro Peer
kai_peer_rtt_min_ms{peer="..."}       # Min RTT
kai_peer_rtt_max_ms{peer="..."}       # Max RTT

# Ranking-Metriken
kai_peer_score{peer="...",component="latency|uptime|reliability|bandwidth"}
kai_peer_rank{peer="..."}             # Rank im Ranking
kai_peers_healthy_total               # Anzahl gesunder Peers
kai_peers_blacklisted_total           # Anzahl geblacklisteter Peers

# Routing-Metriken
kai_routing_path_length{path="..."}   # Pfadlänge
kai_routing_path_weight{path="..."}   # Pfadgewicht
kai_messages_sent{priority="critical|high|normal|low"}
kai_fanout_current{priority="..."}    # Aktueller Fanout

# Queue-Metriken
kai_msgqueue_depth{priority="..."}    # Queue-Tiefe pro Priority
```

### Grafana-Dashboard

- **Panel 1:** Peer-RTT über Zeit (Line Chart, pro Peer)
- **Panel 2:** Peer-Ranking (Bar Chart, sortiert nach Score)
- **Panel 3:** Message-Throughput pro Priority (Stacked Area)
- **Panel 4:** Blacklisted Peers (Stat Panel)
- **Panel 5:** Fanout-Größe über Zeit (Line Chart)
- **Panel 6:** Queue-Tiefe pro Priority (Bar Chart)

---

## Referenzen

- **ATC-01** — Core Node Protocol (P2P-Mesh Basis)
- **ATC-07** — Network Sharding (State Partitioning)
- **ATC-08** — Ephemeral Data Streaming
- **ATC-92** — ATCLang Language Spec
- **ATC-93** — ATCLang VM
- **ATC-94** — ATCLang Stdlib
- **Issue #83** — GitHub Issue
- **Roadmap v2.0** — Sprint 2.2, Task T-007
- **Wiki Kap. 37** — P2P-Netzwerk Technische Details

---

*Spezifikation: Aurora (MasterBrain · Base44) · 05.07.2026 · ATC-06 v1.0.0*
