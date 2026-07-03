<template>
  <v-app>
    <v-app-bar elevation="1">
      <v-app-bar-nav-icon v-if="session.authenticated" @click="drawer = !drawer" />
      <v-app-bar-title>{{ t('appName') }}</v-app-bar-title>
      <v-spacer />
      <v-btn-toggle
        v-model="themeName"
        color="primary"
        density="comfortable"
        mandatory
        variant="text"
      >
        <v-btn value="light">{{ t('light') }}</v-btn>
        <v-btn value="dark">{{ t('dark') }}</v-btn>
      </v-btn-toggle>
      <v-select
        v-model="locale"
        :items="localeItems"
        class="toolbar-select"
        density="comfortable"
        hide-details
        id="language-select"
        item-title="title"
        item-value="value"
        variant="underlined"
      />
      <v-btn v-if="session.authenticated" variant="text" @click="handleLogout">
        {{ t('signOut') }}
      </v-btn>
    </v-app-bar>

    <v-navigation-drawer v-if="session.authenticated" v-model="drawer" mobile-breakpoint="960">
      <v-list nav>
        <v-list-item
          prepend-icon="mdi-account-group-outline"
          :title="t('profiles')"
          :active="section === 'profiles'"
          @click="section = 'profiles'"
        />
        <v-list-item
          prepend-icon="mdi-shape-outline"
          :title="t('spheres')"
          :active="section === 'spheres'"
          @click="section = 'spheres'"
        />
        <v-list-item
          prepend-icon="mdi-controller-classic-outline"
          :title="t('game')"
          :subtitle="t('gameHint')"
          href="/game"
          target="_blank"
        />
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <v-container class="py-8">
        <v-row v-if="!session.authenticated" justify="center">
          <v-col cols="12" sm="10" md="6" lg="4">
            <v-card rounded="xl">
              <v-card-title class="text-h5 pa-6 pb-2">
                {{ t('loginTitle') }}
              </v-card-title>
              <v-card-text class="pa-6 pt-0">
                <p class="text-medium-emphasis mb-6">{{ t('loginSubtitle') }}</p>
                <v-alert
                  v-if="loginError"
                  class="mb-4"
                  density="comfortable"
                  type="error"
                  variant="tonal"
                >
                  {{ loginError }}
                </v-alert>
                <v-form @submit.prevent="handleLogin">
                  <v-text-field
                    id="login-username"
                    v-model="loginForm.username"
                    :label="t('username')"
                    autocomplete="username"
                    class="mb-2"
                    required
                    variant="outlined"
                  />
                  <v-text-field
                    id="login-password"
                    v-model="loginForm.password"
                    :label="t('password')"
                    autocomplete="current-password"
                    required
                    type="password"
                    variant="outlined"
                  />
                  <v-btn
                    id="login-submit"
                    block
                    class="mt-2"
                    color="primary"
                    :loading="loading.login"
                    size="large"
                    type="submit"
                  >
                    {{ t('signIn') }}
                  </v-btn>
                </v-form>
              </v-card-text>
            </v-card>
          </v-col>
        </v-row>

        <template v-else>
          <section v-if="section === 'profiles'">
            <v-row class="mb-4" align="stretch">
              <v-col cols="12" md="4">
                <v-card rounded="xl">
                  <v-card-title class="d-flex align-center">
                    <span>{{ t('profiles') }}</span>
                    <v-spacer />
                    <v-btn id="create-profile-button" color="primary" size="small" @click="openCreateProfile">
                      {{ t('create') }}
                    </v-btn>
                  </v-card-title>
                  <v-card-text>
                    <v-btn class="mb-4" variant="text" @click="loadProfiles">
                      {{ t('reload') }}
                    </v-btn>
                    <v-list lines="two">
                      <v-list-item
                        v-for="profile in profiles"
                        :key="profile.id"
                        :active="selectedProfileId === profile.id"
                        @click="selectProfile(profile.id)"
                      >
                        <template #title>{{ profile.full_name }}</template>
                        <template #subtitle>
                          {{ profile.occupation }} · {{ profile.timezone }}
                        </template>
                        <template #append>
                          <v-btn
                            color="error"
                            icon="mdi-delete-outline"
                            size="small"
                            variant="text"
                            @click.stop="confirmDeleteProfile(profile)"
                          />
                        </template>
                      </v-list-item>
                    </v-list>
                  </v-card-text>
                </v-card>
              </v-col>

              <v-col cols="12" md="8">
                <v-card v-if="!selectedProfileId" rounded="xl">
                  <v-card-text class="pa-8 text-medium-emphasis">
                    {{ t('noProfileSelected') }}
                  </v-card-text>
                </v-card>

                <template v-else>
                  <v-row>
                    <v-col cols="12" md="4">
                      <v-card rounded="xl">
                        <v-card-text>
                          <div class="text-overline">{{ t('currentLevel') }}</div>
                          <div class="text-h4">{{ dashboard.current?.current_level || '-' }}</div>
                        </v-card-text>
                      </v-card>
                    </v-col>
                    <v-col cols="12" md="4">
                      <v-card rounded="xl">
                        <v-card-text>
                          <div class="text-overline">{{ t('currentBalance') }}</div>
                          <div class="text-h4">
                            {{ dashboard.current?.balance_score ?? 0 }}
                          </div>
                        </v-card-text>
                      </v-card>
                    </v-col>
                    <v-col cols="12" md="4">
                      <v-card rounded="xl">
                        <v-card-text>
                          <div class="text-overline">{{ t('levelsConfigured') }}</div>
                          <div class="text-h4">{{ dashboard.levels?.length ?? 0 }}</div>
                        </v-card-text>
                      </v-card>
                    </v-col>
                  </v-row>

                  <v-card class="mt-4" rounded="xl">
                    <v-card-title>{{ t('profileDetails') }}</v-card-title>
                    <v-card-text>
                      <v-row>
                        <v-col cols="12" md="6">
                          <v-text-field
                            v-model="profileForm.full_name"
                            :label="t('fullName')"
                            variant="outlined"
                          />
                        </v-col>
                        <v-col cols="12" md="6">
                          <v-text-field
                            v-model="profileForm.birth_date"
                            :label="t('birthDate')"
                            type="date"
                            variant="outlined"
                          />
                        </v-col>
                        <v-col cols="12" md="6">
                          <v-text-field
                            v-model="profileForm.occupation"
                            :label="t('occupation')"
                            variant="outlined"
                          />
                        </v-col>
                        <v-col cols="12" md="6">
                          <v-text-field
                            v-model="profileForm.timezone"
                            :label="t('timezone')"
                            variant="outlined"
                          />
                        </v-col>
                        <v-col cols="12" md="6">
                          <v-text-field
                            v-model="profileForm.telegram"
                            :label="t('telegram')"
                            variant="outlined"
                          />
                        </v-col>
                        <v-col cols="12" md="6">
                          <v-text-field
                            v-model="profileForm.email"
                            :label="t('email')"
                            variant="outlined"
                          />
                        </v-col>
                      </v-row>
                      <v-btn color="primary" :loading="loading.profile" @click="saveProfile">
                        {{ t('save') }}
                      </v-btn>
                    </v-card-text>
                  </v-card>

                  <v-card class="mt-4" rounded="xl">
                    <v-card-title class="d-flex align-center">
                      <span>{{ t('profilePhotos') }}</span>
                      <v-spacer />
                    </v-card-title>
                    <v-card-text>
                      <v-row align="center">
                        <v-col cols="12" md="8">
                          <v-file-input
                            v-model="photoFile"
                            :label="t('chooseFile')"
                            accept="image/*"
                            prepend-icon="mdi-camera-outline"
                            variant="outlined"
                          />
                        </v-col>
                        <v-col cols="12" md="4">
                          <v-btn
                            block
                            color="primary"
                            :loading="loading.photo"
                            @click="uploadPhoto"
                          >
                            {{ t('photoUpload') }}
                          </v-btn>
                        </v-col>
                      </v-row>
                      <v-row>
                        <v-col
                          v-for="photo in photos"
                          :key="photo.id"
                          cols="12"
                          sm="6"
                          lg="4"
                        >
                          <v-card variant="outlined">
                            <v-img
                              v-if="photoUrls[photo.id]"
                              :src="photoUrls[photo.id]"
                              cover
                              height="160"
                            />
                            <v-card-text>
                              <div class="font-weight-medium">{{ photo.original_name }}</div>
                              <div class="text-caption text-medium-emphasis">
                                {{ photo.mime_type }} · {{ photo.size_bytes }} B
                              </div>
                            </v-card-text>
                            <v-card-actions>
                              <v-btn
                                size="small"
                                variant="text"
                                @click="selectPhoto(photo.id)"
                              >
                                {{
                                  dashboard.profile?.current_photo_id === photo.id
                                    ? t('selected')
                                    : t('select')
                                }}
                              </v-btn>
                              <v-spacer />
                              <v-btn
                                color="error"
                                icon="mdi-delete-outline"
                                size="small"
                                variant="text"
                                @click="confirmDeletePhoto(photo.id)"
                              />
                            </v-card-actions>
                          </v-card>
                        </v-col>
                      </v-row>
                    </v-card-text>
                  </v-card>

                  <v-card class="mt-4" rounded="xl">
                    <v-card-title class="d-flex align-center">
                      <span>{{ t('tasks') }}</span>
                      <v-spacer />
                      <v-btn id="create-task-button" color="primary" size="small" @click="openTaskDialog()">
                        {{ t('create') }}
                      </v-btn>
                    </v-card-title>
                    <v-card-text>
                      <v-table>
                        <thead>
                          <tr>
                            <th>{{ t('title') }}</th>
                            <th>{{ t('kind') }}</th>
                            <th>{{ t('plannedWeight') }}</th>
                            <th>{{ t('status') }}</th>
                            <th>{{ t('cadence') }}</th>
                            <th class="text-right">{{ t('edit') }}</th>
                          </tr>
                        </thead>
                        <tbody>
                          <tr v-for="task in dashboard.tasks || []" :key="task.id">
                            <td>{{ task.title }}</td>
                            <td>{{ enumLabel('kind', task.kind) }}</td>
                            <td>{{ task.planned_weight }}</td>
                            <td>{{ enumLabel('status', task.status) }}</td>
                            <td>{{ enumLabel('cadence', task.cadence) }}</td>
                            <td class="text-right">
                              <v-btn
                                icon="mdi-play-circle-outline"
                                size="small"
                                variant="text"
                                @click="openExecutionDialog(task)"
                              />
                              <v-btn
                                icon="mdi-pencil-outline"
                                size="small"
                                variant="text"
                                @click="openTaskDialog(task)"
                              />
                              <v-btn
                                color="error"
                                icon="mdi-delete-outline"
                                size="small"
                                variant="text"
                                @click="confirmDeleteTask(task.id)"
                              />
                            </td>
                          </tr>
                        </tbody>
                      </v-table>
                    </v-card-text>
                  </v-card>

                  <v-row class="mt-1">
                    <v-col cols="12" xl="6">
                      <v-card rounded="xl">
                        <v-card-title>{{ t('executions') }}</v-card-title>
                        <v-card-text>
                          <v-table>
                            <thead>
                              <tr>
                                <th>{{ t('title') }}</th>
                                <th>{{ t('actualScore') }}</th>
                                <th>{{ t('actualRate') }}</th>
                                <th>{{ t('completedAt') }}</th>
                                <th />
                              </tr>
                            </thead>
                            <tbody>
                              <tr v-for="execution in dashboard.recent_executions || []" :key="execution.id">
                                <td>{{ taskTitle(execution.task_id) }}</td>
                                <td>{{ execution.actual_score }}</td>
                                <td>{{ execution.actual_rate }}</td>
                                <td>{{ formatDateTime(execution.completed_at) }}</td>
                                <td class="text-right">
                                  <v-btn
                                    color="error"
                                    icon="mdi-delete-outline"
                                    size="small"
                                    variant="text"
                                    @click="confirmDeleteExecution(execution.id)"
                                  />
                                </td>
                              </tr>
                            </tbody>
                          </v-table>
                        </v-card-text>
                      </v-card>
                    </v-col>

                    <v-col cols="12" xl="6">
                      <v-card rounded="xl">
                        <v-card-title>{{ t('balances') }}</v-card-title>
                        <v-card-text>
                          <v-table>
                            <thead>
                              <tr>
                                <th>ID</th>
                                <th>{{ t('actualScore') }}</th>
                                <th>{{ t('actualRate') }}</th>
                                <th>{{ t('plannedWeight') }}</th>
                                <th>{{ t('currentBalance') }}</th>
                              </tr>
                            </thead>
                            <tbody>
                              <tr v-for="balance in dashboard.balances || []" :key="balance.id">
                                <td class="mono">{{ shortId(balance.id) }}</td>
                                <td>{{ balance.actual_score }}</td>
                                <td>{{ balance.actual_rate }}</td>
                                <td>{{ balance.actual_weight }}</td>
                                <td>{{ balance.balance_after }}</td>
                              </tr>
                            </tbody>
                          </v-table>
                        </v-card-text>
                      </v-card>
                    </v-col>
                  </v-row>

                  <v-row class="mt-1">
                    <v-col cols="12" xl="6">
                      <v-card rounded="xl">
                        <v-card-title class="d-flex align-center">
                          <span>{{ t('levels') }}</span>
                          <v-spacer />
                          <v-btn color="primary" size="small" @click="openLevelDialog()">
                            {{ t('create') }}
                          </v-btn>
                        </v-card-title>
                        <v-card-text>
                          <v-table>
                            <thead>
                              <tr>
                                <th>{{ t('code') }}</th>
                                <th>{{ t('ordinal') }}</th>
                                <th>{{ t('minBalance') }}</th>
                                <th />
                              </tr>
                            </thead>
                            <tbody>
                              <tr v-for="level in dashboard.levels || []" :key="level.id">
                                <td>{{ level.code }}</td>
                                <td>{{ level.ordinal }}</td>
                                <td>{{ level.min_balance }}</td>
                                <td class="text-right">
                                  <v-btn
                                    icon="mdi-pencil-outline"
                                    size="small"
                                    variant="text"
                                    @click="openLevelDialog(level)"
                                  />
                                  <v-btn
                                    color="error"
                                    icon="mdi-delete-outline"
                                    size="small"
                                    variant="text"
                                    @click="confirmDeleteLevel(level.id)"
                                  />
                                </td>
                              </tr>
                            </tbody>
                          </v-table>
                        </v-card-text>
                      </v-card>
                    </v-col>

                    <v-col cols="12" xl="6">
                      <v-card rounded="xl">
                        <v-card-title>{{ t('levelState') }}</v-card-title>
                        <v-card-text>
                          <div class="mb-2">
                            <strong>{{ t('currentLevelId') }}:</strong>
                            <span class="mono">{{ levelState?.current_level_id || '-' }}</span>
                          </div>
                          <div class="mb-2">
                            <strong>{{ t('lastBalanceId') }}:</strong>
                            <span class="mono">{{ levelState?.last_balance_id || '-' }}</span>
                          </div>
                          <div>
                            <strong>{{ t('updatedAt') }}:</strong>
                            {{ levelState?.updated_at ? formatDateTime(levelState.updated_at) : '-' }}
                          </div>
                        </v-card-text>
                      </v-card>
                    </v-col>
                  </v-row>

                  <v-card class="mt-4" rounded="xl">
                    <v-card-title class="d-flex align-center">
                      <span>{{ t('finalizations') }}</span>
                      <v-spacer />
                      <v-btn color="primary" size="small" @click="openFinalizationDialog()">
                        {{ t('create') }}
                      </v-btn>
                    </v-card-title>
                    <v-card-text>
                      <v-table>
                        <thead>
                          <tr>
                            <th>{{ t('date') }}</th>
                            <th>{{ t('note') }}</th>
                            <th />
                          </tr>
                        </thead>
                        <tbody>
                          <tr v-for="finalization in dashboard.finalizations || []" :key="finalization.id">
                            <td>{{ finalization.date }}</td>
                            <td>{{ finalization.note || '-' }}</td>
                            <td class="text-right">
                              <v-btn
                                color="error"
                                icon="mdi-delete-outline"
                                size="small"
                                variant="text"
                                @click="confirmDeleteFinalization(finalization.id)"
                              />
                            </td>
                          </tr>
                        </tbody>
                      </v-table>
                    </v-card-text>
                  </v-card>
                </template>
              </v-col>
            </v-row>
          </section>

          <section v-else>
            <v-card rounded="xl">
              <v-card-title class="d-flex align-center">
                <span>{{ t('spheres') }}</span>
                <v-spacer />
                <v-btn color="primary" size="small" @click="openSphereDialog()">
                  {{ t('create') }}
                </v-btn>
              </v-card-title>
              <v-card-text>
                <v-table>
                  <thead>
                    <tr>
                      <th>{{ t('name') }}</th>
                      <th>{{ t('weight') }}</th>
                      <th />
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="sphere in spheres" :key="sphere.id">
                      <td>{{ sphere.name }}</td>
                      <td>{{ sphere.weight }}</td>
                      <td class="text-right">
                        <v-btn
                          icon="mdi-pencil-outline"
                          size="small"
                          variant="text"
                          @click="openSphereDialog(sphere)"
                        />
                        <v-btn
                          color="error"
                          icon="mdi-delete-outline"
                          size="small"
                          variant="text"
                          @click="confirmDeleteSphere(sphere.id)"
                        />
                      </td>
                    </tr>
                  </tbody>
                </v-table>
              </v-card-text>
            </v-card>
          </section>
        </template>
      </v-container>
    </v-main>

    <v-dialog v-model="dialogs.profile" max-width="720">
      <v-card rounded="xl">
        <v-card-title>{{ t('createProfile') }}</v-card-title>
        <v-card-text>
          <v-row>
            <v-col cols="12" md="6">
              <v-text-field id="profile-create-full-name" v-model="profileCreate.full_name" :label="t('fullName')" variant="outlined" />
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field id="profile-create-birth-date" v-model="profileCreate.birth_date" :label="t('birthDate')" type="date" variant="outlined" />
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field id="profile-create-occupation" v-model="profileCreate.occupation" :label="t('occupation')" variant="outlined" />
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field id="profile-create-timezone" v-model="profileCreate.timezone" :label="t('timezone')" variant="outlined" />
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field v-model="profileCreate.telegram" :label="t('telegram')" variant="outlined" />
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field v-model="profileCreate.email" :label="t('email')" variant="outlined" />
            </v-col>
          </v-row>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="dialogs.profile = false">{{ t('cancel') }}</v-btn>
          <v-btn id="profile-create-submit" color="primary" :loading="loading.profileCreate" @click="createProfile">
            {{ t('create') }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="dialogs.task" max-width="820">
      <v-card rounded="xl">
        <v-card-title>{{ taskForm.id ? t('edit') : t('create') }} {{ t('tasks') }}</v-card-title>
        <v-card-text>
          <v-row>
            <v-col cols="12" md="6">
              <v-text-field id="task-title" v-model="taskForm.title" :label="t('title')" variant="outlined" />
            </v-col>
            <v-col cols="12" md="6">
              <v-select v-model="taskForm.sphere_id" :items="sphereItems" clearable item-title="title" item-value="value" :label="t('spheres')" variant="outlined" />
            </v-col>
            <v-col cols="12" md="4">
              <v-select v-model="taskForm.kind" :items="kindItems" item-title="title" item-value="value" :label="t('kind')" variant="outlined" />
            </v-col>
            <v-col cols="12" md="4">
              <v-text-field id="task-weight" v-model.number="taskForm.planned_weight" :label="t('plannedWeight')" type="number" variant="outlined" />
            </v-col>
            <v-col cols="12" md="4">
              <v-select v-model="taskForm.status" :items="statusItems" item-title="title" item-value="value" :label="t('status')" variant="outlined" />
            </v-col>
            <v-col cols="12" md="4">
              <v-text-field id="task-score" v-model.number="taskForm.planned_score" :label="t('plannedScore')" type="number" variant="outlined" />
            </v-col>
            <v-col cols="12" md="4">
              <v-text-field id="task-rate" v-model.number="taskForm.planned_rate" :label="t('plannedRate')" type="number" variant="outlined" />
            </v-col>
            <v-col cols="12" md="4">
              <v-select v-model="taskForm.cadence" :items="cadenceItems" item-title="title" item-value="value" :label="t('cadence')" variant="outlined" />
            </v-col>
            <v-col cols="12">
              <v-text-field id="task-starts-on" v-model="taskForm.starts_on" :label="t('startsOn')" type="date" variant="outlined" />
            </v-col>
          </v-row>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="dialogs.task = false">{{ t('cancel') }}</v-btn>
          <v-btn id="task-save" color="primary" :loading="loading.task" @click="saveTask">{{ t('save') }}</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="dialogs.execution" max-width="540">
      <v-card rounded="xl">
        <v-card-title>{{ t('createExecution') }}</v-card-title>
        <v-card-text>
          <v-text-field v-model.number="executionForm.actual_score" :label="t('actualScore')" type="number" variant="outlined" />
          <v-text-field v-model.number="executionForm.actual_rate" :label="t('actualRate')" type="number" variant="outlined" />
          <v-text-field v-model="executionForm.completed_at" :label="t('completedAt')" type="datetime-local" variant="outlined" />
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="dialogs.execution = false">{{ t('cancel') }}</v-btn>
          <v-btn color="primary" :loading="loading.execution" @click="createExecution">
            {{ t('create') }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="dialogs.level" max-width="720">
      <v-card rounded="xl">
        <v-card-title>{{ levelForm.id ? t('edit') : t('createLevel') }}</v-card-title>
        <v-card-text>
          <v-row>
            <v-col cols="12" md="6">
              <v-text-field v-model="levelForm.code" :label="t('code')" variant="outlined" />
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field v-model.number="levelForm.ordinal" :label="t('ordinal')" type="number" variant="outlined" />
            </v-col>
            <v-col cols="12" md="4">
              <v-text-field v-model.number="levelForm.min_balance" :label="t('minBalance')" type="number" variant="outlined" />
            </v-col>
            <v-col cols="12" md="4">
              <v-text-field v-model.number="levelForm.target_planned_score" :label="t('targetPlannedScore')" type="number" variant="outlined" />
            </v-col>
            <v-col cols="12" md="4">
              <v-text-field v-model.number="levelForm.target_planned_rate" :label="t('targetPlannedRate')" type="number" variant="outlined" />
            </v-col>
          </v-row>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="dialogs.level = false">{{ t('cancel') }}</v-btn>
          <v-btn color="primary" :loading="loading.level" @click="saveLevel">{{ t('save') }}</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="dialogs.finalization" max-width="540">
      <v-card rounded="xl">
        <v-card-title>{{ t('createFinalization') }}</v-card-title>
        <v-card-text>
          <v-text-field v-model="finalizationForm.date" :label="t('date')" type="date" variant="outlined" />
          <v-textarea v-model="finalizationForm.note" :label="t('note')" rows="3" variant="outlined" />
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="dialogs.finalization = false">{{ t('cancel') }}</v-btn>
          <v-btn color="primary" :loading="loading.finalization" @click="saveFinalization">
            {{ t('save') }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="dialogs.sphere" max-width="540">
      <v-card rounded="xl">
        <v-card-title>{{ t('createSphere') }}</v-card-title>
        <v-card-text>
          <v-text-field v-model="sphereForm.name" :label="t('name')" variant="outlined" />
          <v-text-field v-model.number="sphereForm.weight" :label="t('weight')" type="number" variant="outlined" />
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="dialogs.sphere = false">{{ t('cancel') }}</v-btn>
          <v-btn color="primary" :loading="loading.sphere" @click="saveSphere">{{ t('save') }}</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="confirmState.open" max-width="420">
      <v-card rounded="xl">
        <v-card-title>{{ t('confirmDelete') }}</v-card-title>
        <v-card-text>{{ confirmState.message }}</v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="closeConfirm">{{ t('cancel') }}</v-btn>
          <v-btn color="error" @click="runConfirm">{{ t('delete') }}</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-snackbar v-model="snackbar.open" :color="snackbar.color">
      {{ snackbar.text }}
    </v-snackbar>
  </v-app>
</template>

<script setup>
import { computed, onMounted, reactive, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useTheme } from 'vuetify'

const { locale, t } = useI18n()
const theme = useTheme()

const drawer = ref(true)
const section = ref('profiles')
const loginError = ref('')
const photoFile = ref([])
const selectedProfileId = ref('')

const loading = reactive({
  login: false,
  profileCreate: false,
  profile: false,
  profileBundle: false,
  photo: false,
  task: false,
  execution: false,
  level: false,
  finalization: false,
  sphere: false,
})

const snackbar = reactive({
  open: false,
  text: '',
  color: 'primary',
})

const session = reactive({
  authenticated: false,
  username: null,
})

const profiles = ref([])
const spheres = ref([])
const photos = ref([])
const photoUrls = reactive({})
const levelState = ref(null)
const dashboard = reactive({
  profile: null,
  current: null,
  tasks: [],
  recent_executions: [],
  balances: [],
  levels: [],
  finalizations: [],
})

const dialogs = reactive({
  profile: false,
  task: false,
  execution: false,
  level: false,
  finalization: false,
  sphere: false,
})

const confirmState = reactive({
  open: false,
  message: '',
  action: null,
})

const loginForm = reactive({
  username: '',
  password: '',
})

const profileCreate = reactive({
  full_name: '',
  birth_date: '',
  occupation: '',
  telegram: '',
  email: '',
  timezone: 'Europe/Samara',
})

const profileForm = reactive({
  full_name: '',
  birth_date: '',
  occupation: '',
  telegram: '',
  email: '',
  timezone: 'Europe/Samara',
})

const taskForm = reactive({
  id: null,
  title: '',
  sphere_id: null,
  kind: 'positive',
  planned_weight: 1,
  planned_score: 3,
  planned_rate: 50,
  cadence: 'day',
  starts_on: new Date().toISOString().slice(0, 10),
  status: 'planned',
})

const executionForm = reactive({
  task_id: null,
  actual_score: 3,
  actual_rate: 50,
  completed_at: new Date().toISOString().slice(0, 16),
})

const levelForm = reactive({
  id: null,
  code: '',
  ordinal: 1,
  min_balance: 0,
  target_planned_score: 3,
  target_planned_rate: 50,
})

const finalizationForm = reactive({
  date: new Date().toISOString().slice(0, 10),
  note: '',
})

const sphereForm = reactive({
  id: null,
  name: '',
  weight: 1,
})

const localeItems = [
  { title: 'EN', value: 'en' },
  { title: 'RU', value: 'ru' },
]

const kindItems = computed(() => [
  { title: t('positive'), value: 'positive' },
  { title: t('negative'), value: 'negative' },
])
const cadenceItems = computed(() => [
  { title: t('day'), value: 'day' },
  { title: t('week'), value: 'week' },
  { title: t('month'), value: 'month' },
  { title: t('year'), value: 'year' },
])
const statusItems = computed(() => [
  { title: t('planned'), value: 'planned' },
  { title: t('archived'), value: 'archived' },
  { title: t('skipped'), value: 'skipped' },
])
const sphereItems = computed(() =>
  spheres.value.map((sphere) => ({ title: sphere.name, value: sphere.id })),
)

const themeName = computed({
  get: () => theme.global.name.value,
  set: (value) => {
    theme.global.name.value = value
  },
})

watch(themeName, (value) => {
  localStorage.setItem('x10.theme', value)
})

watch(locale, (value) => {
  localStorage.setItem('x10.language', value)
})

function resetObject(target, source) {
  Object.keys(target).forEach((key) => {
    target[key] = source[key]
  })
}

function showSnackbar(text, color = 'primary') {
  snackbar.text = text
  snackbar.color = color
  snackbar.open = true
}

function normalizeError(error) {
  return error?.message || t('operationFailed')
}

async function api(path, options = {}) {
  const headers = new Headers(options.headers || {})
  const isFormData = options.body instanceof FormData
  if (!isFormData && options.body !== undefined) {
    headers.set('content-type', 'application/json')
  }
  if (options.actorId) {
    headers.set('x-actor-id', options.actorId)
  }

  const response = await fetch(path, {
    method: options.method || 'GET',
    body: isFormData
      ? options.body
      : options.body !== undefined
        ? JSON.stringify(options.body)
        : undefined,
    credentials: 'same-origin',
    headers,
  })

  if (options.responseType === 'blob') {
    if (!response.ok) {
      throw new Error(t('operationFailed'))
    }
    return response.blob()
  }

  if (response.status === 204) {
    return null
  }

  const text = await response.text()
  const payload = text ? JSON.parse(text) : null
  if (!response.ok) {
    throw new Error(payload?.error?.message || t('operationFailed'))
  }
  return payload
}

async function loadSession() {
  const payload = await api('/api/admin/auth/session')
  session.authenticated = payload.authenticated
  session.username = payload.username
  if (session.authenticated) {
    await Promise.all([loadProfiles(), loadSpheres()])
  }
}

async function handleLogin() {
  loading.login = true
  loginError.value = ''
  try {
    const payload = await api('/api/admin/auth/login', {
      method: 'POST',
      body: loginForm,
    })
    session.authenticated = payload.authenticated
    session.username = payload.username
    loginForm.password = ''
    await Promise.all([loadProfiles(), loadSpheres()])
  } catch (error) {
    loginError.value = normalizeError(error)
  } finally {
    loading.login = false
  }
}

async function handleLogout() {
  await api('/api/admin/auth/logout', { method: 'POST' })
  session.authenticated = false
  session.username = null
  selectedProfileId.value = ''
  profiles.value = []
  spheres.value = []
  showSnackbar(t('logoutDone'))
}

async function loadProfiles() {
  profiles.value = await api('/api/admin/profiles')
  if (selectedProfileId.value) {
    const exists = profiles.value.find((profile) => profile.id === selectedProfileId.value)
    if (!exists) {
      selectedProfileId.value = ''
    }
  }
}

async function loadSpheres() {
  spheres.value = await api('/api/v2/spheres')
}

async function selectProfile(profileId) {
  selectedProfileId.value = profileId
  await loadProfileBundle()
}

async function loadProfileBundle() {
  if (!selectedProfileId.value) {
    return
  }

  loading.profileBundle = true
  try {
    const [profile, dashboardPayload, photosPayload, levelStatePayload] = await Promise.all([
      api(`/api/v2/profiles/${selectedProfileId.value}`, { actorId: selectedProfileId.value }),
      api(`/api/v2/profiles/${selectedProfileId.value}/dashboard`, {
        actorId: selectedProfileId.value,
      }),
      api(`/api/v2/profiles/${selectedProfileId.value}/photos`, {
        actorId: selectedProfileId.value,
      }),
      api(`/api/admin/profiles/${selectedProfileId.value}/level-state`),
    ])

    dashboard.profile = profile
    dashboard.current = dashboardPayload.current
    dashboard.tasks = dashboardPayload.tasks
    dashboard.recent_executions = dashboardPayload.recent_executions
    dashboard.balances = dashboardPayload.balances
    dashboard.levels = dashboardPayload.levels
    dashboard.finalizations = dashboardPayload.finalizations
    photos.value = photosPayload
    levelState.value = levelStatePayload

    resetObject(profileForm, {
      full_name: profile.full_name,
      birth_date: profile.birth_date,
      occupation: profile.occupation,
      telegram: profile.telegram || '',
      email: profile.email || '',
      timezone: profile.timezone,
    })

    await refreshPhotoUrls()
  } finally {
    loading.profileBundle = false
  }
}

async function refreshPhotoUrls() {
  Object.keys(photoUrls).forEach((key) => {
    URL.revokeObjectURL(photoUrls[key])
    delete photoUrls[key]
  })

  for (const photo of photos.value) {
    try {
      const blob = await api(`/api/v2/photos/${photo.id}`, {
        actorId: selectedProfileId.value,
        responseType: 'blob',
      })
      photoUrls[photo.id] = URL.createObjectURL(blob)
    } catch (_error) {
      photoUrls[photo.id] = ''
    }
  }
}

function openCreateProfile() {
  resetObject(profileCreate, {
    full_name: '',
    birth_date: '',
    occupation: '',
    telegram: '',
    email: '',
    timezone: 'Europe/Samara',
  })
  dialogs.profile = true
}

async function createProfile() {
  loading.profileCreate = true
  try {
    const profile = await api('/api/v2/profiles', {
      method: 'POST',
      body: {
        ...profileCreate,
        telegram: profileCreate.telegram || null,
        email: profileCreate.email || null,
      },
    })
    dialogs.profile = false
    await loadProfiles()
    await selectProfile(profile.id)
    showSnackbar(t('operationDone'))
  } catch (error) {
    showSnackbar(normalizeError(error), 'error')
  } finally {
    loading.profileCreate = false
  }
}

async function saveProfile() {
  loading.profile = true
  try {
    await api(`/api/v2/profiles/${selectedProfileId.value}`, {
      method: 'PATCH',
      actorId: selectedProfileId.value,
      body: {
        ...profileForm,
        telegram: profileForm.telegram === '' ? null : profileForm.telegram,
        email: profileForm.email === '' ? null : profileForm.email,
      },
    })
    await Promise.all([loadProfiles(), loadProfileBundle()])
    showSnackbar(t('operationDone'))
  } catch (error) {
    showSnackbar(normalizeError(error), 'error')
  } finally {
    loading.profile = false
  }
}

async function uploadPhoto() {
  const file = Array.isArray(photoFile.value) ? photoFile.value[0] : photoFile.value
  if (!file) {
    return
  }
  loading.photo = true
  try {
    const formData = new FormData()
    formData.append('file', file)
    await api(`/api/v2/profiles/${selectedProfileId.value}/photos`, {
      method: 'POST',
      actorId: selectedProfileId.value,
      body: formData,
    })
    photoFile.value = []
    await loadProfileBundle()
    showSnackbar(t('operationDone'))
  } catch (error) {
    showSnackbar(normalizeError(error), 'error')
  } finally {
    loading.photo = false
  }
}

async function selectPhoto(photoId) {
  try {
    await api(`/api/v2/profiles/${selectedProfileId.value}/photos/${photoId}/select`, {
      method: 'POST',
      actorId: selectedProfileId.value,
    })
    await loadProfileBundle()
  } catch (error) {
    showSnackbar(normalizeError(error), 'error')
  }
}

function openTaskDialog(task = null) {
  resetObject(taskForm, {
    id: task?.id || null,
    title: task?.title || '',
    sphere_id: task?.sphere_id || null,
    kind: task?.kind || 'positive',
    planned_weight: task?.planned_weight || 1,
    planned_score: task?.planned_score || 3,
    planned_rate: task?.planned_rate || 50,
    cadence: task?.cadence || 'day',
    starts_on: task?.starts_on || new Date().toISOString().slice(0, 10),
    status: task?.status || 'planned',
  })
  dialogs.task = true
}

async function saveTask() {
  loading.task = true
  try {
    if (taskForm.id) {
      await api(`/api/v2/tasks/${taskForm.id}`, {
        method: 'PATCH',
        actorId: selectedProfileId.value,
        body: { ...taskForm, sphere_id: taskForm.sphere_id || null },
      })
    } else {
      await api('/api/v2/tasks', {
        method: 'POST',
        actorId: selectedProfileId.value,
        body: {
          profile_id: selectedProfileId.value,
          ...taskForm,
          sphere_id: taskForm.sphere_id || null,
        },
      })
    }
    dialogs.task = false
    await loadProfileBundle()
    showSnackbar(t('operationDone'))
  } catch (error) {
    showSnackbar(normalizeError(error), 'error')
  } finally {
    loading.task = false
  }
}

function openExecutionDialog(task) {
  resetObject(executionForm, {
    task_id: task.id,
    actual_score: 3,
    actual_rate: 50,
    completed_at: new Date().toISOString().slice(0, 16),
  })
  dialogs.execution = true
}

async function createExecution() {
  loading.execution = true
  try {
    await api(`/api/v2/tasks/${executionForm.task_id}/executions`, {
      method: 'POST',
      actorId: selectedProfileId.value,
      body: {
        actual_score: executionForm.actual_score,
        actual_rate: executionForm.actual_rate,
        completed_at: new Date(executionForm.completed_at).toISOString(),
      },
    })
    dialogs.execution = false
    await loadProfileBundle()
    showSnackbar(t('operationDone'))
  } catch (error) {
    showSnackbar(normalizeError(error), 'error')
  } finally {
    loading.execution = false
  }
}

function openLevelDialog(level = null) {
  resetObject(levelForm, {
    id: level?.id || null,
    code: level?.code || '',
    ordinal: level?.ordinal || 1,
    min_balance: level?.min_balance || 0,
    target_planned_score: level?.target_planned_score || 3,
    target_planned_rate: level?.target_planned_rate || 50,
  })
  dialogs.level = true
}

async function saveLevel() {
  loading.level = true
  try {
    if (levelForm.id) {
      await api(`/api/v2/levels/${levelForm.id}`, {
        method: 'PATCH',
        actorId: selectedProfileId.value,
        body: { ...levelForm },
      })
    } else {
      await api(`/api/v2/profiles/${selectedProfileId.value}/levels`, {
        method: 'POST',
        actorId: selectedProfileId.value,
        body: {
          profile_id: selectedProfileId.value,
          ...levelForm,
        },
      })
    }
    dialogs.level = false
    await loadProfileBundle()
    showSnackbar(t('operationDone'))
  } catch (error) {
    showSnackbar(normalizeError(error), 'error')
  } finally {
    loading.level = false
  }
}

function openFinalizationDialog() {
  resetObject(finalizationForm, {
    date: new Date().toISOString().slice(0, 10),
    note: '',
  })
  dialogs.finalization = true
}

async function saveFinalization() {
  loading.finalization = true
  try {
    await api(`/api/v2/profiles/${selectedProfileId.value}/days/${finalizationForm.date}/finalize`, {
      method: 'POST',
      actorId: selectedProfileId.value,
      body: {
        note: finalizationForm.note || null,
      },
    })
    dialogs.finalization = false
    await loadProfileBundle()
    showSnackbar(t('operationDone'))
  } catch (error) {
    showSnackbar(normalizeError(error), 'error')
  } finally {
    loading.finalization = false
  }
}

function openSphereDialog(sphere = null) {
  resetObject(sphereForm, {
    id: sphere?.id || null,
    name: sphere?.name || '',
    weight: sphere?.weight || 1,
  })
  dialogs.sphere = true
}

async function saveSphere() {
  loading.sphere = true
  try {
    if (sphereForm.id) {
      await api(`/api/v2/spheres/${sphereForm.id}`, {
        method: 'PATCH',
        body: {
          name: sphereForm.name,
          weight: sphereForm.weight,
        },
      })
    } else {
      await api('/api/v2/spheres', {
        method: 'POST',
        body: {
          name: sphereForm.name,
          weight: sphereForm.weight,
        },
      })
    }
    dialogs.sphere = false
    await loadSpheres()
    if (selectedProfileId.value) {
      await loadProfileBundle()
    }
    showSnackbar(t('operationDone'))
  } catch (error) {
    showSnackbar(normalizeError(error), 'error')
  } finally {
    loading.sphere = false
  }
}

function openConfirm(message, action) {
  confirmState.message = message
  confirmState.action = action
  confirmState.open = true
}

function closeConfirm() {
  confirmState.open = false
  confirmState.message = ''
  confirmState.action = null
}

async function runConfirm() {
  try {
    if (confirmState.action) {
      await confirmState.action()
    }
  } catch (error) {
    showSnackbar(normalizeError(error), 'error')
  }
  closeConfirm()
}

function confirmDeleteProfile(profile) {
  openConfirm(profile.full_name, async () => {
    await api(`/api/admin/profiles/${profile.id}`, { method: 'DELETE' })
    if (selectedProfileId.value === profile.id) {
      selectedProfileId.value = ''
    }
    await loadProfiles()
    showSnackbar(t('operationDone'))
  })
}

function confirmDeletePhoto(photoId) {
  openConfirm(t('confirmDelete'), async () => {
    await api(`/api/v2/photos/${photoId}`, {
      method: 'DELETE',
      actorId: selectedProfileId.value,
    })
    await loadProfileBundle()
    showSnackbar(t('operationDone'))
  })
}

function confirmDeleteTask(taskId) {
  openConfirm(t('confirmDelete'), async () => {
    await api(`/api/v2/tasks/${taskId}`, {
      method: 'DELETE',
      actorId: selectedProfileId.value,
    })
    await loadProfileBundle()
    showSnackbar(t('operationDone'))
  })
}

function confirmDeleteExecution(executionId) {
  openConfirm(t('confirmDelete'), async () => {
    await api(`/api/v2/executions/${executionId}`, {
      method: 'DELETE',
      actorId: selectedProfileId.value,
    })
    await loadProfileBundle()
    showSnackbar(t('operationDone'))
  })
}

function confirmDeleteLevel(levelId) {
  openConfirm(t('confirmDelete'), async () => {
    await api(`/api/v2/levels/${levelId}`, {
      method: 'DELETE',
      actorId: selectedProfileId.value,
    })
    await loadProfileBundle()
    showSnackbar(t('operationDone'))
  })
}

function confirmDeleteFinalization(finalizationId) {
  openConfirm(t('confirmDelete'), async () => {
    await api(`/api/v2/day-finalizations/${finalizationId}`, {
      method: 'DELETE',
      actorId: selectedProfileId.value,
    })
    await loadProfileBundle()
    showSnackbar(t('operationDone'))
  })
}

function confirmDeleteSphere(sphereId) {
  openConfirm(t('confirmDelete'), async () => {
    await api(`/api/v2/spheres/${sphereId}`, { method: 'DELETE' })
    await loadSpheres()
    if (selectedProfileId.value) {
      await loadProfileBundle()
    }
    showSnackbar(t('operationDone'))
  })
}

function enumLabel(group, value) {
  const map = {
    kind: { positive: t('positive'), negative: t('negative') },
    status: {
      planned: t('planned'),
      archived: t('archived'),
      skipped: t('skipped'),
    },
    cadence: {
      day: t('day'),
      week: t('week'),
      month: t('month'),
      year: t('year'),
    },
  }
  return map[group]?.[value] || value
}

function shortId(value) {
  return value ? value.slice(0, 8) : '-'
}

function taskTitle(taskId) {
  return dashboard.tasks.find((task) => task.id === taskId)?.title || shortId(taskId)
}

function formatDateTime(value) {
  if (!value) {
    return '-'
  }
  return new Date(value).toLocaleString(locale.value)
}

onMounted(async () => {
  await loadSession()
})
</script>

<style scoped>
.toolbar-select {
  max-width: 84px;
  margin-inline: 12px;
}

.mono {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
}
</style>
