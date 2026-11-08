import React, { useState } from 'react';
import CodeMirror from '@uiw/react-codemirror';
import { javascript } from '@codemirror/lang-javascript';
import { oneDark } from '@codemirror/theme-one-dark';
import { Play, CheckCircle, Save, Variable, Boxes } from 'lucide-react';

export function ZkCircuitEditorView() {
  const [code, setCode] = useState(`// ZK Circuit Constraint Definitions
// ATC-Lang Proof Validation

circuit SpendConstraint {
    public_inputs: [root_hash, nullifier_hash];
    private_inputs: [secret_key, path, path_elements];
    
    constraint "Valid merkle branch" {
       verify_merkle(root_hash, path, path_elements, secret_key);
    }
    
    constraint "Nullifier integrity" {
       assert(nullifier_hash == pedersen_hash(secret_key, path));
    }
}
`);

  const [output, setOutput] = useState('');
  const [isCompiling, setIsCompiling] = useState(false);

  const handleCompile = () => {
    setIsCompiling(true);
    setOutput('Validating against ATC ZK protocol standards...\n');

    // Validation Layer
    let isValid = true;
    let errorMsg = '';
    
    if (!code.includes('circuit')) {
       isValid = false;
       errorMsg = 'Error: Must define at least one "circuit" block.\n';
    } else if (!code.includes('public_inputs:') || !code.includes('private_inputs:')) {
       isValid = false;
       errorMsg = 'Error: Protocol standards require both "public_inputs:" and "private_inputs:" declarations.\n';
    } else if (!code.includes('constraint')) {
       isValid = false;
       errorMsg = 'Error: No "constraint" definitions found. Empty circuits are not allowed.\n';
    }

    setTimeout(() => {
      if (!isValid) {
         setOutput(prev => prev + errorMsg + 'Validation failed. Circuit is not compliant.');
         localStorage.setItem('zk_circuit_valid', 'false');
         setIsCompiling(false);
         return;
      }
      
      setOutput(prev => prev + 'Validation passed. Generating Rank-1 Constraint System (R1CS)...\n');
      setTimeout(() => {
        setOutput(prev => prev + 'Synthesizing witness...\n');
        setTimeout(() => {
          setOutput(prev => prev + 'Groth16 setup completed.\nSUCCESS: Circuit constraints compiled.\nTotal gates: 14,204\nProving key generated.');
          localStorage.setItem('zk_circuit_valid', 'true');
          // Dispatch event in case someone is listening
          window.dispatchEvent(new Event('zk_circuit_updated'));
          setIsCompiling(false);
        }, 1000);
      }, 1000);
    }, 500);
  };

  return (
    <div className="flex flex-col h-full bg-[#050811] text-slate-200">
      <div className="flex gap-4 p-4 border-b border-white/5 bg-black/20 shrink-0">
        <h2 className="text-xl font-semibold text-white flex items-center gap-2">
          <Boxes className="w-5 h-5 text-emerald-400" />
          ZK Circuit Editor
        </h2>
      </div>
      <div className="flex-1 grid grid-cols-1 lg:grid-cols-2 gap-0 overflow-hidden">
        <div className="flex flex-col border-r border-white/5 overflow-hidden">
           <div className="p-3 bg-[#090b14] border-b border-white/5 flex items-center justify-between shrink-0">
              <span className="text-xs font-mono text-slate-400">spend_circuit.zk</span>
              <button onClick={handleCompile} disabled={isCompiling} className="flex items-center gap-1 px-3 py-1 bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 rounded text-xs font-bold hover:bg-emerald-500/30 transition-colors disabled:opacity-50">
                 <Play className="w-3 h-3" /> COMPILE CIRCUIT
              </button>
           </div>
           <div className="flex-1 overflow-auto bg-[#282c34]">
             <CodeMirror
               value={code}
               height="100%"
               autoFocus
               theme={oneDark}
               extensions={[javascript()]}
               onChange={(value) => setCode(value)}
               className="h-full text-sm font-mono"
             />
           </div>
        </div>
        <div className="flex flex-col overflow-hidden bg-[#0a0c10]">
           <div className="p-3 bg-[#090b14] border-b border-white/5 flex items-center shrink-0">
              <span className="text-xs font-mono text-slate-500">PROVER STDOUT</span>
           </div>
           <div className="flex-1 p-4 overflow-y-auto font-mono text-xs text-slate-300 whitespace-pre-wrap leading-relaxed">
             {output || <span className="text-slate-600">No output. Click COMPILE CIRCUIT to run constraint synthesis.</span>}
           </div>
        </div>
      </div>
    </div>
  );
}
