import React, { useState, useEffect } from 'react';
import { Shield, Lock, FileKey, Cpu, CheckCircle } from 'lucide-react';
import { motion } from 'motion/react';

interface ZkVisualizationViewProps {}

export const ZkVisualizationView: React.FC<ZkVisualizationViewProps> = () => {
  const [currentStep, setCurrentStep] = useState(0);

  const steps = [
    {
      id: 0,
      title: 'Data Preparation',
      description: 'Agent prepares the private data and local computation bounds.',
      icon: <FileKey className="w-8 h-8 text-blue-400" />
    },
    {
      id: 1,
      title: 'ZKP Generation',
      description: 'SNARK engine generates a zero-knowledge proof of the computation.',
      icon: <Cpu className="w-8 h-8 text-purple-400" />
    },
    {
      id: 2,
      title: 'Hardware Encryption',
      description: 'TEE strictly isolates the proof, encrypting data with AES-GCM-256.',
      icon: <Lock className="w-8 h-8 text-red-400" />
    },
    {
      id: 3,
      title: 'Proof Verification',
      description: 'The Blockchain PLONK Verifier cryptographically checks the proof without seeing data.',
      icon: <Shield className="w-8 h-8 text-cyan-400" />
    },
    {
      id: 4,
      title: 'Task Validated',
      description: 'The Zero-Knowledge transaction is finalized on the A-TownChain network.',
      icon: <CheckCircle className="w-8 h-8 text-green-400" />
    }
  ];

  useEffect(() => {
    const timer = setInterval(() => {
      setCurrentStep((prev) => (prev + 1) % steps.length);
    }, 3000);
    return () => clearInterval(timer);
  }, [steps.length]);

  return (
    <div className="w-full h-full bg-slate-900 border border-slate-800 rounded-lg p-6 overflow-y-auto">
      <div className="flex items-center gap-3 mb-6">
        <Shield className="w-6 h-6 text-atc-cyan" />
        <h2 className="text-xl font-mono text-white tracking-widest uppercase">Zero-Knowledge Proof Pipeline</h2>
      </div>

      <div className="mb-4 text-slate-400 text-sm">
        Visualizing the hardware-level encryption and zk-SNARK execution over the Security Layer.
      </div>

      <div className="flex flex-col gap-6 mt-8 relative">
        <div className="absolute left-8 top-0 bottom-0 w-0.5 bg-slate-800 z-0"></div>

        {steps.map((step, index) => {
          const isActive = index === currentStep;
          const isPast = index < currentStep;

          return (
            <motion.div 
              key={step.id} 
              className={`flex items-start gap-6 z-10 transition-opacity duration-500 ${isActive ? 'opacity-100' : 'opacity-40'}`}
              animate={{ scale: isActive ? 1.05 : 1 }}
            >
              <div className={`w-16 h-16 rounded-full flex items-center justify-center border-4 ${isActive ? 'bg-slate-800 border-atc-cyan shadow-[0_0_15px_rgba(34,211,238,0.5)]' : isPast ? 'bg-slate-800 border-green-500 text-green-500' : 'bg-slate-900 border-slate-700'}`}>
                {step.icon}
              </div>
              <div className="flex-1 pt-3">
                <h3 className={`text-lg font-bold font-mono ${isActive ? 'text-white' : 'text-slate-400'}`}>
                  {step.title}
                </h3>
                <p className="text-sm text-slate-500 mt-1">
                  {step.description}
                </p>
                {isActive && (
                  <motion.div 
                    initial={{ width: 0 }} 
                    animate={{ width: '100%' }} 
                    transition={{ duration: 3, ease: 'linear' }}
                    className="h-0.5 bg-atc-cyan mt-3"
                  />
                )}
              </div>
            </motion.div>
          );
        })}
      </div>
    </div>
  );
};
