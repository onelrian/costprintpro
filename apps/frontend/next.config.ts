import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Enable standalone output for Docker deployment
  output: "standalone",
  
  // Performance optimizations
  compress: true,
  poweredByHeader: false,
  generateEtags: false,
};

export default nextConfig;
