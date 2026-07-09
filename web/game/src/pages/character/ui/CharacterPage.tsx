import { useEffect, useMemo, useRef, useState } from 'react'
import type { RefObject } from 'react'

import { useAppStore } from '../../../app/store'
import { CharacterCard, ProfileEditorForm } from '../../../entities/profile'
import {
  CreateProfileForm,
  useCreateProfileMutation,
  useCurrentProfileUpdateMutation,
  useProfileQuery,
  useProfilesQuery,
} from '../../../features/profile'
import {
  ProfilePhotoPanel,
  resolveCurrentProfilePhoto,
  useDeleteCurrentProfilePhoto,
  useCurrentProfilePhotos,
  useProfilePhotoObjectUrl,
  useSelectCurrentProfilePhoto,
  useUploadCurrentProfilePhoto,
} from '../../../features/profile-photo'
import { isApiError } from '../../../shared/api'
import { Badge, Button, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Select } from '../../../shared/ui'

export function CharacterPage() {
  const profileId = useAppStore((state) => state.profileId)
  const setProfileId = useAppStore((state) => state.setProfileId)
  const clearProfileId = useAppStore((state) => state.clearProfileId)
  const [feedback, setFeedback] = useState<string | null>(null)
  const [selectedProfileId, setSelectedProfileId] = useState('')
  const rosterSelectRef = useRef<HTMLSelectElement | null>(null)
  const hasProfile = Boolean(profileId)
  const scopedProfileId = profileId ?? ''
  const profilesQuery = useProfilesQuery({ retry: false })

  const createProfileMutation = useCreateProfileMutation({
    onSuccess: (profile) => {
      setProfileId(profile.id)
      setFeedback('Персонаж создан. Теперь можно донастроить карточку и выбрать аватар.')
    },
  })

  const profileQuery = useProfileQuery(scopedProfileId, {
    enabled: hasProfile,
    retry: false,
  })
  const updateProfileMutation = useCurrentProfileUpdateMutation({
    onSuccess: () => {
      setFeedback('Карточка героя обновлена.')
    },
  })
  const photosQuery = useCurrentProfilePhotos(scopedProfileId, hasProfile && Boolean(profileQuery.data))
  const uploadPhotoMutation = useUploadCurrentProfilePhoto(scopedProfileId)
  const deletePhotoMutation = useDeleteCurrentProfilePhoto(scopedProfileId)
  const selectPhotoMutation = useSelectCurrentProfilePhoto(scopedProfileId)
  const currentPhoto = profileQuery.data ? resolveCurrentProfilePhoto(profileQuery.data, photosQuery.data ?? []) : null
  const currentPhotoAsset = useProfilePhotoObjectUrl(scopedProfileId, currentPhoto)
  const selectedRosterProfile = useMemo(
    () => profilesQuery.data?.find((profile) => profile.id === selectedProfileId) ?? null,
    [profilesQuery.data, selectedProfileId],
  )

  useEffect(() => {
    if (profileId) {
      setSelectedProfileId(profileId)
    }
  }, [profileId])

  useEffect(() => {
    if (!profileId && !selectedProfileId && profilesQuery.data?.length) {
      setSelectedProfileId(profilesQuery.data[0].id)
    }
  }, [profileId, profilesQuery.data, selectedProfileId])

  function openSelectedProfile() {
    const nextProfileId = rosterSelectRef.current?.value || selectedProfileId

    if (!nextProfileId) {
      return
    }

    setFeedback(null)
    setSelectedProfileId(nextProfileId)
    setProfileId(nextProfileId)
  }

  if (!profileId) {
    return (
      <div className="space-y-6">
        <ProfileRosterCard
          currentProfileId={profileId}
          isLoading={profilesQuery.isLoading}
          isError={profilesQuery.isError}
          onCreateAnother={() => {
            setFeedback(null)
            clearProfileId()
          }}
          onOpenSelectedProfile={openSelectedProfile}
          onSelectProfile={setSelectedProfileId}
          profiles={profilesQuery.data ?? []}
          selectRef={rosterSelectRef}
          selectedProfile={selectedRosterProfile}
          selectedProfileId={selectedProfileId}
        />
        <Card className="border-cyan-300/20 bg-white/10 shadow-2xl shadow-cyan-950/30">
          <CardHeader>
            <p className="text-sm font-semibold uppercase tracking-[0.3em] text-cyan-300">Character</p>
            <CardTitle className="mt-3 text-4xl">Create the hero sheet</CardTitle>
            <CardDescription className="mt-4 text-lg">
              Здесь начинается игровой персонаж. Сначала создаём профиль, потом подключаем аватар и готовим основу для
              квестов следующих шагов.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <CreateProfileForm
              isSaving={createProfileMutation.isPending}
              onSubmit={(payload) => createProfileMutation.mutateAsync(payload)}
              submitError={createProfileMutation.error}
            />
          </CardContent>
        </Card>
      </div>
    )
  }

  if (profileQuery.isLoading) {
    return (
      <Card className="border-cyan-300/20 bg-white/10 shadow-2xl shadow-cyan-950/30">
        <CardHeader>
          <p className="text-sm font-semibold uppercase tracking-[0.3em] text-cyan-300">Character</p>
          <CardTitle className="mt-3">Loading hero profile</CardTitle>
          <CardDescription className="mt-4">
            Подтягиваем профиль и готовим живую карточку персонажа вместо пустого placeholder-экрана.
          </CardDescription>
        </CardHeader>
      </Card>
    )
  }

  if (profileQuery.isError || !profileQuery.data) {
    const message = profileQuery.isError
      ? isApiError(profileQuery.error)
        ? profileQuery.error.details?.message ?? profileQuery.error.message
        : profileQuery.error.message
      : 'Profile response returned no data.'

    return (
      <Card className="border-rose-400/30 bg-rose-400/10 shadow-2xl shadow-rose-950/20">
        <CardHeader>
          <p className="text-sm font-semibold uppercase tracking-[0.3em] text-rose-200">Character</p>
          <CardTitle className="mt-3">Profile failed to load</CardTitle>
          <CardDescription className="mt-4 text-rose-100">{message}</CardDescription>
        </CardHeader>
        <CardFooter>
          <Button onClick={() => void profileQuery.refetch()}>Retry</Button>
          <Button variant="ghost" onClick={clearProfileId}>
            Clear profile context
          </Button>
        </CardFooter>
      </Card>
    )
  }

  const busy =
    updateProfileMutation.isPending ||
    uploadPhotoMutation.isPending ||
    selectPhotoMutation.isPending ||
    deletePhotoMutation.isPending

  return (
    <div className="space-y-6">
      <ProfileRosterCard
        currentProfileId={profileId}
        isLoading={profilesQuery.isLoading}
        isError={profilesQuery.isError}
        onCreateAnother={() => {
          setFeedback(null)
          clearProfileId()
        }}
        onOpenSelectedProfile={openSelectedProfile}
        onSelectProfile={setSelectedProfileId}
        profiles={profilesQuery.data ?? []}
        selectRef={rosterSelectRef}
        selectedProfile={selectedRosterProfile}
        selectedProfileId={selectedProfileId}
      />

      <Card className="border-cyan-300/20 bg-white/10 shadow-2xl shadow-cyan-950/30">
        <CardHeader>
          <div className="flex flex-wrap items-start justify-between gap-4">
            <div>
              <p className="text-sm font-semibold uppercase tracking-[0.3em] text-cyan-300">Epic 01</p>
              <CardTitle className="mt-3 text-4xl">Character foundation is live</CardTitle>
              <CardDescription className="mt-4 text-lg">
                Профиль, редактирование и фотогалерея уже собраны в один поток. Следующие игровые экраны смогут
                опираться на эту основу без переписывания маршрута.
              </CardDescription>
            </div>
            <div className="flex flex-wrap gap-3">
              <Badge tone={currentPhotoAsset.url ? 'success' : 'muted'}>{currentPhotoAsset.url ? 'avatar ready' : 'avatar pending'}</Badge>
              <Badge tone={photosQuery.data?.length ? 'success' : 'muted'}>
                {photosQuery.data?.length ? `${photosQuery.data.length} photo(s)` : 'gallery empty'}
              </Badge>
            </div>
          </div>
        </CardHeader>
      </Card>

      {feedback ? (
        <Card className="border-emerald-400/30 bg-emerald-400/10">
          <CardHeader>
            <Badge tone="success">saved</Badge>
            <CardDescription className="mt-2">{feedback}</CardDescription>
          </CardHeader>
        </Card>
      ) : null}

      {photosQuery.isError ? (
        <Card className="border-amber-300/30 bg-amber-300/10">
          <CardHeader>
            <Badge>photo warning</Badge>
            <CardDescription className="mt-2">
              Галерея пока недоступна: {photosQuery.error.message}. Остальная карточка героя продолжает работать.
            </CardDescription>
          </CardHeader>
        </Card>
      ) : null}

      <div className="grid gap-6 xl:grid-cols-[1.1fr,0.9fr]">
        <CharacterCard avatarUrl={currentPhotoAsset.url} profile={profileQuery.data} />
        <ProfileEditorForm
          isSaving={updateProfileMutation.isPending}
          onSubmit={(payload) => updateProfileMutation.mutateAsync(payload)}
          profile={profileQuery.data}
          submitError={updateProfileMutation.error}
        />
      </div>

      <ProfilePhotoPanel
        busy={busy}
        currentPhotoId={profileQuery.data.current_photo_id}
        onDelete={async (photoId) => {
          if (profileQuery.data.current_photo_id === photoId) {
            throw new Error('Сначала выбери другой аватар или загрузить новый. Активное фото backend пока не даёт удалить безопасно.')
          }

          setFeedback(null)
          await deletePhotoMutation.mutateAsync({ photoId })
          setFeedback('Фото удалено из галереи.')
        }}
        onSelect={async (photoId) => {
          setFeedback(null)
          await selectPhotoMutation.mutateAsync({ photoId })
          setFeedback('Аватар обновлён.')
        }}
        onUpload={async (file) => {
          setFeedback(null)
          await uploadPhotoMutation.mutateAsync(file)
          setFeedback('Фото загружено. Теперь его можно выбрать как основной аватар.')
        }}
        photos={photosQuery.data ?? []}
        profileId={profileId}
      />
    </div>
  )
}

