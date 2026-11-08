import React, { useState } from 'react';
import { Activity, Server, Github, Play, CheckCircle, Clock } from 'lucide-react';

export function CiCdPipelineView() {
  const workflowCode = `name: ATC Node CI/CD Pipeline
on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test-and-lint:
    name: Code Qualität
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          override: true
      - name: Run Tests
        run: cargo test --verbose
      - name: Run Linter
        run: cargo fmt -- --check

  build-and-push:
    name: Container Build & Push
    needs: test-and-lint
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Docker Login
        uses: docker/login-action@v2
        with:
          username: \${{ secrets.DOCKER_USERNAME }}
          password: \${{ secrets.DOCKER_PASSWORD }}
      - name: Build and Push Docker Image
        uses: docker/build-push-action@v4
        with:
          context: ./core_nodes
          push: true
          tags: shivacoredev/atc-core-node:latest

  deploy-k8s:
    name: Kubernetes Rollout
    needs: build-and-push
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Kubectl Apply
        uses: azure/k8s-deploy@v4
        with:
          manifests: |
            ./orchestrator/deployment.yaml
          kubectl-version: 'latest'`;

  return (
    <div className="flex flex-col h-full bg-[#050B14] border border-white/5 rounded-xl overflow-hidden font-sans">
      <div className="flex items-center gap-3 px-6 py-4 border-b border-white/5 bg-slate-900/50">
        <Server className="w-5 h-5 text-indigo-400" />
        <div>
          <h2 className="text-sm font-bold text-white tracking-widest font-mono uppercase">ATC Node CI/CD Pipeline</h2>
          <p className="text-xs text-slate-400 mt-0.5">Automatisierter GitHub Actions Workflow für ShivaCore</p>
        </div>
      </div>
      
      <div className="flex-1 flex overflow-hidden">
        <div className="w-1/2 p-6 overflow-y-auto border-r border-white/5 custom-scrollbar bg-[#020408]">
          <div className="flex items-center justify-between mb-4">
            <h3 className="font-mono text-xs text-slate-300 uppercase flex items-center gap-2">
              <Github className="w-4 h-4 text-slate-400" /> main.yml
            </h3>
            <button className="px-3 py-1 bg-indigo-500/10 text-indigo-400 border border-indigo-500/30 hover:bg-indigo-500/20 rounded font-mono text-[10px] transition-colors flex items-center gap-2">
              <Play className="w-3 h-3" /> Trigger Workflow
            </button>
          </div>
          <pre className="font-mono text-[10px] text-slate-400 whitespace-pre-wrap">
            {workflowCode}
          </pre>
        </div>

        <div className="w-1/2 p-6 bg-[#030711] overflow-y-auto custom-scrollbar flex flex-col items-center justify-center">
            <h3 className="font-mono text-sm text-white mb-8">Pipeline Visualisierung</h3>
            
            <div className="flex flex-col gap-6 w-full max-w-sm relative">
                <div className="absolute left-[19px] top-4 bottom-4 w-0.5 bg-slate-800" />
                
                <PipelineStep 
                  icon={<Activity className="w-4 h-4 text-blue-400" />}
                  title="Code Qualität (test-and-lint)"
                  status="success"
                  time="2m 14s"
                  borderColor="border-blue-500/20"
                  iconBg="bg-blue-500/10"
                />
                
                <PipelineStep 
                  icon={<Server className="w-4 h-4 text-purple-400" />}
                  title="Container Build & Push"
                  status="in_progress"
                  time="45s..."
                  borderColor="border-purple-500/20"
                  iconBg="bg-purple-500/10"
                />
                
                <PipelineStep 
                  icon={<Github className="w-4 h-4 text-slate-400" />}
                  title="Kubernetes Rollout"
                  status="pending"
                  time="-"
                  borderColor="border-slate-800"
                  iconBg="bg-slate-800/50"
                  opacity="opacity-50"
                />
            </div>

            <div className="mt-12 bg-indigo-500/5 border border-indigo-500/10 rounded-xl p-6 w-full max-w-sm">
                <h4 className="text-xs font-mono font-bold text-indigo-400 uppercase mb-3 text-center">Nächste Option Wählen</h4>
                <div className="space-y-2">
                    <button className="w-full text-left px-4 py-2 rounded-lg bg-white/5 border border-white/5 hover:border-indigo-500/30 hover:bg-indigo-500/10 transition-colors group">
                        <span className="text-xs font-bold text-white block mb-0.5 group-hover:text-indigo-300">Option A: On-Chain Governance</span>
                        <span className="text-[10px] text-slate-400">Proposals und Abstimmungs-Logik in Rust</span>
                    </button>
                    <button className="w-full text-left px-4 py-2 rounded-lg bg-white/5 border border-white/5 hover:border-indigo-500/30 hover:bg-indigo-500/10 transition-colors group">
                        <span className="text-xs font-bold text-white block mb-0.5 group-hover:text-indigo-300">Option B: Zero-Knowledge Proofs</span>
                        <span className="text-[10px] text-slate-400">Datenschutzfreundliche Transaktionen via arkworks</span>
                    </button>
                    <button className="w-full text-left px-4 py-2 rounded-lg bg-white/5 border border-white/5 hover:border-indigo-500/30 hover:bg-indigo-500/10 transition-colors group">
                        <span className="text-xs font-bold text-white block mb-0.5 group-hover:text-indigo-300">Option C: Whitepaper Finalisierung</span>
                        <span className="text-[10px] text-slate-400">Ökonomisches Modell & ATC-Emissionskurve</span>
                    </button>
                </div>
            </div>
        </div>
      </div>
    </div>
  );
}

function PipelineStep({ icon, title, status, time, borderColor, iconBg, opacity = "" }: any) {
  return (
    <div className={`flex items-center gap-4 relative z-10 ${opacity}`}>
        <div className={`w-10 h-10 rounded-full flex items-center justify-center shrink-0 border ${borderColor} ${iconBg}`}>
            {status === 'success' ? <CheckCircle className="w-5 h-5 text-emerald-400" /> : icon}
        </div>
        <div className="flex-1 bg-slate-900/40 border border-white/5 p-3 rounded-lg flex items-center justify-between">
            <span className="text-xs font-mono text-slate-200">{title}</span>
            <span className="text-[10px] font-mono text-slate-500 flex items-center gap-1">
                {status === 'in_progress' && <Activity className="w-3 h-3 text-purple-400 animate-pulse" />}
                {status === 'success' && <Clock className="w-3 h-3 text-slate-400" />}
                {time}
            </span>
        </div>
    </div>
  );
}
