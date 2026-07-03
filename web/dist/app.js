const urlParams = new URLSearchParams(window.location.search);

const translations = {
  en: {
    pageTitle: "X10 Progression",
    heroEyebrow: "Progression Dashboard",
    heroTitleEmpty: "Create your hero",
    heroMetaEmpty: "No active profile yet",
    statLevel: "Level",
    statBalance: "Balance",
    themeLabel: "Theme",
    themeDendy: "Dendy",
    themeApple: "Apple",
    languageLabel: "Language",
    authTitle: "Bootstrap Profile",
    authSubtitle: "Create a profile once, then the app stores your X-Actor-Id locally.",
    placeholderFullName: "Full name",
    placeholderOccupation: "Occupation",
    placeholderTelegram: "Telegram (optional)",
    placeholderEmail: "Email (optional)",
    buttonCreateProfile: "Create Profile",
    planTitle: "Plan Tasks",
    planSubtitle: "Build repeatable templates ranked by weight.",
    placeholderTaskTitle: "Task title",
    kindPositive: "Positive",
    kindNegative: "Negative",
    labelScore: "Score",
    labelRate: "Rate",
    cadenceDay: "Day",
    cadenceWeek: "Week",
    cadenceMonth: "Month",
    cadenceYear: "Year",
    noSphere: "No sphere",
    buttonCreateTask: "Create Task",
    taskQueueTitle: "Task Queue",
    sortedByWeight: "Sorted by weight",
    executeTitle: "Execute Today",
    executeSubtitle: "Close the same queue in the same canonical order.",
    ritualTitle: "Ritual Close",
    ritualSubtitle: "Finalize the day without recalculating balance.",
    placeholderDayNote: "Optional note for the day",
    buttonFinalizeDay: "Finalize Day",
    buttonAlreadyFinalized: "Already Finalized",
    photoTitle: "Profile Photo",
    photoSubtitle: "Upload a local image and switch your avatar.",
    buttonUploadPhoto: "Upload Photo",
    balanceTrailTitle: "Balance Trail",
    balanceTrailSubtitle: "Every execution appends a ledger row. The chart shows momentum over time.",
    ariaBalanceChart: "Balance chart",
    weight: "weight",
    score: "score",
    rate: "rate",
    startsOn: "Starts on",
    levelsConfigured: "levels configured",
    usePhoto: "Use",
    executeWithDefaults: "Execute with defaults",
    noBalanceEntries: "No balance entries yet",
    dayFinalizedToast: "Day finalized",
    pickFileFirst: "Pick a file first",
    photoAlt: "Profile photo",
    genericRequestFailed: "Request failed",
    error_FORBIDDEN: "Access denied",
    error_NOT_FOUND: "Resource not found",
    error_VALIDATION_ERROR: "Please check the entered values",
    error_CONFLICT: "This action conflicts with the current state",
    error_INVALID_MULTIPART: "Could not read the uploaded file",
    error_FILE_REQUIRED: "Select a file before upload",
    error_INVALID_ACTOR_ID: "Invalid actor id",
    error_WEB_ASSET_NOT_FOUND: "Web asset not found",
    profileMetaPattern: "{occupation} · {timezone} · {count} {levelsConfigured}",
    historyBalance: "{value} balance",
  },
  ru: {
    pageTitle: "X10 Прогресс",
    heroEyebrow: "Панель прогресса",
    heroTitleEmpty: "Создай своего героя",
    heroMetaEmpty: "Профиль пока не активирован",
    statLevel: "Уровень",
    statBalance: "Баланс",
    themeLabel: "Тема",
    themeDendy: "Денди",
    themeApple: "Apple",
    languageLabel: "Язык",
    authTitle: "Создание профиля",
    authSubtitle: "Создай профиль один раз, и приложение сохранит твой X-Actor-Id локально.",
    placeholderFullName: "ФИО",
    placeholderOccupation: "Роль / занятие",
    placeholderTelegram: "Telegram (необязательно)",
    placeholderEmail: "Email (необязательно)",
    buttonCreateProfile: "Создать профиль",
    planTitle: "План задач",
    planSubtitle: "Собирай повторяемые шаблоны задач, отсортированные по весу.",
    placeholderTaskTitle: "Название задачи",
    kindPositive: "Позитивная",
    kindNegative: "Негативная",
    labelScore: "Оценка",
    labelRate: "Процент",
    cadenceDay: "День",
    cadenceWeek: "Неделя",
    cadenceMonth: "Месяц",
    cadenceYear: "Год",
    noSphere: "Без сферы",
    buttonCreateTask: "Создать задачу",
    taskQueueTitle: "Очередь задач",
    sortedByWeight: "Сортировка по весу",
    executeTitle: "Выполнить сегодня",
    executeSubtitle: "Закрывай ту же очередь в том же каноническом порядке.",
    ritualTitle: "Ритуал закрытия дня",
    ritualSubtitle: "Закрой день без пересчёта баланса.",
    placeholderDayNote: "Необязательная заметка по дню",
    buttonFinalizeDay: "Закрыть день",
    buttonAlreadyFinalized: "День уже закрыт",
    photoTitle: "Фото профиля",
    photoSubtitle: "Загрузи локальное изображение и переключай аватар.",
    buttonUploadPhoto: "Загрузить фото",
    balanceTrailTitle: "След баланса",
    balanceTrailSubtitle: "Каждое выполнение добавляет запись в ledger. График показывает траекторию прогресса.",
    ariaBalanceChart: "График баланса",
    weight: "вес",
    score: "оценка",
    rate: "процент",
    startsOn: "Старт",
    levelsConfigured: "уровней настроено",
    usePhoto: "Выбрать",
    executeWithDefaults: "Выполнить с текущими значениями",
    noBalanceEntries: "Пока нет записей баланса",
    dayFinalizedToast: "День закрыт",
    pickFileFirst: "Сначала выбери файл",
    photoAlt: "Фото профиля",
    genericRequestFailed: "Не удалось выполнить запрос",
    error_FORBIDDEN: "Доступ запрещён",
    error_NOT_FOUND: "Ресурс не найден",
    error_VALIDATION_ERROR: "Проверь введённые значения",
    error_CONFLICT: "Действие конфликтует с текущим состоянием",
    error_INVALID_MULTIPART: "Не удалось прочитать загруженный файл",
    error_FILE_REQUIRED: "Перед загрузкой выбери файл",
    error_INVALID_ACTOR_ID: "Некорректный actor id",
    error_WEB_ASSET_NOT_FOUND: "Веб-ресурс не найден",
    profileMetaPattern: "{occupation} · {timezone} · {count} {levelsConfigured}",
    historyBalance: "{value} к балансу",
  },
};

