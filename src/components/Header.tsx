import Link from "next/link";
import { FC, ReactNode } from "react";

interface HeaderProps {
  children?: ReactNode;
  className?: string;
}

const Header: FC<HeaderProps> = ({ children, className = "" }) => {
  return (
    <header
      className={`relative w-full border-b border-border bg-surface flex items-center ${className}`}
    >
      <div className="absolute top-0 left-0 right-0 h-[2px] bg-accent" />

      <div className="w-full px-6 py-4 flex items-center justify-between gap-4">
        <Link
          href="/"
          className="text-title font-bold text-text hover:text-accent transition-colors duration-150"
        >
          NeuroSketch
        </Link>

        {children}
      </div>
    </header>
  );
};

export default Header;
