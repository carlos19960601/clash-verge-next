/** @type {import('next').NextConfig} */
const nextConfig = {
    output: "export",
    webpack(config) {
        config.module.rules.push({
            test: /\.svg$/,
            use: '@svgr/webpack',
            options: {
                svgo: true,
                svgoConfig: {
                  plugins: [
                    {
                      name: 'preset-default',
                      params: {
                        overrides: { removeViewBox: false },
                      },
                    },
                  ],
                },
            }
        })

        return config
    },
};

export default nextConfig;
