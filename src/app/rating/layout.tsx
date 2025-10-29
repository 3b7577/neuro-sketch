import RatingHeader from "./components/RatingHeader";
import RatingSidebar from "./components/RatingSidebar";
import { FC, ReactNode } from "react";

interface RatingLayoutProps {
  children: ReactNode;
}

const RatingLayout: FC<RatingLayoutProps> = ({ children }) => (
  <main className="flex flex-col h-screen">
    <RatingHeader></RatingHeader>

    <div className="flex flex-1 overflow-hidden">
      <RatingSidebar />
      <div className="flex-1 overflow-auto">{children}</div>
    </div>
  </main>
);

export default RatingLayout;