const state = {
  actorId: urlParams.get("actorId") || localStorage.getItem("x10.actorId"),
  profileId: urlParams.get("profileId") || localStorage.getItem("x10.profileId"),
  theme: urlParams.get("theme") || localStorage.getItem("x10.theme") || "dendy",
  language: urlParams.get("lang") || localStorage.getItem("x10.language") || "en",
  dashboard: null,
  photos: [],
  spheres: [],
};

const nodes = {
  authPanel: document.querySelector("#auth-panel"),
  workspace: document.querySelector("#workspace"),
  historyPanel: document.querySelector("#history-panel"),
  profileForm: document.querySelector("#profile-form"),
  taskForm: document.querySelector("#task-form"),
  finalizeForm: document.querySelector("#finalize-form"),
  photoForm: document.querySelector("#photo-form"),
  themeSelect: document.querySelector("#theme-select"),
  languageSelect: document.querySelector("#language-select"),
  profileName: document.querySelector("#profile-name"),
  profileMeta: document.querySelector("#profile-meta"),
  currentLevel: document.querySelector("#current-level"),
  currentBalance: document.querySelector("#current-balance"),
  profilePhoto: document.querySelector("#profile-photo"),
  profilePhotoFallback: document.querySelector("#profile-photo-fallback"),
  sphereSelect: document.querySelector("#sphere-select"),
  taskList: document.querySelector("#task-list"),
  executionList: document.querySelector("#execution-list"),
  photoList: document.querySelector("#photo-list"),
  historyList: document.querySelector("#history-list"),
  balanceChart: document.querySelector("#balance-chart"),
  toast: document.querySelector("#toast"),
};

document.documentElement.dataset.theme = state.theme;
document.documentElement.lang = state.language;
nodes.themeSelect.value = state.theme;
nodes.languageSelect.value = state.language;
nodes.finalizeForm.date.value = new Date().toISOString().slice(0, 10);
nodes.taskForm.starts_on.value = new Date().toISOString().slice(0, 10);
applyTranslations();

nodes.themeSelect.addEventListener("change", () => {
  state.theme = nodes.themeSelect.value;
  document.documentElement.dataset.theme = state.theme;
  localStorage.setItem("x10.theme", state.theme);
});

