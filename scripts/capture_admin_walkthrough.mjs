import { chromium } from '../web/node_modules/playwright-core/index.mjs'
import fs from 'node:fs/promises'
import path from 'node:path'

const outputDir = process.argv[2] || 'docs/admin-vuetify/gif-frames'
const baseUrl = process.argv[3] || 'http://127.0.0.1:3000'

await fs.mkdir(outputDir, { recursive: true })

const browser = await chromium.launch({
  executablePath: '/usr/bin/chromium',
  headless: true,
})

const context = await browser.newContext({
  viewport: { width: 1440, height: 1280 },
  colorScheme: 'light',
})
const page = await context.newPage()

async function shot(name) {
  await page.screenshot({
    path: path.join(outputDir, name),
    fullPage: true,
  })
}

await page.goto(`${baseUrl}/`, { waitUntil: 'networkidle' })
await shot('01-login.png')

await page.locator('[autocomplete="username"]').fill('admin')
await page.locator('[autocomplete="current-password"]').fill('admin123')
await page.locator('#login-submit').click()
await page.waitForSelector('#create-profile-button')
await shot('02-shell.png')

await page.locator('#create-profile-button').click()
await page.locator('#profile-create-full-name').fill('Admin Demo')
await page.locator('#profile-create-birth-date').fill('1990-01-01')
await page.locator('#profile-create-occupation').fill('Operator')
await page.locator('#profile-create-timezone').fill('Europe/Samara')
await page.locator('#profile-create-submit').click()
await page.waitForTimeout(1200)
await shot('03-profile-created.png')

await page.locator('#create-task-button').click()
await page.locator('#task-title').fill('Daily planning')
await page.locator('#task-weight').fill('2')
await page.locator('#task-score').fill('4')
await page.locator('#task-rate').fill('80')
await page.locator('#task-save').click()
await page.waitForTimeout(1200)
await shot('04-task-created.png')

await page
  .locator('button')
  .filter({ has: page.locator('.mdi-play-circle-outline') })
  .first()
  .click()
await page.waitForTimeout(300)
await page.locator('input[type="datetime-local"]').fill('2026-07-03T14:30')
await page.getByRole('button', { name: 'Create' }).last().click()
await page.waitForTimeout(1200)
await shot('05-execution-created.png')

await page.getByRole('button', { name: 'Dark' }).click()
await page.locator('#language-select').click({ force: true })
await page.getByText('RU').click()
await page.waitForTimeout(600)
await shot('06-theme-language.png')

await page.goto(`${baseUrl}/game`, { waitUntil: 'networkidle' })
await shot('07-game.png')

await browser.close()
