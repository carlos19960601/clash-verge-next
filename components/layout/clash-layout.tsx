import { PropsWithChildren } from "react";
import Sidebar from "./sidebar";

const ClashLayout = ({ children }: PropsWithChildren) => {
  return (
    <div className="h-screen flex">
      <Sidebar />
      <div className="grow">{children}</div>
    </div>
  );
};

export default ClashLayout;
