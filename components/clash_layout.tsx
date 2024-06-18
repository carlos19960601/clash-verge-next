import { PropsWithChildren } from "react";
import Sidebar from "./sidebar";

const ClashLayout = ({ children }: PropsWithChildren) => {
  return (
    <div className="h-screen w-full flex overflow-hidden">
      <Sidebar />
      <div className="grow shrink basis-full">{children}</div>
    </div>
  );
};

export default ClashLayout;
