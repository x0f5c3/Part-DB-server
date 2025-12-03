/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  // Enable rewrites to proxy API requests to the Rust backend during development
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: 'http://localhost:3000/api/:path*',
      },
    ];
  },
};

module.exports = nextConfig;
