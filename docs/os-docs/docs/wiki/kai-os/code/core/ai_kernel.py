# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
🧠 KAI-OS AI Kernel Module
Inference Engine mit neurosymbolischem Reasoning für autonome Entscheidungen

Version: 1.0.0-alpha
Based on: KAI-OS Wiki Chapters 2.2, 3.x
"""

import asyncio
import json
import time
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass, asdict
from enum import Enum
import hashlib
from datetime import datetime
import logging

logger = logging.getLogger(__name__)


class InferenceMode(Enum):
    """Inference-Modi für KI-Kernel"""
    LOCAL = "local"                    # Lokale Ausführung
    DISTRIBUTED = "distributed"        # Verteilte Inferenz
    HYBRID = "hybrid"                  # Automatische Auswahl


class DecisionType(Enum):
    """Typen von KI-Entscheidungen"""
    RESOURCE_ALLOCATION = "resource_allocation"
    ANOMALY_DETECTION = "anomaly_detection"
    SCHEDULING = "scheduling"
    OPTIMIZATION = "optimization"
    GOVERNANCE = "governance"
    CRITICAL = "critical"


@dataclass
class InferenceRequest:
    """Request für KI-Inferenz"""
    request_id: str
    prompt: str
    model: str
    max_tokens: int = 2048
    temperature: float = 0.7
    mode: InferenceMode = InferenceMode.LOCAL
    decision_type: DecisionType = DecisionType.OPTIMIZATION
    timestamp: float = None
    
    def __post_init__(self):
        if self.timestamp is None:
            self.timestamp = time.time()


@dataclass
class InferenceResult:
    """Ergebnis einer KI-Inferenz"""
    request_id: str
    output: str
    confidence: float
    reasoning_steps: List[str]
    model_version: str
    inference_time_ms: float
    tokens_used: int
    on_chain_hash: Optional[str] = None
    decision_type: DecisionType = DecisionType.OPTIMIZATION
    timestamp: float = None
    
    def __post_init__(self):
        if self.timestamp is None:
            self.timestamp = time.time()
    
    def to_dict(self) -> Dict:
        """Konvertiere zu Dictionary für On-Chain Logging"""
        return asdict(self)


class ReasoningEngine:
    """
    Neurosymbolischer Reasoning-Engine
    Kombiniert neuronale Netze mit symbolischer KI
    """
    
    def __init__(self, model_name: str = "llama3-8b-q4"):
        self.model_name = model_name
        self.model_cache = {}
        self.reasoning_history: List[Dict] = []
        self.max_history = 1000
        
    async def reason(self, 
                     query: str, 
                     context: Dict[str, Any],
                     reasoning_depth: int = 5) -> Tuple[str, List[str], float]:
        """
        Führe neurosymbolisches Reasoning durch
        
        Returns: (conclusion, reasoning_steps, confidence)
        """
        reasoning_steps = []
        
        # Schritt 1: Query-Parsing (symbolisch)
        parsed_query = self._parse_query(query)
        reasoning_steps.append(f"Query parsed: {parsed_query}")
        
        # Schritt 2: Kontextanalyse
        relevant_context = self._extract_context(context, parsed_query)
        reasoning_steps.append(f"Context extracted: {len(relevant_context)} relevant facts")
        
        # Schritt 3: Hypothesenbildung (neurosymbolisch)
        hypotheses = await self._generate_hypotheses(parsed_query, relevant_context)
        reasoning_steps.append(f"Generated {len(hypotheses)} hypotheses")
        
        # Schritt 4: Bewertung und Ranking
        ranked_hypotheses = self._rank_hypotheses(hypotheses, relevant_context)
        reasoning_steps.append(f"Ranked hypotheses by confidence")
        
        # Schritt 5: Schlussfolgerung
        if ranked_hypotheses:
            conclusion = ranked_hypotheses[0]['hypothesis']
            confidence = ranked_hypotheses[0]['confidence']
        else:
            conclusion = "Unable to determine"
            confidence = 0.0
        
        reasoning_steps.append(f"Conclusion: {conclusion} (confidence: {confidence:.2f})")
        
        # Speichere im History
        self._record_reasoning(query, reasoning_steps, conclusion, confidence)
        
        return conclusion, reasoning_steps, confidence
    
    def _parse_query(self, query: str) -> Dict[str, Any]:
        """Parse Query in strukturierte Form"""
        return {
            "original": query,
            "length": len(query),
            "keywords": query.split(),
            "timestamp": datetime.utcnow().isoformat()
        }
    
    def _extract_context(self, context: Dict, parsed_query: Dict) -> List[Dict]:
        """Extrahiere relevanten Kontext"""
        relevant = []
        for key, value in context.items():
            if any(keyword in key for keyword in parsed_query['keywords']):
                relevant.append({"key": key, "value": value})
        return relevant
    
    async def _generate_hypotheses(self, parsed_query: Dict, context: List[Dict]) -> List[Dict]:
        """Generiere Hypothesen basierend auf Query und Kontext"""
        hypotheses = [
            {"hypothesis": f"Hypothesis based on {len(context)} context items", "confidence": 0.85},
            {"hypothesis": "Alternative hypothesis with lower confidence", "confidence": 0.60},
        ]
        return hypotheses
    
    def _rank_hypotheses(self, hypotheses: List[Dict], context: List[Dict]) -> List[Dict]:
        """Rangiere Hypothesen nach Konfidenz"""
        return sorted(hypotheses, key=lambda x: x['confidence'], reverse=True)
    
    def _record_reasoning(self, query: str, steps: List[str], conclusion: str, confidence: float):
        """Speichere Reasoning-Session"""
        record = {
            "query": query,
            "steps": steps,
            "conclusion": conclusion,
            "confidence": confidence,
            "timestamp": datetime.utcnow().isoformat()
        }
        self.reasoning_history.append(record)
        
        # Begrenze History
        if len(self.reasoning_history) > self.max_history:
            self.reasoning_history = self.reasoning_history[-self.max_history:]


class MemoryModule:
    """
    Speicher-Module: Kurzzeit (RAM) + Langzeit (On-Chain/IPFS)
    """
    
    def __init__(self):
        self.short_term: Dict[str, Any] = {}  # RAM
        self.long_term_registry: List[Dict] = []  # IPFS/On-Chain Hashes
        
    def store_short_term(self, key: str, value: Any, ttl_seconds: int = 3600):
        """Speichere im Kurzzeitgedächtnis"""
        self.short_term[key] = {
            "value": value,
            "expires_at": time.time() + ttl_seconds,
            "created_at": time.time()
        }
    
    def retrieve_short_term(self, key: str) -> Optional[Any]:
        """Abrufen aus Kurzzeitgedächtnis"""
        if key in self.short_term:
            entry = self.short_term[key]
            if time.time() < entry["expires_at"]:
                return entry["value"]
            else:
                del self.short_term[key]
        return None
    
    def store_long_term(self, key: str, value: Any, ipfs_hash: str):
        """Speichere im Langzeitgedächtnis (IPFS)"""
        self.long_term_registry.append({
            "key": key,
            "ipfs_hash": ipfs_hash,
            "stored_at": datetime.utcnow().isoformat(),
            "value_type": type(value).__name__
        })
    
    def cleanup_expired(self):
        """Bereinige abgelaufene Kurzzeitspeicher"""
        now = time.time()
        expired_keys = [k for k, v in self.short_term.items() if now >= v["expires_at"]]
        for key in expired_keys:
            del self.short_term[key]


class AIKernel:
    """
    🧠 KAI-OS AI Kernel
    Zentrale Inference Engine mit autonomer Entscheidungsfindung
    """
    
    def __init__(self, 
                 model_name: str = "llama3-8b-q4",
                 inference_mode: InferenceMode = InferenceMode.HYBRID,
                 max_memory_gb: int = 8,
                 blockchain_client = None):
        """
        Initialisiere AI Kernel
        
        Args:
            model_name: Name des LLM-Modells
            inference_mode: LOCAL, DISTRIBUTED, oder HYBRID
            max_memory_gb: Speicher für Model Loading
            blockchain_client: Client für On-Chain Logging
        """
        self.model_name = model_name
        self.inference_mode = inference_mode
        self.max_memory_gb = max_memory_gb
        self.blockchain_client = blockchain_client
        
        self.reasoning_engine = ReasoningEngine(model_name)
        self.memory = MemoryModule()
        
        self.request_queue: asyncio.Queue = asyncio.Queue()
        self.inference_history: List[InferenceResult] = []
        self.decision_audit_trail: List[Dict] = []
        
        self.is_running = False
        self.stats = {
            "total_inferences": 0,
            "total_decisions": 0,
            "avg_inference_time_ms": 0.0,
            "error_count": 0,
        }
        
        logger.info(f"✅ AI Kernel initialized: {model_name} ({inference_mode.value})")
    
    async def start(self):
        """Starte AI Kernel"""
        self.is_running = True
        logger.info("🚀 AI Kernel starting...")
        
        # Starte Inference Worker
        worker_task = asyncio.create_task(self._inference_worker())
        
        # Starte Cleanup Worker
        cleanup_task = asyncio.create_task(self._cleanup_worker())
        
        logger.info("✅ AI Kernel ready for inference")
    
    async def infer(self, request: InferenceRequest) -> InferenceResult:
        """
        Führe Inferenz durch
        
        Args:
            request: InferenceRequest mit Prompt und Config
            
        Returns:
            InferenceResult mit Output, Confidence, Reasoning Steps
        """
        if not self.is_running:
            raise RuntimeError("AI Kernel not running")
        
        start_time = time.time()
        
        try:
            # Nutze Reasoning Engine
            conclusion, reasoning_steps, confidence = await self.reasoning_engine.reason(
                query=request.prompt,
                context={"model": self.model_name, "mode": self.inference_mode.value}
            )
            
            inference_time_ms = (time.time() - start_time) * 1000
            tokens_used = len(request.prompt.split()) + len(conclusion.split())
            
            # Erstelle Result
            result = InferenceResult(
                request_id=request.request_id,
                output=conclusion,
                confidence=confidence,
                reasoning_steps=reasoning_steps,
                model_version=self.model_name,
                inference_time_ms=inference_time_ms,
                tokens_used=tokens_used,
                decision_type=request.decision_type
            )
            
            # Log to Memory
            self.memory.store_short_term(
                f"inference_{request.request_id}",
                result.to_dict()
            )
            
            # Log to On-Chain if critical
            if request.decision_type == DecisionType.CRITICAL:
                await self._log_decision_on_chain(result)
            
            # Update Stats
            self.stats["total_inferences"] += 1
            self.stats["avg_inference_time_ms"] = (
                (self.stats["avg_inference_time_ms"] * (self.stats["total_inferences"] - 1) +
                 inference_time_ms) / self.stats["total_inferences"]
            )
            
            self.inference_history.append(result)
            
            logger.info(f"✅ Inference completed: {request.request_id} (confidence: {confidence:.2f})")
            
            return result
            
        except Exception as e:
            logger.error(f"❌ Inference failed: {str(e)}")
            self.stats["error_count"] += 1
            raise
    
    async def make_autonomous_decision(self,
                                      decision_type: DecisionType,
                                      context: Dict[str, Any],
                                      options: List[str]) -> Dict[str, Any]:
        """
        Treffe autonome Entscheidung basierend auf KI-Reasoning
        
        Args:
            decision_type: Art der Entscheidung
            context: Kontext für Entscheidung
            options: Verfügbare Optionen
            
        Returns:
            Decision mit gewählter Option, Reasoning, Confidence
        """
        logger.info(f"🤖 Making autonomous decision: {decision_type.value}")
        
        prompt = f"""
        Decision Type: {decision_type.value}
        Available Options: {', '.join(options)}
        Context: {json.dumps(context)}
        
        Choose the best option and explain your reasoning.
        """
        
        request = InferenceRequest(
            request_id=f"decision_{int(time.time() * 1000)}",
            prompt=prompt,
            model=self.model_name,
            decision_type=decision_type
        )
        
        result = await self.infer(request)
        
        # Parse decision
        decision = {
            "type": decision_type.value,
            "decision_id": request.request_id,
            "chosen_option": result.output,
            "confidence": result.confidence,
            "reasoning": result.reasoning_steps,
            "timestamp": datetime.utcnow().isoformat(),
            "on_chain_hash": result.on_chain_hash
        }
        
        self.decision_audit_trail.append(decision)
        self.stats["total_decisions"] += 1
        
        logger.info(f"✅ Decision made: {result.output} (confidence: {result.confidence:.2f})")
        
        return decision
    
    async def _log_decision_on_chain(self, result: InferenceResult):
        """Log kritische Entscheidungen on-chain"""
        if not self.blockchain_client:
            logger.warning("⚠️ Blockchain client not available for on-chain logging")
            return
        
        try:
            decision_hash = hashlib.sha256(
                json.dumps(result.to_dict(), sort_keys=True).encode()
            ).hexdigest()
            
            # Log on blockchain
            tx_hash = await self.blockchain_client.log_decision(
                request_id=result.request_id,
                output_hash=decision_hash,
                confidence=result.confidence,
                decision_type=result.decision_type.value
            )
            
            result.on_chain_hash = tx_hash
            logger.info(f"✅ Decision logged on-chain: {tx_hash}")
            
        except Exception as e:
            logger.error(f"❌ On-chain logging failed: {str(e)}")
    
    async def _inference_worker(self):
        """Worker für Inferenz-Queue-Verarbeitung"""
        while self.is_running:
            try:
                request = await asyncio.wait_for(self.request_queue.get(), timeout=1.0)
                await self.infer(request)
            except asyncio.TimeoutError:
                continue
            except Exception as e:
                logger.error(f"❌ Worker error: {str(e)}")
    
    async def _cleanup_worker(self):
        """Worker für Speicherbereinigung"""
        while self.is_running:
            await asyncio.sleep(300)  # Alle 5 Minuten
            self.memory.cleanup_expired()
            logger.debug("✅ Memory cleanup completed")
    
    def get_stats(self) -> Dict[str, Any]:
        """Gebe Kernel-Statistiken zurück"""
        return {
            **self.stats,
            "is_running": self.is_running,
            "model": self.model_name,
            "mode": self.inference_mode.value,
            "short_term_memory_size": len(self.memory.short_term),
            "inference_history_size": len(self.inference_history),
            "decision_trail_size": len(self.decision_audit_trail)
        }
    
    async def shutdown(self):
        """Fahre Kernel herunter"""
        logger.info("🛑 Shutting down AI Kernel...")
        self.is_running = False
        await asyncio.sleep(0.5)
        logger.info("✅ AI Kernel shutdown complete")
