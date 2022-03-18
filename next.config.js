/** @type {import('next').NextConfig} */
const nextConfig = {
  experimental: { esmExternals: true },
  webpack: (config, options) => {
    config.experiments = {
      asyncWebAssembly: true,
      layers: true,
    };
    return config;
  },
  reactStrictMode: true,
};

module.exports = nextConfig;
