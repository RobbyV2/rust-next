const isGitHubPages = process.env.GITHUB_PAGES === 'true'
const basePath = process.env.NEXT_PUBLIC_BASE_PATH || ''
const appMode = process.env.APP_MODE || 'full'

/** @type {import('next').NextConfig} */
const nextConfig = {
  output: isGitHubPages ? 'export' : 'standalone',
  basePath: basePath || undefined,
  assetPrefix: basePath || undefined,
  trailingSlash: true,
  allowedDevOrigins: ['127.0.0.1', 'localhost'],
  images: {
    unoptimized: isGitHubPages,
  },
}

if (appMode === 'api-only') {
  const serverPort = process.env.SERVER_PORT || '3000'
  const serverHost = process.env.HOST || 'localhost'
  const apiUrl = process.env.NEXT_PUBLIC_API_URL || `http://${serverHost}:${serverPort}`
  nextConfig.rewrites = async () => ({
    beforeFiles: [],
    afterFiles: [],
    fallback: [{ source: '/api/:path*', destination: `${apiUrl}/api/:path*` }],
  })
}

export default nextConfig
