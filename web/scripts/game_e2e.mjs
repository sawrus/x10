import assert from 'node:assert/strict'
import { spawn } from 'node:child_process'
import fs from 'node:fs/promises'
import os from 'node:os'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

import { chromium } from 'playwright-core'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const webDir = path.resolve(__dirname, '..')
const repoRoot = path.resolve(webDir, '..')
const gameDir = path.join(webDir, 'game')
const backendPort = Number(process.env.X10_E2E_BACKEND_PORT || '3010')
const frontendPort = Number(process.env.X10_E2E_FRONTEND_PORT || '4173')
const backendUrl = `http://127.0.0.1:${backendPort}`
const frontendUrl = `http://127.0.0.1:${frontendPort}/game/`
const chromiumExecutablePath = process.env.PLAYWRIGHT_EXECUTABLE_PATH || '/usr/bin/chromium-browser'
const adminPasswordHash =
  process.env.X10_E2E_ADMIN_PASSWORD_HASH ||
  '$argon2id$v=19$m=19456,t=2,p=1$ZfmqeR+rQgaHE96FVRGIvQ$fGXssSG70hSSBbuc01AbAPZU/RFok1+JCAWovmZ38Yo'

const tempRoot = await fs.mkdtemp(path.join(os.tmpdir(), 'x10-game-e2e-'))
const uploadsPath = path.join(tempRoot, 'uploads')
const databasePath = path.join(tempRoot, 'x10.sqlite3')
const screenshotDir = path.join(repoRoot, 'artifacts', 'e2e')

await fs.mkdir(uploadsPath, { recursive: true })
await fs.mkdir(screenshotDir, { recursive: true })

function pipeOutput(processHandle, label) {
  processHandle.stdout?.on('data', (chunk) => {
    process.stdout.write(`[${label}] ${chunk}`)
  })
  processHandle.stderr?.on('data', (chunk) => {
    process.stderr.write(`[${label}] ${chunk}`)
  })
}

function spawnProcess(command, args, options) {
  const child = spawn(command, args, options)
  pipeOutput(child, options.label)
  return child
}

const backend = spawnProcess(
  'cargo',
  ['run', '--bin', 'x10-backend'],
  {
    cwd: repoRoot,
    env: {
      ...process.env,
      X10_ADMIN_PASSWORD_HASH: adminPasswordHash,
      X10_ADMIN_SESSION_SECRET: 'e2e-secret',
      X10_ADMIN_SESSION_SECURE: 'false',
      X10_ADMIN_USERNAME: 'admin',
      X10_DATABASE_PATH: databasePath,
      X10_HOST: '127.0.0.1',
      X10_PORT: String(backendPort),
      X10_UPLOADS_PATH: uploadsPath,
      X10_WEB_DIST_PATH: path.join(webDir, 'dist'),
    },
    label: 'backend',
    stdio: ['ignore', 'pipe', 'pipe'],
  },
)

const frontend = spawnProcess(
  'npm',
  ['run', 'dev', '--', '--port', String(frontendPort)],
  {
    cwd: gameDir,
    env: {
      ...process.env,
      VITE_API_PROXY_TARGET: backendUrl,
    },
    label: 'game-dev',
    stdio: ['ignore', 'pipe', 'pipe'],
  },
)

async function waitFor(url, label, timeoutMs = 60_000) {
  const startedAt = Date.now()

  while (Date.now() - startedAt < timeoutMs) {
    try {
      const response = await fetch(url)
      if (response.ok) {
        return
      }
    } catch {}

    await new Promise((resolve) => setTimeout(resolve, 500))
  }

  throw new Error(`Timed out waiting for ${label} at ${url}`)
}

function pngBuffer() {
  return Buffer.from(
    'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAusB9WZ0pV8AAAAASUVORK5CYII=',
    'base64',
  )
}

async function takeScreenshot(page, name) {
  await page.screenshot({
    fullPage: true,
    path: path.join(screenshotDir, name),
  })
}

async function stopProcess(processHandle) {
  if (!processHandle || processHandle.killed) {
    return
  }

  processHandle.kill('SIGTERM')

  await new Promise((resolve) => {
    const timer = setTimeout(() => {
      processHandle.kill('SIGKILL')
      resolve()
    }, 5_000)

    processHandle.once('exit', () => {
      clearTimeout(timer)
      resolve()
    })
  })
}

