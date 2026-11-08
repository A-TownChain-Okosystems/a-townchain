import React, { useEffect, useRef, useState } from 'react';
import * as d3 from 'd3';
import { Shield } from 'lucide-react';
import * as ed from '@noble/ed25519';
import { CryptoEngine, toHex } from '../lib/CryptoEngine';

interface NodeData extends d3.SimulationNodeDatum {
  id: string;
  label: string;
  type: string;
  color: string;
  x: number;
  y: number;
}

interface LinkData extends d3.SimulationLinkDatum<NodeData> {
  source: string | NodeData;
  target: string | NodeData;
  label: string;
}

export function CryptoVisualizationView() {
  const d3Container = useRef<SVGSVGElement | null>(null);
  const [keys, setKeys] = useState<{privateStr: string, publicStr: string, privateBytes: Uint8Array, publicBytes: Uint8Array} | null>(null);
  const [message, setMessage] = useState("Hello A-TownChain");
  const [signature, setSignature] = useState<string | null>(null);
  const [verifyResult, setVerifyResult] = useState<boolean | null>(null);

  useEffect(() => {
    if (!d3Container.current) return;

    // Clear any existing content
    d3.select(d3Container.current).selectAll('*').remove();

    const w = 840;
    const h = 400;

    const svg = d3.select(d3Container.current)
      .attr('viewBox', `0 0 ${w} ${h}`)
      .attr('preserveAspectRatio', 'xMidYMid meet')
      .style('width', '100%')
      .style('height', '100%');

    // Define Arrow Marker
    svg.append("defs").append("marker")
      .attr("id", "arrow")
      .attr("viewBox", "0 -5 10 10")
      .attr("refX", 62) // 55 for rect half-width + some padding
      .attr("refY", 0)
      .attr("markerWidth", 6)
      .attr("markerHeight", 6)
      .attr("orient", "auto")
      .append("path")
      .attr("d", "M0,-5L10,0L0,5")
      .attr("fill", "#64748b"); // slate-500

    // Define Filter for Glow
    svg.append("defs").append("filter")
        .attr("id", "blur")
        .append("feGaussianBlur")
        .attr("stdDeviation", "5");

    const nodesData: NodeData[] = [
      { id: "Message", x: 100, y: 100, type: "data", color: "#3b82f6", label: "Message" },
      { id: "PrivateKey", x: 100, y: 280, type: "key", color: "#8b5cf6", label: "Private Key" },
      { id: "Sign", x: 260, y: 190, type: "process", color: "#f59e0b", label: "Ed25519 Sign" },
      { id: "Signature", x: 420, y: 100, type: "data", color: "#ec4899", label: "Signature" },
      { id: "PublicKey", x: 420, y: 280, type: "key", color: "#10b981", label: "Public Key" },
      { id: "Verify", x: 600, y: 190, type: "process", color: "#f59e0b", label: "Ed25519 Verify" },
      { id: "Result", x: 760, y: 190, type: "result", color: "#22c55e", label: "Valid ✅" }
    ];

    const linksData: LinkData[] = [
      { source: "Message", target: "Sign", label: "" },
      { source: "PrivateKey", target: "Sign", label: "" },
      { source: "Sign", target: "Signature", label: "" },
      { source: "PrivateKey", target: "PublicKey", label: "Derives" },
      { source: "Signature", target: "Verify", label: "" },
      { source: "PublicKey", target: "Verify", label: "" },
      { source: "Message", target: "Verify", label: "" },
      { source: "Verify", target: "Result", label: "" }
    ];

    const nodeById = new Map<string, NodeData>(nodesData.map(d => [d.id, d]));
    
    const links = svg.append("g")
      .selectAll("g")
      .data(linksData)
      .enter()
      .append("g");

    links.append("path")
      .attr("id", d => `path-${d.source}-${d.target}`)
      .attr("d", (d) => {
        const source = nodeById.get(d.source as string)!;
        const target = nodeById.get(d.target as string)!;
        
        if (d.source === "Message" && d.target === "Verify") {
            const dx = target.x - source.x;
            const dy = target.y - source.y;
            const dr = Math.sqrt(dx * dx + dy * dy) * 1.5;
            return `M${source.x},${source.y} A${dr},${dr} 0 0,1 ${target.x},${target.y}`;
        }
        
        return `M${source.x},${source.y} L${target.x},${target.y}`;
      })
      .attr("fill", "none")
      .attr("stroke", "#475569")
      .attr("stroke-width", 2)
      .attr("stroke-dasharray", (d) => d.label === "Derives" ? "5,5" : "none")
      .attr("marker-end", "url(#arrow)");

    links.filter(d => Boolean(d.label)).append("text")
      .attr("x", (d) => {
        const source = nodeById.get(d.source as string)!;
        const target = nodeById.get(d.target as string)!;
        return (source.x + target.x) / 2;
      })
      .attr("y", (d) => {
        const source = nodeById.get(d.source as string)!;
        const target = nodeById.get(d.target as string)!;
        return (source.y + target.y) / 2 - 8;
      })
      .attr("text-anchor", "middle")
      .style("fill", "#94a3b8")
      .style("font-size", "12px")
      .style("font-family", "monospace")
      .text(d => d.label);

    const nodes = svg.append("g")
      .selectAll("g")
      .data(nodesData)
      .enter()
      .append("g")
      .attr("id", d => `node-${d.id}`)
      .attr("transform", d => `translate(${d.x},${d.y})`);

    nodes.append("rect")
      .attr("x", -55)
      .attr("y", -22)
      .attr("width", 110)
      .attr("height", 44)
      .attr("rx", d => d.type === "process" ? 22 : 8)
      .attr("fill", d => `${d.color}22`)
      .attr("stroke", d => d.color)
      .attr("stroke-width", 2);

    nodes.append("text")
      .attr("text-anchor", "middle")
      .attr("alignment-baseline", "middle")
      .style("fill", "#f8fafc")
      .style("font-size", "12px")
      .style("font-weight", "500")
      .style("font-family", "sans-serif")
      .text(d => d.label);
      
    const glowNodes = nodes.filter(d => d.type === 'process' || d.type === 'result');
    glowNodes.append('circle')
        .attr('r', 35)
        .attr('fill', d => d.color)
        .attr('opacity', 0)
        .attr('filter', 'url(#blur)')
        .transition()
        .duration(2000)
        .ease(d3.easeLinear)
        .style('opacity', 0.2)
        .transition()
        .duration(2000)
        .ease(d3.easeLinear)
        .style('opacity', 0)
        .on('end', function animate() {
            d3.select(this)
                .transition()
                .duration(2000)
                .style('opacity', 0.2)
                .transition()
                .duration(2000)
                .style('opacity', 0)
                .on('end', animate);
        });

  }, []);

  const animatePacket = (sourceId: string, targetId: string, color: string, duration = 1000, delay = 0, payloadData?: string) => {
    if (!d3Container.current) return;
    const svg = d3.select(d3Container.current);
    const path = svg.select(`#path-${sourceId}-${targetId}`).node() as SVGPathElement;
    if (!path) return;
    
    const length = path.getTotalLength();
    
    const packetGroup = svg.append("g")
      .attr("opacity", 0);
      
    packetGroup.append("circle")
      .attr("r", 6)
      .attr("fill", color)
      .attr("filter", "url(#blur)");
      
    packetGroup.append("circle")
      .attr("r", 4)
      .attr("fill", "#fff");

    if (payloadData) {
      packetGroup.append("rect")
        .attr("x", -20)
        .attr("y", -25)
        .attr("width", 40)
        .attr("height", 14)
        .attr("rx", 2)
        .attr("fill", "#0f172a")
        .attr("stroke", color)
        .attr("stroke-width", 1);
        
      packetGroup.append("text")
        .attr("y", -14)
        .attr("text-anchor", "middle")
        .style("fill", "#e2e8f0")
        .style("font-size", "8px")
        .style("font-family", "monospace")
        .text(payloadData.substring(0, 6) + "..");
    }
      
    packetGroup.transition()
      .delay(delay)
      .duration(200)
      .attr("opacity", 1)
      .transition()
      .duration(duration)
      .ease(d3.easeCubicInOut)
      .attrTween("transform", function() {
        return function(t) {
          const point = path.getPointAtLength(t * length);
          return `translate(${point.x},${point.y})`;
        }
      })
      .transition()
      .duration(200)
      .attr("opacity", 0)
      .remove();
      
    // Pulse node effect
    setTimeout(() => {
        const node = svg.select(`#node-${targetId} rect`);
        if (node.node()) {
            const originalFill = node.attr("fill");
            node.transition()
                .duration(150)
                .attr("fill", color)
                .transition()
                .duration(300)
                .attr("fill", originalFill);
        }
    }, delay + duration);
  };

  const generateKeys = async () => {
    setKeys(null);
    setSignature(null);
    setVerifyResult(null);
    
    // Generate key pair via CryptoEngine
    const keyPair = await CryptoEngine.generate_keypair();
    
    setTimeout(async () => {
        setKeys({ 
            privateStr: keyPair.privateHex, 
            publicStr: keyPair.publicHex,
            privateBytes: keyPair.privateKey,
            publicBytes: keyPair.publicKey 
        });
        animatePacket("PrivateKey", "PublicKey", "#10b981", 1000, 0, keyPair.publicHex);

        // Encrypt and store locally via webcrypto
        try {
            const secret = "ATC_LOCAL_ENCRYPTION_PASS";
            const enc = new TextEncoder();
            const keyMaterial = await window.crypto.subtle.importKey(
              "raw", enc.encode(secret), { name: "PBKDF2" }, false, ["deriveBits", "deriveKey"]
            );
            const salt = window.crypto.getRandomValues(new Uint8Array(16));
            const cryptoKey = await window.crypto.subtle.deriveKey(
              { name: "PBKDF2", salt, iterations: 100000, hash: "SHA-256" },
              keyMaterial, { name: "AES-GCM", length: 256 }, false, ["encrypt"]
            );
            const iv = window.crypto.getRandomValues(new Uint8Array(12));
            const dataToEncrypt = enc.encode(JSON.stringify({
                privateKey: keyPair.privateHex,
                publicKey: keyPair.publicHex,
                timestamp: new Date().toISOString()
            }));

            const encryptedBuffer = await window.crypto.subtle.encrypt(
                { name: "AES-GCM", iv }, cryptoKey, dataToEncrypt
            );

            localStorage.setItem("atc_encrypted_keypair", JSON.stringify({
                salt: toHex(salt),
                iv: toHex(iv),
                data: toHex(new Uint8Array(encryptedBuffer))
            }));
        } catch (e) {
            console.error("Local storage encryption failed", e);
        }
    }, 100);
  };

  const signMessage = async () => {
    if (!keys) return;
    setSignature(null);
    setVerifyResult(null);

    animatePacket("Message", "Sign", "#3b82f6", 1000, 0, message);
    animatePacket("PrivateKey", "Sign", "#8b5cf6", 1000, 200, keys.privateStr);

    const result = await CryptoEngine.sign_message(message, keys.privateBytes);

    setTimeout(() => {
        setSignature(result.signatureHex);
        animatePacket("Sign", "Signature", "#ec4899", 800, 0, result.signatureHex);
    }, 1200);
  };

  const verifySignature = async () => {
    if (!keys || !signature) return;
    setVerifyResult(null);

    // Reset visual feedback
    if (d3Container.current) {
        const svg = d3.select(d3Container.current);
        svg.select("#node-Verify rect").attr("stroke", "#f59e0b").attr("fill", "#f59e0b22");
    }

    animatePacket("Message", "Verify", "#3b82f6", 1200, 0, message);
    animatePacket("PublicKey", "Verify", "#10b981", 1200, 100, keys.publicStr);
    animatePacket("Signature", "Verify", "#ec4899", 1200, 200, signature);

    const isValid = await CryptoEngine.verify_message(signature, message, keys.publicBytes);

    setTimeout(() => {
        setVerifyResult(isValid);
        if (isValid) {
            animatePacket("Verify", "Result", "#22c55e", 800, 0, "Valid");
            
            // Visual feedback on the D3 view
            if (d3Container.current) {
               const svg = d3.select(d3Container.current);
               
               // Success pulse for Result node
               svg.select("#node-Result rect")
                  .transition().duration(400).attr("fill", "#22c55e88")
                  .transition().duration(600).attr("fill", "#22c55e22");
                  
               // Make Verify node green to indicate success
               svg.select("#node-Verify rect")
                  .transition().duration(400).attr("stroke", "#22c55e").attr("fill", "#22c55e33");
            }
        } else {
            if (d3Container.current) {
               const svg = d3.select(d3Container.current);
               // Make Verify node red to indicate failure
               svg.select("#node-Verify rect")
                  .transition().duration(400).attr("stroke", "#ef4444").attr("fill", "#ef444433");
            }
        }
    }, 1500);
  };

  return (
    <div className="flex flex-col h-full bg-slate-950 border border-white/5 rounded-xl overflow-hidden">
      <div className="flex items-center gap-3 px-6 py-4 border-b border-white/5 bg-slate-900/50">
        <Shield className="w-5 h-5 text-indigo-400" />
        <div>
          <h2 className="font-bold text-white text-sm">Ed25519 Cryptography Engine</h2>
          <p className="text-xs text-slate-400 mt-0.5">Real-time visualization of the digital signature pipeline</p>
        </div>
      </div>
      <div className="flex-1 flex overflow-hidden">
          <div className="flex-1 p-6 relative flex items-center justify-center">
            <div className="absolute top-4 left-4 flex flex-col gap-2 z-10">
               <div className="flex items-center gap-2 text-xs text-slate-400 font-mono">
                   <div className="w-3 h-3 rounded-full border border-blue-500 bg-blue-500/20" /> Raw Data
               </div>
               <div className="flex items-center gap-2 text-xs text-slate-400 font-mono">
                   <div className="w-3 h-3 rounded-full border border-purple-500 bg-purple-500/20" /> Extracted Keys
               </div>
               <div className="flex items-center gap-2 text-xs text-slate-400 font-mono">
                   <div className="w-3 h-3 rounded-full border border-amber-500 bg-amber-500/20" /> Execution Enviroment
               </div>
            </div>
            <svg ref={d3Container} className="w-full h-full max-h-[500px]" />
          </div>
          
          <div className="w-80 bg-slate-900/50 border-l border-white/5 p-5 flex flex-col gap-6 overflow-y-auto custom-scrollbar">
            <div>
              <h3 className="font-bold text-white text-xs uppercase tracking-wider mb-3">1. Key Generation</h3>
              <button 
                onClick={generateKeys} 
                className="w-full py-2 bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 rounded font-mono text-xs hover:bg-emerald-500/20 transition-colors"
              >
                Generate Ed25519 Pair
              </button>
              
              {keys && (
                  <div className="mt-3 space-y-3">
                      <div>
                         <div className="text-[10px] text-purple-400 font-mono mb-1">Private Key (32 bytes)</div>
                         <div className="font-mono text-[10px] text-slate-300 break-all bg-[#050B14] p-2 rounded border border-white/5 leading-relaxed">
                             {keys.privateStr}
                         </div>
                      </div>
                      <div>
                         <div className="text-[10px] text-emerald-400 font-mono mb-1">Public Key (32 bytes)</div>
                         <div className="font-mono text-[10px] text-slate-300 break-all bg-[#050B14] p-2 rounded border border-white/5 leading-relaxed">
                             {keys.publicStr}
                         </div>
                      </div>
                  </div>
              )}
            </div>

            <div className={`transition-opacity duration-500 ${!keys ? 'opacity-30 pointer-events-none' : 'opacity-100'}`}>
              <h3 className="font-bold text-white text-xs uppercase tracking-wider mb-3">2. Transaction Signing</h3>
              
              <div className="mb-3">
                  <label className="text-[10px] text-slate-400 font-mono mb-1 block">Payload Message</label>
                  <input 
                      type="text" 
                      value={message} 
                      onChange={(e) => setMessage(e.target.value)}
                      className="w-full bg-[#050B14] border border-white/10 rounded px-3 py-2 text-xs text-blue-300 font-mono outline-none focus:border-blue-500/50"
                  />
              </div>

              <button 
                onClick={signMessage} 
                className="w-full py-2 bg-amber-500/10 text-amber-400 border border-amber-500/20 rounded font-mono text-xs hover:bg-amber-500/20 transition-colors"
              >
                Sign with Private Key
              </button>

              {signature && (
                  <div className="mt-3">
                      <div className="text-[10px] text-pink-400 font-mono mb-1">Signature output (64 bytes)</div>
                      <div className="font-mono text-[10px] text-slate-300 break-all bg-[#050B14] p-2 rounded border border-white/5 leading-relaxed">
                          {signature}
                      </div>
                  </div>
              )}
            </div>

            <div className={`transition-opacity duration-500 ${!signature ? 'opacity-30 pointer-events-none' : 'opacity-100'}`}>
              <h3 className="font-bold text-white text-xs uppercase tracking-wider mb-3">3. Verification</h3>
              
              <button 
                onClick={verifySignature} 
                className="w-full py-2 bg-blue-500/10 text-blue-400 border border-blue-500/20 rounded font-mono text-xs hover:bg-blue-500/20 transition-colors mb-3"
              >
                Verify Signature
              </button>

              {verifyResult !== null && (
                  <div className={`text-center py-2 rounded border font-mono text-xs ${verifyResult ? 'bg-green-500/10 text-green-400 border-green-500/20' : 'bg-red-500/10 text-red-400 border-red-500/20'}`}>
                      {verifyResult ? 'Signature Verified ✅' : 'Invalid Signature ❌'}
                  </div>
              )}
            </div>
          </div>
      </div>
    </div>
  );
}

