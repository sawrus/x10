import { readdir, readFile } from 'node:fs/promises'
import path from 'node:path'

const sourceRoot = new URL('../src/', import.meta.url)
const failures = []

async function walk(directoryUrl) {
  const entries = await readdir(directoryUrl, { withFileTypes: true })
  const files = await Promise.all(
    entries.map(async (entry) => {
      const entryUrl = new URL(`${entry.name}${entry.isDirectory() ? '/' : ''}`, directoryUrl)
      if (entry.isDirectory()) {
        return walk(entryUrl)
      }

      if (!/\.(ts|tsx)$/.test(entry.name)) {
        return []
      }

      return [entryUrl]
    }),
  )

  return files.flat()
}

function recordMatches(content, expression, message, filePath) {
  for (const match of content.matchAll(expression)) {
    const line = content.slice(0, match.index ?? 0).split('\n').length
    failures.push(`${filePath}:${line} ${message}`)
  }
}

for (const fileUrl of await walk(sourceRoot)) {
  const content = await readFile(fileUrl, 'utf8')
  const filePath = path.relative(process.cwd(), fileUrl.pathname)

  recordMatches(content, /\bconsole\.log\s*\(/g, 'remove console.log before commit', filePath)
  recordMatches(content, /\bdebugger\b/g, 'remove debugger before commit', filePath)
}

if (failures.length > 0) {
  console.error('web/game lint failed:')
  for (const failure of failures) {
    console.error(`- ${failure}`)
  }
  process.exit(1)
}

console.log('web/game lint passed')
