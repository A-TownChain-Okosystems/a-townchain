import React, { useState, useEffect, useRef } from 'react';
import { Play, RotateCcw, Trophy } from 'lucide-react';

const GRID_SIZE = 20;
const CELL_SIZE = 15;
const INITAL_SPEED = 150;

type Point = { x: number; y: number };

export function SnakeGame() {
  const [snake, setSnake] = useState<Point[]>([{ x: 10, y: 10 }]);
  const [food, setFood] = useState<Point>({ x: 15, y: 15 });
  const [dir, setDir] = useState<Point>({ x: 0, y: -1 });
  const [gameOver, setGameOver] = useState(false);
  const [isPaused, setIsPaused] = useState(true);
  const [score, setScore] = useState(0);
  const [highScore, setHighScore] = useState(0);
  
  const gameRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Only capture if game is running or if they explicitly press game keys
      if (!gameRef.current?.contains(document.activeElement as Node)) {
        return;
      }
      
      if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(e.key)) {
        e.preventDefault();
      }
      switch (e.key) {
        case 'ArrowUp': if (dir.y !== 1) setDir({ x: 0, y: -1 }); break;
        case 'ArrowDown': if (dir.y !== -1) setDir({ x: 0, y: 1 }); break;
        case 'ArrowLeft': if (dir.x !== 1) setDir({ x: -1, y: 0 }); break;
        case 'ArrowRight': if (dir.x !== -1) setDir({ x: 1, y: 0 }); break;
        case ' ': 
          e.preventDefault();
          setIsPaused(p => !p); 
          break;
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [dir]);

  useEffect(() => {
    if (isPaused || gameOver) return;

    const moveSnake = () => {
      setSnake(prev => {
        const head = prev[0];
        const newHead = { x: head.x + dir.x, y: head.y + dir.y };

        if (
          newHead.x < 0 || newHead.x >= GRID_SIZE ||
          newHead.y < 0 || newHead.y >= GRID_SIZE ||
          prev.some(segment => segment.x === newHead.x && segment.y === newHead.y)
        ) {
          setGameOver(true);
          if (score > highScore) setHighScore(score);
          return prev;
        }

        const newSnake = [newHead, ...prev];
        if (newHead.x === food.x && newHead.y === food.y) {
          setScore(s => s + 10);
          setFood({
            x: Math.floor(Math.random() * GRID_SIZE),
            y: Math.floor(Math.random() * GRID_SIZE)
          });
        } else {
          newSnake.pop();
        }

        return newSnake;
      });
    };

    const speed = Math.max(50, INITAL_SPEED - Math.floor(score / 50) * 10);
    const intervalId = setInterval(moveSnake, speed);
    return () => clearInterval(intervalId);
  }, [dir, isPaused, gameOver, food, score, highScore]);

  const resetGame = () => {
    setSnake([{ x: 10, y: 10 }]);
    setDir({ x: 0, y: -1 });
    setScore(0);
    setGameOver(false);
    setIsPaused(false);
    setFood({
      x: Math.floor(Math.random() * GRID_SIZE),
      y: Math.floor(Math.random() * GRID_SIZE)
    });
    gameRef.current?.focus();
  };

  return (
    <div className="flex flex-col items-center justify-center p-4 font-sans h-full w-full bg-black/20" ref={gameRef} tabIndex={0}>
      <div className="flex items-center gap-6 mb-4">
        <div className="flex flex-col items-center">
          <span className="text-xs text-slate-500 uppercase tracking-widest font-bold">Score</span>
          <span className="text-2xl font-bold text-cyan-400">{score}</span>
        </div>
        <div className="h-8 w-px bg-white/10"></div>
        <div className="flex flex-col items-center">
          <span className="text-xs text-slate-500 uppercase tracking-widest font-bold flex items-center gap-1">
            <Trophy className="w-3 h-3" /> Best
          </span>
          <span className="text-2xl font-bold text-emerald-400">{highScore}</span>
        </div>
      </div>

      <div 
        className="relative bg-black border border-white/10 rounded overflow-hidden"
        style={{ width: GRID_SIZE * CELL_SIZE, height: GRID_SIZE * CELL_SIZE }}
      >
        {snake.map((segment, i) => (
          <div
            key={i}
            className="absolute rounded-sm"
            style={{
              left: segment.x * CELL_SIZE,
              top: segment.y * CELL_SIZE,
              width: CELL_SIZE,
              height: CELL_SIZE,
              backgroundColor: i === 0 ? '#22d3ee' : '#0891b2',
              border: '1px solid black'
            }}
          />
        ))}
        <div
          className="absolute bg-red-500 rounded-full"
          style={{
            left: food.x * CELL_SIZE,
            top: food.y * CELL_SIZE,
            width: CELL_SIZE,
            height: CELL_SIZE,
          }}
        />

        {(gameOver || isPaused) && (
          <div className="absolute inset-0 bg-black/60 backdrop-blur-sm flex flex-col items-center justify-center">
            {gameOver ? (
              <>
                <h3 className="text-xl font-bold text-red-400 mb-2">GAME OVER</h3>
                <button
                  onClick={resetGame}
                  className="px-4 py-2 bg-white/10 hover:bg-white/20 rounded text-sm font-bold text-white transition-colors flex items-center gap-2"
                >
                  <RotateCcw className="w-4 h-4" /> PLAY AGAIN
                </button>
              </>
            ) : (
              <button
                onClick={() => setIsPaused(false)}
                className="px-4 py-2 bg-cyan-500/20 hover:bg-cyan-500/30 text-cyan-400 border border-cyan-500/50 rounded text-sm font-bold transition-colors flex items-center gap-2"
              >
                <Play className="w-4 h-4" /> {score > 0 ? 'RESUME' : 'START GAME'}
              </button>
            )}
            {!gameOver && <div className="text-xs text-slate-500 mt-4">Use Arrow Keys to move</div>}
          </div>
        )}
      </div>
    </div>
  );
}
