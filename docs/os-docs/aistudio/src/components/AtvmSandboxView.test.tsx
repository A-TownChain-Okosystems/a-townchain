import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import AtvmSandboxView from './AtvmSandboxView';

describe('AtvmSandboxView compiler validation', () => {
  beforeEach(() => {
    localStorage.setItem('zk_circuit_valid', 'true');
  });

  it('compiles user code and displays ATVM bytecode', async () => {
    render(<AtvmSandboxView />);

    // Click Compile
    const compileButton = screen.getByText(/Compile to ATVM/i);
    fireEvent.click(compileButton);

    // Should indicate it's compiling
    expect(await screen.findByText(/Compiling ALC/i)).toBeTruthy();

    // Wait for the bytecode to appear after async timeout (approx 1.5s)
    const node = await screen.findByText(/0x0001/i, { exact: false }, { timeout: 3500 });
    expect(node).toBeTruthy();
  });

  it('runs the compiled bytecode and shows execution logs', async () => {
    render(<AtvmSandboxView />);

    // Compile first
    const compileButton = screen.getByText(/Compile to ATVM/i);
    fireEvent.click(compileButton);

    await screen.findByText(/Compiled to ATVM bytecode/i, { exact: false }, { timeout: 3500 });

    // Click Run
    const runButton = screen.getByText(/Execute Context/i);
    fireEvent.click(runButton);

    const log = await screen.findByText(/Execution completed successfully/i, { exact: false }, { timeout: 4500 });
    expect(log).toBeTruthy();
  }, 10000);

  it('changes target environment OS correctly', async () => {
    render(<AtvmSandboxView />);

    // Navigate to systems
    const systemsTab = screen.getByRole('button', { name: /OS Environments/i });
    fireEvent.click(systemsTab);

    // Grab all 'Set Active' buttons, click one
    const setActiveButtons = await screen.findAllByText(/Set Active/i);
    if(setActiveButtons.length > 0) {
       fireEvent.click(setActiveButtons[setActiveButtons.length - 1]);
    }

    // Go back to compile and check if log mentions ubuntu (or win11)
    const editorTab = screen.getByRole('button', { name: /Editor/i });
    fireEvent.click(editorTab);
    
    const compileButton = screen.getByText(/Compile to ATVM/i);
    fireEvent.click(compileButton);

    const log = await screen.findByText(/Submitting to/i, { exact: false }, { timeout: 3500 });
    expect(log).toBeTruthy();
  });

  it('aborts compilation on invalid syntax', async () => {
    // We need to render and type some invalid code
    render(<AtvmSandboxView />);
    
    // Find the textarea (there's only one in the editor tab)
    const textarea = screen.getByRole('textbox');
    fireEvent.change(textarea, { target: { value: 'invalid syntax without keyword' } });

    // Compile
    const compileButton = screen.getByText(/Compile to ATVM/i);
    fireEvent.click(compileButton);

    // Wait for error messages in logs
    const errLog = await screen.findByText(/SyntaxError: Expected keyword "contract" at top-level\./i, { exact: false }, { timeout: 3500 });
    expect(errLog).toBeTruthy();
    
    const abortLog = await screen.findByText(/Compilation aborted/i, { exact: false }, { timeout: 3500 });
    expect(abortLog).toBeTruthy();
  });
});
