import { Virtuoso } from "react-virtuoso";
import { ProxyRender } from "./proxy-render";
import { useRenderList } from "./use-render-list";

interface Props {
  mode: string;
}

const ProxyGroups = (props: Props) => {
  const { mode } = props;

  const { renderList, onProxies, onHeadState } = useRenderList(mode);

  return <Virtuoso itemContent={(index) => <ProxyRender />} />;
};

export default ProxyGroups;