function ProfileRosterCard({
  currentProfileId,
  isLoading,
  isError,
  onCreateAnother,
  onOpenSelectedProfile,
  onSelectProfile,
  profiles,
  selectRef,
  selectedProfile,
  selectedProfileId,
}: {
  currentProfileId: string | null
  isLoading: boolean
  isError: boolean
  onCreateAnother: () => void
  onOpenSelectedProfile: () => void
  onSelectProfile: (profileId: string) => void
  profiles: Array<{
    birth_date: string
    created_at: string
    full_name: string
    id: string
    occupation: string
    timezone: string
    updated_at: string
  }>
  selectRef: RefObject<HTMLSelectElement | null>
  selectedProfile: {
    birth_date: string
    created_at: string
    full_name: string
    id: string
    occupation: string
    timezone: string
    updated_at: string
  } | null
  selectedProfileId: string
}) {
  const options = [
    { label: profiles.length ? 'Select a saved hero' : 'No saved heroes yet', value: '' },
    ...profiles.map((profile) => ({
      label: `${profile.full_name} · ${profile.occupation} (${formatProfileRosterTimestamp(profile)})`,
      value: profile.id,
    })),
  ]
  const hasProfiles = profiles.length > 0
  const openButtonLabel = currentProfileId ? 'Open selected hero' : 'Continue with selected hero'

  return (
    <Card className="border-cyan-300/20 bg-white/10 shadow-2xl shadow-cyan-950/20">
      <CardHeader>
        <p className="text-sm font-semibold uppercase tracking-[0.3em] text-cyan-300">Character roster</p>
        <CardTitle className="mt-3 text-3xl">Reopen an existing hero</CardTitle>
        <CardDescription className="mt-4 text-lg">
          Current character context now survives refreshes. You can also reopen any saved hero from the roster instead
          of creating a new one every time.
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <Select
          ref={selectRef}
          label="Saved heroes"
          value={selectedProfileId}
          onChange={(event) => onSelectProfile(event.target.value)}
          options={options}
          hint={
            isLoading
              ? 'Loading saved heroes...'
              : isError
                ? 'Saved heroes list is temporarily unavailable.'
                : hasProfiles
                  ? 'Pick a hero to open their character card.'
                  : 'Create the first hero below and they will appear here.'
          }
        />

        {selectedProfile ? (
          <div className="grid gap-3 rounded-[1.5rem] border border-white/10 bg-slate-950/55 px-4 py-4 sm:grid-cols-3">
            <InfoChip label="Hero" value={selectedProfile.full_name} />
            <InfoChip label="Occupation" value={selectedProfile.occupation} />
            <InfoChip label="Timezone" value={selectedProfile.timezone} />
          </div>
        ) : null}
      </CardContent>
      <CardFooter>
        <Button disabled={!selectedProfileId || !hasProfiles} onClick={onOpenSelectedProfile}>
          {openButtonLabel}
        </Button>
        <Button variant="secondary" onClick={onCreateAnother}>
          Create another
        </Button>
        {currentProfileId ? <Badge tone="success">current hero saved across refresh</Badge> : null}
      </CardFooter>
    </Card>
  )
}

function InfoChip({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-[1.25rem] border border-white/10 bg-slate-900/65 px-4 py-3">
      <p className="text-xs font-semibold uppercase tracking-[0.24em] text-slate-400">{label}</p>
      <p className="mt-2 text-sm font-semibold text-slate-100">{value}</p>
    </div>
  )
}

function formatProfileRosterTimestamp(profile: { created_at: string; updated_at: string }) {
  if (!profile.created_at && !profile.updated_at) {
    return 'дата недоступна'
  }

  const createdAt = Date.parse(profile.created_at)
  const updatedAt = Date.parse(profile.updated_at)

  const createdLabel = Number.isNaN(createdAt) ? profile.created_at : formatRosterDate(createdAt)
  const updatedLabel = Number.isNaN(updatedAt) ? profile.updated_at : formatRosterDate(updatedAt)

  if (profile.created_at === profile.updated_at) {
    return `создан ${createdLabel}`
  }

  return `изменён ${updatedLabel}`
}

function formatRosterDate(timestamp: number) {
  return new Intl.DateTimeFormat('ru-RU', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
  }).format(timestamp)
}
