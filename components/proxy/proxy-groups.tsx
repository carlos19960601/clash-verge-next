import { Virtuoso } from "react-virtuoso";
import { ProxyRender } from "./proxy-render";

interface Props {
  mode: string;
}

const ProxyGroups = (props: Props) => {
  const { mode } = props;
  return <Virtuoso itemContent={(index) => <ProxyRender />} />;
};

export default ProxyGroups;
