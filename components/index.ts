
// nextjs 导入svg图片作为组件需要 @svgr/webpack

import dynamic from "next/dynamic";

const IconDark = dynamic(() => import('./svgs/icon_dark.svg'), { ssr: false })
const IconLight = dynamic(() => import('./svgs/icon_light.svg'), { ssr: false })
const IconLogo = dynamic(() => import('./svgs/logo.svg'), { ssr: false })
const IconIcon = dynamic(() => import('./svgs/icon.svg'), { ssr: false })


export { IconDark, IconIcon, IconLight, IconLogo };

// layout
export { default as ClashLayout } from './layout/clash-layout';

// components
export { default as Sidebar } from './layout/sidebar';
