import { useState } from 'react'

import { useAppStore } from '../../../app/store'
import { CharacterCard, ProfileEditorForm } from '../../../entities/profile'
import { CreateProfileForm, useCreateProfileMutation, useCurrentProfileUpdateMutation, useProfileQuery } from '../../../features/profile'
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
import { Badge, Button, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '../../../shared/ui'

export function CharacterPage() {
  const profileId = useAppStore((state) => state.profileId)
  const setProfileId = useAppStore((state) => state.setProfileId)
  const clearProfileId = useAppStore((state) => state.clearProfileId)
  const [feedback, setFeedback] = useState<string | null>(null)
  const hasProfile = Boolean(profileId)
  const scopedProfileId = profileId ?? ''

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

  if (!profileId) {
    return (
      <div className="space-y-6">
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
