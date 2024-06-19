import { useTheme } from "next-themes";
import { useEffect, useState } from "react";

const useIsDark = () => {
    const { theme } = useTheme();
    const [isDark, setIsDark] = useState(true);

    useEffect(() => {
        setIsDark(theme === "dark")
    }, [theme, setIsDark])

    return isDark
}

export default useIsDark;