nodes.languageSelect.addEventListener("change", () => {
  state.language = nodes.languageSelect.value;
  document.documentElement.lang = state.language;
  localStorage.setItem("x10.language", state.language);
  applyTranslations();
  render();
});

nodes.profileForm.addEventListener("submit", async (event) => {
  event.preventDefault();
  const form = new FormData(event.currentTarget);
  const payload = Object.fromEntries(form.entries());
  try {
    const profile = await api("/api/v2/profiles", {
      method: "POST",
      body: JSON.stringify(payload),
    });
    state.actorId = profile.id;
    state.profileId = profile.id;
    localStorage.setItem("x10.actorId", profile.id);
    localStorage.setItem("x10.profileId", profile.id);
    await bootstrap();
  } catch (error) {
    toast(error.message);
  }
});

nodes.taskForm.addEventListener("submit", async (event) => {
  event.preventDefault();
  if (!state.profileId) return;
  const form = new FormData(event.currentTarget);
  const payload = {
    profile_id: state.profileId,
    title: form.get("title"),
    sphere_id: form.get("sphere_id") || null,
    kind: form.get("kind"),
    planned_weight: Number(form.get("planned_weight")),
    planned_score: Number(form.get("planned_score")),
    planned_rate: Number(form.get("planned_rate")),
    cadence: form.get("cadence"),
    starts_on: form.get("starts_on"),
  };
  try {
    await api("/api/v2/tasks", { method: "POST", body: JSON.stringify(payload) });
    event.currentTarget.reset();
    nodes.taskForm.starts_on.value = new Date().toISOString().slice(0, 10);
    await loadData();
  } catch (error) {
    toast(error.message);
  }
});

nodes.finalizeForm.addEventListener("submit", async (event) => {
  event.preventDefault();
  if (!state.profileId) return;
  const form = new FormData(event.currentTarget);
  try {
    await api(`/api/v2/profiles/${state.profileId}/days/${form.get("date")}/finalize`, {
      method: "POST",
      body: JSON.stringify({ note: form.get("note") || null }),
    });
    toast(t("dayFinalizedToast"));
    await loadData();
  } catch (error) {
    toast(error.message);
  }
});

nodes.photoForm.addEventListener("submit", async (event) => {
  event.preventDefault();
  if (!state.profileId) return;
  const form = new FormData(event.currentTarget);
  if (!form.get("file") || form.get("file").size === 0) {
    toast(t("pickFileFirst"));
    return;
  }
  try {
    await api(`/api/v2/profiles/${state.profileId}/photos`, {
      method: "POST",
      body: form,
      isMultipart: true,
    });
    event.currentTarget.reset();
    await loadData();
  } catch (error) {
    toast(error.message);
  }
});

async function bootstrap() {
  if (!state.actorId || !state.profileId) {
    showAnonymous();
    await loadSpheres();
    return;
  }
  try {
    await loadData();
    showDashboard();
  } catch (error) {
    showAnonymous();
    toast(error.message);
  }
}

async function loadData() {
  await loadSpheres();
  state.dashboard = await api(`/api/v2/profiles/${state.profileId}/dashboard`);
  state.photos = await api(`/api/v2/profiles/${state.profileId}/photos`);
  render();
}

async function loadSpheres() {
  state.spheres = await api("/api/v2/spheres");
  renderSphereOptions();
}

function showAnonymous() {
  nodes.authPanel.hidden = false;
  nodes.workspace.hidden = true;
  nodes.historyPanel.hidden = true;
}

function showDashboard() {
  nodes.authPanel.hidden = true;
  nodes.workspace.hidden = false;
  nodes.historyPanel.hidden = false;
}

function renderSphereOptions() {
  const current = nodes.sphereSelect.value;
  nodes.sphereSelect.innerHTML = `<option value="">${t("noSphere")}</option>`;
  state.spheres.forEach((sphere) => {
    const option = document.createElement("option");
    option.value = sphere.id;
    option.textContent = `${sphere.name} (${t("weight")}:${sphere.weight})`;
    if (sphere.id === current) option.selected = true;
    nodes.sphereSelect.append(option);
  });
}

