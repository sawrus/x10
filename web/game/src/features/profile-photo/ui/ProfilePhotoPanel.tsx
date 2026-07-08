import { useState } from 'react'

import type { ProfilePhotoSummary } from '../../../entities/profile'
import { Button, Card, CardContent, CardDescription, CardHeader, CardTitle } from '../../../shared/ui'

import { useProfilePhotoObjectUrl } from '../model/hooks'

type ProfilePhotoPanelProps = {
  busy?: boolean
  currentPhotoId: string | null
  onDelete: (photoId: string) => Promise<unknown>
  onSelect: (photoId: string) => Promise<unknown>
  onUpload: (file: File) => Promise<unknown>
  photos: ProfilePhotoSummary[]
  profileId: string
}

function PhotoTile({
  busy,
  currentPhotoId,
  onDelete,
  onSelect,
  photo,
  profileId,
}: {
  busy: boolean
  currentPhotoId: string | null
  onDelete: (photoId: string) => Promise<unknown>
  onSelect: (photoId: string) => Promise<unknown>
  photo: ProfilePhotoSummary
  profileId: string
}) {
  const { error, url } = useProfilePhotoObjectUrl(profileId, photo)
  const isSelected = currentPhotoId === photo.id

  return (
    <div className="space-y-3 rounded-[1.5rem] border border-white/10 bg-slate-900/60 p-4">
      <div className="flex h-40 items-center justify-center overflow-hidden rounded-[1.25rem] bg-slate-950/70">
        {url ? (
          <img alt={photo.original_name} className="h-full w-full object-cover" src={url} />
        ) : (
          <div className="px-4 text-center text-sm text-slate-400">
            {error ? 'Не удалось показать превью. Фото всё равно можно выбрать или удалить.' : 'Загружаем превью...'}
          </div>
        )}
      </div>
      <div>
        <p className="font-semibold text-slate-100">{photo.original_name}</p>
        <p className="mt-1 text-xs uppercase tracking-[0.2em] text-slate-400">{photo.mime_type}</p>
      </div>
      <div className="flex flex-wrap gap-3">
        <Button disabled={busy || isSelected} size="sm" onClick={() => void onSelect(photo.id)}>
          {isSelected ? 'Selected' : 'Use as avatar'}
        </Button>
        <Button disabled={busy} size="sm" variant="ghost" onClick={() => void onDelete(photo.id)}>
          Remove
        </Button>
      </div>
    </div>
  )
}

export function ProfilePhotoPanel({
  busy = false,
  currentPhotoId,
  onDelete,
  onSelect,
  onUpload,
  photos,
  profileId,
}: ProfilePhotoPanelProps) {
  const [actionError, setActionError] = useState<string | null>(null)
  const [uploadError, setUploadError] = useState<string | null>(null)

  return (
    <Card className="rounded-[2rem] bg-slate-900/70">
      <CardHeader>
        <p className="text-sm font-semibold uppercase tracking-[0.3em] text-cyan-300">Profile photos</p>
        <CardTitle className="mt-3 text-3xl">Choose a face for the hero</CardTitle>
        <CardDescription className="mt-4">
          A gentle avatar helps the character feel real. Upload a few options, then keep the one that fits this cycle best.
        </CardDescription>
      </CardHeader>

      <CardContent className="space-y-5">
        <div className="rounded-[1.5rem] border border-dashed border-white/15 bg-slate-950/50 p-4">
          <label className="flex cursor-pointer flex-col gap-3">
            <span className="text-sm font-semibold text-slate-100">Upload image</span>
            <span className="text-sm text-slate-400">PNG, JPG or WEBP. The API expects a multipart field named `file`.</span>
            <input
              accept="image/*"
              className="text-sm text-slate-300 file:mr-4 file:rounded-full file:border-0 file:bg-cyan-300 file:px-4 file:py-2 file:font-semibold file:text-slate-950 hover:file:bg-cyan-200"
              disabled={busy}
              type="file"
              onChange={async (event) => {
                const file = event.target.files?.[0]
                if (!file) {
                  return
                }

                setUploadError(null)

                try {
                  await onUpload(file)
                  setActionError(null)
                } catch (error) {
                  setUploadError(error instanceof Error ? error.message : 'Не удалось загрузить фото.')
                } finally {
                  event.target.value = ''
                }
              }}
            />
          </label>
          {uploadError ? <p className="mt-3 text-sm text-rose-200">{uploadError}</p> : null}
        </div>

        {photos.length ? (
          <div className="grid gap-4 lg:grid-cols-2">
            {photos.map((photo) => (
              <PhotoTile
                key={photo.id}
                busy={busy}
                currentPhotoId={currentPhotoId}
                onDelete={async (photoId) => {
                  try {
                    await onDelete(photoId)
                    setActionError(null)
                  } catch (error) {
                    setActionError(error instanceof Error ? error.message : 'Не удалось удалить фото.')
                  }
                }}
                onSelect={async (photoId) => {
                  try {
                    await onSelect(photoId)
                    setActionError(null)
                  } catch (error) {
                    setActionError(error instanceof Error ? error.message : 'Не удалось выбрать фото.')
                  }
                }}
                photo={photo}
                profileId={profileId}
              />
            ))}
          </div>
        ) : (
          <div className="rounded-[1.5rem] border border-white/10 bg-slate-950/50 px-4 py-5 text-sm text-slate-300">
            Галерея пока пустая. Это не ошибка: можно сначала сохранить героя, а потом спокойно подобрать аватар.
          </div>
        )}
        {actionError ? <p className="text-sm text-rose-200">{actionError}</p> : null}
      </CardContent>
    </Card>
  )
}
