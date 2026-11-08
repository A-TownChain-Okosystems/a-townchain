import React, { useRef, useEffect, useState } from "react";
import Prism from "prismjs";

interface MiniMapProps {
  code: string;
  containerId: string;
}

export function MiniMap({ code, containerId }: MiniMapProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const [isDragging, setIsDragging] = useState(false);
  const [scrollTop, setScrollTop] = useState(0);
  const [scrollHeight, setScrollHeight] = useState(1);
  const [viewportHeight, setViewportHeight] = useState(1);

  const syncScrollStats = () => {
    const el = document.getElementById(containerId);
    if (el) {
      setScrollTop(el.scrollTop);
      setScrollHeight(el.scrollHeight || 1);
      setViewportHeight(el.clientHeight || 1);
    }
  };

  useEffect(() => {
    const el = document.getElementById(containerId);
    if (!el) return;

    if (window.ResizeObserver) {
      const ro = new ResizeObserver(() => syncScrollStats());
      ro.observe(el);
      return () => ro.disconnect();
    }
  }, [containerId, code]);

  useEffect(() => {
    const el = document.getElementById(containerId);
    if (!el) return;
    
    el.addEventListener("scroll", syncScrollStats);
    syncScrollStats();
    
    return () => {
      el.removeEventListener("scroll", syncScrollStats);
    };
  }, [containerId, code]);

  const scrollToPercentage = (percentage: number) => {
    const el = document.getElementById(containerId);
    if (el) {
      const targetTop = percentage * el.scrollHeight - el.clientHeight / 2;
      el.scrollTop = Math.max(0, Math.min(el.scrollHeight - el.clientHeight, targetTop));
    }
  };

  const handleMouseDown = (e: React.MouseEvent) => {
    setIsDragging(true);
    handleMouseMove(e);
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  const handleMouseMove = (e: React.MouseEvent | MouseEvent) => {
    if (!isDragging && e.type !== "mousedown") return;
    if (containerRef.current) {
      const rect = containerRef.current.getBoundingClientRect();
      const y = (e as MouseEvent).clientY - rect.top;
      let percentage = y / rect.height;
      percentage = Math.max(0, Math.min(1, percentage));
      scrollToPercentage(percentage);
    }
  };

  useEffect(() => {
    if (isDragging) {
      window.addEventListener("mousemove", handleMouseMove);
      window.addEventListener("mouseup", handleMouseUp);
    } else {
      window.removeEventListener("mousemove", handleMouseMove);
      window.removeEventListener("mouseup", handleMouseUp);
    }
    return () => {
      window.removeEventListener("mousemove", handleMouseMove);
      window.removeEventListener("mouseup", handleMouseUp);
    };
  }, [isDragging]);

  const thumbHeightRatio = scrollHeight > 0 ? viewportHeight / scrollHeight : 1;
  const thumbHeightPercent = thumbHeightRatio * 100;
  
  const scrollRatio = scrollHeight > viewportHeight ? scrollTop / (scrollHeight - viewportHeight) : 0;
  const thumbTopPercent = scrollRatio * (100 - thumbHeightPercent);

  // We highlight the code for the minimap
  return (
    <div 
      className="hidden md:block w-24 bg-[#08080a] border-l border-white/5 relative overflow-hidden select-none shrink-0 cursor-pointer group" 
      ref={containerRef}
      onMouseDown={handleMouseDown}
    >
      <div 
        className="text-[2px] leading-[3px] font-mono text-slate-500 m-0 p-1 bg-transparent whitespace-pre-wrap break-all opacity-40 group-hover:opacity-60 transition-opacity"
        dangerouslySetInnerHTML={{ __html: Prism.highlight(code, Prism.languages.lumino || Prism.languages.javascript, "lumino") }}
      />
      <div 
        className="absolute w-full bg-white/10 group-hover:bg-white/20 transition-colors border-y border-white/20"
        style={{
          top: `${thumbTopPercent}%`,
          height: `${thumbHeightPercent}%`,
        }}
      />
    </div>
  );
}