function render() {
  applyTranslations();
  if (!state.dashboard) return;
  const { profile, current, current_photo, tasks, execution_queue, balances, levels, finalizations } = state.dashboard;

  nodes.profileName.textContent = profile.full_name;
  nodes.profileMeta.textContent = format("profileMetaPattern", {
    occupation: profile.occupation,
    timezone: profile.timezone,
    count: levels.length,
    levelsConfigured: t("levelsConfigured"),
  });
  nodes.currentLevel.textContent = current.current_level;
  nodes.currentBalance.textContent = String(current.balance_score);

  if (current_photo) {
    nodes.profilePhoto.src = current_photo.content_url;
    nodes.profilePhoto.alt = t("photoAlt");
    nodes.profilePhoto.hidden = false;
    nodes.profilePhotoFallback.hidden = true;
  } else {
    nodes.profilePhoto.hidden = true;
    nodes.profilePhotoFallback.hidden = false;
    nodes.profilePhotoFallback.textContent = profile.full_name.slice(0, 2).toUpperCase();
  }

  nodes.taskList.innerHTML = tasks.map(renderTaskCard).join("");
  nodes.executionList.innerHTML = execution_queue.map(renderExecutionCard).join("");
  nodes.historyList.innerHTML = balances
    .slice()
    .reverse()
    .map((entry) => {
      const level = levels.find((item) => item.min_balance <= entry.balance_after) || levels[0];
      return `
        <article class="history-item">
          <h4>${format("historyBalance", { value: `${entry.actual_weight >= 0 ? "+" : ""}${entry.actual_weight}` })}</h4>
          <div class="history-meta">
            <span class="pill">${t("score")} ${entry.actual_score}</span>
            <span class="pill">${t("rate")} ${entry.actual_rate}%</span>
            <span class="pill">${t("statLevel").toLowerCase()} ${level ? level.code : current.current_level}</span>
          </div>
          <p class="muted">${formatDateTime(entry.created_at)}</p>
        </article>
      `;
    })
    .join("");

  nodes.photoList.innerHTML = state.photos
    .map(
      (photo) => `
        <article class="photo-card">
          <img src="${photo.content_url}" alt="${escapeHtml(photo.original_name)}" />
          <div class="task-meta">
            <span class="pill">${escapeHtml(photo.original_name)}</span>
          </div>
          <button type="button" data-action="select-photo" data-photo-id="${photo.id}">${t("usePhoto")}</button>
        </article>
      `,
    )
    .join("");

  drawChart(balances);
  attachDynamicActions(finalizations);
}

function renderTaskCard(task) {
  return `
    <article class="task-card">
      <h4>${escapeHtml(task.title)}</h4>
      <div class="task-meta">
        <span class="pill ${task.kind}">${translateKind(task.kind)}</span>
        <span class="pill">${t("weight")} ${task.planned_weight}</span>
        <span class="pill">${t("score")} ${task.planned_score}</span>
        <span class="pill">${t("rate")} ${task.planned_rate}%</span>
        <span class="pill">${translateCadence(task.cadence)}</span>
      </div>
      <p class="muted">${t("startsOn")} ${task.starts_on}</p>
    </article>
  `;
}

function renderExecutionCard(task) {
  return `
    <article class="task-card">
      <h4>${escapeHtml(task.title)}</h4>
      <div class="task-meta">
        <span class="pill ${task.kind}">${translateKind(task.kind)}</span>
        <span class="pill">${t("weight")} ${task.planned_weight}</span>
      </div>
      <div class="execution-controls">
        <input type="number" min="1" max="5" value="${task.planned_score}" data-role="actual-score" data-task-id="${task.id}" />
        <input type="number" min="0" max="100" value="${task.planned_rate}" data-role="actual-rate" data-task-id="${task.id}" />
        <button type="button" data-action="execute-task" data-task-id="${task.id}">${t("executeWithDefaults")}</button>
      </div>
    </article>
  `;
}

function attachDynamicActions(finalizations) {
  document.querySelectorAll('[data-action="execute-task"]').forEach((button) => {
    button.onclick = async () => {
      const taskId = button.dataset.taskId;
      const scoreInput = document.querySelector(`[data-role="actual-score"][data-task-id="${taskId}"]`);
      const rateInput = document.querySelector(`[data-role="actual-rate"][data-task-id="${taskId}"]`);
      try {
        await api(`/api/v2/tasks/${taskId}/executions`, {
          method: "POST",
          body: JSON.stringify({
            actual_score: Number(scoreInput.value),
            actual_rate: Number(rateInput.value),
          }),
        });
        await loadData();
      } catch (error) {
        toast(error.message);
      }
    };
  });

  document.querySelectorAll('[data-action="select-photo"]').forEach((button) => {
    button.onclick = async () => {
      try {
        await api(`/api/v2/profiles/${state.profileId}/photos/${button.dataset.photoId}/select`, {
          method: "POST",
        });
        await loadData();
      } catch (error) {
        toast(error.message);
      }
    };
  });

  const finalizedDates = new Set(finalizations.map((item) => item.date));
  nodes.finalizeForm.querySelector("button").textContent = finalizedDates.has(nodes.finalizeForm.date.value)
    ? t("buttonAlreadyFinalized")
    : t("buttonFinalizeDay");
}

