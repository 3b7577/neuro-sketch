import Link from "next/link";
import { FC, ReactNode } from "react";

interface HeaderProps {
  children?: ReactNode;
  className?: string;
}

const Header: FC<HeaderProps> = ({ children, className = "" }) => {
  return (
    <header
      className={`w-full border-b border-border bg-surface flex items-center ${className}`}
    >
      <Link
        href="/"
        className="text-xl font-bold text-text hover:text-accent transition-colors"
      >
        NeuroSketch
      </Link>

      <div className="w-full px-4 py-4 flex items-center justify-between gap-4">
        {children}
      </div>
    </header>
  );
};

export default Header;