let browser

try {
  await waitFor(`${backendUrl}/health`, 'backend health')
  await waitFor(frontendUrl, 'game dev server')

  browser = await chromium.launch({
    executablePath: chromiumExecutablePath,
    headless: true,
  })

  const context = await browser.newContext({
    viewport: { width: 1440, height: 1100 },
  })
  const page = await context.newPage()

  await page.goto(frontendUrl, { waitUntil: 'networkidle' })
  await page.getByRole('heading', { name: 'X10 Game Frontend' }).waitFor()
  await page.getByRole('button', { name: 'Show health details' }).click()
  await page.getByRole('dialog', { name: 'Health integration details' }).waitFor()
  await page.getByRole('button', { name: 'Close panel' }).click()
  await page.getByRole('button', { name: 'Show health details' }).waitFor()
  await takeScreenshot(page, 'dashboard.png')

  await page.getByRole('link', { name: 'Character' }).click()
  await page.getByRole('heading', { name: 'Create the hero sheet' }).waitFor()
  await page.getByLabel('Full name').fill('Test Hero')
  await page.getByLabel('Occupation').fill('programmer')
  await page.getByLabel('Birth date').fill('1989-06-27')
  await page.getByLabel('Timezone').fill('Europe/Samara')
  await page.getByLabel('Telegram').fill('@testhero')
  await page.getByLabel('Email').fill('hero@example.com')
  await page.getByRole('button', { name: 'Create character' }).click()
  await page.getByText('Персонаж создан. Теперь можно донастроить карточку и выбрать аватар.').waitFor()
  await page.getByRole('heading', { name: 'Character foundation is live' }).waitFor()
  await page.getByRole('heading', { name: 'Test Hero', exact: true }).waitFor()

  await page.getByLabel('Occupation').fill('senior programmer')
  await page.getByRole('button', { name: 'Save changes' }).click()
  await page.getByText('Карточка героя обновлена.').waitFor()

  const avatarOnePath = path.join(tempRoot, 'avatar-one.png')
  const avatarTwoPath = path.join(tempRoot, 'avatar-two.png')
  await fs.writeFile(avatarOnePath, pngBuffer())
  await fs.writeFile(avatarTwoPath, pngBuffer())

  await page.locator('input[type="file"]').setInputFiles(avatarOnePath)
  await page.getByText('Фото загружено. Теперь его можно выбрать как основной аватар.').waitFor()
  await page.getByText('1 photo(s)').waitFor()
  await page.getByRole('button', { name: 'Use as avatar' }).click()
  await page.getByText('Аватар обновлён.').waitFor()
  await page.getByText('avatar ready').waitFor()

  await page.locator('input[type="file"]').setInputFiles(avatarTwoPath)
  await page.getByText('2 photo(s)').waitFor()
  assert.ok((await page.getByRole('button', { name: 'Use as avatar' }).count()) >= 1, 'expected a non-selected avatar action')
  await page.getByRole('button', { name: 'Remove' }).last().click()
  await page.getByText('Фото удалено из галереи.').waitFor()
  await page.getByText('1 photo(s)').waitFor()
  await takeScreenshot(page, 'character.png')

  await page.getByRole('link', { name: 'Quests' }).click()
  await page.getByRole('heading', { name: 'Quest board route is wired' }).waitFor()
  await takeScreenshot(page, 'quests.png')

  await page.getByRole('link', { name: 'History' }).click()
  await page.getByRole('heading', { name: 'Progress history route is ready' }).waitFor()
  await takeScreenshot(page, 'history.png')

  await page.getByRole('link', { name: 'Onboarding' }).click()
  await page.getByRole('heading', { name: 'Starter flow entrypoint' }).waitFor()
  await takeScreenshot(page, 'onboarding.png')

  await page.getByRole('link', { name: 'Dashboard' }).click()
  await page.getByText('profile selected').waitFor()
  await takeScreenshot(page, 'dashboard-after-profile.png')

  await browser.close()
  console.log(`game e2e passed against ${frontendUrl}`)
} finally {
  await Promise.allSettled([
    browser?.close(),
    stopProcess(frontend),
    stopProcess(backend),
  ])
}