function drawChart(balances) {
  const svg = nodes.balanceChart;
  if (!balances.length) {
    svg.innerHTML = `<text x="50%" y="50%" text-anchor="middle" fill="currentColor">${escapeHtml(t("noBalanceEntries"))}</text>`;
    return;
  }
  const points = balances.map((entry, index) => ({
    x: 40 + index * (760 / Math.max(1, balances.length - 1)),
    y: entry.balance_after,
  }));
  const values = points.map((point) => point.y);
  const min = Math.min(...values, 0);
  const max = Math.max(...values, 1);
  const normalized = points.map((point) => {
    const range = max - min || 1;
    const chartY = 200 - ((point.y - min) / range) * 160;
    return `${point.x},${chartY}`;
  });
  svg.innerHTML = `
    <rect x="0" y="0" width="800" height="240" fill="transparent"></rect>
    <line x1="30" y1="200" x2="780" y2="200" stroke="currentColor" opacity="0.3"></line>
    <polyline fill="none" stroke="var(--accent)" stroke-width="4" points="${normalized.join(" ")}"></polyline>
    ${normalized
      .map(
        (point) =>
          `<circle cx="${point.split(",")[0]}" cy="${point.split(",")[1]}" r="5" fill="var(--accent-2)"></circle>`,
      )
      .join("")}
  `;
}

async function api(path, options = {}) {
  const headers = new Headers(options.headers || {});
  if (state.actorId) headers.set("X-Actor-Id", state.actorId);
  if (!options.isMultipart) headers.set("Content-Type", "application/json");

  const response = await fetch(path, {
    ...options,
    headers,
  });
  if (!response.ok) {
    let message = `${t("genericRequestFailed")} (${response.status})`;
    try {
      const data = await response.json();
      const code = data.error?.code;
      message = localizedErrorMessage(code, data.error?.message) || message;
    } catch (_error) {
      // ignore non-json errors
    }
    throw new Error(message);
  }
  const contentType = response.headers.get("Content-Type") || "";
  if (contentType.includes("application/json")) {
    return response.json();
  }
  return response.text();
}

function toast(message) {
  nodes.toast.textContent = message;
  nodes.toast.hidden = false;
  clearTimeout(toast.timer);
  toast.timer = setTimeout(() => {
    nodes.toast.hidden = true;
  }, 3000);
}

function applyTranslations() {
  document.title = t("pageTitle");
  document.querySelectorAll("[data-i18n]").forEach((node) => {
    node.textContent = t(node.dataset.i18n);
  });
  document.querySelectorAll("[data-i18n-placeholder]").forEach((node) => {
    node.setAttribute("placeholder", t(node.dataset.i18nPlaceholder));
  });
  document.querySelectorAll("[data-i18n-aria-label]").forEach((node) => {
    node.setAttribute("aria-label", t(node.dataset.i18nAriaLabel));
  });
}

function t(key) {
  return translations[state.language]?.[key] || translations.en[key] || key;
}

function format(key, values) {
  return Object.entries(values).reduce(
    (result, [name, value]) => result.replaceAll(`{${name}}`, String(value)),
    t(key),
  );
}

function translateKind(value) {
  return value === "negative" ? t("kindNegative") : t("kindPositive");
}

function translateCadence(value) {
  const mapping = {
    day: t("cadenceDay"),
    week: t("cadenceWeek"),
    month: t("cadenceMonth"),
    year: t("cadenceYear"),
  };
  return mapping[value] || value;
}

function localizedErrorMessage(code, fallbackMessage) {
  if (code) {
    const translated = t(`error_${code}`);
    if (translated !== `error_${code}`) {
      return translated;
    }
  }
  return fallbackMessage || t("genericRequestFailed");
}

function formatDateTime(value) {
  return new Intl.DateTimeFormat(state.language, {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(new Date(value));
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;");
}

bootstrap();
