import { FC, ReactNode } from "react";

interface SidebarProps {
  children?: ReactNode;
  className?: string;
}

const Sidebar: FC<SidebarProps> = ({ children, className = "" }) => {
  return (
    <aside
      className={`h-full border-r border-border bg-panel flex flex-col ${className}`}
    >
      <div className="flex-1 p-6 flex flex-col gap-6">{children}</div>
    </aside>
  );
};

export default Sidebar;
