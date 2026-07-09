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
const adminPasswordHash =
  process.env.X10_E2E_ADMIN_PASSWORD_HASH ||
  '$argon2id$v=19$m=19456,t=2,p=1$ZfmqeR+rQgaHE96FVRGIvQ$fGXssSG70hSSBbuc01AbAPZU/RFok1+JCAWovmZ38Yo'

const tempRoot = await fs.mkdtemp(path.join(os.tmpdir(), 'x10-game-e2e-'))
const uploadsPath = path.join(tempRoot, 'uploads')
const databasePath = path.join(tempRoot, 'x10.sqlite3')
const screenshotDir = path.join(repoRoot, 'artifacts', 'e2e')

await fs.mkdir(uploadsPath, { recursive: true })
await fs.mkdir(screenshotDir, { recursive: true })

const chromiumExecutablePath = await resolveChromiumExecutablePath()

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

async function resolveChromiumExecutablePath() {
  const candidates = [
    process.env.PLAYWRIGHT_EXECUTABLE_PATH,
    '/usr/bin/chromium',
    '/usr/bin/chromium-browser',
  ].filter(Boolean)

  for (const candidate of candidates) {
    try {
      await fs.access(candidate)
      return candidate
    } catch {}
  }

  throw new Error(
    `Unable to find a Chromium executable. Checked: ${candidates.join(', ')}`,
  )
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
  await page.getByRole('button', { name: 'Hide health details' }).waitFor()
  await page.getByRole('button', { name: 'Preview modal' }).click()
  await page.getByRole('dialog', { name: 'Health integration details' }).waitFor()
  await page.getByRole('button', { name: 'Close panel' }).click()
  await page.getByRole('button', { name: 'Hide health details' }).click()
  await page.getByRole('button', { name: 'Show health details' }).waitFor()
  await takeScreenshot(page, 'dashboard.png')

  await page.getByRole('link', { name: 'Character', exact: true }).click()
  await page.getByRole('heading', { name: 'Create the hero sheet' }).waitFor()
  await page.getByRole('heading', { name: 'Reopen an existing hero' }).waitFor()
  await page.getByText('Create the first hero below and they will appear here.').waitFor()
  await page.getByRole('button', { name: 'Create character' }).click()
  await page.getByText('Full name is required.').waitFor()
  await page.getByText('Birth date is required.').waitFor()
  await page.getByText('Occupation is required.').waitFor()
  await page.getByLabel('Full name').fill('Test Hero')
  await page.getByLabel('Occupation').fill('programmer')
  await page.getByLabel('Birth date').fill('1989-06-27')
  await page.getByLabel('Timezone').fill('Europe/Samara')
  await page.getByLabel('Telegram').fill('@testhero')
  await page.getByLabel('Email').fill('hero@example.com')
  await page.getByRole('button', { name: 'Create character' }).click()
  await page.getByText('Персонаж создан. Теперь можно донастроить карточку и выбрать аватар.').waitFor()
  await page.getByRole('heading', { name: 'Character foundation is live' }).waitFor()
  await page.getByRole('heading', { name: 'Reopen an existing hero' }).waitFor()
  await page.getByRole('heading', { name: 'Test Hero', exact: true }).waitFor()
  await page.getByText('avatar pending', { exact: true }).first().waitFor()
  await page.getByText('gallery empty').waitFor()
  await page.getByText('@testhero').waitFor()
  await page.getByText('hero@example.com').waitFor()
  await page.getByText('current hero saved across refresh').waitFor()

  const editor = page.locator('form').last()
  const saveButton = editor.getByRole('button', { name: 'Save changes' })
  const resetButton = editor.getByRole('button', { name: 'Reset' })

  await editor.getByLabel('Occupation').fill('architect')
  await resetButton.click()
  await assert.equal(await editor.getByLabel('Occupation').inputValue(), 'programmer')

  await editor.getByLabel('Occupation').fill('')
  await editor.getByText('Occupation is required.').waitFor()
  await assert.equal(await saveButton.isDisabled(), true)

  await editor.getByLabel('Occupation').fill('senior programmer')
  await editor.getByLabel('Telegram').fill('')
  await editor.getByLabel('Email').fill('hero+updated@example.com')
  await assert.equal(await saveButton.isDisabled(), false)
  await saveButton.click()
  await page.getByText('Карточка героя обновлена.').waitFor()
  await editor.getByRole('button', { name: 'Saved' }).waitFor()
  await assert.equal(await editor.getByLabel('Occupation').inputValue(), 'senior programmer')
  await page.getByText('hero+updated@example.com').waitFor()

  const avatarOnePath = path.join(tempRoot, 'avatar-one.png')
  const avatarTwoPath = path.join(tempRoot, 'avatar-two.png')
  await fs.writeFile(avatarOnePath, pngBuffer())
  await fs.writeFile(avatarTwoPath, pngBuffer())
  const avatarOneTile = page
    .getByText('avatar-one.png', { exact: true })
    .locator('xpath=ancestor::div[.//button[normalize-space()="Remove"]][1]')
  const avatarTwoTile = page
    .getByText('avatar-two.png', { exact: true })
    .locator('xpath=ancestor::div[.//button[normalize-space()="Remove"]][1]')

  await page.locator('input[type="file"]').setInputFiles(avatarOnePath)
  await page.getByText('Фото загружено. Теперь его можно выбрать как основной аватар.').waitFor()
  await page.getByText('1 photo(s)').waitFor()
  await page.getByRole('button', { name: 'Use as avatar' }).click()
  await page.getByText('Аватар обновлён.').waitFor()
  await page.getByText('avatar ready', { exact: true }).first().waitFor()
  await avatarOneTile.getByRole('button', { name: 'Selected' }).waitFor()
  await avatarOneTile.getByRole('button', { name: 'Remove' }).click()
  await page.getByText('Сначала выбери другой аватар или загрузить новый. Активное фото backend пока не даёт удалить безопасно.').waitFor()
  await page.getByText('1 photo(s)').waitFor()

  await page.locator('input[type="file"]').setInputFiles(avatarTwoPath)
  await page.getByText('2 photo(s)').waitFor()
  await avatarTwoTile.getByRole('button', { name: 'Use as avatar' }).click()
  await page.getByText('Аватар обновлён.').waitFor()
  await avatarTwoTile.getByRole('button', { name: 'Selected' }).waitFor()
  await avatarOneTile.getByRole('button', { name: 'Remove' }).click()
  await page.getByText('Фото удалено из галереи.').waitFor()
  await page.getByText('1 photo(s)').waitFor()
  await takeScreenshot(page, 'character.png')

  await page.reload({ waitUntil: 'networkidle' })
  await page.getByRole('heading', { name: 'Character foundation is live' }).waitFor()
  await page.getByRole('heading', { name: 'Test Hero', exact: true }).waitFor()
  const reloadedEditor = page.locator('form').last()
  await assert.equal(await reloadedEditor.getByLabel('Occupation').inputValue(), 'senior programmer')
  await page.getByText('hero+updated@example.com').waitFor()
  await page.getByText('1 photo(s)').waitFor()
  await page.getByText('avatar ready', { exact: true }).first().waitFor()
  await takeScreenshot(page, 'character-after-reload.png')

  await page.getByRole('button', { name: 'Create another' }).click()
  await page.getByRole('heading', { name: 'Create the hero sheet' }).waitFor()
  const savedHeroesSelect = page.getByLabel('Saved heroes')
  const savedHeroOptions = await savedHeroesSelect.locator('option').evaluateAll((options) =>
    options.map((option) => ({
      label: option.label,
      value: option.value,
    })),
  )
  const testHeroOption = savedHeroOptions.find((option) => option.label.includes('Test Hero · senior programmer'))
  assert.ok(testHeroOption?.value, 'Expected Test Hero to appear in the saved heroes roster.')
  await savedHeroesSelect.selectOption(testHeroOption.value)
  await page.getByRole('button', { name: 'Continue with selected hero' }).click()
  await page.getByRole('heading', { name: 'Character foundation is live' }).waitFor()
  await page.getByRole('heading', { name: 'Test Hero', exact: true }).waitFor()
  const reopenedEditor = page.locator('form').last()
  await assert.equal(await reopenedEditor.getByLabel('Occupation').inputValue(), 'senior programmer')
  await page.getByText('1 photo(s)').waitFor()

  await page.getByRole('button', { name: 'Create another' }).click()
  await page.getByRole('heading', { name: 'Create the hero sheet' }).waitFor()
  await page.getByLabel('Full name').fill('Switch Hero')
  await page.getByLabel('Occupation').fill('designer')
  await page.getByLabel('Birth date').fill('1991-02-14')
  await page.getByLabel('Timezone').fill('Europe/Samara')
  await page.getByRole('button', { name: 'Create character' }).click()
  await page.getByText('Персонаж создан. Теперь можно донастроить карточку и выбрать аватар.').waitFor()
  await page.getByRole('heading', { name: 'Switch Hero', exact: true }).waitFor()

  const rosterWhileLoaded = page.getByLabel('Saved heroes')
  const loadedRosterOptions = await rosterWhileLoaded.locator('option').evaluateAll((options) =>
    options.map((option) => ({
      label: option.label,
      value: option.value,
    })),
  )
  const reopenedHeroOption = loadedRosterOptions.find((option) => option.label.includes('Test Hero · senior programmer'))
  assert.ok(reopenedHeroOption?.value, 'Expected roster to keep Test Hero available while another profile is open.')
  await rosterWhileLoaded.selectOption(reopenedHeroOption.value)
  await page.getByRole('button', { name: 'Open selected hero' }).click()
  await page.getByRole('heading', { name: 'Test Hero', exact: true }).waitFor()
  const switchedEditor = page.locator('form').last()
  await assert.equal(await switchedEditor.getByLabel('Occupation').inputValue(), 'senior programmer')
  await page.getByText('hero+updated@example.com').waitFor()

  await page.getByRole('link', { name: 'Quests', exact: true }).click()
  await page.getByRole('heading', { name: 'Quest board route is wired' }).waitFor()
  await takeScreenshot(page, 'quests.png')

  await page.getByRole('link', { name: 'History', exact: true }).click()
  await page.getByRole('heading', { name: 'Progress history route is ready' }).waitFor()
  await takeScreenshot(page, 'history.png')

  await page.getByRole('link', { name: 'Onboarding', exact: true }).click()
  await page.getByRole('heading', { name: 'Starter flow entrypoint' }).waitFor()
  await takeScreenshot(page, 'onboarding.png')

  await page.getByRole('link', { name: 'Dashboard', exact: true }).click()
  await page.getByText('profile selected').waitFor()
  await takeScreenshot(page, 'dashboard-after-profile.png')

  await page.getByRole('link', { name: 'Character', exact: true }).click()
  await page.getByRole('heading', { name: 'Character foundation is live' }).waitFor()
  await page.getByRole('heading', { name: 'Test Hero', exact: true }).waitFor()
  const finalEditor = page.locator('form').last()
  await assert.equal(await finalEditor.getByLabel('Occupation').inputValue(), 'senior programmer')
  await page.getByText('hero+updated@example.com').waitFor()
  await page.getByText('1 photo(s)').waitFor()
  await page.getByText('avatar ready', { exact: true }).first().waitFor()
  await takeScreenshot(page, 'character-after-navigation.png')

  await browser.close()
  console.log(`game e2e passed against ${frontendUrl}`)
} finally {
  await Promise.allSettled([
    browser?.close(),
    stopProcess(frontend),
    stopProcess(backend),
  ])
}
