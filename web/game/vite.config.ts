import { defineConfig, loadEnv, type ProxyOptions } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), '')
  const proxyTarget = env.VITE_API_PROXY_TARGET || 'http://127.0.0.1:3000'

  function createApiProxy(): ProxyOptions {
    return {
      changeOrigin: true,
      configure(proxy) {
        proxy.on('error', (_error, request, response) => {
          const targetPath = request.url || ''
          const payload = JSON.stringify({
            error: {
              code: 'BACKEND_UNAVAILABLE',
              details: {
                path: targetPath,
                target: proxyTarget,
              },
              message: `Game frontend cannot reach backend API at ${proxyTarget}. Start x10-backend or set VITE_API_PROXY_TARGET.`,
            },
          })

          if (!response.headersSent) {
            response.writeHead(502, {
              'content-type': 'application/json',
            })
          }

          response.end(payload)
        })
      },
      target: proxyTarget,
    }
  }

  return {
    base: '/game/',
    plugins: [react()],
    build: {
      outDir: 'dist',
    },
    preview: {
      proxy: {
        '/api': createApiProxy(),
        '/health': createApiProxy(),
        '/metrics': createApiProxy(),
      },
    },
    server: {
      proxy: {
        '/api': createApiProxy(),
        '/health': createApiProxy(),
        '/metrics': createApiProxy(),
      },
    },
  }
})
