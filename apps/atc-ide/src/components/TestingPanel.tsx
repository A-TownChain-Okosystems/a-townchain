import React, { useState, useMemo } from "react";
import { CheckCircle2, XCircle, Play, Beaker, TerminalSquare, Zap, Check } from "lucide-react";
import { FileState } from "../App";
import { Lexer } from "../interpreter/lexer";
import { Parser } from "../interpreter/parser";
import { Evaluator, Environment } from "../interpreter/evaluator";

type TestResult = {
  id: string;
  name: string;
  status: "pass" | "fail" | "error" | "pending";
  expected?: string;
  actual?: string;
  error?: string;
};

interface TestingPanelProps {
  files: FileState[];
  onOpenCiCdGenerator?: () => void;
}

export function TestingPanel({ files, onOpenCiCdGenerator }: TestingPanelProps) {
  const hasWorkflow = useMemo(() => {
    return files.some((f) => f.name === ".github/workflows/lumino-build.yml");
  }, [files]);
  const [testCode, setTestCode] = useState(`// Write your test code here
// Define variables and expected output
let expected = 10;
let actual = 5 + 5;
print actual;`);
  
  const [results, setResults] = useState<TestResult[]>([]);
  const [isRunning, setIsRunning] = useState(false);
  const [consoleOutput, setConsoleOutput] = useState<string[]>([]);
  
  const [targetFileIndex, setTargetFileIndex] = useState(0);
  const [expectedOutput, setExpectedOutput] = useState("");

  const runTest = () => {
    setIsRunning(true);
    setConsoleOutput([]);
    setResults([]);
    
    // Create new test result
    const newResult: TestResult = {
      id: Math.random().toString(36).substring(2, 9),
      name: "Test Run",
      status: "pending"
    };
    
    setTimeout(() => {
      let combinedCode = "";
      if (files[targetFileIndex]) {
         combinedCode += files[targetFileIndex].content + "\n";
      }
      combinedCode += testCode;

      const lexer = new Lexer(combinedCode);
      const parser = new Parser(lexer);
      const program = parser.parseProgram();

      if (parser.errors.length > 0) {
        newResult.status = "error";
        newResult.error = parser.errors.join("\n");
        setConsoleOutput(parser.errors.map((err) => `Syntax Error: ${err}`));
        setResults([newResult]);
        setIsRunning(false);
        return;
      }

      const env = new Environment();
      const evaluator = new Evaluator(env);
      evaluator.eval(program);
      
      const out = evaluator.getOutput();
      setConsoleOutput(out);
      
      if (expectedOutput) {
         const lastOutput = out.length > 0 ? out[out.length - 1] : "";
         if (lastOutput === expectedOutput) {
            newResult.status = "pass";
            newResult.expected = expectedOutput;
            newResult.actual = lastOutput;
            out.push("✅ Test PASSED");
         } else {
            newResult.status = "fail";
            newResult.expected = expectedOutput;
            newResult.actual = lastOutput;
            out.push(`❌ Test FAILED. Expected: ${expectedOutput}, Actual: ${lastOutput}`);
         }
      } else {
         newResult.status = "pass"; // If no expected output, just passing syntax is enough
         out.push("✅ Test PASSED (No return expectation)");
      }
      
      setConsoleOutput([...out]);
      
      setResults([newResult]);
      setIsRunning(false);
    }, 100);
  };

  return (
    <div className="flex-1 flex flex-col bg-[#0f0f13] text-slate-300 relative z-10 overflow-hidden shadow-2xl">
      <div className="h-10 bg-[#15151a] border-b border-white/5 flex items-center justify-between px-4 shrink-0">
        <div className="flex items-center">
          <Beaker className="w-5 h-5 text-indigo-400 mr-2" />
          <span className="font-bold text-sm tracking-wide text-slate-200">UNIT TESTING</span>
        </div>
        {onOpenCiCdGenerator && (
          <button
            onClick={onOpenCiCdGenerator}
            className="flex items-center gap-1.5 px-3 py-1 bg-gradient-to-r from-indigo-500/20 to-purple-500/20 hover:from-indigo-500/30 hover:to-purple-500/30 text-indigo-300 border border-indigo-500/40 rounded-lg text-xs font-semibold transition-colors cursor-pointer"
            title="GitHub Actions CI/CD für automatisierte Unit Tests konfigurieren"
          >
            <Zap className="w-3.5 h-3.5 text-indigo-400" />
            <span>CI/CD Workflow (.github/workflows/lumino-build.yml)</span>
            {hasWorkflow && (
              <span className="ml-1 px-1.5 py-0.2 text-[10px] bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 rounded">
                Aktiv
              </span>
            )}
          </button>
        )}
      </div>

      <div className="flex-1 flex overflow-hidden">
        {/* Test Editor */}
        <div className="w-1/2 border-r border-white/5 flex flex-col bg-black/20 p-4 shrink-0 gap-4">
           <div>
              <h3 className="text-xs font-bold text-slate-400 mb-2 uppercase tracking-wide">Target File</h3>
              <select 
                value={targetFileIndex} 
                onChange={(e) => setTargetFileIndex(Number(e.target.value))}
                className="w-full bg-black/40 border border-white/10 rounded px-3 py-2 text-sm outline-none focus:border-indigo-500/50 mb-4 text-slate-200 appearance-none"
              >
                {files.map((f, i) => (
                  <option key={i} value={i}>{f.name}</option>
                ))}
              </select>

              <h3 className="text-xs font-bold text-slate-400 mb-2 uppercase tracking-wide">Test Code</h3>
              <textarea
                value={testCode}
                onChange={(e) => setTestCode(e.target.value)}
                placeholder="Write your test script here..."
                className="w-full bg-black/40 border border-white/10 rounded px-3 py-2 text-sm font-mono outline-none focus:border-indigo-500/50 resize-none h-40 mb-4 text-slate-300"
              />

              <h3 className="text-xs font-bold text-slate-400 mb-2 uppercase tracking-wide">Expected Output (Optional)</h3>
              <input
                type="text"
                value={expectedOutput}
                onChange={(e) => setExpectedOutput(e.target.value)}
                placeholder="e.g. 10"
                className="w-full bg-black/40 border border-white/10 rounded px-3 py-2 text-sm outline-none focus:border-indigo-500/50 mb-4 text-slate-300"
              />

              <button
                onClick={runTest}
                disabled={isRunning}
                className="w-full bg-indigo-500/20 hover:bg-indigo-500/30 text-indigo-400 disabled:opacity-50 disabled:hover:bg-indigo-500/20 border border-indigo-500/30 py-2 rounded text-sm font-bold flex items-center justify-center transition-colors"
               >
                 <Play className="w-4 h-4 mr-2" />
                 {isRunning ? "Running..." : "Run Test"}
              </button>
           </div>
        </div>

        {/* Test Results */}
        <div className="w-1/2 flex flex-col bg-black/10">
           {/* Results Header */}
           <div className="p-4 border-b border-white/5 shrink-0">
             <h3 className="text-xs font-bold text-slate-400 mb-4 uppercase tracking-wide flex items-center">
               <CheckCircle2 className="w-4 h-4 mr-2" />
               Test Results
             </h3>
             <div className="space-y-3">
               {results.length === 0 ? (
                 <div className="text-slate-500 text-sm italic py-4">No tests run yet</div>
               ) : (
                 results.map((result) => (
                   <div key={result.id} className="bg-white/[0.02] border border-white/5 rounded-lg p-3">
                      <div className="flex items-center gap-2 mb-2">
                         {result.status === "pass" ? (
                           <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                         ) : result.status === "fail" ? (
                           <XCircle className="w-4 h-4 text-rose-400" />
                         ) : (
                           <XCircle className="w-4 h-4 text-orange-400" />
                         )}
                         <span className="font-bold text-sm text-slate-200">{result.name}</span>
                         <span className={`text-xs px-2 py-0.5 rounded font-bold uppercase ${
                           result.status === "pass" ? "bg-emerald-500/20 text-emerald-400" :
                           result.status === "fail" ? "bg-rose-500/20 text-rose-400" :
                           "bg-orange-500/20 text-orange-400"
                         }`}>
                           {result.status}
                         </span>
                      </div>
                      
                      {result.status === "fail" && (
                         <div className="mt-2 text-sm bg-black/40 p-2 rounded border border-white/5">
                            <div className="text-rose-400"><span className="text-slate-500">Expected:</span> {result.expected}</div>
                            <div className="text-emerald-400"><span className="text-slate-500">Actual:</span> {result.actual}</div>
                         </div>
                      )}
                      
                      {result.status === "error" && (
                         <div className="mt-2 text-sm bg-rose-500/10 text-rose-400 p-2 rounded border border-rose-500/20 whitespace-pre-wrap">
                            {result.error}
                         </div>
                      )}
                   </div>
                 ))
               )}
             </div>
           </div>
           
           {/* Console Output */}
           <div className="flex-1 p-4 overflow-y-auto bg-[#0a0a0c]">
             <h3 className="text-xs font-bold text-slate-500 mb-4 uppercase tracking-wide flex items-center">
               <TerminalSquare className="w-4 h-4 mr-2" />
               Console Output
             </h3>
             <div className="font-mono text-[11px] leading-relaxed space-y-1">
               {consoleOutput.length === 0 ? (
                 <div className="text-slate-600 italic">No console output...</div>
               ) : (
                 consoleOutput.map((line, i) => (
                   <div key={i} className="text-slate-300 whitespace-pre-wrap">{line}</div>
                 ))
               )}
             </div>
           </div>
        </div>
      </div>
    </div>
  );
}
